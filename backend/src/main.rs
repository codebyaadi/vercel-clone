use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use std::net::SocketAddr;

mod aws;
mod config;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let con = config::get_redis_conn()
        .await
        .expect("Failed to get Redis connection");

    let app = Router::new().route("/", get(handlers::home)).route(
        "/deploy",
        post(move |body: Json<Value>| handlers::deploy(body, con.clone())),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {addr}\n");

    axum::serve(listener, app).await.unwrap();
}
