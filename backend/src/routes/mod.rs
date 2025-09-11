use crate::prelude::*;

pub mod auth;
pub mod basket;
pub mod categories;
pub mod products;
pub mod regions;
pub mod search;
pub mod tags;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/products").configure(products::config))
        .service(scope("/search").configure(search::config))
        .service(scope("/basket").configure(basket::config))
        .service(scope("/categories").configure(categories::config))
        .service(scope("/tags").configure(tags::config))
        .service(scope("/regions").configure(regions::config));
}
