use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    dtos::{DtoTrait, product::ProductDto},
    entities::order_line,
};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "order_line::Entity", from_query_result)]
pub struct OrderLineDto {
    id: i32,
    quantity: i32,
    unit_price: f32,
    #[sea_orm(nested)]
    product: ProductDto,
    order_id: Option<i32>,
}

impl DtoTrait for OrderLineDto {}
