use crate::{dtos::region::RegionDto, prelude::*};
use actix_web::{HttpResponse, get, web::Data};
use sea_orm::EntityTrait;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns region list",
    tag="Regions",
    responses(
        (
            status=200,
            description="Region list successfully returned",
            content_type="application/json",
            body=Vec<RegionDto>,
            example=json!([{"id": 1, "name": "Auvergne-Rhône-Alpes", "description": null}, {"id": 2, "name": "Grand Est", "description": null}]),
        ),
    ),
)]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let regions: Vec<RegionDto> = Region::find().into_dto().all(db).await?;

    Ok(HttpResponse::Ok().json(regions))
}
