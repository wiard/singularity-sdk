use outixs::transaction::{OutixsTransaction, UTXOInput, UTXOOutput, NostrEvent, TransactionState};

#[test]
fn test_transaction() {
    // Create a UTXO input
    let input = UTXOInput {
        txid: "test_txid".to_string(),
        index: 0,
    };

    // Create a UTXO output
    let output = UTXOOutput {
        address: "1BitcoinAddress".to_string(),
        amount: 100_000,
    };

    // Create a Nostr event
    let event = NostrEvent {
        event_id: "event_id".to_string(),
        event_data: "Test event".to_string(),
        timestamp: 1672531200,
    };

    // Create an Outixs transaction
    let mut transaction = OutixsTransaction {
        id: "transaction_id".to_string(),
        inputs: vec![input],
        outputs: vec![output],
        nostr_event: Some(event),
        timestamp: 1672531200,
        state: TransactionState::Pending,
    };

    // Assertions
    assert_eq!(transaction.id, "transaction_id");
    assert_eq!(transaction.inputs.len(), 1);
    assert_eq!(transaction.outputs.len(), 1);
    assert!(transaction.nostr_event.is_some());
    assert_eq!(transaction.state, TransactionState::Pending);

    // Update transaction state
    transaction.fulfill();
    assert_eq!(transaction.state, TransactionState::Fulfilled);

    transaction.fail();
    assert_eq!(transaction.state, TransactionState::Failed);
}

