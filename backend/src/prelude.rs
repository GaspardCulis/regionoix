#[allow(unused_imports)]
pub use crate::dtos::{DtoTrait, IntoDto as _, PartialDto};
#[allow(unused_imports)]
pub use crate::entities::{prelude::*, *};
#[allow(unused_imports)]
pub use crate::services::*;
#[allow(unused_imports)]
pub use actix_web::{
    HttpMessage as _, HttpRequest, HttpResponse, Responder, delete, get, patch, post, put, web,
};
#[allow(unused_imports)]
pub use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
pub use tracing::{debug, info, warn};
#[allow(unused_imports)]
pub use utoipa::ToSchema;
#[allow(unused_imports)]
pub use utoipa_actix_web::{scope, service_config::ServiceConfig};
