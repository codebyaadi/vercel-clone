use axum::Json;
use git2::Repository;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use redis::RedisResult;
use serde_json::{json, Value};
use std::env;
use std::path::PathBuf;

use crate::aws;
use crate::utils;

pub async fn deploy(Json(body): Json<Value>, mut redis_con: MultiplexedConnection) -> Json<Value> {
    let repo_url = body["repo_url"].as_str().unwrap_or_default();
    let id = utils::generate();

    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap_or_default();
    let repo_path = format!("{}/output/{}", current_dir_str, id);

    match Repository::clone(repo_url, &repo_path) {
        Ok(_) => println!("Repo cloned to {}", repo_path),
        Err(e) => eprintln!("Failed to clone repo {}", e),
    }

    let repo_path_buf = PathBuf::from(repo_path);
    let files = match utils::get_files_in_folder(&repo_path_buf) {
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
        let relative_file = file.split_at(current_dir_str.len() + 1).1;
        if let Err(e) = aws::upload_file(relative_file, &file).await {
            eprintln!("Failed to upload file {}: {}", file, e);
        }
    }

    let result: RedisResult<()> = redis_con.lpush("build-queue", &id).await;
    match result {
        Ok(_) => println!("Successfully pushed id '{}' to 'build-queue'", id),
        Err(e) => eprintln!("Failed to push id '{}' to 'build-queue': {}", id, e),
    }

    Json(json!({"id": id, "files": files}))
}
