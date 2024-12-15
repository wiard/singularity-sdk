use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

/// Fetch UTXOs for a given Bitcoin address using mempool.space API
pub fn fetch_utxos(address: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://mempool.space/api/address/{}/utxo", address);

    let client = Client::new();
    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let utxos: Value = response.json()?;
        Ok(utxos)
    } else {
        let status = response.status();
        let text = response.text().unwrap_or_else(|_| "No response body".to_string());
        Err(format!(
            "Failed to fetch UTXOs: HTTP {}: {}",
            status, text
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_utxos() {
        // Use a valid test address
        let test_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        match fetch_utxos(test_address) {
            Ok(utxos) => {
                println!("Fetched UTXOs: {:?}", utxos);
                assert!(utxos.is_array());
            }
            Err(e) => panic!("Failed to fetch UTXOs: {}", e),
        }
    }
}

