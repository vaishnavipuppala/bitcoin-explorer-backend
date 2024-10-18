use tokio::time::{sleep, Duration};
use sqlx::Pool;
use sqlx::Postgres;
use crate::bitcoin;
use crate::db;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct PriceResponse {
    bitcoin: PriceData,
}

#[derive(Deserialize)]
struct PriceData {
    usd: f64,
    usd_24h_change: f64,
}

async fn fetch_market_data() -> Result<(f64, f64), reqwest::Error> {
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_24hr_change=true")
        .await?
        .json::<PriceResponse>()
        .await?;
    
    Ok((response.bitcoin.usd, response.bitcoin.usd_24h_change))
}

pub async fn start_ingestion(pool: Pool<Postgres>) {
    let client = match bitcoin::get_client() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Bitcoin RPC client: {:?}", e);
            return;
        }
    };

    loop {
        match bitcoin::fetch_block_data(&client) {
            Ok((block_height, tx_volume, transaction_count, block_size, total_fees, difficulty, hash_rate, mempool_size)) => {
                println!("Fetched block data successfully");
                match fetch_market_data().await {
                    Ok((market_price, price_change)) => {
                        println!("Fetched market data successfully");
                        match db::insert_metrics(
                            &pool, 
                            block_height, 
                            tx_volume, 
                            market_price, 
                            price_change, 
                            transaction_count, 
                            block_size, 
                            total_fees,
                            difficulty,
                            hash_rate,
                            mempool_size
                        ).await {
                            Ok(_) => println!("Successfully inserted metrics into database"),
                            Err(e) => eprintln!("Failed to insert metrics: {:?}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to fetch market data: {:?}", e),
                }
            },
            Err(e) => eprintln!("Failed to fetch block data: {:?}", e),
        }

        sleep(Duration::from_secs(30)).await;
    }
}