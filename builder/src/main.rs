mod aws;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Development Server Started!");
    let con = config::get_redis_conn()
        .await
        .expect("Failed to get Redis connection");

    utils::start_deployment(con.clone()).await;
}
