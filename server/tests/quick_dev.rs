


use anyhow::Result;
use serde_json::json;


#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_post("/users", json!({
        "username": "chance"
    })).await?.print().await?;
    hc.do_post("/users", json!({
        "username": "mike"
    })).await?.print().await?;
 
    hc.do_get("/listusers").await?.print().await?;
    Ok(())
}
