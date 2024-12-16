use reqwest::blocking::Client;
use serde_json::Value;

/// Discover available Nostr relays.
pub fn discover_relays(url: &str) -> Result<Vec<String>, String> {
    let client = Client::new();
    let response = client.get(url).send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let relays: Vec<String> = resp.json().unwrap_or_default();
            Ok(relays)
        }
        Ok(resp) => Err(format!("Failed to discover relays: HTTP {}", resp.status())),
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

/// Send a message to a specific Nostr relay.
pub fn send_message(relay: &str, message: &str) -> Result<(), String> {
    let client = Client::new();
    let response = client.post(relay).body(message.to_string()).send();

    match response {
        Ok(resp) if resp.status().is_success() => Ok(()),
        Ok(resp) => Err(format!("Failed to send message: HTTP {}", resp.status())),
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

/// Fetch a Nostr event by its event ID.
pub fn fetch_nostr_event(event_id: &str) -> Result<Value, String> {
    let url = format!("https://mempool.space/api/event/{}", event_id);
    let client = Client::new();
    let response = client.get(&url).send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let event: Value = resp.json().unwrap_or_default();
            Ok(event)
        }
        Ok(resp) => Err(format!("Event not found: HTTP {}", resp.status())),
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

