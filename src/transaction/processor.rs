pub fn compress_transaction(transaction: &UnifiedTransaction) -> String {
    let serialized = serde_json::to_string(transaction).unwrap();
    let compressed = base64::encode(serialized);
    compressed
}

pub fn anchor_transaction_to_bitcoin(transaction: &UnifiedTransaction) {
    let compressed = compress_transaction(transaction);
    println!("Anchor to Bitcoin: {}", compressed);
}

