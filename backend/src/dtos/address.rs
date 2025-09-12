use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::DtoTrait, entities::address};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "address::Entity", from_query_result)]
pub struct AddressDto {
    id: i32,
    city: String,
    country: String,
    street: String,
    postal_code: String,
    lastname: String,
    firstname: String,
}

impl DtoTrait for AddressDto {}
