use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web::Data};
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod routes;

pub use error::*;
use regionoix::{
    services::{database::DatabaseService, meilisearch::SearchService, s3::S3Service},
    utils::get_env_var,
    *,
};

pub struct AppState {
    db: DatabaseService,
    search: SearchService,
    s3: S3Service,
}

#[derive(OpenApi)]
#[openapi(
  tags(
    (name = "Regionoix", description = "Regionoix API endpoints.")
  ),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    if dotenv::dotenv().is_err() {
        warn!("Failed to read .env, falling back to existing env vars");
    }

    let secret_key: String = get_env_var("SECRET_KEY").unwrap();

    info!("Connecting to database");
    let db = DatabaseService::build()
        .await
        .expect("failed to build DB service");

    info!("Connecting to Redis session store");
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .expect("Failed to connect to Redis session store");

    info!("Connecting to Meilisearch indexer");
    let search = SearchService::build_search().expect("failed to build Meilisearch service");

    info!("Connecting to S3 bucket");
    let s3 = S3Service::build().expect("failed to build S3 service");

    info!("Starting server app");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(secret_key.as_bytes()),
            ))
            .app_data(Data::new(AppState {
                db: db.clone(),
                search: search.clone(),
                s3: s3.clone(),
            }))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(routes::config))
            .openapi_service(|api| {
                SwaggerUi::new("/api-docs/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
