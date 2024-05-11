use std::time::Duration;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};

pub async fn start_deployment(mut redis_con: MultiplexedConnection) {
    println!("Started development");
    
    loop {
        let result: Result<Option<Vec<u8>>, RedisError> = redis_con.lpop("build-queue", None).await;

        match result {
            Ok(Some(build_id_bytes)) => {
                // Convert bytes to string for printing
                if let Ok(build_id_str) = String::from_utf8(build_id_bytes.clone()) {
                    println!("Fetched the build id (as string): {}", build_id_str);
                } else {
                    println!("Fetched the build id (as bytes): {:?}", build_id_bytes);
                }
            }
            Ok(None) => {
                println!("No id found");
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
            Err(e) => {
                eprintln!("Error while popping from queue: {}", e);
            }
        }
    }   
}
