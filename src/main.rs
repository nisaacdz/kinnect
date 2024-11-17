use axum::response::{Html, IntoResponse};
use axum::{routing::get, serve};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app = axum::Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> impl IntoResponse {
    Html("<h1>Hello, World!</h1>")
}
