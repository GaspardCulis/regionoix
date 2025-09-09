use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::brand;

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "brand::Entity", from_query_result)]
pub struct BrandDto {
    id: i32,
    name: String,
    description: Option<String>,
}
