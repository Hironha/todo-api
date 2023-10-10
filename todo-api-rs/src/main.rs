mod adapters;
mod application;
mod domain;
mod framework;

use std::collections::HashMap;
use std::net::SocketAddr;

use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use framework::rest_api::routes::tag;
use framework::rest_api::routes::todo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("failed loading .env");

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO)
        .init();

    let pool = create_db_pool(5).await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed running migrations");

    let app = Router::new()
        .merge(todo::create_router(pool.clone()))
        .merge(tag::create_tag_router(pool))
        .layer(CorsLayer::very_permissive())
        .layer(create_tracing_layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("server listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed initiating server");
    
    Ok(())
}

async fn create_db_pool(connections: u32) -> Pool<Postgres> {
    let env = std::env::vars().collect::<HashMap<String, String>>();
    let url = env.get("DB_URL").expect("missing DB_URL env");

    PgPoolOptions::new()
        .max_connections(connections)
        .connect(url.as_str())
        .await
        .expect("failed connecting to postgres database")
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
