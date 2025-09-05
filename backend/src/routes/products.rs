use crate::{
    AppState,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, ProductCategory, Region},
        product, region,
    },
};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet, Set},
    EntityName, EntityTrait as _, LoaderTrait, ModelTrait,
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
pub async fn get(data: web::Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let products: Vec<product::Model> = Product::find().all(db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[get("/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: u8 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?;

    Ok(HttpResponse::Ok().json(product))
}

#[get("/{id}/expand")]
pub async fn get_by_id_expand(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: u8 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?;

    let region = product.find_related(Region).one(db).await?;
    let brand = product.find_related(Brand).one(db).await?;
    let categories_products = product.find_related(ProductCategory).all(db).await?;
    let categories = categories_products.load_one(Category, db).await?;

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

    Ok(HttpResponse::Ok().json(response))
}

#[post("")]
pub async fn create(
    data: web::Data<AppState>,
    form_data: web::Json<product::Model>,
) -> crate::Result<HttpResponse> {
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
    .await?;

    Ok(HttpResponse::Ok().body("Product succesfully created"))
}

#[delete("/{id}")]
pub async fn delete_by_id(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id = req.match_info().query("id").parse()?;

    let product = product::ActiveModel {
        id: ActiveValue::Set(id),
        ..Default::default()
    };
    product.delete(db).await?;

    Ok(HttpResponse::Ok().body("Product succesfully deleted"))
}

#[put("/{id}")]
pub async fn update_by_id(
    data: web::Data<AppState>,
    req: HttpRequest,
    form_data: web::Json<product::Model>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?;

    // Into ActiveModel
    let mut product: product::ActiveModel = product.into();

    product.name = Set(form_data.name.to_owned());
    product.description = Set(form_data.description.to_owned());
    product.weight = Set(form_data.weight.to_owned());
    product.price = Set(form_data.price.to_owned());
    product.brand_id = Set(form_data.brand_id.to_owned());
    product.image = Set(form_data.image.to_owned());
    product.stock = Set(form_data.stock.to_owned());
    product.region_id = Set(form_data.region_id.to_owned());

    product.update(db).await?;

    Ok(HttpResponse::Ok().body("Product succesfully updated"))
}
