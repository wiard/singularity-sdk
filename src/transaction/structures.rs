use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)] // Added PartialEq
pub enum TransactionState {
    Pending,
    Fulfilled,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UTXOInput {
    pub txid: String,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UTXOOutput {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NostrEvent {
    pub event_id: String,
    pub event_data: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutixsTransaction {
    pub id: String,
    pub inputs: Vec<UTXOInput>,
    pub outputs: Vec<UTXOOutput>,
    pub nostr_event: Option<NostrEvent>,
    pub timestamp: u64,
    pub state: TransactionState,
}

impl OutixsTransaction {
    pub fn new(
        id: String,
        inputs: Vec<UTXOInput>,
        outputs: Vec<UTXOOutput>,
        nostr_event: Option<NostrEvent>,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            inputs,
            outputs,
            nostr_event,
            timestamp,
            state: TransactionState::Pending,
        }
    }

    pub fn fulfill(&mut self) {
        self.state = TransactionState::Fulfilled;
    }

    pub fn fail(&mut self) {
        self.state = TransactionState::Failed;
    }
}

