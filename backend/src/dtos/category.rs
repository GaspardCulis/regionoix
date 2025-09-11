use sea_orm::{ColumnTrait, DerivePartialModel, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    dtos::{DtoTrait, IntoDto, PartialDto, subcategory::SubCategoryDto},
    entities::category,
};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "category::Entity", from_query_result)]
pub struct CategoryDto {
    id: i32,
    name: String,
    description: Option<String>,
    category_parent: Option<i32>,
    #[sea_orm(skip)]
    childs: Option<Vec<SubCategoryDto>>,
}

impl PartialDto for CategoryDto {
    async fn finalize(mut self, db: &sea_orm::DatabaseConnection) -> crate::Result<Self> {
        let categories_child = category::Entity::find()
            .filter(category::Column::CategoryParent.eq(self.id))
            .into_dto()
            .all(db)
            .await?;

        self.childs = Some(categories_child);

        Ok(self)
    }
}

impl DtoTrait for CategoryDto {}
