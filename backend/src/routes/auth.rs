use std::future::{Ready, ready};

use crate::prelude::{sea_orm_active_enums::Roles, *};
use actix_identity::Identity;
use actix_web::{FromRequest, dev::Payload};
use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use sea_orm::{ColumnTrait, EntityName, EntityTrait as _, QueryFilter};

use crate::{AppState, entities::user};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout).service(status);
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct LoginRequest {
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

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoggedUser {
    pub id: i32,
    pub email: String,
    pub role: Roles,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl FromRequest for LoggedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<LoggedUser, actix_web::Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(crate::Error::Unauthenticated.into()))
    }
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

#[utoipa::path(
    summary="Logs user out",
    tag="Authentification",
    responses(
        (
            status=200,
            description="Logged out successfully",
        ),
    ),
)]
#[post("/logout")]
async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}

#[utoipa::path(
    summary="Get user authentication status",
    tag="Authentification",
    responses(
        (
            status=200,
            content_type="Application/Json",
            body=LoggedUser,
        ),
    ),
)]
#[get("/status")]
async fn status(logged_user: LoggedUser) -> impl Responder {
    HttpResponse::Ok().json(logged_user)
}

fn check_password(login_request: &LoginRequest, user: &user::Model) -> crate::Result<()> {
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|err| crate::Error::InternalError(anyhow::Error::msg(err)))?;

    Argon2::default()
        .verify_password(login_request.password.as_bytes(), &parsed_hash)
        .map_err(|_| crate::Error::AuthenticationFailure)
}
