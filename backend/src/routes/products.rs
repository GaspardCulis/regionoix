use crate::{
    AppState,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, ProductCategory, Region},
        product, region,
    },
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sea_orm::{EntityTrait as _, LoaderTrait, ModelTrait};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_by_id);
    cfg.service(get_by_id_expand);
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
        .expect(&format!("Failed to get product of id {}", id));

    if product.is_none() {
        todo!("Throw 404 error")
    }

    HttpResponse::Ok().json(product)
}

#[get("/{id}/expand")]
pub async fn get_by_id_expand(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &data.db;
    let id: u8 = req.match_info().query("id").parse().unwrap();
    println!("id sent {0}", id);
    let product = Product::find_by_id(id)
        .one(db)
        .await
        .expect(&format!("Failed to get product of id {}", id));

    if product.is_none() {
        todo!("Throw 404 error")
    } else {
        let product = product.unwrap();

        let region: Option<region::Model> = product
            .find_related(Region)
            .one(db)
            .await
            .expect("Failed to get region product");

        let brand: Option<brand::Model> = product
            .find_related(Brand)
            .one(db)
            .await
            .expect("Failed to get brand product");

        let categories_products = product
            .find_related(ProductCategory)
            .all(db)
            .await
            .expect("Failed to get category product");

        let categories = categories_products
            .load_one(Category, db)
            .await
            .expect("Failed to get categories product");

        #[derive(serde::Serialize)]
        struct ProductExpanded {
            product: product::Model,
            region: Option<region::Model>,
            brand: Option<brand::Model>,
            categories: Vec<Option<category::Model>>,
        }

        let response = ProductExpanded {
            product,
            region,
            brand,
            categories,
        };

        HttpResponse::Ok().json(response)
    }
}
