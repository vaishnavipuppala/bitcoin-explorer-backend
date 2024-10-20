use sqlx::{migrate::MigrateDatabase, Error, Pool, Postgres};

pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, Error> {
    let database_name = "bitcoin_explorer";
    let base_url = database_url.split('/').take(3).collect::<Vec<&str>>().join("/");
    let full_url = format!("{}/{}", base_url, database_name);

    if !Postgres::database_exists(&full_url).await.unwrap_or(false) {
        println!("Database does not exist. Creating...");
        match Postgres::create_database(&full_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(e) => eprintln!("Error creating database: {:?}", e),
        }
    } else {
        println!("Database already exists");
    }

    println!("Connecting to database...");
    let pool = Pool::<Postgres>::connect(&full_url).await?;
    println!("Connected to database successfully");

    create_tables(&pool).await?;
    add_missing_columns(&pool).await?;

    Ok(pool)
}

async fn create_tables(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metrics (
            id SERIAL PRIMARY KEY,
            block_height INTEGER NOT NULL,
            tx_volume FLOAT8 NOT NULL,
            market_price FLOAT8 NOT NULL,
            price_change_24h FLOAT8 NOT NULL,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(pool)
    .await?;

    println!("metrics table created or already exists.");
    Ok(())
}

async fn add_missing_columns(pool: &Pool<Postgres>) -> Result<(), Error> {
    let columns_to_add = vec![
        ("transaction_count", "INTEGER"),
        ("block_size", "FLOAT8"),
        ("total_fees", "FLOAT8"),
        ("difficulty", "FLOAT8"),
        ("hash_rate", "FLOAT8"),
        ("mempool_size", "INTEGER"),
    ];

    for (column_name, column_type) in columns_to_add {
        let query = format!(
            "ALTER TABLE metrics ADD COLUMN IF NOT EXISTS {} {} NOT NULL DEFAULT 0",
            column_name, column_type
        );
        sqlx::query(&query).execute(pool).await?;
        println!("Added column {} to metrics table (if it didn't exist)", column_name);
    }

    Ok(())
} pub async fn insert_metrics(
    pool: &Pool<Postgres>,
    block_height: i64,
    tx_volume: f64,
    market_price: f64,
    price_change: f64,
    transaction_count: usize,
    block_size: f64,
    total_fees: f64,
    difficulty: f64,
    hash_rate: f64,
    mempool_size: i64
) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO metrics (block_height, tx_volume, market_price, price_change_24h, transaction_count, block_size, total_fees, difficulty, hash_rate, mempool_size) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
    )
    .bind(block_height)
    .bind(tx_volume)
    .bind(market_price)
    .bind(price_change)
    .bind(transaction_count as i32)
    .bind(block_size)
    .bind(total_fees)
    .bind(difficulty)
    .bind(hash_rate)
    .bind(mempool_size)
    .execute(pool)
    .await?;
    
    println!("Inserted metrics: block_height={}, tx_volume={}, market_price={}, price_change={}, transaction_count={}, block_size={}, total_fees={}, difficulty={}, hash_rate={}, mempool_size={}", 
             block_height, tx_volume, market_price, price_change, transaction_count, block_size, total_fees, difficulty, hash_rate, mempool_size);
    
    Ok(())
}