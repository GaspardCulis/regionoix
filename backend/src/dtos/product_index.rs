use serde::{Deserialize, Serialize};

use crate::dtos::product::ProductDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductIndex {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub price: f32,
    pub brand_name: Option<String>,
    pub region_name: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

impl ProductIndex {
    pub fn filterable_attributes() -> &'static [&'static str] {
        &[
            "weight",
            "price",
            "categories",
            "tags",
            "brand_name",
            "region_name",
        ]
    }

    pub fn sortable_attributes() -> &'static [&'static str] {
        &["name", "price", "weight"]
    }
}

impl From<ProductDto> for ProductIndex {
    fn from(value: ProductDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            weight: value.weight,
            price: value.price,
            brand_name: if let Some(brand) = value.brand {
                Some(brand.name)
            } else {
                None
            },
            region_name: if let Some(region) = value.region {
                Some(region.name)
            } else {
                None
            },
            categories: if let Some(category) = value.category {
                // TODO: Recursive category fetching
                vec![category.name]
            } else {
                vec![]
            },
            tags: if let Some(tags) = value.tags {
                tags.into_iter().map(|t| t.name).collect()
            } else {
                vec![]
            },
        }
    }
}
