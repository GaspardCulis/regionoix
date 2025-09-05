use actix_web::web;

mod products;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/products").configure(products::config))
        .service(web::scope("/users").configure(users::config));
}
