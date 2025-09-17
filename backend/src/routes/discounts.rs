use regionoix::{dtos::discount::DiscountDto, prelude::*, utils::PaginateQuery};
use sea_orm::EntityTrait as _;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get);
}

#[utoipa::path(
    summary="Returns discounts",
    tag="Discounts",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Discounts successfully returned",
            content_type="application/json",
            body=Vec<DiscountDto>,
        ),
    ),
)]
#[get("")]
pub async fn get(
    query: web::Query<PaginateQuery>,
    data: web::Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let discounts: Vec<DiscountDto> = query
        .paginate(Discount::find().into_dto(), &db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(discounts))
}
