use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web};
use sea_orm::{Database, DatabaseConnection};

pub mod entities;
mod error;
mod routes;
mod secrets;

pub use error::*;

struct AppState {
    db: DatabaseConnection,
}

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
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(web::scope("/api").configure(routes::config))
    })
    .bind((secrets.api_host, secrets.api_port))?
    .run()
    .await
}
