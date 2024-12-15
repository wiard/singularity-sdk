use reqwest::blocking::Client;
use serde_json::Value;

pub fn verify_transaction(tx_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!("https://blockstream.info/api/tx/{}", tx_id);
    let response = Client::new().get(&url).send()?.json()?;
    Ok(response)
}

