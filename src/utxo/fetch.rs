pub fn fetch_utxos(address: &str) -> Result<Vec<UnifiedInput>, reqwest::Error> {
    let url = format!("https://mempool.space/api/address/{}/utxo", address);
    let response: Vec<serde_json::Value> = reqwest::blocking::get(&url)?.json()?;
    
    let utxos = response.into_iter()
        .map(|utxo| UnifiedInput {
            source_type: InputSourceType::BitcoinUTXO,
            metadata: InputMetadata {
                id: utxo["txid"].as_str().unwrap_or_default().to_string(),
                value: Some(utxo["value"].as_u64().unwrap_or(0)),
                timestamp: None,
                script_pubkey: Some(utxo["scriptpubkey"].as_str().unwrap_or_default().to_string()),
                pubkey: None,
                content: None,
                tags: None,
            },
        })
        .collect();

    Ok(utxos)
}

