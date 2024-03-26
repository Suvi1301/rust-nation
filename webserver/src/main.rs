use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connected to: {database_url}");

    // Setup the database
    let pool = get_connection_pool(&database_url).await?;
    println!("Running migrations");
    run_migrations(pool.clone()).await?;
    Ok(())
}

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_pool = sqlx::SqlitePool::connect(url)
        .await?;
    Ok(connection_pool)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(())
}
