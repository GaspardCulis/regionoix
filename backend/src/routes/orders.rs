use crate::AppState;
use crate::dtos::order::OrderDto;
use crate::prelude::*;
use actix_web::{HttpResponse, get, web::Data};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait as _, QueryFilter};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_in_progress);
}

#[utoipa::path(
    summary="Returns order list in progress",
    tag="Orders in progress for current user",
    responses(
        (
            status=200,
            description="Order list in progress successfully returned",
            content_type="application/json",
            body=[OrderDto],
        ),
    ),
)]
#[get("/in-progress")]
pub async fn get_in_progress(
    data: Data<AppState>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let today = Utc::now().date_naive();

    let orders: Vec<OrderDto> = Order::find()
        .filter(order::Column::UserId.eq(logged_user.id))
        .filter(order::Column::ArrivalDate.gte(today))
        .into_dto()
        .all(db)
        .await?;

    Ok(HttpResponse::Ok().json(orders))
}

