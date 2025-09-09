use crate::{
    AppState, Error,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, Region, Tag},
        product, region, tag,
    },
};
use actix_web::web::Data;
use sea_orm::{DatabaseConnection, EntityName, EntityTrait, ModelTrait};

#[derive(Clone)]
pub struct ProductRepository {
    db: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(state: Data<AppState>) -> Self {
        Self {
            db: state.db.clone(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<product::Model>, Error> {
        let products = Product::find().all(&self.db).await?;
        Ok(products)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<product::Model, Error> {
        let product = Product::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(Error::EntityNotFound {
                table_name: product::Entity.table_name(),
            })?;
        Ok(product)
    }

    pub async fn get_expanded(
        &self,
        id: i32,
    ) -> Result<
        (
            product::Model,
            Option<region::Model>,
            Option<brand::Model>,
            Option<category::Model>,
            Vec<tag::Model>,
        ),
        Error,
    > {
        let product = self.get_by_id(id).await?;
        let region = product.find_related(Region).one(&self.db).await?;
        let brand = product.find_related(Brand).one(&self.db).await?;
        let category = product.find_related(Category).one(&self.db).await?;
        let tags = product.find_related(Tag).all(&self.db).await?;

        Ok((product, region, brand, category, tags))
    }
}
