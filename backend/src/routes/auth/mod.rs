use std::future::{Ready, ready};

use actix_identity::Identity;
use actix_web::{FromRequest, dev::Payload};
use regionoix::prelude::{sea_orm_active_enums::Roles, *};

mod login;
mod logout;
mod status;
mod utils;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login::login)
        .service(logout::logout)
        .service(status::status);
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
