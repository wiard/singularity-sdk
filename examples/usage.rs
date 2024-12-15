use outixs::{
    utxo::{fetch_utxos, storage::{initialize_db, store_utxos}},
    nostr::relay::{discover_relays, send_message},
    verification::verify_transaction,
};

fn main() {
    // Example usage of fetch_utxos
    let address = "some_bitcoin_address";
    match fetch_utxos(address) {
        Ok(utxos) => {
            println!("Fetched UTXOs: {:?}", utxos);

            // Initialize database
            let mut conn = initialize_db().expect("Failed to initialize database");

            // Store UTXOs in database
            store_utxos(&mut conn, &utxos).expect("Failed to store UTXOs");
        }
        Err(err) => {
            eprintln!("Error fetching UTXOs: {}", err);
        }
    }

    // Discover Nostr relays
    match discover_relays() {
        Ok(relays) => println!("Discovered relays: {:?}", relays),
        Err(err) => eprintln!("Error discovering relays: {}", err),
    }

    // Send a message via Nostr relay
    if let Err(err) = send_message("some_relay_url", "Hello, Nostr!") {
        eprintln!("Error sending message: {}", err);
    }

    // Verify a transaction
    match verify_transaction("some_transaction_id") {
        Ok(verification) => println!("Transaction verified: {:?}", verification),
        Err(err) => eprintln!("Error verifying transaction: {}", err),
    }
}

