use axum::response::{Html, IntoResponse};
use axum::{routing::get, serve};
use std::net::SocketAddr;

pub mod lib;


#[tokio::main]
async fn main() -> std::io::Result<()> {
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
