# Outixs-SDK

Outixs-SDK is a lightweight software development kit designed to bridge the worlds of Bitcoin's UTXO model and Nostr's event-based architecture. The SDK provides a modular framework to fetch, process, and map Bitcoin UTXOs into an event-driven data structure, enabling efficient transaction management in off-chain layers and compressing validated transactions back to Bitcoin for immutable proof.

## Key Concept: Transactions as the Foundation

At the heart of the Outixs-SDK lies the **transaction**, which serves as the base unit for interfacing with both Bitcoin and Nostr. Transactions are central to the SDK for the following reasons:

### 1. **Mapping UTXOs to the Outixs Layer**
   - Bitcoin's UTXO (Unspent Transaction Output) model is inherently stateless and immutable. Outixs-SDK builds upon this by fetching UTXOs from Bitcoin, processing them, and mapping them into the `OutixsTransaction` data structure.
   - Each transaction is modeled in Outixs using the `OutixsTransaction` structure, which includes:
     - Inputs (`UTXOInput`)
     - Outputs (`UTXOOutput`)
     - Associated Nostr events
     - A state tracker (`TransactionState`) for managing off-chain workflows.

### 2. **Event Matching with Nostr**
   - The SDK provides a seamless way to integrate Nostr events into the transaction workflow.
   - By associating Nostr events with transactions, Outixs enables:
     - Proof of intent (via Nostr events).
     - Collaborative workflows (e.g., matching buyers and sellers or coordinating multi-sig operations).
     - Metadata anchoring for enhanced transparency and traceability.

### 3. **Off-Chain Processing and Compression**
   - Transactions in Outixs are validated, processed, and aggregated off-chain. This reduces the load on Bitcoin's blockchain while still providing a transparent, event-driven workflow.
   - Validated transactions can be compressed and re-anchored on Bitcoin using `OP_RETURN`, ensuring immutable proof of their existence and state in time.

### 4. **State Management**
   - The SDK tracks each transaction's lifecycle using `TransactionState`, which includes states like `Pending`, `Completed`, or `Failed`.
   - This simplifies workflow management and ensures clarity in off-chain operations.

---

## How It Works

### Transaction Lifecycle in Outixs-SDK

1. **Fetching UTXOs**:
   - UTXOs are fetched from Bitcoin via the `utxo::fetch` module.
   - These UTXOs are stored in a local database (`utxo::storage`) and mapped to the `OutixsTransaction` structure.

2. **Integrating Nostr Events**:
   - Each transaction can be associated with Nostr events (e.g., intents, agreements, or proofs) via the `nostr::relay` module.

3. **Processing Transactions Off-Chain**:
   - Transactions are processed, validated, and matched (e.g., buyer-seller matching) in the off-chain layer using `matching` and `transaction`.

4. **Anchoring to Bitcoin**:
   - Once validated, a batch of transactions can be compressed and re-anchored to Bitcoin using an `OP_RETURN` output. This ensures transparent, immutable proof.

---

## Example

Here's a quick example of creating and processing a transaction with Outixs-SDK:

```rust
use outixs::transaction::{OutixsTransaction, UTXOInput, UTXOOutput, TransactionState};
use serde_json::json;

fn main() {
    // Define UTXO input
    let input = UTXOInput {
        txid: "prev_txid".to_string(),
    };

    // Define UTXO output
    let output = UTXOOutput {
        address: "new_address".to_string(),
        amount: 100_000,
    };

    // Define a transaction
    let transaction = OutixsTransaction {
        id: "txid123".to_string(),
        inputs: vec![input],
        outputs: vec![output],
        nostr_event: None, // Optionally include a Nostr event
        timestamp: 0,
        state: TransactionState::Pending,
    };

    println!("Created transaction: {:?}", transaction);
}
```plaintext
outixs
├── Cargo.lock
├── Cargo.toml
├── README.md
├── examples
│   └── usage.rs
├── src
│   ├── anchor
│   │   └── mod.rs
│   ├── lib.rs
│   ├── matching
│   │   └── mod.rs
│   ├── nostr
│   │   ├── mod.rs
│   │   └── relay.rs
│   ├── transaction
│   │   ├── mod.rs
│   │   └── structures.rs
│   ├── utxo
│   │   ├── fetch.rs
│   │   ├── mod.rs
│   │   └── storage.rs
│   └── verification
│       └── mod.rs
└── tests
    ├── nostr_tests.rs
    ├── test_anchor.rs
    ├── test_matching.rs
    ├── test_nostr.rs
    ├── test_transaction.rs
    ├── test_utxo.rs
    ├── test_verification.rs
    ├── utxo_tests.rs
    └── verification_tests.rs
```

# Outixs SDK

Outixs SDK is a lightweight toolkit designed to interface between Bitcoin UTXOs and the event-based nature of Nostr. This SDK provides a foundational data structure and tools to fetch, manage, and verify transactions both off-chain and on-chain.

## Download and Installation

You can download the Outixs SDK by cloning the GitHub repository:

 ```bash
git clone [(https://github.com/wiard/Outixs-SDK.git)]
cd outixs
 ```
