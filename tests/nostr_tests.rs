use outixs::nostr::relay::send_message;
use mockito::{mock, server_url};

#[test]
fn test_send_message() {
    let _m = mock("POST", "/")
        .with_status(200)
        .with_body("OK")
        .create();

    let relay_url = "https://nostr-relay.wlvs.space"; 
    let message = "Test message";

    let result = send_message(relay_url, message);
    assert!(result.is_ok(), "Failed to send message: {:?}", result.err());
}

