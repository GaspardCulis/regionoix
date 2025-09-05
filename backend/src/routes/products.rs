use crate::{
    AppState,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, ProductCategory, Region},
        product, region,
    },
};
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet, Set},
    EntityTrait as _, LoaderTrait, ModelTrait,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get)
        .service(get_by_id)
        .service(get_by_id_expand)
        .service(create)
        .service(delete_by_id)
        .service(update_by_id);
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

#[post("")]
pub async fn create(
    data: web::Data<AppState>,
    form_data: web::Json<product::Model>,
) -> impl Responder {
    let db = &data.db;
    let form_data = form_data.into_inner();

    product::ActiveModel {
        id: NotSet,
        name: Set(form_data.name),
        description: Set(form_data.description.to_owned()),
        weight: Set(form_data.weight.to_owned()),
        price: Set(form_data.price.to_owned()),
        brand_id: Set(form_data.brand_id.to_owned()),
        image: Set(form_data.image.to_owned()),
        stock: Set(form_data.stock.to_owned()),
        region_id: Set(form_data.region_id.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("Failed to save new product");

    HttpResponse::Ok().body("Product succesfully created")
}

#[delete("/{id}")]
pub async fn delete_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &data.db;
    let id = req.match_info().query("id").parse().unwrap();
    println!("id sent {0}", id);

    let product = product::ActiveModel {
        id: ActiveValue::Set(id),
        ..Default::default()
    };
    product
        .delete(db)
        .await
        .expect(&format!("Failed to delete product of id {}", id));

    HttpResponse::Ok().body("Product succesfully deleted")
}

#[put("/{id}")]
pub async fn update_by_id(
    data: web::Data<AppState>,
    req: HttpRequest,
    form_data: web::Json<product::Model>,
) -> impl Responder {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse().unwrap();
    println!("id sent {0}", id);

    let product: Option<product::Model> = Product::find_by_id(id)
        .one(db)
        .await
        .expect(&format!("Failed to get product of id {}", id));

    // Into ActiveModel
    let mut product: product::ActiveModel = product.unwrap().into();

    product.name = Set(form_data.name.to_owned());
    product.description = Set(form_data.description.to_owned());
    product.weight = Set(form_data.weight.to_owned());
    product.price = Set(form_data.price.to_owned());
    product.brand_id = Set(form_data.brand_id.to_owned());
    product.image = Set(form_data.image.to_owned());
    product.stock = Set(form_data.stock.to_owned());
    product.region_id = Set(form_data.region_id.to_owned());

    product.update(db).await.expect("Failed to update product");

    HttpResponse::Ok().body("Product succesfully updated")
}
