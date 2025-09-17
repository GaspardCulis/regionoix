use chrono::{Duration, Utc};
use regionoix::prelude::{sea_orm_active_enums::OrderStatus, *};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    QueryOrder as _, QuerySelect as _, TransactionTrait as _,
    prelude::*,
};

use crate::routes::auth::LoggedUser;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormDataMakeOrder {
    firstname: String,
    lastname: String,
    city: String,
    country: String,
    postal_code: String,
    street: String,
}

#[utoipa::path(
    summary = "Make order from current user's basket",
    tag="Basket",
    request_body(content_type = "Application/Json",
    content = FormDataMakeOrder),
    responses(
        (
            status = 200,
            description="Order successfully created",
            body=String,
        ),
    ),
)]
#[post("/order")]
async fn make(
    db: web::Data<DatabaseService>,
    form_data: web::Json<FormDataMakeOrder>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    const DELIVERY_DAYS: i64 = 4;
    let txn = db.begin().await?;

    // Get cart
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&txn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get cart lines ordered to avoid deadlock
    let cart_lines: Vec<cart_line::Model> = cart
        .find_related(cart_line::Entity)
        .order_by_asc(cart_line::Column::ProductId)
        .all(&txn)
        .await?;

    if cart_lines.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Cart is empty"));
    }

    let mut total_price = 0.0;
    let mut order_lines = Vec::new();

    // Create Order lines
    for cl in cart_lines {
        // Lock on the read of product to avoid dirty read
        let product = product::Entity::find_by_id(cl.product_id)
            .lock_exclusive()
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
        total_price += product.price * cl.quantity as f32;

        // if product stock is not enough rollback
        if product.stock < cl.quantity {
            txn.rollback().await?;
            return Ok(HttpResponse::InternalServerError().body("Not enough stock"));
        }

        // Decrement stock
        let mut product_am: product::ActiveModel = product.into();
        product_am.stock = Set(product_am.stock.unwrap() - cl.quantity);
        product_am.update(&txn).await?;
    }

    // Create address
    let addr = address::ActiveModel {
        id: NotSet,
        firstname: Set(form_data.firstname.to_owned()),
        lastname: Set(form_data.lastname.to_owned()),
        city: Set(form_data.city.to_owned()),
        country: Set(form_data.country.to_owned()),
        street: Set(form_data.street.to_owned()),
        postal_code: Set(form_data.postal_code.to_owned()),
    };
    let addr = addr.insert(&txn).await?;

    // Order dates
    let today = Utc::now().date_naive();
    let arrival = (Utc::now() + Duration::days(DELIVERY_DAYS)).date_naive();

    // Create command commande
    let order = order::ActiveModel {
        id: NotSet,
        total_price: Set(total_price.to_owned()),
        status: Set(OrderStatus::Payed),
        creation_date: Set(today.into()),
        arrival_date: Set(arrival.into()),
        user_id: Set(logged_user.id.into()),
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

    txn.commit().await?;

    Ok(HttpResponse::Ok().body("Order successfully created"))
}
