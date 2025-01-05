use std::collections::{HashSet, HashMap};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Static store to track registered relays
lazy_static! {
    static ref REGISTERED_RELAYS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

// Static metadata map for relay inscriptions
lazy_static! {
    static ref RELAY_METADATA: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// Sync relays discovered from Bitmap inscriptions
#[allow(dead_code)]
pub fn sync_relays_from_bitmap(bitmap_range: Vec<&str>) {
    let relays = query_relays(bitmap_range);
    for relay in relays {
        if let Some(url) = relay.get("url") {
            if register_nostr_relay(url) {
                println!("Registered Nostr relay: {}", url);
            }
        }
    }
}

// Register a relay if not already registered
#[allow(dead_code)]
pub fn register_nostr_relay(url: &str) -> bool {
    let mut relays = REGISTERED_RELAYS.lock().unwrap();

    if relays.contains(url) {
        println!("Relay already registered: {}", url);
        return false;
    }

    relays.insert(url.to_string());
    println!("Nostr relay added: {}", url);
    true
}

// Inscribe relay metadata into RELAY_METADATA
#[allow(dead_code)]
pub fn inscribe_relay(parcel: &str, relay_url: &str, relay_pubkey: &str) {
    let inscription = format!("{{\"url\":\"{}\",\"pubkey\":\"{}\"}}", relay_url, relay_pubkey);
    let mut metadata = RELAY_METADATA.lock().unwrap();
    metadata.insert(parcel.to_string(), inscription);

    println!("Relay inscribed to parcel {}: {}", parcel, relay_url);
}

// Query and return list of registered relays
#[allow(dead_code)]
pub fn list_registered_relays() -> Vec<String> {
    let relays = REGISTERED_RELAYS.lock().unwrap();
    relays.iter().cloned().collect()
}

// Clear all registered relays (for testing or reset)
#[allow(dead_code)]
pub fn clear_registered_relays() {
    let mut relays = REGISTERED_RELAYS.lock().unwrap();
    relays.clear();
    println!("Cleared all registered relays.");
}

// Fetch relays from Bitmap parcels
#[allow(dead_code)]
pub fn query_relays(parcel_range: Vec<&str>) -> Vec<HashMap<String, String>> {
    let metadata = RELAY_METADATA.lock().unwrap();
    parcel_range
        .into_iter()
        .filter_map(|parcel| metadata.get(parcel))
        .filter_map(|data| serde_json::from_str(data).ok())
        .collect()
}

// Discover relays (stub for future expansion)
#[allow(dead_code)]
pub fn discover_relays() {
    println!("Discovering relays...");
}

// Send message to relays
#[allow(dead_code)]
pub fn send_message(message: &str) {
    println!("Sending message: {}", message);
}

// Fetch specific Nostr event (stub)
#[allow(dead_code)]
pub fn fetch_nostr_event(event_id: &str) {
    println!("Fetching Nostr event: {}", event_id);
}

// Unit tests for relay synchronization and inscription
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_nostr_relay() {
        clear_registered_relays();
        let url = "wss://relay1.example.com";
        assert!(register_nostr_relay(url));
        assert!(!register_nostr_relay(url));  // Should not register twice
    }

    #[test]
    fn test_inscribe_relay() {
        let parcel = "123456";
        inscribe_relay(parcel, "wss://relay2.example.com", "pubkey1234");
        let metadata = RELAY_METADATA.lock().unwrap();
        assert!(metadata.contains_key(parcel));
    }

    #[test]
    fn test_sync_relays_from_bitmap() {
        clear_registered_relays();
        let parcel = "654321";
        inscribe_relay(parcel, "wss://relay3.example.com", "pubkey5678");

        let parcels = vec![parcel];
        sync_relays_from_bitmap(parcels);
        let registered = list_registered_relays();
        assert_eq!(registered.len(), 1);
    }
}

