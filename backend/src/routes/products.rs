use actix_web::{HttpResponse, Responder, get, web};
use sea_orm::EntityTrait as _;

use crate::{
    AppState,
    entities::{prelude::Product, product},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/")]
pub async fn get(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let products: Vec<product::Model> = Product::find()
        .all(db)
        .await
        .expect("Failed to get products");

    HttpResponse::Ok().json(products)
}
