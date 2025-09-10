use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::tag};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "tag::Entity", from_query_result)]
pub struct TagDto {
    id: i32,
    name: String,
}

impl DtoTrait<tag::Entity> for TagDto {}
