use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web::Data};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod routes;

pub use error::*;
use regionoix::*;

pub struct AppState {
    db: DatabaseConnection,
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

    info!("Loading environment variables from .env");
    let secrets = secrets::Secrets::load().expect("load secrets");

    info!("Connecting to database");
    let db = Database::connect(secrets.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connecting to Redis session store");
    let redis_store = RedisSessionStore::new(secrets.redis_url)
        .await
        .expect("Failed to connect to Redis session store");

    info!("Starting server app");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(secrets.secret_key.as_bytes()),
            ))
            .app_data(Data::new(AppState { db: db.clone() }))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api").configure(routes::config))
            .openapi_service(|api| {
                SwaggerUi::new("/api-docs/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind((secrets.api_host, secrets.api_port))?
    .run()
    .await
}
