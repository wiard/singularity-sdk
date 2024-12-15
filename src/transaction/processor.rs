use crate::transaction::structures::{OutixsTransaction, TransactionState};
use crate::compression;

/// Transaction Processor for Outixs
///
/// This module handles the processing, validation, and preparation
/// of transactions before compression and submission to Bitcoin.

pub fn process_transaction(transaction: &mut OutixsTransaction) {
    // Validate the transaction
    validate_transaction(transaction);

    // Transition state to "Processed"
    transaction.state = TransactionState::Processed;

    // Optionally compress the transaction data
    let serialized = serde_json::to_string(transaction).expect("Failed to serialize transaction");
    let compressed = compression::compress_data(&serialized);

    // Log the compressed data size
    println!(
        "Transaction processed and compressed. Size: {} bytes",
        compressed.len()
    );
}

fn validate_transaction(transaction: &OutixsTransaction) {
    if transaction.inputs.is_empty() {
        panic!("Transaction must have at least one input");
    }
    if transaction.outputs.is_empty() {
        panic!("Transaction must have at least one output");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::structures::{OutixsTransaction, UTXOInput, UTXOOutput, NostrEvent};

    #[test]
    fn test_process_transaction() {
        let input = UTXOInput {
            txid: "input_txid".to_string(),
        };

        let output = UTXOOutput {
            address: "output_address".to_string(),
            value: 1000,
        };

        let nostr_event = NostrEvent {
            event_data: "Sample Nostr Event".to_string(),
        };

        let mut transaction = OutixsTransaction {
            id: "tx_123".to_string(),
            inputs: vec![input],
            outputs: vec![output],
            nostr_event,
            state: TransactionState::Pending,
            timestamp: 0,
        };

        process_transaction(&mut transaction);

        assert_eq!(transaction.state, TransactionState::Processed);
    }
}

