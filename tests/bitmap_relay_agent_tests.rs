use crate::agents::bitmap_relay_agent::{register_relay, query_relays, verify_relay};

// Test relay registration
#[test]
fn test_register_relay() {
    let result = register_relay("123456", "wss://relay.example.com", "abcd1234");
    assert!(result.is_ok());
}

// Test querying relays
#[test]
fn test_query_relays() {
    let relays = query_relays(vec!["123456", "654321"]);
    assert!(!relays.is_empty());
}

// Test verification
#[test]
fn test_verify_relay() {
    let relay_data = "{\"url\":\"wss://relay.example.com\",\"pubkey\":\"abcd1234\"}";
    let is_verified = verify_relay("123456", relay_data);
    assert!(is_verified);
}

