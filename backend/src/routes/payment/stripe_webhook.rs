use regionoix::{
    prelude::{sea_orm_active_enums::OrderStatus, *},
    utils::get_header_value,
};
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, IntoActiveModel as _, QueryOrder, QuerySelect,
    TransactionTrait as _, prelude::*,
};
use stripe::*;
use tracing::error;

#[utoipa::path(summary = "Stripe reserved webhooks endpoint", tag = "Payment")]
#[post("/stripe-webhooks")]
pub async fn webhook(
    req: HttpRequest,
    payload: web::Payload,
    db: web::Data<DatabaseService>,
    stripe: web::Data<StripeService>,
) -> crate::Result<HttpResponse> {
    let payload = payload
        .to_bytes()
        .await
        .map_err(|_| crate::Error::BadRequestError("Failed to get payload bytes".into()))?;
    let event = build_webhook(&req, &payload, &stripe)
        .map_err(|_| crate::Error::BadRequestError("Failed to build stripe webhook".into()))?;

    match event.type_ {
        EventType::CheckoutSessionCompleted | EventType::CheckoutSessionExpired => {
            let EventObject::CheckoutSession(session) = event.data.object else {
                error!("Couldn't pattern-match checkout-session, shouldn't happen");
                return Err(crate::Error::BadRequestError(
                    "Couldn't retrieve checkout-session from request".into(),
                ));
            };

            info!("Received {} webhook with id: {:?}", event.type_, session.id);

            let order_id = session
                .metadata
                .ok_or(crate::Error::BadRequestError("Missing metadata".into()))?
                .get("order-id")
                .ok_or(crate::Error::BadRequestError(
                    "Missing 'order-id' metadata key".into(),
                ))?
                .parse::<i32>()?;

            let txn = db.begin().await?;
            let order = order::Entity::find_by_id(order_id).one(&txn).await?.ok_or(
                crate::Error::EntityNotFound {
                    table_name: order::Entity.table_name(),
                },
            )?;

            match event.type_ {
                EventType::CheckoutSessionCompleted => handle_successful_payment(order, &txn).await,
                EventType::CheckoutSessionExpired => handle_expired_payment(order, &txn).await,
                _ => unimplemented!("cannot happen"),
            }?;

            txn.commit().await?;
            Ok(HttpResponse::Ok().finish())
        }
        e => {
            info!("Received unknown Stripe webhook event type: {}", e);
            // Let it pass
            Ok(HttpResponse::Ok().finish())
        }
    }
}

/// Updates order status to Payed
async fn handle_successful_payment(
    order: order::Model,
    txn: &DatabaseTransaction,
) -> crate::Result<()> {
    if order.status != OrderStatus::PendingPayment {
        return Err(crate::Error::BadRequestError("order already payed".into()));
    }
    // Update order status
    let mut order_am = order.into_active_model();
    order_am.status = Set(OrderStatus::Payed);
    order_am.update(txn).await?;

    info!("order status updated");
    Ok(())
}

/// Re-stocks reserved products
async fn handle_expired_payment(
    order: order::Model,
    txn: &DatabaseTransaction,
) -> crate::Result<()> {
    // Get order lines ordered by product to avoid deadlock
    let order_lines = order
        .find_related(order_line::Entity)
        .find_also_related(product::Entity)
        .order_by_asc(order_line::Column::ProductId)
        .lock_exclusive()
        .all(txn)
        .await?;

    // Re-increment stock
    for (line, product) in order_lines.into_iter() {
        let product = product.ok_or(crate::Error::InternalError(anyhow::anyhow!(
            "Failed to find order-line product"
        )))?;

        let mut product_am: product::ActiveModel = product.into();
        product_am.stock = Set(product_am.stock.unwrap() + line.quantity);
        product_am.update(txn).await?;
    }

    // Delete expired order
    order.into_active_model().delete(txn).await?;

    info!("stock updated");
    Ok(())
}

fn build_webhook(
    req: &HttpRequest,
    payload: &web::Bytes,
    stripe: &StripeService,
) -> Result<Event, WebhookError> {
    let payload_str = std::str::from_utf8(payload).unwrap();

    let stripe_signature = get_header_value(req, "Stripe-Signature").unwrap_or_default();

    Webhook::construct_event(payload_str, stripe_signature, &stripe.webhook_signing_key)
}
