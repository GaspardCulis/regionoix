use crate::Result;
use actix_identity::Identity;
use actix_web::{HttpMessage as _, HttpRequest, HttpResponse, Responder, get, post, web};
use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use sea_orm::{EntityName, EntityTrait as _};
use serde::{Deserialize, Serialize};

use crate::{AppState, entities::user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(status);
}

#[derive(Deserialize)]
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

#[post("/login")]
pub async fn login(
    request: HttpRequest,
    login_request: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let db = &data.db;
    let users: Vec<user::Model> = user::Entity::find().all(db).await?;

    let user = match users
        .into_iter()
        .find(|user| user.email == login_request.email)
    {
        Some(user) => Ok(user),
        None => Err(crate::Error::EntityNotFound {
            table_name: user::Entity.table_name(),
        }),
    }?;

    check_password(&login_request, &user)?;

    Identity::login(&request.extensions(), format!("{}", user.id))
        .map_err(|err| anyhow::Error::new(err))?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/logout")]
async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}

fn check_password(login_request: &LoginRequest, user: &user::Model) -> Result<()> {
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|err| crate::Error::InternalError(anyhow::Error::msg(err)))?;

    Argon2::default()
        .verify_password(login_request.password.as_bytes(), &parsed_hash)
        .map_err(|_| crate::Error::AuthenticationFailure)
}

#[get("/status")]
async fn status(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("authenticated as {}", user.id().unwrap())
    } else {
        "unauthenticated".to_owned()
    }
}
