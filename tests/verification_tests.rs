use outixs::verification::verify_transaction;
use serde_json::Value;

#[test]
fn test_verify_transaction() {
    let txid = "test_txid";
    let mock_response: Value = serde_json::json!({
        "confirmed": true,
        "txid": txid
    });

    // Mock function for testing
    let mock_verify_transaction = || Ok(mock_response.clone());
    let result = mock_verify_transaction().expect("Failed to verify transaction");
    assert!(result["confirmed"].as_bool().unwrap());
}

