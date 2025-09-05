use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sea_orm::EntityTrait as _;

use crate::{
    AppState,
    entities::{prelude::Product, product},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_by_id);
}

#[get("")]
pub async fn get(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let products: Vec<product::Model> = Product::find()
        .all(db)
        .await
        .expect("Failed to get products");

    HttpResponse::Ok().json(products)
}

#[get("/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &data.db;
    let id: u8 = req.match_info().query("id").parse().unwrap();
    println!("id sent {0}", id);
    let product: Option<product::Model> = Product::find_by_id(id)
        .one(db)
        .await
        .expect(&format!("Failed to get product of id  {}", id));

    if product.is_none() {
        todo!("Throw 404 error")
    }

    HttpResponse::Ok().json(product)
}
