use sea_orm::{DerivePartialModel, EntityTrait, JoinType, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    dtos::{DtoTrait, product::ProductDto},
    entities::order_line,
};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "order_line::Entity", from_query_result)]
pub struct OrderLineDto {
    pub id: i32,
    pub quantity: i32,
    pub unit_price: f32,
    #[sea_orm(nested)]
    pub product: ProductDto,
    pub order_id: Option<i32>,
}

impl DtoTrait for OrderLineDto {
    fn add_nested_joins<E: EntityTrait>(selector: sea_orm::Select<E>) -> sea_orm::Select<E> {
        let selector = selector.join(JoinType::LeftJoin, order_line::Relation::Product.def());
        // We also need to three-way join the product nested DTOs
        ProductDto::add_nested_joins(selector)
    }
}
