use reqwest::blocking::Client;
use serde_json::Value;

/// Fetches UTXOs for a given Bitcoin address
pub fn fetch_utxos(address: &str) -> Result<Value, reqwest::Error> {
    let url = format!("https://blockstream.info/api/address/{}/utxo", address);
    let response = Client::new().get(&url).send()?.json()?;
    Ok(response)
}

