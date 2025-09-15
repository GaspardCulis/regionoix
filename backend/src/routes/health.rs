use std::time::Duration;

use actix_session::storage::RedisSessionStore;
use regionoix::{prelude::*, utils::get_env_var};
use reqwest::Client;
use rusty_s3::S3Action as _;

use crate::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health);
}

#[utoipa::path(
    summary="Get API health. Checks all connector services.",
    responses(
        (
            status=200,
            description="All services online",
            body=String
        ),
    ),
)]
#[get("")]
async fn health(data: web::Data<AppState>) -> crate::Result<HttpResponse> {
    // Database connection check
    let db = &data.db;
    db.ping().await?;

    // S3 connection check
    let s3 = &data.s3;
    let client = Client::new();
    let action = s3.api_bucket.list_objects_v2(Some(&s3.credentials));
    let url = action.sign(Duration::from_secs(300));
    let resp = client.get(url).send().await?.error_for_status()?;
    let _ = resp.text().await?;

    // Meilisearch conn check
    let search = &data.search;
    let health = search.health().await.map_err(|e| anyhow::Error::from(e))?;
    if health.status != "available" {
        return Err(crate::Error::InternalError(anyhow::anyhow!(
            "Meilisearch connection not healthy: {}",
            health.status
        )));
    }

    // Redis
    // FIX: Ugly implem, reuse existing RedisSessionStore somehow
    let redis_url: String = get_env_var("REDIS_URL").unwrap();
    let _redis_store = RedisSessionStore::new(redis_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to Redis session store: {}", e))?;

    Ok(HttpResponse::Ok().body("healthy"))
}
