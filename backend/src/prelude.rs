#[allow(unused_imports)]
pub use crate::{
    AppState,
    entities::{prelude::*, *},
};
#[allow(unused_imports)]
pub use actix_web::{
    HttpMessage as _, HttpRequest, HttpResponse, Responder, delete, get, post, put,
    web::{Data, Json},
};
#[allow(unused_imports)]
pub use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
pub use utoipa_actix_web::{scope, service_config::ServiceConfig};
