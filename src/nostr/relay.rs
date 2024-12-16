pub fn fetch_nostr_event(event_id: &str) -> Result<UnifiedInput, reqwest::Error> {
    let url = format!("https://relay.nostr.example.com/event/{}", event_id);
    let event: serde_json::Value = reqwest::blocking::get(&url)?.json()?;
    
    let nostr_event = UnifiedInput {
        source_type: InputSourceType::NostrEvent,
        metadata: InputMetadata {
            id: event["id"].as_str().unwrap_or_default().to_string(),
            value: None,
            timestamp: Some(event["created_at"].as_u64().unwrap_or(0)),
            script_pubkey: None,
            pubkey: Some(event["pubkey"].as_str().unwrap_or_default().to_string()),
            content: Some(event["content"].as_str().unwrap_or_default().to_string()),
            tags: event["tags"].as_array().map(|tags| {
                tags.iter().map(|tag| tag.as_str().unwrap_or_default().to_string()).collect()
            }),
        },
    };

    Ok(nostr_event)
}

