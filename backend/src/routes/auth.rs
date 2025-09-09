use crate::prelude::*;
use actix_identity::Identity;
use utoipa::ToSchema;
use actix_web::{web, HttpResponse};

use crate::{dtos::user_dto::UserDto, services::auth_service::AuthService, Error};

pub fn config(cfg: &mut web::ServiceConfig, auth_service: web::Data<AuthService>) {
    cfg.app_data(auth_service.clone())
        .service(login)
        .service(logout)
        .service(status);
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[utoipa::path()]
#[post("/login")]
pub async fn login(
    id: Identity,
    auth_service: web::Data<AuthService>,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    // Authenticate via service
    let user: UserDto = auth_service
        .authenticate(&login_request.email, &login_request.password)
        .await?;

    // Log in with Identity
    id.login(format!("{}", user.id))
        .map_err(|err| Error::InternalError(anyhow::Error::msg(err)))?;

    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path()]
#[post("/logout")]
pub async fn logout(id: Option<Identity>) -> HttpResponse {
    if let Some(user) = id {
        user.logout();
    }
    HttpResponse::Ok().finish()
}

#[utoipa::path()]
#[get("/status")]
pub async fn status(id: Option<Identity>) -> HttpResponse {
    if let Some(user) = id {
        HttpResponse::Ok().body(format!("authenticated as {}", user.id().unwrap()))
    } else {
        HttpResponse::Ok().body("unauthenticated")
    }
}
