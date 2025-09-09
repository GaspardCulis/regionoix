use crate::entities::product;
use sea_orm::{DerivePartialModel, EntityTrait, Select, SelectModel, Selector};
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
    brand: Option<Brand>,
}

impl ProductDto {
    pub fn find() -> Selector<SelectModel<ProductDto>> {
        product::Entity::find()
            .left_join(crate::entities::brand::Entity)
            .into_partial_model()
    }

    pub fn find_by_id(id: i32) -> Selector<SelectModel<ProductDto>> {
        product::Entity::find_by_id(id)
            .left_join(crate::entities::brand::Entity)
            .into_partial_model()
    }
}

#[derive(DerivePartialModel, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[sea_orm(entity = "crate::entities::brand::Entity", from_query_result)]
struct Brand {
    id: i32,
    name: String,
}
