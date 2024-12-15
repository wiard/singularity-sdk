pub fn process_utxo(tx_id: &str) -> Result<String, String> {
    if tx_id.is_empty() {
        Err("Transaction ID is empty".to_string())
    } else {
        Ok(format!("Processed UTXO with tx_id: {}", tx_id))
    }
}

