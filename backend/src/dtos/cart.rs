use sea_orm::{ConnectionTrait, DbErr, DerivePartialModel, EntityTrait as _, ModelTrait as _};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::cart_line::CartLineDto;
use crate::dtos::{DtoTrait, IntoDto, PartialDto};
use crate::entities::{cart, cart_line};

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "cart::Entity", from_query_result)]
pub struct CartDto {
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(skip)]
    /// Won't be fectched unless `finalize` is called.
    pub lines: Option<Vec<CartLineDto>>,
}

impl DtoTrait for CartDto {}

impl PartialDto for CartDto {
    async fn finalize<C: ConnectionTrait>(mut self, db: &C) -> Result<Self, DbErr> {
        let cart = cart::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .expect("should exist");

        let cart_lines = cart
            .find_related(cart_line::Entity)
            .into_dto::<CartLineDto>()
            .all(db)
            .await?;

        self.lines = Some(cart_lines);

        Ok(self)
    }
}
