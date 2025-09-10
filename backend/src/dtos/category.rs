use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::category;

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "category::Entity", from_query_result)]
pub struct CategoryDto {
    id: i32,
    name: String,
    category_parent: Option<i32>,
}
