mod ech;
mod agents;
mod utils;

use ech::orchestrator::Orchestrator;
use utils::bitmap_utils::{load_inscriptions_from_file, list_all_inscriptions};

fn main() {
    println!("Loading inscriptions from file...");
    load_inscriptions_from_file();
    list_all_inscriptions();

    let mut orchestrator = Orchestrator::new();

    // Register Bitmap Agent with RPC connection details
    orchestrator.register_bitmap_agent(
        "http://localhost:8332",
        "rpcuser",
        "rpcpassword",
    );

    // Replace with a valid Bitcoin address
    orchestrator.register_utxo_agent(
        "http://localhost:8332",
        "rpcuser",
        "rpcpassword",
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",  // Real Bitcoin address
    );

    orchestrator.run();

    // Comment out to prevent clearing inscriptions after each run
    // clear_inscriptions();
}

