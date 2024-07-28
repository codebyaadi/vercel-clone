use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
// use std::time::Duration;

use crate::aws::{download_files, upload_file};
use crate::utils::{build_project, get_files_in_folder};

async fn upload_to_r2(build_id: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap_or_default();
    let file_path = format!("{}/downloads/output/{}/dist", current_dir_str, build_id);
    println!("path: {}", file_path);

    let file_path_buf = std::path::PathBuf::from(&file_path);
    let files = match get_files_in_folder(&file_path_buf) {
        Ok(files) => files
            .into_iter()
            .map(|path| path.to_string_lossy().replace("\\", "/"))
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Failed to get files: {}", e);
            Vec::new()
        }
    };

    for file in &files {
        let relative_file = file.split_at(&file_path.len() + 1).1;
        if let Err(e) = upload_file(relative_file, &file).await {
            eprintln!("Failed to upload file {}: {}", file, e);
        }
    }
}

pub async fn start_deployment(mut redis_con: MultiplexedConnection) {
    loop {
        let result: Result<Option<Vec<u8>>, RedisError> = redis_con.lpop("build-queue", None).await;

        match result {
            Ok(Some(build_id_bytes)) => {
                // Convert bytes to string for printing
                if let Ok(build_id_str) = String::from_utf8(build_id_bytes.clone()) {
                    println!("Fetched the build id (as string): {}", build_id_str);
                    if let Err(err) = download_files(&build_id_str).await {
                        eprintln!("Error while downloading files: {}", err);
                    } else if let Err(err) = build_project(&build_id_str).await {
                        eprintln!("Error while building project: {}", err);
                    }
                    upload_to_r2(&build_id_str).await;
                } else {
                    println!("Fetched the build id (as bytes): {:?}", build_id_bytes);
                }
            }
            Ok(None) => {
                // println!("No id found");
                // tokio::time::sleep(Duration::from_secs(20)).await;
            }
            Err(e) => {
                eprintln!("Error while popping from queue: {}", e);
            }
        }
    }
}
