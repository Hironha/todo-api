mod adapters;
mod application;
mod domain;
mod framework;

use axum::Router;
use framework::rest_api::routes::todo;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize db connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://hironha:rafaelhiro123@0.0.0.0:5432/todo_api")
        .await
        .expect("failed to connect into postgres db");

    sqlx::migrate!("./migrations").run(&pool).await?;

    // initialize all app routes
    let routes = Router::new().merge(todo::create_router(pool.clone()));

    // Creates an IPv4 socket address for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server listening on {addr}");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
