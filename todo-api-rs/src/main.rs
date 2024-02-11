mod adapters;
mod application;
mod domain;
mod framework;

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use framework::rest_api::routes::todo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO)
        .init();

    if let Err(err) = dotenvy::dotenv() {
        tracing::error!("Failed loading .env {err}");
    }

    let pool = create_db_pool(5).await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed running migrations");

    let app = Router::new()
        .merge(todo::create_router(pool))
        .layer(CorsLayer::very_permissive())
        .layer(create_tracing_layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed binding tcp listener to address {addr}"));

    tracing::info!("Server listening on {addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed initiating server");

    Ok(())
}

async fn create_db_pool(connections: u32) -> Pool<Postgres> {
    let env = std::env::vars().collect::<HashMap<String, String>>();
    let user = env.get("DB_USER").expect("Missing DB_USER env");
    let password = env.get("DB_PASSWORD").expect("Missing DB_PASSWORD env");
    let host = env.get("DB_HOST").expect("Missing DB_HOST env");
    let port = env.get("DB_PORT").expect("Missing DB_PORT env");
    let db_name = env.get("DB_NAME").expect("Missing DB_NAME env");

    let url = format!("postgres://{user}:{password}@{host}:{port}/{db_name}");

    tracing::info!("Connecting to database at {url}");

    PgPoolOptions::new()
        .max_connections(connections)
        .connect(url.as_str())
        .await
        .expect("Failed connecting to postgres database")
}

fn create_tracing_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(false)
                .level(Level::INFO),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(tower_http::LatencyUnit::Micros),
        )
}
