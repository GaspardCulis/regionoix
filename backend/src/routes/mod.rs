use crate::prelude::*;

pub mod admin;
pub mod auth;
pub mod basket;
pub mod brands;
pub mod categories;
pub mod discounts;
pub mod health;
pub mod orders;
pub mod payment;
pub mod products;
pub mod regions;
pub mod search;
pub mod tags;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/admin").configure(admin::config))
        .service(scope("/auth").configure(auth::config))
        .service(scope("/basket").configure(basket::config))
        .service(scope("/brands").configure(brands::config))
        .service(scope("/categories").configure(categories::config))
        .service(scope("/health").configure(health::config))
        .service(scope("/search").configure(search::config))
        .service(scope("/products").configure(products::config))
        .service(scope("/regions").configure(regions::config))
        .service(scope("/orders").configure(orders::config))
        .service(scope("/payment").configure(payment::config))
        .service(scope("/tags").configure(tags::config));
        .service(scope("/discounts").configure(discounts::config));
}
