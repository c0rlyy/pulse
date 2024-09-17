use anyhow::{Context, Ok};
use dotenvy;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::env;
use std::time::Duration;

use pulse::config;
use pulse::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;
    let hmac_key = env::var("HMAC_KEY").context("HMAC_KEY not set in .env")?;

    let cfg = config::Config::new(database_url, hmac_key, Duration::from_secs(1800));
    let pool = PgPoolOptions::new()
        .max_connections(90) // setting almost max connections
        .acquire_timeout(Duration::from_secs(3)) // had a dumb error when db was down and i was waiting like a 1 min for a 500 err
        .connect(&cfg.database_url)
        .await
        .context("Could not connect to the database")?;

    sqlx::migrate!().run(&pool).await?;

    http::serve(cfg, pool).await.unwrap();

    Ok(())
}
