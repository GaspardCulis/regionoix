use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::tag};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "tag::Entity", from_query_result)]
pub struct TagDto {
    pub id: i32,
    pub name: String,
}

impl DtoTrait for TagDto {}
