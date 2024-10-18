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
    transaction_count: i32,
    block_size: f64,
    total_fees: f64,
    difficulty: f64,
    hash_rate: f64,
    mempool_size: i32,
}

#[derive(Serialize)]
struct HistoricalData {
    timestamp: chrono::NaiveDateTime,
    value: f64,
}

#[get("/block_metrics")]
async fn get_block_metrics(pool: &State<Pool<Postgres>>) -> Option<Json<BlockMetrics>> {
    let result = sqlx::query(
        "SELECT block_height, tx_volume, market_price, price_change_24h, transaction_count, block_size, total_fees, difficulty, hash_rate, mempool_size 
         FROM metrics 
         ORDER BY id DESC 
         LIMIT 1"
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
                transaction_count: record.try_get("transaction_count").ok()?,
                block_size: record.try_get("block_size").ok()?,
                total_fees: record.try_get("total_fees").ok()?,
                difficulty: record.try_get("difficulty").ok()?,
                hash_rate: record.try_get("hash_rate").ok()?,
                mempool_size: record.try_get("mempool_size").ok()?,
            }))
        },
        Err(e) => {
            eprintln!("Error fetching block metrics: {:?}", e);
            None
        },
    }
}

#[get("/historical/<metric>")]
async fn get_historical_data(pool: &State<Pool<Postgres>>, metric: String) -> Json<Vec<HistoricalData>> {
    let query = format!(
        "SELECT timestamp, {} FROM metrics ORDER BY timestamp DESC LIMIT 100",
        metric
    );

    let result = sqlx::query(&query)
        .fetch_all(pool.inner())
        .await;

    match result {
        Ok(records) => {
            let data: Vec<HistoricalData> = records
                .into_iter()
                .filter_map(|record| {
                    Some(HistoricalData {
                        timestamp: record.try_get("timestamp").ok()?,
                        value: record.try_get(metric.as_str()).ok()?,
                    })
                })
                .collect();
            Json(data)
        },
        Err(e) => {
            eprintln!("Error fetching historical data: {:?}", e);
            Json(vec![])
        },
    }
}

#[get("/metrics")]
async fn get_available_metrics() -> Json<Vec<String>> {
    Json(vec![
        "block_height".to_string(),
        "tx_volume".to_string(),
        "market_price".to_string(),
        "price_change_24h".to_string(),
        "transaction_count".to_string(),
        "block_size".to_string(),
        "total_fees".to_string(),
        "difficulty".to_string(),
        "hash_rate".to_string(),
        "mempool_size".to_string(),
    ])
}

pub fn start_server(pool: Pool<Postgres>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(pool)
        .mount("/", routes![get_block_metrics, get_historical_data, get_available_metrics])
}