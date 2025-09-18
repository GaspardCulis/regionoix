use crate::dtos::product::ProductDto;
use crate::entities::{prelude::Product, product};
use crate::prelude::*;
use regionoix::utils::PaginateQuery;
use sea_orm::ColumnTrait;
use sea_orm::{EntityName, EntityTrait as _, QueryFilter};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get)
        .service(get_discounts)
        .service(get_by_id)
        .service(delete_by_id);
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
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
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
pub async fn get_by_id(
    req: HttpRequest,
    db: web::Data<DatabaseService>,
) -> crate::Result<HttpResponse> {
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
    summary="Returns product with discount list",
    tag="Products",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Product list with discount successfully returned",
            content_type="application/json",
            body=Vec<ProductDto>,
            example=json!([{"id": 1, "name": "Confiture du triève", "description": "Super confiture", "weight": 0.600, "price" : 5.80, "image" : "/product1.jpg", "stock":10, "region_id": 1, "brand_id" : 5, "category_id": null, "discount": {"id":1, "percentage_off":20, "end_date": "" } }]),
        ),
    ),
)]
#[get("/discounts")]
pub async fn get_discounts(
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
    let products: Vec<ProductDto> = query
        .paginate(
            Product::find()
                .filter(product::Column::DiscountId.is_not_null())
                .into_dto(),
            &db.conn,
        )
        .await?;

    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    summary="Delete product by id",
    description="Product and its orders are deleted if product does not appear in carts or orders. Please refer to database schema to learn on cascade actions.",
    tag="Products",
    params(("id" = i32, Path, description = "Product id")),
    responses(
        (
            status=200,
            description="Product successfully deleted",
            body=String
        ),
         (
            status=409,
            description="Product could not be deleted because of its presence in orders or carts",
        ),
    ),
)]
#[delete("/{id}")]
pub async fn delete_by_id(
    req: HttpRequest,
    db: web::Data<DatabaseService>,
) -> crate::Result<HttpResponse> {
    let id: i32 = req.match_info().query("id").parse()?;

    // Check if product exists in cart lines or order lines
    // Throw error if product in cart lines or order lines
    let cart_lines = cart_line::Entity::find()
        .filter(cart_line::Column::ProductId.eq(id))
        .all(&db.conn)
        .await?;

    if !cart_lines.is_empty() {
        return Err(crate::Error::Conflict);
    }
    let order_lines = order_line::Entity::find()
        .filter(order_line::Column::ProductId.eq(id))
        .all(&db.conn)
        .await?;

    if !order_lines.is_empty() {
        return Err(crate::Error::Conflict);
    }

    Product::delete_by_id(id).exec(&db.conn).await?;

    Ok(HttpResponse::Ok().body("Product successfully deleted"))
}
