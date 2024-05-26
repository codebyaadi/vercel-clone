use aws_credential_types::Credentials;
use aws_sdk_s3 as s3;
use std::{env, time::SystemTime};

pub async fn download_files(build_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Download started...");
    let r2_access_key =
        env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID environment variable not set");
    let r2_secret_access = env::var("AWS_SECRET_ACCESS_KEY")
        .expect("AWS_SECRET_ACCESS_KEY environment variable not set");
    let r2_endpoint = env::var("ENDPOINT").expect("ENDPOINT environment variable not set");

    let creds = Credentials::new(
        r2_access_key,
        r2_secret_access,
        None,
        Some(SystemTime::now()),
        "dummy",
    );
    let config = aws_config::from_env()
        .region("auto")
        .credentials_provider(creds)
        .load()
        .await;
    let r2_config = s3::config::Builder::from(&config)
        .endpoint_url(r2_endpoint)
        .build();
    let client = s3::Client::from_conf(r2_config);

    let mut prefix_path = String::from("output/");
    prefix_path.push_str(&build_id.to_string());

    println!("prefix_path: {}", prefix_path);

    let response = client
        .list_objects_v2()
        .bucket("vercel-clone")
        .prefix(prefix_path)
        .send()
        .await;

    println!("response: {:?}", response);

    Ok(())
}
