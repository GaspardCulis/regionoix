use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web::Data};
use sea_orm::{Database, DatabaseConnection};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub mod entities;
mod error;
mod prelude;
mod routes;
mod secrets;

pub use error::*;

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
    println!("Loading environment variables from .env");
    let secrets = secrets::Secrets::load().expect("load secrets");

    println!("Connecting to database");
    let db = Database::connect(secrets.database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    println!("Connecting to Redis session store");
    let redis_store = RedisSessionStore::new(secrets.redis_url).await.unwrap();
    println!("Connected to Redis session store");

    HttpServer::new(move || {
        App::new()
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
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind((secrets.api_host, secrets.api_port))?
    .run()
    .await
}
