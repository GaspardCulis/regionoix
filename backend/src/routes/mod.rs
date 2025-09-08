use crate::prelude::*;

mod auth;
mod basket;
mod products;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/products").configure(products::config))
        .service(scope("/basket").configure(basket::config));
}
