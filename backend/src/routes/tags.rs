use crate::{dtos::tag::TagDto, prelude::*};
use actix_web::{HttpResponse, get, web::Data};
use sea_orm::EntityTrait;

use crate::{AppState, dtos::category::CategoryDto};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns tag list",
    tag="Tags",
    responses(
        (
            status=200,
            description="Tag list successfully returned",
            content_type="application/json",
            body=CategoryDto,
            example=json!([{"id": 1, "name": "Vegan"}, {"id": 2, "name": "Végétarien"}]),
        ),
    ),
)]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let tags: Vec<TagDto> = Tag::find().into_dto().all(&db.conn).await?;

    Ok(HttpResponse::Ok().json(tags))
}
