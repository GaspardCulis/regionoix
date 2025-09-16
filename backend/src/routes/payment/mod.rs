use regionoix::{dtos::order::OrderDto, prelude::*};
use sea_orm::prelude::*;
use stripe::*;

use crate::{AppState, routes::auth::LoggedUser};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(webhook);
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormDataPaymentChechout {
    order_id: i32,
    card_number: String,
    card_exp_month: i32,
    card_exp_year: i32,
    card_cvc: String,
}

#[utoipa::path()]
#[post("/checkout")]
async fn webhook(
    data: web::Data<AppState>,
    form_data: web::Json<FormDataPaymentChechout>,
    // logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let client = &data.stripe.client;

    let order = order::Entity::find_by_id(form_data.order_id)
        .into_dto::<OrderDto>()
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: order::Entity.table_name(),
        })?
        .finalize(&db.conn)
        .await?;

    struct L {
        id: i32,
    }
    let logged_user = L {
        id: order.user_id.unwrap(),
    };

    if order
        .user_id
        .ok_or(anyhow::anyhow!("Failed to get order user id"))?
        != logged_user.id
    {
        return Err(crate::Error::Unauthorized);
    }

    let user = user::Entity::find_by_id(logged_user.id)
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: user::Entity.table_name(),
        })?;

    /*
        let customer_id: CustomerId =
            CustomerId::from_str(format!("{}", user.id).as_str()).expect("valid number");
        let customer = match Customer::retrieve(client, &customer_id, &[]).await {
            Ok(customer) => Ok(customer),
            Err(err) => match err {
              // Check if the error means the customer does not exist
                StripeError::Stripe(err) => match err.error_type {
                    ErrorType::InvalidRequest => {
                        // We sure the customer  does not exist, let's create him
                        Customer::create(
                            client,
                            CreateCustomer {
                                name: Some(&format!(
                                    "{} {}",
                                    user.fistname.as_ref().unwrap_or(&"Unknown".into()),
                                    user.lastname.as_ref().unwrap_or(&"Unknown".into())
                                )),
                                email: Some(&user.email),
                                ..Default::default()
                            },
                        )
                        .await
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
        };
    */

    let customer = Customer::create(
        client,
        CreateCustomer {
            name: Some(&format!(
                "{} {}",
                user.fistname.unwrap_or("Unknown".into()),
                user.lastname.unwrap_or("Unknown".into())
            )),
            email: Some(&user.email),
            ..Default::default()
        },
    )
    .await?;

    info!(
        "created a customer at https://dashboard.stripe.com/test/customers/{}",
        customer.id
    );

    let total_price_cents = (order.total_price * 100.0).round() as i64;
    let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(total_price_cents, Currency::EUR);
        create_intent.payment_method_types = Some(vec!["card".to_string()]);
        // TODO: More info in metadata

        PaymentIntent::create(&client, create_intent).await?
    };

    info!(
        "created a payment intent at https://dashboard.stripe.com/test/payments/{} with status '{}'",
        payment_intent.id, payment_intent.status
    );

    let payment_method = {
        let pm = PaymentMethod::create(
            &client,
            CreatePaymentMethod {
                type_: Some(PaymentMethodTypeFilter::Card),
                card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(
                    CardDetailsParams {
                        number: form_data.card_number.clone(),
                        exp_year: form_data.card_exp_year,
                        exp_month: form_data.card_exp_month,
                        cvc: Some(form_data.card_cvc.clone()),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
        )
        .await?;

        PaymentMethod::attach(
            &client,
            &pm.id,
            AttachPaymentMethod {
                customer: customer.id.clone(),
            },
        )
        .await?;

        pm
    };

    info!(
        "created a payment method with id {} and attached it to {}",
        payment_method.id,
        customer.name.unwrap()
    );

    // lets update the payment intent with their details
    let payment_intent = PaymentIntent::update(
        &client,
        &payment_intent.id,
        UpdatePaymentIntent {
            payment_method: Some(payment_method.id),
            customer: Some(customer.id), // this is not strictly required but good practice to ensure we have the right person
            ..Default::default()
        },
    )
    .await?;

    info!(
        "updated payment intent with status '{}'",
        payment_intent.status
    );

    let payment_intent = PaymentIntent::confirm(
        &client,
        &payment_intent.id,
        PaymentIntentConfirmParams {
            ..Default::default()
        },
    )
    .await?;

    info!(
        "completed payment intent with status {}",
        payment_intent.status
    );

    todo!()
}
