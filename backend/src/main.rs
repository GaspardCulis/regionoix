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
use regionoix::{utils::get_env_var, *};

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
    let listen_addr: String = get_env_var("API_HOST").unwrap();
    let listen_port: u16 = get_env_var("API_PORT").unwrap();
    let redis_url: String = get_env_var("REDIS_URL").unwrap();

    let database_service = services::DatabaseService::build().await.unwrap();
    let s3_service = services::S3Service::build().unwrap();
    let search_service = services::SearchService::build_search().unwrap();

    info!("Connecting to Redis session store");
    let redis_store = RedisSessionStore::new(redis_url)
        .await
        .expect("Failed to connect to Redis session store");

    info!("Starting server app");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(secret_key.as_bytes()),
            ))
            .app_data(Data::new(database_service.clone()))
            .app_data(Data::new(s3_service.clone()))
            .app_data(Data::new(search_service.clone()))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(routes::config))
            .openapi_service(|api| {
                SwaggerUi::new("/api-docs/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind((listen_addr, listen_port))?
    .run()
    .await
}
