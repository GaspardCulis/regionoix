use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::{brand::BrandDto, category::CategoryDto, region::RegionDto, tag::TagDto};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub price: f32,
    pub image: Option<String>,
    pub stock: i32,
    pub brand: Option<BrandDto>,
    pub region: Option<RegionDto>,
    pub category: Option<CategoryDto>,
    pub tags: Vec<TagDto>,
}
