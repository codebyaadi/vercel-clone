use aws_credential_types::Credentials;
use aws_sdk_s3 as s3;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use std::env;
use std::path::Path;
use std::time::SystemTime;

pub async fn upload_file(
    file_name: &str,
    local_file_path: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
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

    let body = ByteStream::from_path(Path::new(local_file_path)).await;

    let request = match body {
        Ok(f) => {
            client
                .put_object()
                .bucket("vercel-clone")
                .key(file_name.to_string())
                .body(f)
                .send()
                .await?
        }
        Err(e) => {
            panic!("Error uploading file: {:?}", e);
        }
    };

    println!("File uploaded successfully: {:?}", request);
    Ok(request)
}
