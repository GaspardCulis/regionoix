use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RegionDto {
    id: i32,
    name: String,
    description: Option<String>,
}
