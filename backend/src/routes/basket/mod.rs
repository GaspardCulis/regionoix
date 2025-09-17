use crate::{prelude::*, routes::auth::LoggedUser};
use actix_web::web;
use regionoix::dtos::cart::CartDto;
use sea_orm::prelude::*;

mod count;
mod items;
mod order;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get)
        .service(items::add)
        .service(items::update_quantity)
        .service(items::remove)
        .service(empty)
        .service(order::make)
        .service(count::get);
}

#[utoipa::path(
    summary = "Returns basket details of current user",
    tag="Basket",
    responses(
      (
          status = 200,
          description="Basket details successfully returned",
          content_type = "application/json",
          body=CartDto,
      ),
))]
#[get("")]
async fn get(
    db: web::Data<DatabaseService>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let basket = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .into_dto::<CartDto>()
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?
        .finalize(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(basket))
}

#[utoipa::path(
    summary="Empty current user's basket",
    tag="Basket",
    responses(
        (
            status = 200,
            description="Basket successfully emptied",
            body=String,
        ),
    ),
)]
#[delete("")]
async fn empty(
    db: web::Data<DatabaseService>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    cart_line::Entity::delete_many()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .exec(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().body("Cart emptied successfully"))
}
