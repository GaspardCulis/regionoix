use crate::dtos::order::OrderDto;
use crate::prelude::*;
use crate::{AppState, routes::auth::LoggedUser};
use chrono::Utc;
use regionoix::utils::PaginateQuery;
use sea_orm::{ColumnTrait, EntityTrait as _, QueryFilter};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get).service(get_in_progress).service(get_past);
}

#[utoipa::path(
    summary="Returns orders",
    description="Orders for current user",
    tag="Orders",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Orders successfully returned",
            content_type="application/json",
            body=Vec<OrderDto>,
        ),
    ),
)]
#[get("")]
pub async fn get(
    data: web::Data<AppState>,
    query: web::Query<PaginateQuery>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let orders: Vec<OrderDto> = query
        .paginate(
            Order::find()
                .filter(order::Column::UserId.eq(logged_user.id))
                .into_dto(),
            &db.conn,
        )
        .await?;

    let mut results: Vec<OrderDto> = Vec::new();

    for o in orders {
        results.push(o.finalize(&db.conn).await?);
    }

    Ok(HttpResponse::Ok().json(results))
}

#[utoipa::path(
    summary="Returns order list in progress",
    description="Orders in progress for current user",
    tag="Orders",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Order list in progress successfully returned",
            content_type="application/json",
            body=Vec<OrderDto>,
        ),
    ),
)]
#[get("/in-progress")]
pub async fn get_in_progress(
    data: web::Data<AppState>,
    query: web::Query<PaginateQuery>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let today = Utc::now().date_naive();

    let orders: Vec<OrderDto> = query
        .paginate(
            Order::find()
                .filter(order::Column::UserId.eq(logged_user.id))
                .filter(order::Column::ArrivalDate.gte(today))
                .into_dto(),
            &db.conn,
        )
        .await?;

    let mut results: Vec<OrderDto> = Vec::new();

    for o in orders {
        results.push(o.finalize(&db.conn).await?);
    }

    Ok(HttpResponse::Ok().json(results))
}

#[utoipa::path(
    summary="Returns completed order list",
    description="Orders completed for current user",
    tag="Orders",
    params(PaginateQuery),
    responses(
        (
            status=200,
            description="Order completed list successfully returned",
            content_type="application/json",
            body=Vec<OrderDto>,
        ),
    ),
)]
#[get("/past")]
pub async fn get_past(
    data: web::Data<AppState>,
    query: web::Query<PaginateQuery>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let today = Utc::now().date_naive();

    let orders: Vec<OrderDto> = query
        .paginate(
            Order::find()
                .filter(order::Column::UserId.eq(logged_user.id))
                .filter(order::Column::ArrivalDate.lt(today))
                .into_dto(),
            &db.conn,
        )
        .await?;

    let mut results: Vec<OrderDto> = Vec::new();

    for o in orders {
        results.push(o.finalize(&db.conn).await?);
    }

    Ok(HttpResponse::Ok().json(results))
}
