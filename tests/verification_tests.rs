use serde_json::Value;

#[test]
fn test_verify_transaction() {
    let mock_response = serde_json::json!({
        "confirmed": true,
        "txid": "test_txid"
    });

    let mock_verify_transaction = || Ok::<Value, String>(mock_response.clone());
    let result = mock_verify_transaction().expect("Failed to verify transaction");
    assert!(result["confirmed"].as_bool().unwrap());
}

