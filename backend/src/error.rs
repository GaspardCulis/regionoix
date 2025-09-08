use std::num::ParseIntError;

use actix_web::{ResponseError, http::StatusCode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an unspecified internal error occurred: {0}")]
    InternalError(#[from] anyhow::Error),
    #[error("an error occurred while interacting with the database: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("could not find requested {table_name}")]
    EntityNotFound {
        /// Can be retreived using `you_entity_mod::Entity.table_name()`
        table_name: &'static str,
    },
    #[error("failed to parse: {0}")]
    ParseIntFailure(#[from] ParseIntError),
    #[error("authentication failed: password mismatch")]
    AuthenticationFailure,
    #[error("unauthenticated")]
    Unauthenticated,
    #[error("bad request: {0}")]
    BadRequestError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::EntityNotFound { table_name: _ } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
