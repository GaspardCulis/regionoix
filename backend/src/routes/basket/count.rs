use regionoix::prelude::*;
use sea_orm::prelude::*;

use crate::{AppState, routes::auth::LoggedUser};

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct CountBasket {
    count: i32,
}

#[utoipa::path(
    summary = "Returns basket count of current user",
    description = "Returns count of differents products in cart of current user.",
    tag="Basket",
    responses(
      (
          status = 200,
          description="Basket count successfully returned",
          content_type = "application/json",
          body=CountBasket,
      ),
))]
#[get("/count")]
async fn get(data: web::Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get count of all cart lines with at least quantity of 1
    let count = CartLine::find()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .filter(cart_line::Column::Quantity.gte(1))
        .count(&db.conn)
        .await? as i32;

    Ok(HttpResponse::Ok().json(CountBasket { count }))
}
