use outixs::nostr::relay::{discover_relays, send_message};

fn main() {
    let relay_url = "wss://nostr-pub.wellorder.net"; // Use a real relay URL
    match discover_relays(relay_url) {
        Ok(relays) => println!("Discovered relays: {:?}", relays),
        Err(e) => eprintln!("Failed to discover relays: {}", e),
    }

    if let Err(e) = send_message(relay_url, "Hello, Nostr!") {
        eprintln!("Failed to send message: {}", e);
    }
}

