mod config;
mod utils;

#[tokio::main]
async fn main() {
    println!("Development Server Started!");
    let con = config::get_redis_conn()
        .await
        .expect("Failed to get Redis connection");

    utils::start_deployment(con.clone()).await;
}
