use regionoix::{
    dtos::{IntoDto, PartialDto},
    *,
};

use meilisearch_sdk::client::Client;
use sea_orm::{Database, EntityTrait};
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Loading environment variables from .env");
    let secrets = secrets::Secrets::load().expect("load secrets");
    let meili_secrets = secrets::MeiliSecrets::load().expect("load secrets");

    info!("Connecting to database");
    let db = Database::connect(secrets.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connecting to Meilisearch API");
    let client = Client::new(meili_secrets.api_url, Some(meili_secrets.admin_api_key)).unwrap();

    let products_index = client.index("products");

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

    info!("Adding {} products to index", products.len());
    products_index
        .add_documents(products.as_slice(), Some("id"))
        .await
        .expect("Failed to index");
}
