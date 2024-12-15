use reqwest::blocking::Client;
use serde_json::Value;

pub fn fetch_utxos(address: &str) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://blockstream.info/api/address/{}/utxo", address);
    let response = client.get(&url).send()?.json()?;
    Ok(response)
}

