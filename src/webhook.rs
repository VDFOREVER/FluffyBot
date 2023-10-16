use reqwest::Client;

pub async fn webhook_send(url: &str, json: serde_json::Value) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let _ = client.post(url).json(&json).send().await?;
    Ok(())
}