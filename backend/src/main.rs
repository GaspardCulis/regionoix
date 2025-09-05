use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
pub mod entities;
mod routes;
use sea_orm::{Database, DatabaseConnection};

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
    println!("Connecting to database");
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    HttpServer::new(move || {
        App::new()
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
