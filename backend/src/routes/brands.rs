use regionoix::{dtos::brand::BrandDto, prelude::*};
use sea_orm::EntityTrait as _;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns brands",
    tag="Brands",
    responses(
        (
            status=200,
            description="Brands successfully returned",
            content_type="application/json",
            body=Vec<BrandDto>,
            example=json!([{"id": 1, "name": "Jaaj Corp", "description": null}, {"id": 2, "name": "Isère confitures", "description": null}]),
        ),
    ),
)]
#[get("")]
pub async fn get(data: web::Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let brands: Vec<BrandDto> = Brand::find().into_dto().all(&db.conn).await?;

    Ok(HttpResponse::Ok().json(brands))
}
