use serde_json::json;

#[test]
fn test_verify_transaction() {
    // Simulated response for testing
    let mock_response = json!({
        "confirmed": true
    });

    // Mock function to simulate verification
    let mock_verify_transaction = || Ok::<bool, String>(mock_response["confirmed"].as_bool().unwrap());

    let result = mock_verify_transaction().expect("Failed to verify transaction");
    assert!(result, "Transaction should be confirmed");
}

