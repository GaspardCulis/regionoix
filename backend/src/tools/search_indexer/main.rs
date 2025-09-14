use regionoix::{
    dtos::{IntoDto, PartialDto, product_index::ProductIndex},
    services::{database::DatabaseService, meilisearch::SearchService},
    *,
};

use sea_orm::EntityTrait;
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Connecting to database");
    let db = DatabaseService::build()
        .await
        .expect("failed to build DB service");

    info!("Connecting to Meilisearch indexer");
    let search = SearchService::build_admin().expect("failed to build Meilisearch service");

    info!("Querying products");
    let products = entities::product::Entity::find()
        .into_dto::<dtos::product::ProductDto>()
        .all(&db.conn)
        .await
        .expect("valid connection");

    // Finalize (fetch tags)
    let products: Vec<_> =
        futures::future::try_join_all(products.into_iter().map(|p| p.finalize(&db)))
            .await
            .unwrap();

    // Index
    let products_index = search.index("products");

    info!("Clearing previous documents");
    products_index
        .delete_all_documents()
        .await
        .expect("Failed to delete all documents");

    let indexed_products: Vec<ProductIndex> = products.into_iter().map(|p| p.into()).collect();

    products_index
        .set_filterable_attributes(ProductIndex::filterable_attributes())
        .await
        .expect("Failed to set filterable attributes");

    products_index
        .set_sortable_attributes(ProductIndex::sortable_attributes())
        .await
        .expect("Failed to set sortable attributes");

    info!("Adding {} products to index", indexed_products.len());
    products_index
        .add_documents(indexed_products.as_slice(), Some("id"))
        .await
        .expect("Failed to index");
}
