#[derive(Debug, Clone, PartialEq)]
pub struct UnifiedInput {
    pub source_type: InputSourceType,
    pub metadata: InputMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputSourceType {
    BitcoinUTXO,
    NostrEvent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputMetadata {
    pub txid: Option<String>,
    pub index: Option<u32>,
    pub amount: Option<u64>,
    pub script_pubkey: Option<String>,
    pub content: Option<String>,
    pub pubkey: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: Option<u64>,
    pub event_id: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnifiedOutput {
    pub address: String,
    pub amount: u64,
    pub script_pubkey: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnifiedTransaction {
    pub inputs: Vec<UnifiedInput>,
    pub outputs: Vec<UnifiedOutput>,
    pub state: TransactionState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Pending,
    Fulfilled,
    Finalized,
    Canceled,
}

