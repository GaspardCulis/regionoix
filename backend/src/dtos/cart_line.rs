use sea_orm::{DerivePartialModel, EntityTrait, JoinType, QuerySelect, RelationTrait as _};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dtos::product::ProductDto, prelude::*};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "cart_line::Entity", from_query_result)]
pub struct CartLineDto {
    pub id: i32,
    pub cart_id: i32,
    #[sea_orm(nested)]
    pub product: ProductDto,
    pub quantity: i32,
}

impl DtoTrait for CartLineDto {
    fn add_nested_joins<E: EntityTrait>(selector: sea_orm::Select<E>) -> sea_orm::Select<E> {
        let selector = selector.join(JoinType::LeftJoin, cart_line::Relation::Product.def());
        // We also need to three-way join the product nested DTOs
        ProductDto::add_nested_joins(selector)
    }
}
