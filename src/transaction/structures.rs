use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedTransaction {
    pub id: String,                      // Unique identifier for the transaction
    pub timestamp: u64,                  // UNIX timestamp of creation
    pub state: TransactionState,         // Current state of the transaction
    pub inputs: Vec<UnifiedInput>,       // Consolidated input details (UTXOs or Nostr events)
    pub outputs: Vec<UnifiedOutput>,     // Consolidated output details
    pub signature: Option<String>,       // Cryptographic signature (for verification)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedInput {
    pub source_type: InputSourceType,    // Type of the input (UTXO, NostrEvent)
    pub metadata: InputMetadata,         // Associated metadata for the input
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputSourceType {
    BitcoinUTXO,
    NostrEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMetadata {
    pub id: String,                      // ID of the source (txid for UTXO, event_id for Nostr)
    pub value: Option<u64>,              // Value in satoshis (optional for non-financial events)
    pub timestamp: Option<u64>,          // Timestamp of the source creation
    pub script_pubkey: Option<String>,   // Bitcoin-specific scriptPubKey
    pub pubkey: Option<String>,          // Public key (for Nostr events)
    pub content: Option<String>,         // Content of the Nostr event
    pub tags: Option<Vec<String>>,       // Tags associated with the Nostr event
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedOutput {
    pub address: Option<String>,         // Recipient address for Bitcoin transactions
    pub value: Option<u64>,              // Value in satoshis
    pub metadata: Option<String>,        // Metadata for non-financial outputs
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionState {
    Pending,
    Fulfilled,
    Finalized,
    Canceled,
}

