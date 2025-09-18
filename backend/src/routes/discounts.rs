use regionoix::{dtos::discount::DiscountDto, prelude::*, utils::PaginateQuery};
use sea_orm::EntityTrait as _;

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
    db: web::Data<DatabaseService>,
    query: web::Query<PaginateQuery>,
) -> crate::Result<HttpResponse> {
    let discounts: Vec<DiscountDto> = query
        .paginate(Discount::find().into_dto(), &db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(discounts))
}
