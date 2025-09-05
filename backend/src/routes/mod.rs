use actix_web::web;

mod auth;
mod products;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::config))
        .service(web::scope("/products").configure(products::config));
}
