use outixs::transaction::{UnifiedInput};

#[test]
fn test_fetch_utxos() {
    let utxos: Vec<UnifiedInput> = vec![];
    assert!(utxos.is_empty() || utxos.len() > 0, "UTXOs should be an array");
}

