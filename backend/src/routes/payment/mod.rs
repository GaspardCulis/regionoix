use regionoix::{prelude::*, utils::get_header_value};
use stripe::{Event, EventObject, EventType, Webhook, WebhookError};

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(webhook);
}

#[utoipa::path()]
#[post("/webhook")]
async fn webhook(req: HttpRequest, payload: web::Payload) -> crate::Result<HttpResponse> {
    let payload = payload
        .to_bytes()
        .await
        .map_err(|_| crate::Error::BadRequestError("Failed to get payload bytes".into()))?;
    let event = build_webhook(&req, &payload)
        .map_err(|_| crate::Error::BadRequestError("Failed to build stripe webhook".into()))?;

    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            handle_successful_payment(event);
            println!("Stonks");
        }
        EventType::PaymentIntentPaymentFailed => {
            println!("bru");
        }
        _ => todo!(),
    };

    todo!()
}

fn handle_successful_payment(event: Event) {
    if let EventObject::CheckoutSession(session) = event.data.object {
        todo!()
    }
}

fn build_webhook(req: &HttpRequest, payload: &web::Bytes) -> Result<Event, WebhookError> {
    let app_data: &AppState = req.app_data().expect("app state intialized");
    let payload_str = std::str::from_utf8(payload).unwrap();

    let stripe_signature = get_header_value(req, "Stripe-Signature").unwrap_or_default();

    Webhook::construct_event(payload_str, stripe_signature, &app_data.stripe.api_key)
}
