mod db;
mod api;
mod bitcoin;
mod ingestion;

use dotenv::dotenv;
use std::env;
use rocket::tokio;
use rocket::Rocket;
use rocket::Build;
use rocket_cors::{CorsOptions, AllowedOrigins};

async fn build_rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db_pool = db::init_db(&database_url).await.expect("Failed to initialize database");

    tokio::spawn(ingestion::start_ingestion(db_pool.clone()));

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::some_exact(&[
        "http://34.210.188.43:3000",
        "http://localhost:3000",
    ]))
        .to_cors()
        .expect("Failed to create CORS fairing");

    api::start_server(db_pool)
        .attach(cors)
}

#[rocket::launch]
async fn rocket() -> _ {
    build_rocket().await
}