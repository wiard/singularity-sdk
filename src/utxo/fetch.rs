use crate::transaction::{UnifiedInput, InputSourceType, InputMetadata};
use reqwest::blocking::Client;
use serde_json::Value;

/// Fetch UTXOs for a given Bitcoin address.
pub fn fetch_utxos(address: &str) -> Result<Vec<UnifiedInput>, String> {
    let url = format!("https://mempool.space/api/address/{}/utxo", address);
    let client = Client::new();
    let response = client.get(&url).send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let utxos: Vec<Value> = resp.json().unwrap_or_default();
            let unified_inputs = utxos
                .into_iter()
                .map(|utxo| UnifiedInput {
                    source_type: InputSourceType::BitcoinUTXO,
                    metadata: InputMetadata {
                        txid: Some(utxo["txid"].as_str().unwrap_or_default().to_string()),
                        index: utxo["vout"].as_u64().map(|v| v as u32),
                        amount: utxo["value"].as_u64(),
                        script_pubkey: Some(utxo["scriptPubKey"].as_str().unwrap_or_default().to_string()),
                        content: None,
                        pubkey: None,
                        tags: Some(Vec::new()),
                        timestamp: None,
                        event_id: None,
                        signature: None,
                    },
                })
                .collect();
            Ok(unified_inputs)
        }
        Ok(resp) => Err(format!("Failed to fetch UTXOs: HTTP {}", resp.status())),
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

