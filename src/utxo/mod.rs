pub mod fetch;
pub mod storage;

pub use fetch::fetch_utxos; // Re-export fetch_utxos for external usage
pub use storage::{initialize_db, store_utxos}; // Re-export storage functions

