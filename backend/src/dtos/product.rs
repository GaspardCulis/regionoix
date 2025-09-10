use sea_orm::{DerivePartialModel, EntityTrait, ModelTrait, Related};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::{DtoTrait, PartialDto};
use crate::dtos::{brand::BrandDto, category::CategoryDto, region::RegionDto, tag::TagDto};
use crate::prelude::*;

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
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
    pub brand: Option<BrandDto>,
    #[sea_orm(nested)]
    pub region: Option<RegionDto>,
    #[sea_orm(nested)]
    pub category: Option<CategoryDto>,
    #[sea_orm(skip)]
    pub tags: Vec<TagDto>,
}

impl DtoTrait<product::Entity> for ProductDto {
    fn add_nested_joins(
        selector: sea_orm::Select<product::Entity>,
    ) -> sea_orm::Select<product::Entity> {
        selector
            .left_join(brand::Entity)
            .left_join(region::Entity)
            .left_join(category::Entity)
    }
}

impl PartialDto for ProductDto {
    async fn finalize(mut self, db: &sea_orm::DatabaseConnection) -> crate::Result<Self> {
        let product = product::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .expect("should exist");

        let tags = product
            .find_related(tag::Entity)
            .into_partial_model()
            .all(db)
            .await?;

        self.tags = tags;

        Ok(self)
    }
}
