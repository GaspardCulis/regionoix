use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub mod entities;
mod error;
mod routes;

pub use error::*;

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Loading environment variables from .env");
    dotenv().ok();

    let api_host = std::env::var("API_HOST").expect("API_HOST must be set.");
    let api_port = std::env::var("API_PORT").expect("API_PORT must be set.");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");

    println!("Connecting to database");
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    println!("Connecting to Redis session store");
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    println!("Connected to Redis session store");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(secret_key.as_bytes()),
            ))
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(web::scope("/api").configure(routes::config))
    })
    .bind((
        api_host,
        api_port.parse().expect("Failed to parse API_PORT"),
    ))?
    .run()
    .await
}
