use std::collections::HashMap;

use chrono::{Duration, Utc};
use regionoix::{
    dtos::order::OrderDto,
    prelude::{sea_orm_active_enums::OrderStatus, *},
};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DatabaseTransaction, IntoActiveModel, QueryOrder as _, QuerySelect as _, TransactionTrait as _,
    prelude::*,
};
use stripe::*;

use crate::routes::auth::LoggedUser;

// TODO: Contact DHL
const DELIVERY_DAYS: i64 = 4;
/// Customer has 30min to confirm payment (Stripe's minimum)
const PAYMENT_TIMEOUT_SECS: Duration = Duration::seconds(1800);

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct PostalInfo {
    firstname: String,
    lastname: String,
    city: String,
    country: String,
    postal_code: String,
    street: String,
}
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormDataCreateCheckoutSession {
    postal_info: PostalInfo,
    success_url: String,
    cancel_url: String,
}

#[utoipa::path(
    summary="Creates a Stripe CheckoutSession",
    tag="Payment",
    request_body(content_type="application/json", content=FormDataCreateCheckoutSession),
    responses(
        (
            status=200,
            description="Link to Stripe payment page",
            body=String,
        ),
        // TODO: Document failure cases
    ),
)]
#[post("/create-checkout-session")]
pub async fn create_checkout_session(
    db: web::Data<DatabaseService>,
    stripe: web::Data<StripeService>,
    form_data: web::Json<FormDataCreateCheckoutSession>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let (order, txn) = build_order(&db.conn, &logged_user, &form_data.postal_info).await?;
    let line_items = build_stripe_line_items(&order, &txn, &stripe.client).await?;

    let customer = Customer::create(
        &stripe.client,
        CreateCustomer {
            name: Some(&format!(
                "{} {}",
                logged_user.firstname.unwrap_or("Unknown".into()),
                logged_user.lastname.unwrap_or("Unknown".into())
            )),
            email: Some(&logged_user.email),
            ..Default::default()
        },
    )
    .await?;

    info!(
        "created a customer at https://dashboard.stripe.com/test/customers/{}",
        customer.id
    );

    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.cancel_url = Some(&form_data.cancel_url);
        params.success_url = Some(&form_data.success_url);
        params.customer = Some(customer.id);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(line_items);
        params.expires_at = Some(
            Utc::now()
                .checked_add_signed(PAYMENT_TIMEOUT_SECS)
                .expect("valid time")
                .timestamp(),
        );
        params.metadata = Some(HashMap::from([(
            "order-id".into(),
            format!("{}", order.id).into(),
        )]));

        CheckoutSession::create(&stripe.client, params).await?
    };

    txn.commit().await?;

    Ok(HttpResponse::Ok().body(checkout_session.url.unwrap()))
}

async fn build_order(
    db: &DatabaseConnection,
    user: &LoggedUser,
    postal_info: &PostalInfo,
) -> crate::Result<(order::Model, DatabaseTransaction)> {
    let txn = db.begin().await?;
    // Get cart
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user.id))
        .one(&txn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get cart lines ordered to avoid deadlock
    let cart_lines = cart
        .find_related(cart_line::Entity)
        .order_by_asc(cart_line::Column::ProductId)
        .all(&txn)
        .await?;

    if cart_lines.is_empty() {
        return Err(crate::Error::BadRequestError("Cart is empty".into()));
    }

    let mut total_price = 0.0;
    let mut order_lines = Vec::new();

    // Create Order lines
    for cl in cart_lines {
        // Lock on the read of product to avoid dirty read
        let (product, discount) = product::Entity::find_by_id(cl.product_id)
            .lock_exclusive()
            .find_also_related(discount::Entity)
            .one(&txn)
            .await?
            .ok_or(crate::Error::EntityNotFound {
                table_name: product::Entity.table_name(),
            })?;

        // Order line
        let ol = order_line::ActiveModel {
            id: NotSet,
            quantity: Set(cl.quantity),
            unit_price: Set(product.price),
            product_id: Set(product.id.into()),
            order_id: NotSet,
        };
        order_lines.push(ol);

        // Total
        total_price += if let Some(discount) = discount {
            product.price * cl.quantity as f32 * (1.0 - discount.percentage_off as f32 / 100.0)
        } else {
            product.price * cl.quantity as f32
        };

        // if product stock is not enough rollback
        if product.stock < cl.quantity {
            txn.rollback().await?;
            return Err(crate::Error::BadRequestError("Not enough stock".into()));
        }

        // Decrement stock
        let mut product_am = product.into_active_model();
        product_am.stock = Set(product_am.stock.unwrap() - cl.quantity);
        product_am.update(&txn).await?;
    }

    // Create address
    let addr = address::ActiveModel {
        id: NotSet,
        firstname: Set(postal_info.firstname.to_owned()),
        lastname: Set(postal_info.lastname.to_owned()),
        city: Set(postal_info.city.to_owned()),
        country: Set(postal_info.country.to_owned()),
        street: Set(postal_info.street.to_owned()),
        postal_code: Set(postal_info.postal_code.to_owned()),
    };
    let addr = addr.insert(&txn).await?;

    // Order dates
    let today = Utc::now().date_naive();
    let arrival = (Utc::now() + Duration::days(DELIVERY_DAYS)).date_naive();

    // Create command commande
    let order = order::ActiveModel {
        id: NotSet,
        total_price: Set(total_price.to_owned()),
        status: Set(OrderStatus::PendingPayment),
        creation_date: Set(today.into()),
        arrival_date: Set(arrival.into()),
        user_id: Set(user.id.into()),
        adress_id: Set(addr.id.into()),
        ..Default::default()
    };
    let order = order.insert(&txn).await?;

    // For all order_lines insert order ids
    for mut ol in order_lines {
        ol.order_id = Set(order.id.into());
        ol.insert(&txn).await?;
    }

    // Empty cart
    cart_line::Entity::delete_many()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .exec(&txn)
        .await?;

    Ok((order, txn))
}

async fn build_stripe_line_items(
    order: &order::Model,
    txn: &DatabaseTransaction,
    client: &Client,
) -> crate::Result<Vec<CreateCheckoutSessionLineItems>> {
    let order_dto = order::Entity::find_by_id(order.id)
        .into_dto::<OrderDto>()
        .one(txn)
        .await?
        .expect("has just been built")
        .finalize(txn)
        .await?;

    let mut line_items = Vec::new();
    for order_line in order_dto.order_lines.unwrap_or_default().iter() {
        let db_product = &order_line.product;
        let product_price_cents = if let Some(discount) = &db_product.discount {
            ((db_product.price * (1.0 - discount.percentage_off as f32 / 100.0)) * 100.0).round()
                as i64
        } else {
            (db_product.price * 100.0).round() as i64
        };

        let product = {
            let mut create_product = CreateProduct::new(&db_product.name);
            create_product.images = db_product.image.clone().map(|image| vec![image]);
            stripe::Product::create(&client, create_product).await?
        };

        let price = {
            let mut create_price = CreatePrice::new(Currency::EUR);
            create_price.product = Some(IdOrCreate::Id(&product.id));
            create_price.unit_amount = Some(product_price_cents);
            create_price.expand = &["product"];
            Price::create(&client, create_price).await?
        };

        let line_item = CreateCheckoutSessionLineItems {
            quantity: Some(order_line.quantity as u64),
            price: Some(price.id.to_string()),
            ..Default::default()
        };
        line_items.push(line_item);
    }

    Ok(line_items)
}
