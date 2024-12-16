use outixs::transaction::{
    UnifiedInput, UnifiedOutput, UnifiedTransaction, InputSourceType, InputMetadata, TransactionState,
};

#[test]
fn test_transaction_structure() {
    let input = UnifiedInput {
        source_type: InputSourceType::BitcoinUTXO,
        metadata: InputMetadata {
            txid: Some("test_txid".to_string()),
            index: Some(0),
            amount: Some(100_000),
            script_pubkey: Some("76a914...88ac".to_string()),
            content: None,
            pubkey: None,
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            timestamp: Some(1672531200),
            event_id: None,
            signature: None,
        },
    };

    let output = UnifiedOutput {
        address: "test_address".to_string(),
        amount: 100_000,
        script_pubkey: Some("76a914...88ac".to_string()),
    };

    let transaction = UnifiedTransaction {
        inputs: vec![input],
        outputs: vec![output],
        state: TransactionState::Pending,
    };

    assert_eq!(transaction.state, TransactionState::Pending);
}

