use serde_json::Value;
use reqwest::blocking::Client;

pub fn verify_transaction(tx_id: &str) -> Result<bool, String> {
    let url = format!("https://mempool.space/api/tx/{}/status", tx_id);
    let response = Client::new()
        .get(&url)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to verify transaction: HTTP {}",
            response.status()
        ));
    }

    let json: Value = response
        .json()
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    json["confirmed"]
        .as_bool()
        .ok_or_else(|| "Missing 'confirmed' field in JSON response".to_string())
}

