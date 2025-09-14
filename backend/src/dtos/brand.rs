use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::brand};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "brand::Entity", from_query_result)]
pub struct BrandDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl DtoTrait for BrandDto {}
