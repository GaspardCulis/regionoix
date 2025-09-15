use chrono::NaiveDate;
use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::discount};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug, Clone)]
#[sea_orm(entity = "discount::Entity", from_query_result)]
pub struct DiscountDto {
    pub id: i32,
    pub percentage_off: i32,
    pub end_date: NaiveDate,
}

impl DtoTrait for DiscountDto {}
