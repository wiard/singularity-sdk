use mockito::{mock, server_url};
use outixs::nostr::relay::{discover_relays, send_message};

#[test]
fn test_discover_relays() {
    let _m = mock("GET", "/relays")
        .with_status(200)
        .with_body(r#"["relay1", "relay2", "relay3"]"#)
        .create();

    let url = format!("{}/relays", server_url());
    let relays = discover_relays(&url).expect("Failed to discover relays");
    assert_eq!(relays, vec!["relay1", "relay2", "relay3"]);
}

#[test]
fn test_send_message() {
    let _m = mock("POST", "/")
        .with_status(200)
        .with_body("OK")
        .create();

    let relay_url = &server_url();
    let message = "Test message";

    let result = send_message(relay_url, message);
    assert!(result.is_ok(), "Failed to send message: {}", result.unwrap_err());
}

