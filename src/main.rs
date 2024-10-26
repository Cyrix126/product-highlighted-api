mod api;
mod config;
mod db;
mod error;
mod schema;

use api::{highlight, highlight_flip, highlighted_disabled, highlighted_enabled};
use axum::{
    routing::{get, post, put},
    serve, Router,
};
use config::Config;
use db::run_migrations;
use deadpool_diesel::postgres::Pool;
#[derive(Clone)]
struct AppState {
    config: Config,
    pool: Pool,
    client_product: doli_client_api_rs::Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let config: Config = confy::load_path("/etc/name_api/config.toml")?;
    let pool = Pool::builder(deadpool_diesel::Manager::new(
        config.db_uri.as_str(),
        deadpool_diesel::Runtime::Tokio1,
    ))
    .build()?;
    run_migrations(&pool).await?;
    let client_product = config.product_client()?;
    let state = AppState {
        config,
        pool,
        client_product,
    };
    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", state.config.listen_port)).await?;
    serve(listener, router(state)).await?;
    Ok(())
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/highlights", get(highlighted_enabled))
        .route("/highlights_disabled", get(highlighted_disabled))
        .route("/highlights", post(highlight))
        .route("/highlights", put(highlight_flip))
        .with_state(state)
}
