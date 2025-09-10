use crate::prelude::*;
use actix_web::{HttpResponse, get, web::Data};
use sea_orm::EntityTrait;

use crate::{AppState, dtos::category::CategoryDto, prelude::Category};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns category list",
    tag="Categories",
    responses(
        (
            status=200,
            description="Product list successfully returned",
            content_type="application/json",
            body=CategoryDto,
            example=json!([{"id": 1, "name": "Confitures", "description": null, "category_parent": null}, {"id": 2, "name": "Miels", "description": null,"category_parent": null}]),
        ),
    ),
)]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let categories: Vec<CategoryDto> = Category::find().into_dto().all(db).await?;

    Ok(HttpResponse::Ok().json(categories))
}
