use regionoix::prelude::*;

mod create_checkout_session;
mod stripe_webhook;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_checkout_session::create_checkout_session)
        .service(stripe_webhook::webhook);
}
