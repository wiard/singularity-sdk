use outixs::utxo::fetch::fetch_utxos;

#[test]
fn test_fetch_utxos() {
    // Replace with a valid Bitcoin address
    let test_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";

    match fetch_utxos(test_address) {
        Ok(utxos) => {
            println!("Fetched UTXOs: {:?}", utxos);
            assert!(utxos.is_array(), "UTXOs should be an array");
        }
        Err(e) => panic!("Failed to fetch UTXOs: {}", e),
    }
}

