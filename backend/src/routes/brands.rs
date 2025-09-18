use regionoix::{dtos::brand::BrandDto, prelude::*, utils::PaginateQuery};
use sea_orm::EntityTrait as _;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns brands",
    tag="Brands",
    params(PaginateQuery),
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
pub async fn get(
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
    let brands: Vec<BrandDto> = query.paginate(Brand::find().into_dto(), &db.conn).await?;

    Ok(HttpResponse::Ok().json(brands))
}
