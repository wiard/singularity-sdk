use crate::utils::bitmap_utils::inscribe_to_bitmap;
use std::collections::HashMap;

// Registers a relay by inscribing metadata to a Bitmap parcel
#[allow(dead_code)]
pub fn register_relay(parcel: &str, relay_url: &str, relay_pubkey: &str) -> Result<(), &'static str> {
    let relay_data = format!("{{\"url\":\"{}\",\"pubkey\":\"{}\"}}", relay_url, relay_pubkey);
    match inscribe_to_bitmap(parcel, &relay_data) {
        Ok(_) => {
            println!("Relay successfully registered to Bitmap parcel: {}", parcel);
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to inscribe relay: {}", err);
            Err(err)
        }
    }
}

// Queries parcels for relays and returns their metadata
#[allow(dead_code)]
pub fn query_relays(parcel_range: Vec<&str>) -> Vec<HashMap<String, String>> {
    parcel_range
        .into_iter()
        .filter_map(|parcel| get_inscription(parcel))
        .filter_map(|inscription| serde_json::from_str(&inscription).ok())
        .collect()
}

// Verifies relay data by comparing on-chain inscription
#[allow(dead_code)]
pub fn verify_relay(parcel: &str, relay_data: &str) -> bool {
    if let Some(onchain_data) = get_inscription(parcel) {
        return relay_data == onchain_data;
    }
    false
}

// Mock function to simulate fetching an inscription from a parcel
#[allow(dead_code)]
fn get_inscription(_parcel: &str) -> Option<String> {
    Some(format!("{{\"url\":\"wss://relay.example.com\",\"pubkey\":\"mockpubkey\"}}"))
}

