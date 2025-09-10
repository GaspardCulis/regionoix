use sea_orm::{
    DerivePartialModel, EntityTrait, JoinType, ModelTrait, QuerySelect as _, RelationTrait as _,
};
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
    /// Won't be fectched unless `finalize` is called.
    pub tags: Option<Vec<TagDto>>,
}

impl DtoTrait for ProductDto {
    fn add_nested_joins<E: EntityTrait>(selector: sea_orm::Select<E>) -> sea_orm::Select<E> {
        selector
            .join(JoinType::LeftJoin, product::Relation::Brand.def())
            .join(JoinType::LeftJoin, product::Relation::Region.def())
            .join(JoinType::LeftJoin, product::Relation::Category.def())
    }
}

impl PartialDto for ProductDto {
    async fn finalize(mut self, db: &sea_orm::DatabaseConnection) -> crate::Result<Self> {
        let product = product::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .expect("should exist");

        let tags = product.find_related(tag::Entity).into_dto().all(db).await?;

        self.tags = Some(tags);

        Ok(self)
    }
}
