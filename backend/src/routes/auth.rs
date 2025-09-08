use crate::prelude::*;
use actix_identity::Identity;
use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use sea_orm::{ColumnTrait, EntityName, EntityTrait as _, QueryFilter};
use utoipa::ToSchema;

use crate::{AppState, entities::user};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout).service(status);
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JwtClaims {
    /// User id
    sub: i32,
    /// Expiration time
    exp: usize,
}

#[utoipa::path()]
#[post("/login")]
pub async fn login(
    request: HttpRequest,
    login_request: Json<LoginRequest>,
    data: Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&login_request.email))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: user::Entity.table_name(),
        })?;

    check_password(&login_request, &user)?;

    Identity::login(&request.extensions(), format!("{}", user.id))
        .map_err(|err| anyhow::Error::new(err))?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path()]
#[post("/logout")]
async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}

#[utoipa::path()]
#[get("/status")]
async fn status(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("authenticated as {}", user.id().unwrap())
    } else {
        "unauthenticated".to_owned()
    }
}

fn check_password(login_request: &LoginRequest, user: &user::Model) -> crate::Result<()> {
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|err| crate::Error::InternalError(anyhow::Error::msg(err)))?;

    Argon2::default()
        .verify_password(login_request.password.as_bytes(), &parsed_hash)
        .map_err(|_| crate::Error::AuthenticationFailure)
}
