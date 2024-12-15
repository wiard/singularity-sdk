use outixs::{anchor, utxo};

fn main() {
    // Initialize an anchor
    anchor::initialize_anchor();

    // Process a UTXO
    let tx_id = "sample_tx_id";
    if let Ok(result) = utxo::process_utxo(tx_id) {
        println!("UTXO processed: {:?}", result);
    }
}

