use crate::prelude::*;
use regionoix::utils::PaginateQuery;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{dtos::category::CategoryDto, prelude::Category};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(get_parents);
}

#[utoipa::path(
    summary="Returns category list",
    tag="Categories",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Product list successfully returned",
            content_type="application/json",
            body=Vec<CategoryDto>,
            example=json!([{"id": 1, "name": "Confitures", "description": null, "category_parent": null}, {"id": 2, "name": "Miels", "description": null,"category_parent": null}]),
        ),
    ),
)]
#[get("")]
pub async fn get(
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
    let categories: Vec<CategoryDto> = query
        .paginate(Category::find().into_dto(), &db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(categories))
}

#[utoipa::path(
    summary="Returns categories with hierarchy",
    tag="Categories",
    responses(
        (
            status=200,
            description="Categories hierarchy successfully returned",
            content_type="application/json",
            body=Vec<CategoryDto>,
            example=json!([{"id": 1, "name": "Epicerie sucrée", "description": null, "childs": [{"id": 2, "name": "Confitures", "description": null, "category_parent" : 1}]}]),
        ),
    ),
)]
#[get("/hierarchy")]
pub async fn get_parents(db: web::Data<DatabaseService>) -> crate::Result<HttpResponse> {
    let mut result: Vec<CategoryDto> = Vec::new();
    let categories_parents: Vec<CategoryDto> = Category::find()
        .filter(category::Column::CategoryParent.is_null())
        .into_dto()
        .all(&db.conn)
        .await?;

    for category in categories_parents {
        result.push(category.finalize(&db.conn).await?);
    }

    Ok(HttpResponse::Ok().json(result))
}
