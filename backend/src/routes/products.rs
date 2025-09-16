use crate::dtos::product::ProductDto;
use crate::prelude::*;
use crate::{
    AppState,
    entities::{prelude::Product, product},
};
use regionoix::utils::PaginateQuery;
use sea_orm::{EntityName, EntityTrait as _};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(get_by_id).service(delete_by_id);
}

#[utoipa::path(
    summary="Returns product list",
    tag="Products",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Product list successfully returned",
            content_type="application/json",
            body=Vec<ProductDto>,
            example=json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region_id": 1, "brand_id" : 5, "category_id": null, "discount": null }]),
        ),
    ),
)]
#[get("")]
pub async fn get(
    query: web::Query<PaginateQuery>,
    data: web::Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let products: Vec<ProductDto> = query.paginate(Product::find().into_dto(), &db.conn).await?;

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
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .into_dto::<ProductDto>()
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?
        .finalize(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path(
    summary="Delete product by id",
    description="Product and its orders are deleted. Please refer to database schema to learn on cascade actions.",
    tag="Products",
    params(("id" = i32, Path, description = "Product id")),
    responses(
        (
            status=200,
            description="Product successfully deleted",
            body=String
        ),
    ),
)]
#[delete("/{id}")]
pub async fn delete_by_id(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    Product::delete_by_id(id).exec(&db.conn).await?;

    Ok(HttpResponse::Ok().body("Product successfully deleted"))
}
