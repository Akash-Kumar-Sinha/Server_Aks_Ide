use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState{
    pub db: Pool<Postgres>
}

pub async fn get_pool(db_url: &str) -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("Database connection established successfully.");

    pool
}