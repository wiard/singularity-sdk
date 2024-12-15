use outixs::utxo::fetch_utxos;

#[test]
fn test_fetch_utxos() {
    let address = "test_address";
    let utxos = fetch_utxos(address).expect("Failed to fetch UTXOs");
    assert!(utxos.is_array());
}

