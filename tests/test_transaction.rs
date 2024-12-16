#[cfg(test)]
mod tests {
    use super::super::transaction::structures::*;

    #[test]
    fn test_create_unified_transaction() {
        let input = UnifiedInput {
            source_type: InputSourceType::BitcoinUTXO,
            metadata: InputMetadata {
                id: "utxo_txid".to_string(),
                value: Some(100_000),
                timestamp: Some(1670000000),
                script_pubkey: Some("76a914...88ac".to_string()),
                pubkey: None,
                content: None,
                tags: None,
            },
        };

        let output = UnifiedOutput {
            address: Some("1BitcoinAddress...".to_string()),
            value: Some(90_000),
            metadata: None,
        };

        let transaction = UnifiedTransaction {
            id: "txn_123".to_string(),
            timestamp: 1670001000,
            state: TransactionState::Pending,
            inputs: vec![input],
            outputs: vec![output],
            signature: None,
        };

        assert_eq!(transaction.state, TransactionState::Pending);
        assert_eq!(transaction.inputs.len(), 1);
        assert_eq!(transaction.outputs.len(), 1);
    }
}

