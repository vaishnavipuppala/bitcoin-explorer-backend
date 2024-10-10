use rocket::{get, routes, State};
use sqlx::{Pool, Postgres, Row};
use serde::Serialize;
use rocket::serde::json::Json;

#[derive(Serialize)]
struct BlockMetrics {
    block_height: i32,
    tx_volume: f64,
    market_price: f64,
    price_change_24h: f64,
}

#[get("/block_metrics")]
async fn get_block_metrics(pool: &State<Pool<Postgres>>) -> Option<Json<BlockMetrics>> {
    let result = sqlx::query(
        "SELECT block_height, tx_volume, market_price, price_change_24h FROM metrics ORDER BY id DESC LIMIT 1"
    )
    .fetch_one(pool.inner())
    .await;

    match result {
        Ok(record) => {
            Some(Json(BlockMetrics {
                block_height: record.try_get("block_height").ok()?,
                tx_volume: record.try_get("tx_volume").ok()?,
                market_price: record.try_get("market_price").ok()?,
                price_change_24h: record.try_get("price_change_24h").ok()?,
            }))
        },
        Err(e) => {
            eprintln!("Error fetching block metrics: {:?}", e);
            None
        },
    }
}

pub fn start_server(pool: Pool<Postgres>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(pool)
        .mount("/", routes![get_block_metrics])
}