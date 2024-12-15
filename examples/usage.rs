use outixs::{fetch_utxos, initialize_db, store_utxos, discover_relays, send_message, verify_transaction};

#[tokio::main]
async fn main() {
    // Step 1: Fetch UTXOs
    let address = "your_bitcoin_address_here";
    let utxos = fetch_utxos(address).await.expect("Failed to fetch UTXOs");
    println!("Fetched UTXOs: {:?}", utxos);

    // Step 2: Store UTXOs
    let conn = initialize_db("utxos.db").expect("Failed to initialize database");
    store_utxos(&conn, vec![
        ("txid1".to_string(), 0, 100000, "script_pubkey".to_string()),
    ])
    .expect("Failed to store UTXOs");

    // Step 3: Discover Nostr Relays
    let relays = discover_relays().await;
    println!("Discovered relays: {:?}", relays);

    // Step 4: Send a message
    let relay = &relays[0];
    send_message(relay, "Hello, Nostr!").await.expect("Failed to send message");

    // Step 5: Verify a transaction
    let txid = "your_txid_here";
    let is_confirmed = verify_transaction(txid).await.expect("Failed to verify transaction");
    println!("Transaction confirmed: {}", is_confirmed);
}

