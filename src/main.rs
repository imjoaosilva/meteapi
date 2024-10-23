use anyhow::{self, Context, Ok};
use api::http::{self, v1::models::client_context::ClientContext};
use sqlx::{migrate::Migrator, mysql::MySqlPool};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    let port = std::env::var("PORT").unwrap_or("3035".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;

    let pool: sqlx::Pool<sqlx::MySql> = MySqlPool::connect(&database_url)
        .await
        .context("Failed to connect to MySQL")?;

    println!("✨ Connected to MySQL");

    MIGRATOR
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    let pool_http = ClientContext { db: pool };

    http::v1::serve(pool_http, port)
        .await
        .context("Error starting the server")?;

    Ok(())
}
