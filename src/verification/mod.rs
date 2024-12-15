use reqwest::Error;

pub async fn verify_transaction(txid: &str) -> Result<bool, Error> {
    let url = format!("https://blockstream.info/api/tx/{}/status", txid);
    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;
    Ok(response["confirmed"].as_bool().unwrap_or(false))
}

