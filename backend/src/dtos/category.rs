use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CategoryDto {
    id: i32,
    name: String,
    category_parent: Option<i32>,
}
