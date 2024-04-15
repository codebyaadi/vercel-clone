use redis::{Client, RedisError};

pub async fn get_redis_conn() -> Result<redis::aio::MultiplexedConnection, RedisError> {
    let client = Client::open("redis://127.0.0.1/")?;
    let con = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            eprintln!("Error getting Redis connection: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    println!("Successfully connected to Redis!");
    Ok(con)
}
