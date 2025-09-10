use crate::dtos::product::ProductDto;
use crate::prelude::*;
use crate::{
    AppState,
    entities::{prelude::Product, product},
};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, get, web::Data};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{ExprTrait, Func};
use sea_orm::{Condition, EntityName, EntityTrait as _, QueryFilter};
use utoipa::IntoParams;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(search).service(get_by_id);
}

#[utoipa::path(
    summary="Returns product list",
    tag="Products",
    responses(
        (
            status=200,
            description="Product list successfully returned",
            content_type="application/json",
            body=product::Model,
            example=json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region_id": 1, "brand_id" : 5, "category_id": null }]),
        ),
    ),
)]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let products: Vec<ProductDto> = Product::find().into_dto().all(db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    summary="Returns single product",
    tag="Products",
    params(("id" = i32, Path, description = "Product id")),
    responses(
        (
            status=200,
            description="Product successfully returned",
            content_type="application/json",
            body=ProductDto,
            example = json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region": {"id":1, "name": "Auvergne", "description": null}, "brand" : {"id": 3, "name": "Jaaj Coorp", "description": null}, "category": {"id": 7, "name": "Confiture", "category_parent": 6} }]),
        ),
    ),
)]
#[get("/{id}")]
pub async fn get_by_id(data: Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .into_dto::<ProductDto>()
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?
        .finalize(db)
        .await?;

    Ok(HttpResponse::Ok().json(product))
}

#[derive(Deserialize, Serialize, ToSchema, IntoParams)]
struct SearchQuery {
    name: Option<String>,
}

#[utoipa::path(
    summary="Returns products with name matching",
    tag="Products",
    params(SearchQuery),
    responses(
        (
            status=200,
            description="Products successfully returned",
            content_type="application/json",
            body=[ProductDto],
            example = json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region": {"id":1, "name": "Auvergne", "description": null}, "brand" : {"id": 3, "name": "Jaaj Coorp", "description": null}, "category": {"id": 7, "name": "Confiture", "category_parent": 6} }]),
        ),
    ),
)]
#[get("/search")]
pub async fn search(
    data: Data<AppState>,
    search_query: web::Query<SearchQuery>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let mut query = Product::find();

    if let Some(name) = &search_query.name {
        let pattern = format!("%{}%", name.to_lowercase());
        let cond =
            Condition::all().add(Func::lower(Expr::col(product::Column::Name)).like(pattern));
        query = query.filter(cond);
    }

    let products = query.all(db).await?;

    Ok(HttpResponse::Ok().json(products))
}
