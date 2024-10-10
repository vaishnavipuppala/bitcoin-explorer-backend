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

    Ok(pool)
}

async fn create_tables(pool: &Pool<Postgres>) -> Result<(), Error> {
    match sqlx::query(
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
    .await {
        Ok(_) => println!("metrics table created or already exists."),
        Err(e) => eprintln!("Failed to create metrics table: {:?}", e),
    };

    Ok(())
}

pub async fn insert_metrics(pool: &Pool<Postgres>, block_height: i64, tx_volume: f64, market_price: f64, price_change: f64) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO metrics (block_height, tx_volume, market_price, price_change_24h) VALUES ($1, $2, $3, $4)"
    )
    .bind(block_height)
    .bind(tx_volume)
    .bind(market_price)
    .bind(price_change)
    .execute(pool)
    .await?;
    Ok(())
}