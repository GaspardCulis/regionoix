use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::region};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "region::Entity", from_query_result)]
pub struct RegionDto {
    id: i32,
    name: String,
    description: Option<String>,
}

impl DtoTrait for RegionDto {}
