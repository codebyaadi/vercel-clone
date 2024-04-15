use anyhow::{Ok, Result};

#[tokio::test]
async fn dev_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}
