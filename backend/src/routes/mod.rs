use crate::prelude::*;

mod auth;
mod products;
mod users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/products").configure(products::config))
        .service(scope("/users").configure(users::config));
}
