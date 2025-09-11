use regionoix::{
    dtos::{IntoDto, PartialDto},
    *,
};

use meilisearch_sdk::client::Client;
use sea_orm::{Database, EntityTrait};
use tracing::info;

use crate::product_index::ProductIndex;

mod product_index;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Loading environment variables from .env");
    let secrets = secrets::Secrets::load().expect("load secrets");

    info!("Connecting to database");
    let db = Database::connect(secrets.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connecting to Meilisearch API");
    let client = Client::new(secrets.meili.api_url, Some(secrets.meili.admin_api_key)).unwrap();

    info!("Querying products");
    let products = entities::product::Entity::find()
        .into_dto::<dtos::product::ProductDto>()
        .all(&db)
        .await
        .expect("valid connection");

    // Finalize (fetch tags)
    let products: Vec<_> =
        futures::future::try_join_all(products.into_iter().map(|p| p.finalize(&db)))
            .await
            .unwrap();

    // Index
    let products_index = client.index("products");

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

    info!("Adding {} products to index", indexed_products.len());
    products_index
        .add_documents(indexed_products.as_slice(), Some("id"))
        .await
        .expect("Failed to index");
}
