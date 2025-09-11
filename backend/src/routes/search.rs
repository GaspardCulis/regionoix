use actix_web::web::Query;
use regionoix::{
    dtos::{product::ProductDto, product_index::ProductIndex},
    prelude::*,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use utoipa::IntoParams;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(search);
}

#[derive(Deserialize, IntoParams)]
struct SearchQuery {
    /// The raw search query
    query: String,
    /// Search filters in SQL form (eg: id > 1 AND genres = Action).
    /// The list of filterable attributes is `["weight", "price", "categories", "tags"]`.
    filters: Option<String>,
}

#[utoipa::path(
    summary="Search for products",
    tag="Products",
    params(SearchQuery),
    responses(
        (
            status=200,
            description="Products successfully searched and returned",
            content_type="application/json",
            body=Vec<ProductDto>,
        ),
    ),
)]
#[get("/products")]
async fn search(query: Query<SearchQuery>, data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let search = &data.search;

    let results = search
        .index("products")
        .search()
        .with_query(&query.query)
        .with_filter(query.filters.as_ref().unwrap_or(&String::new()))
        .execute::<ProductIndex>()
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let ids: Vec<_> = results.hits.iter().map(|hit| hit.result.id).collect();

    // PERF: Using `IS IN` might be sub optimal performanc-wise for large amount of search results
    let product_results = Product::find()
        .filter(product::Column::Id.is_in(ids))
        .into_dto::<ProductDto>()
        .all(db)
        .await?;

    Ok(HttpResponse::Ok().json(product_results))
}
