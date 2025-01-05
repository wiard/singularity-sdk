use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Simulated UTXO inscription store for testing and development
lazy_static! {
    static ref UTXO_INSCRIPTIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// Fetch inscriptions from UTXO parcels
#[allow(dead_code)]
pub fn get_inscription(parcel: &str) -> Option<String> {
    let utxo_store = UTXO_INSCRIPTIONS.lock().unwrap();
    utxo_store.get(parcel).cloned()
}

// Store a new inscription to simulate UTXO anchoring
#[allow(dead_code)]
pub fn store_inscription(parcel: &str, inscription: &str) {
    let mut utxo_store = UTXO_INSCRIPTIONS.lock().unwrap();
    utxo_store.insert(parcel.to_string(), inscription.to_string());
    println!("Inscription stored for parcel {}: {}", parcel, inscription);
}

// Fetch all UTXOs (Simulated) – This would call Bitcoin RPC in real implementation
#[allow(dead_code)]
pub fn fetch_utxos() -> Vec<String> {
    let utxo_store = UTXO_INSCRIPTIONS.lock().unwrap();
    utxo_store.keys().cloned().collect()
}

// Check if a parcel has an inscription
#[allow(dead_code)]
pub fn has_inscription(parcel: &str) -> bool {
    let utxo_store = UTXO_INSCRIPTIONS.lock().unwrap();
    utxo_store.contains_key(parcel)
}

// Clear all stored UTXO inscriptions (for testing or reset)
#[allow(dead_code)]
pub fn clear_inscriptions() {
    let mut utxo_store = UTXO_INSCRIPTIONS.lock().unwrap();
    utxo_store.clear();
    println!("All UTXO inscriptions cleared.");
}

// Preload mock data for testing
#[allow(dead_code)]
pub fn preload_mock_inscriptions() {
    store_inscription("123456", "{\"url\":\"wss://relay1.example.com\",\"pubkey\":\"abcd1234\"}");
    store_inscription("654321", "{\"url\":\"wss://relay2.example.com\",\"pubkey\":\"efgh5678\"}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_fetch_inscription() {
        clear_inscriptions();
        store_inscription("789101", "wss://relay.test.com");
        let inscription = get_inscription("789101");
        assert_eq!(inscription, Some("wss://relay.test.com".to_string()));
    }

    #[test]
    fn test_inscription_existence() {
        clear_inscriptions();
        store_inscription("555555", "wss://relay.exist.com");
        assert!(has_inscription("555555"));
        assert!(!has_inscription("notfound"));
    }

    #[test]
    fn test_fetch_utxos() {
        clear_inscriptions();
        store_inscription("333333", "wss://relay3.example.com");
        let utxos = fetch_utxos();
        assert!(utxos.contains(&"333333".to_string()));
    }
}

