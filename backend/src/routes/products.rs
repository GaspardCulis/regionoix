use crate::dtos::IntoDto;
use crate::dtos::product::ProductDto;
use crate::prelude::*;
use crate::{
    AppState,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, Region, Tag},
        product, region, tag,
    },
};
use actix_web::{HttpRequest, HttpResponse, get, web::Data};
use sea_orm::{EntityName, EntityTrait as _, ModelTrait};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(get_by_id);
}

#[utoipa::path(
    summary = "Returns product list",
    tag="Products",
    responses(
        (
            status = 200,
            description="Product list successfully returned",
            content_type = "application/json",
            body=product::Model,
            example = json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region_id": 1, "brand_id" : 5, "category_id": null }])
        )
))]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let products: Vec<ProductDto> = Product::find().into_dto().all(db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    summary = "Returns single product",
    tag="Products",
    params (("id" = i32, Path, description = "Product id")),
    responses(
        (
            status = 200,
            description="Product successfully returned",
            content_type = "application/json",
            body=ProductDto,
            example = json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region": {"id":1, "name": "Auvergne", "description": null}, "brand" : {"id": 3, "name": "Jaaj Coorp", "description": null}, "category": {"id": 7, "name": "Confiture", "category_parent": 6} }])
        )))]
#[get("/{id}")]
pub async fn get_by_id(data: Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let product: ProductDto =
        Product::find_by_id(id)
            .into_dto()
            .one(db)
            .await?
            .ok_or(crate::Error::EntityNotFound {
                table_name: product::Entity.table_name(),
            })?;

    Ok(HttpResponse::Ok().json(product))
}
