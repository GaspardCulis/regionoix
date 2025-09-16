use regionoix::prelude::*;

mod create_checkout_session;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_checkout_session::create_checkout_session);
}
