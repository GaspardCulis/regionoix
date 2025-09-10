use sea_orm::{DerivePartialModel, JoinType, QuerySelect, RelationTrait as _};
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

impl DtoTrait<cart_line::Entity> for CartLineDto {
    fn add_nested_joins(
        selector: sea_orm::Select<cart_line::Entity>,
    ) -> sea_orm::Select<cart_line::Entity> {
        selector
            .left_join(product::Entity)
            // We also need to three-way join the product nested DTOs
            .join(JoinType::LeftJoin, product::Relation::Brand.def())
            .join(JoinType::LeftJoin, product::Relation::Region.def())
            .join(JoinType::LeftJoin, product::Relation::Category.def())
    }
}
