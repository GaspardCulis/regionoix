use crate::prelude::*;

pub mod auth;
pub mod basket;
pub mod products;
pub mod search;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/products").configure(products::config))
        .service(scope("/search").configure(search::config))
        .service(scope("/basket").configure(basket::config));
}
