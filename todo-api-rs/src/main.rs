mod adapters;
mod application;
mod domain;
mod framework;

use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use framework::rest_api::routes::todo;
use std::collections::HashMap;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("failed to load .env file");

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pool = create_db_pool(5).await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let app = Router::new()
        .merge(todo::create_router(pool))
        .layer(create_tracing_layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("server listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn create_db_pool(connections: u32) -> Pool<Postgres> {
    let env = std::env::vars().collect::<HashMap<String, String>>();
    let user = env
        .get("DB_USER")
        .expect("failed to load DB_USER from .env");
    let password = env
        .get("DB_PASSWORD")
        .expect("failed to load DB_PASSWORD from .env");
    let host = env
        .get("DB_HOST")
        .expect("failed to load DB_HOST from .env");
    let db_name = env
        .get("DB_NAME")
        .expect("failed to load DB_NAME from .env");

    PgPoolOptions::new()
        .max_connections(connections)
        .connect(&format!("postgres://{user}:{password}@{host}/{db_name}"))
        .await
        .expect("failed to connect to Postgres database")
}

fn create_tracing_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(tower_http::LatencyUnit::Micros),
        )
}
