use outixs::agents::bitmap_relay_agent::register_relay as imported_register_relay;
use tungstenite::{connect, Message};
use tungstenite::error::Error as WsError;
use url::Url;
use std::{thread, time};
use outixs::freedom_mesh::freedom_mesh::FreedomMesh;


fn main() {
    println!("Starting Bitmap relay usage example...");

    // New relay URL for testing
    let relay_url = "wss://relay.snort.social";
    let parcel = "123456";
    let pubkey = "pubkey1234";

    // Register relay using the imported function
    match imported_register_relay(parcel, relay_url, pubkey) {
        Ok(_) => println!("Relay successfully registered."),
        Err(err) => {
            eprintln!("Error during relay registration: {}", err);
            return; // Exit if registration fails
        }
    }

    // Attempt to connect to the relay
    let max_attempts = 3;
    for attempt in 1..=max_attempts {
        println!("Attempt {}: Connecting to relay at {}...", attempt, relay_url);

        match connect(Url::parse(relay_url).expect("Invalid relay URL")) {
            Ok((mut ws, _)) => {
                println!("Connected successfully to {} on attempt {}!", relay_url, attempt);

                // Send a basic Nostr event
                let event = serde_json::json!({
                    "id": "event_id_1",
                    "pubkey": pubkey,
                    "created_at": 1694281639,
                    "kind": 1,
                    "tags": [],
                    "content": "Hello from Bitmap usage example!"
                });

                println!("Sending Nostr event...");
                if let Err(err) = ws.write_message(Message::Text(event.to_string())) {
                    eprintln!("Failed to send message: {}", err);
                }

                // Listen for messages from the relay
                println!("Listening for messages...");
                loop {
                    match ws.read_message() {
                        Ok(message) => println!("Received: {}", message),
                        Err(WsError::ConnectionClosed) => {
                            println!("Connection closed by relay.");
                            break;
                        }
                        Err(err) => {
                            eprintln!("Error receiving message: {}", err);
                            break;
                        }
                    }
                }

                ws.close(None).expect("Failed to close the connection");
                println!("Connection closed gracefully.");
                break;
            }
            Err(err) => {
                eprintln!("Failed to connect on attempt {}: {}", attempt, err);
                if attempt < max_attempts {
                    println!("Retrying in 2 seconds...");
                    thread::sleep(time::Duration::from_secs(2));
                } else {
                    eprintln!("Max connection attempts reached. Exiting.");
                }
            }
        }
    }

    // Additional FreedomMesh logic
    println!("Integrating FreedomMesh functionality...");
    let mesh = FreedomMesh::new(relay_url, pubkey);
    mesh.connect_and_communicate();

    println!("Exiting Bitmap relay usage example...");
}

