use actix_identity::Identity;
use regionoix::prelude::*;
use sea_orm::prelude::*;

use crate::AppState;

use super::{LoggedUser, utils::check_password};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    summary="Logs user in",
    tag="Authentification",
    request_body(content_type="application/json", content=LoginRequest),
    responses(
        (
            status=200,
            description="Logged in successfully",
        ),
        (
            status=404,
            description="Email not found",
        ),
    ),
)]
#[post("/login")]
async fn login(
    request: HttpRequest,
    login_request: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&login_request.email))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: user::Entity.table_name(),
        })?;

    check_password(&login_request.password, &user)?;

    let logged_user = LoggedUser {
        id: user.id,
        email: user.email,
        role: user.role,
        lastname: user.lastname,
        firstname: user.fistname,
    };
    let user_string = serde_json::to_string(&logged_user).unwrap();
    Identity::login(&request.extensions(), user_string).map_err(|err| anyhow::Error::new(err))?;

    Ok(HttpResponse::Ok().finish())
}
