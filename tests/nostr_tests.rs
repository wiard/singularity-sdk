use mockito::{mock, server_url};
use outixs::nostr::relay::{discover_relays, send_message};

#[test]
fn test_send_message() {
    let mock_relay = mock("POST", "/")
        .with_status(200)
        .create();

    let relay_url = &server_url();
    let message = "Test message";

    let result = send_message(relay_url, message);
    assert!(result.is_ok());

    mock_relay.assert();
}

#[test]
fn test_discover_relays() {
    let mock_discover = mock("GET", "/")
        .with_status(200)
        .with_body(r#"["relay1", "relay2"]"#)
        .create();

    let relay_url = &server_url();
    let relays = discover_relays(relay_url).expect("Failed to discover relays");

    assert_eq!(relays, vec!["relay1", "relay2"]);

    mock_discover.assert();
}

