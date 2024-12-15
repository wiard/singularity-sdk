use reqwest::blocking::Client;
use serde_json::Value;

pub fn discover_relays(url: &str) -> Result<Vec<String>, reqwest::Error> {
    let response: Value = Client::new().get(url).send()?.json()?;
    Ok(response
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect())
}

pub fn send_message(relay_url: &str, message: &str) -> Result<(), String> {
    let client = Client::new();
    let res = client.post(relay_url).body(message.to_string()).send();

    match res {
        Ok(response) if response.status().is_success() => Ok(()),
        Ok(response) => Err(format!(
            "Failed to send message. HTTP Status: {}",
            response.status()
        )),
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

