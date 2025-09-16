use std::collections::HashMap;

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
    /// Search filters in this example form: `id > 1 AND genres = Action`.
    /// The list of filterable attributes is `["weight", "price", "categories", "tags", "brand_name", "region_name"]`.
    /// See the [Meilisearch filter expression reference](https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_expression_reference#filter-expression-reference) for more info.
    filters: Option<String>,
    /// Sort by some specific attribute in the format `attribute:method` where `method: asc | desc`.
    /// Ex: `price:asc`.
    /// The list of sortable attributes is `["name", "price", "weight"]`.
    /// See the [Meilisearch sorting API](https://www.meilisearch.com/docs/reference/api/search#sort) for more info.
    sort: Option<String>,
    /// Number of results per page in a search. Defaults to 128 results.
    page_size: Option<usize>,
    /// Specific page to fetch; page index starts from 1. Defaults to 1.
    page_index: Option<usize>,
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
async fn search(
    query: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let search = &data.search;

    let search_results = search
        .index("products")
        .search()
        .with_query(&query.query)
        .with_filter(query.filters.as_ref().unwrap_or(&String::new()))
        .with_sort(
            query
                .sort
                .as_ref()
                .map(|s| vec![s.as_str()])
                .unwrap_or(vec![])
                .as_slice(),
        )
        .with_hits_per_page(query.page_size.unwrap_or(128))
        .with_page(query.page_index.unwrap_or(1))
        .execute::<ProductIndex>()
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let ids: Vec<_> = search_results
        .hits
        .iter()
        .map(|hit| hit.result.id)
        .collect();
    let id_to_order: HashMap<_, _> = ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();

    let mut product_results = Product::find()
        .filter(product::Column::Id.is_in(ids))
        .into_dto::<ProductDto>()
        .all(&db.conn)
        .await?;

    if product_results.len() != search_results.hits.len() {
        warn!("Some indexed products are missing from the database (sync to be checked)");
    }

    product_results.sort_by_key(|p| id_to_order[&p.id]);

    Ok(HttpResponse::Ok().json(product_results))
}
