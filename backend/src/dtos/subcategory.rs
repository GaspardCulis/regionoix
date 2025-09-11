use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::category};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "category::Entity", from_query_result)]
pub struct SubCategoryDto {
    id: i32,
    name: String,
    description: Option<String>,
    category_parent: Option<i32>,
}

impl DtoTrait for SubCategoryDto {}
