use anyhow::anyhow;
use tracing::info;

use crate::services::{database::DatabaseService, meilisearch::SearchService, s3::S3Service};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseService,
    pub search: SearchService,
    pub s3: S3Service,
}

impl AppState {
    pub async fn build() -> anyhow::Result<Self> {
        info!("Connecting to database");
        let db = DatabaseService::build()
            .await
            .map_err(|e| anyhow!("Failed to build database service: {}", e))?;

        info!("Connecting to Meilisearch indexer");
        let search = SearchService::build_search()
            .map_err(|e| anyhow!("Failed to build search service: {}", e))?;

        info!("Connecting to S3 bucket");
        let s3 = S3Service::build().map_err(|e| anyhow!("Failed to build s3 service: {}", e))?;

        Ok(Self { db, search, s3 })
    }
}
