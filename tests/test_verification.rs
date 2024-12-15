use outixs::verification::verify_transaction;

#[test]
fn test_verify_transaction() {
    let txid = "test_txid";
    let response = verify_transaction(txid).expect("Failed to verify transaction");
    assert!(response.is_object());
}

