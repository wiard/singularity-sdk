use outixs::agents::bitmap_relay_agent::register_relay;

fn main() {
    println!("Starting Bitmap relay usage example...");

    // Public Nostr relay URL for testing
    let relay_url = "wss://relay.damus.io";  
    let parcel = "123456";
    let pubkey = "pubkey1234";

    match register_relay(parcel, relay_url, pubkey) {
        Ok(_) => println!("Relay successfully registered."),
        Err(err) => eprintln!("Error: {}", err),
    }

    println!("Bitmap relay usage example complete.");
}

