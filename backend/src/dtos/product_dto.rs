use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::entities::product::Model as ProductModel;
use crate::entities::region::Model as RegionModel;
use crate::entities::brand::Model as BrandModel;
use crate::entities::category::Model as CategoryModel;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub price: Option<Decimal>,
    pub image: Option<String>,
    pub stock: i32,
    pub region: Option<String>,
    pub brand: Option<String>,
    pub category: Option<String>,
}

impl ProductDto {
    pub fn from(product: ProductModel) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            weight: product.weight,
            price: product.price,
            image: product.image,
            stock: product.stock,
            region: None,
            brand: None,
            category: None,
        }
    }

    pub fn from_parts(
        product: ProductModel,
        region: Option<RegionModel>,
        brand: Option<BrandModel>,
        category: Option<CategoryModel>,
    ) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            weight: product.weight,
            price: product.price,
            image: product.image,
            stock: product.stock,
            region: region.map(|r| r.name),
            brand: brand.map(|b| b.name),
            category: category.map(|c| c.name),
        }
    }
}

