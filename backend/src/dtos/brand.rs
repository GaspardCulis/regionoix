use sea_orm::{DerivePartialModel, Select};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, prelude::brand};

#[derive(DerivePartialModel, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[sea_orm(entity = "crate::entities::brand::Entity", from_query_result)]
pub struct BrandDto {
    id: i32,
    name: String,
}

impl DtoTrait<brand::Entity> for BrandDto {
    fn add_nested_joins(selector: Select<brand::Entity>) -> Select<brand::Entity> {
        selector
    }
}
