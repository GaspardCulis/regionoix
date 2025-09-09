use crate::{
    dtos::product_dto::ProductDto,
    repositories::product_repository::ProductRepository,
    Error,
};

pub struct ProductService {
    repo: ProductRepository,
}

impl ProductService {
    pub fn new(repo: ProductRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<ProductDto>, Error> {
        let models = self.repo.get_all().await?;
        Ok(models.into_iter().map(ProductDto::from).collect())
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<ProductDto, Error> {
        let model = self.repo.get_by_id(id).await?;
        Ok(ProductDto::from_parts(model, None, None, None))
    }

    pub async fn get_product_expanded(&self, id: i32) -> Result<ProductDto, Error> {
        let (product, _region, _brand, _category, _tags) = self.repo.get_expanded(id).await?;
        Ok(ProductDto::from_parts(product, _region, _brand, _category))
    }
}
