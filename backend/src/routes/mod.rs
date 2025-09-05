use actix_web::web;

mod products;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/products").configure(products::config));
}
