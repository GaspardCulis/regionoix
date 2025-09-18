use crate::{dtos::region::RegionDto, prelude::*};
use regionoix::utils::PaginateQuery;
use sea_orm::EntityTrait;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns region list",
    tag="Regions",
    params(PaginateQuery),
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
pub async fn get(
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
    let regions: Vec<RegionDto> = query.paginate(Region::find().into_dto(), &db.conn).await?;

    Ok(HttpResponse::Ok().json(regions))
}
