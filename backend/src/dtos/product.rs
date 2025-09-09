use crate::{
    dtos::{DtoTrait, brand::BrandDto},
    entities::product,
    prelude::brand,
};
use sea_orm::{DerivePartialModel, Select};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(DerivePartialModel, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[sea_orm(entity = "product::Entity", from_query_result)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub price: f32,
    pub image: Option<String>,
    pub stock: i32,
    #[sea_orm(nested)]
    brand: Option<BrandDto>,
}

impl DtoTrait<product::Entity> for ProductDto {
    fn add_nested_joins(selector: Select<product::Entity>) -> Select<product::Entity> {
        selector.left_join(brand::Entity)
    }
}
