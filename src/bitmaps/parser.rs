use crate::utxo::fetch::get_inscription;
use std::collections::HashMap;

// Parses relay data from a specific Bitmap parcel
#[allow(dead_code)]
pub fn parse_relay(parcel: &str) -> Option<HashMap<String, String>> {
    let inscription = get_inscription(parcel)?;
    if inscription.contains("url") {
        let relay_info: HashMap<String, String> = serde_json::from_str(&inscription).ok()?;
        Some(relay_info)
    } else {
        None
    }
}

