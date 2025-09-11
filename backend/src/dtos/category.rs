use sea_orm::{ColumnTrait, DbErr, DerivePartialModel, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    dtos::{DtoTrait, IntoDto, PartialDto, subcategory::SubCategoryDto},
    entities::category,
};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "category::Entity", from_query_result)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_parent: Option<i32>,
    #[sea_orm(skip)]
    pub childs: Option<Vec<SubCategoryDto>>,
}

impl PartialDto for CategoryDto {
    async fn finalize(mut self, db: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
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
