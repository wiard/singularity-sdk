pub mod anchor;
pub mod matching;
pub mod utxo;
pub mod verification;

// Re-exports for SDK functions
pub use utxo::fetch::fetch_utxos;
pub use utxo::storage::{initialize_db, store_utxos};
pub use nostr::relay::{discover_relays, send_message};
pub use verification::verify_transaction;

// Example re-export
pub use anchor::initialize_anchor;

// Exposing `process_utxo` from the `utxo` module
pub use utxo::process_utxo;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

