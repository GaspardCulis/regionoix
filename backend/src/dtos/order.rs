use crate::dtos::order_line::OrderLineDto;
use crate::entities::order;
use chrono::NaiveDate;
use sea_orm::{
    DbErr, DerivePartialModel, EntityTrait, JoinType, ModelTrait, QuerySelect as _,
    RelationTrait as _,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::address::AddressDto;
use crate::dtos::{DtoTrait, PartialDto};
use crate::prelude::sea_orm_active_enums::OrderStatus;
use crate::prelude::*;

#[derive(DerivePartialModel, Serialize, Deserialize, ToSchema, Debug)]
#[sea_orm(entity = "order::Entity", from_query_result)]
pub struct OrderDto {
    pub id: i32,
    pub total_price: f32,
    #[sea_orm(skip)]
    pub status: Option<OrderStatus>,
    pub arrival_date: Option<NaiveDate>,
    pub creation_date: Option<NaiveDate>,
    pub user_id: Option<i32>,
    #[sea_orm(nested)]
    pub adress: AddressDto,
    #[sea_orm(skip)]
    /// Won't be fectched unless `finalize` is called.
    pub order_lines: Option<Vec<OrderLineDto>>,
}

impl DtoTrait for OrderDto {
    fn add_nested_joins<E: EntityTrait>(selector: sea_orm::Select<E>) -> sea_orm::Select<E> {
        selector.join(JoinType::LeftJoin, order::Relation::Address.def())
    }
}

impl PartialDto for OrderDto {
    async fn finalize(mut self, db: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let order = order::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .expect("should exist");

        let order_lines = order
            .find_related(order_line::Entity)
            .into_dto()
            .all(db)
            .await?;

        self.status = Some(order.status);
        self.order_lines = Some(order_lines);

        Ok(self)
    }
}
