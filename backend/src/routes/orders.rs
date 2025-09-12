use crate::AppState;
use crate::dtos::order::OrderDto;
use crate::prelude::*;
use actix_web::{HttpResponse, get, web::Data};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait as _, QueryFilter};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(get_in_progress).service(get_past);
}

#[utoipa::path(
    summary="Returns orders",
    description="Orders for current user",
    tag="Orders",
    responses(
        (
            status=200,
            description="Orders successfully returned",
            content_type="application/json",
            body=[OrderDto],
        ),
    ),
)]
#[get("")]
pub async fn get(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let orders: Vec<OrderDto> = Order::find()
        .filter(order::Column::UserId.eq(logged_user.id))
        .into_dto()
        .all(db)
        .await?;

    Ok(HttpResponse::Ok().json(orders))
}

#[utoipa::path(
    summary="Returns order list in progress",
    description="Orders in progress for current user",
    tag="Orders",
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

#[utoipa::path(
    summary="Returns completed order list",
    description="Orders completed for current user",
    tag="Orders",
    responses(
        (
            status=200,
            description="Order completed list successfully returned",
            content_type="application/json",
            body=[OrderDto],
        ),
    ),
)]
#[get("/past")]
pub async fn get_past(
    data: Data<AppState>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let today = Utc::now().date_naive();

    let orders: Vec<OrderDto> = Order::find()
        .filter(order::Column::UserId.eq(logged_user.id))
        .filter(order::Column::ArrivalDate.lt(today))
        .into_dto()
        .all(db)
        .await?;

    Ok(HttpResponse::Ok().json(orders))
}
