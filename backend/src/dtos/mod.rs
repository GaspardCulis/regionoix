use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, FromQueryResult, PartialModelTrait, Select,
    SelectModel, Selector,
};

pub mod brand;
pub mod cart;
pub mod cart_line;
pub mod category;
pub mod product;
pub mod region;
pub mod tag;

pub trait DtoTrait: FromQueryResult + PartialModelTrait {
    fn add_nested_joins<E: EntityTrait>(selector: Select<E>) -> Select<E> {
        selector
    }
}

pub trait PartialDto: FromQueryResult + PartialModelTrait {
    async fn finalize(self, db: &DatabaseConnection) -> Result<Self, DbErr>;
}

pub trait IntoDto<E: EntityTrait> {
    fn into_dto<D: DtoTrait>(self) -> Selector<SelectModel<D>>;
}

impl<E: EntityTrait> IntoDto<E> for Select<E> {
    fn into_dto<D: DtoTrait>(self) -> Selector<SelectModel<D>> {
        D::add_nested_joins(self).into_partial_model()
    }
}
