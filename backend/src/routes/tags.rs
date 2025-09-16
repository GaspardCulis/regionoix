use crate::{dtos::tag::TagDto, prelude::*};
use regionoix::utils::PaginateQuery;
use sea_orm::EntityTrait;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns tag list",
    tag="Tags",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Tag list successfully returned",
            content_type="application/json",
            body=Vec<TagDto>,
            example=json!([{"id": 1, "name": "Vegan"}, {"id": 2, "name": "Végétarien"}]),
        ),
    ),
)]
#[get("")]
pub async fn get(
    query: web::Query<PaginateQuery>,
    data: web::Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let tags: Vec<TagDto> = query.paginate(Tag::find().into_dto(), &db.conn).await?;

    Ok(HttpResponse::Ok().json(tags))
}
