use tungstenite::{connect, Message};
use tungstenite::error::Error as WsError;
use url::Url;
use std::{thread, time};

pub struct FreedomMesh {
    relay_url: String,
    pubkey: String,
}

impl FreedomMesh {
    pub fn new(relay_url: &str, pubkey: &str) -> Self {
        Self {
            relay_url: relay_url.to_string(),
            pubkey: pubkey.to_string(),
        }
    }

    pub fn register_relay(&self, parcel: &str) -> Result<(), String> {
        println!(
            "Registering relay for parcel {} with URL {} and public key {}...",
            parcel, self.relay_url, self.pubkey
        );

        if self.relay_url.is_empty() || parcel.is_empty() || self.pubkey.is_empty() {
            return Err("Invalid relay registration parameters.".to_string());
        }

        println!("Relay registered successfully!");
        Ok(())
    }

    pub fn connect_and_communicate(&self) {
        let max_attempts = 3;

        for attempt in 1..=max_attempts {
            println!(
                "Attempt {}: Connecting to relay at {}...",
                attempt, self.relay_url
            );

            match connect(Url::parse(&self.relay_url).expect("Invalid relay URL")) {
                Ok((mut ws, _)) => {
                    println!("Connected successfully to {} on attempt {}!", self.relay_url, attempt);

                    let event = serde_json::json!({
                        "id": "event_id_1",
                        "pubkey": self.pubkey,
                        "created_at": 1694281639,
                        "kind": 1,
                        "tags": [],
                        "content": "Hello from FreedomMesh!"
                    });

                    println!("Sending Nostr event...");
                    if let Err(err) = ws.write_message(Message::Text(event.to_string())) {
                        eprintln!("Failed to send message: {}", err);
                    }

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
    }
}

