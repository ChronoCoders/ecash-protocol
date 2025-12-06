mod cache;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod state;
mod types;

use crate::cache::RedisCache;
use crate::config::Config;
use crate::db::Database;
use crate::state::{generate_or_load_keys, AppState};
use axum::routing::{get, post};
use axum::Router;
use ecash_core::Institution;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,ecash_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting eCash Protocol Server");

    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    let db_pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;
    tracing::info!("Database connected");

    sqlx::migrate!("./migrations").run(&db_pool).await.ok();

    let database = Database::new(db_pool);

    let cache = RedisCache::new(&config.redis.url).await?;
    tracing::info!("Redis connected");

    let (private_key, public_key) = generate_or_load_keys()?;

    let institution = Institution::new(
        private_key,
        config.institution.institution_id.clone(),
        config.institution.key_id.clone(),
        config.institution.denominations.clone(),
        config.institution.token_expiry_days,
    );

    tracing::info!(
        "Institution initialized: {} (key: {})",
        institution.institution_id(),
        config.institution.key_id
    );

    let state = AppState::new(institution, public_key, database, cache, config.clone()).await;

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/v1/keys", get(handlers::get_public_key))
        .route("/api/v1/withdraw", post(handlers::withdraw))
        .route("/api/v1/redeem", post(handlers::redeem))
        .route("/api/v1/verify", post(handlers::verify))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
