mod ech;
mod agents;
mod utils;

use ech::orchestrator::Orchestrator;
use utils::bitmap_utils::load_inscriptions_from_file;

fn main() {
    // Load existing inscriptions from file
    load_inscriptions_from_file();

    let mut orchestrator = Orchestrator::new();

    // Register agents with RPC connection details
    orchestrator.register_bitmap_agent(
        "http://localhost:8332",
        "rpcuser",
        "rpcpassword",
    );

    orchestrator.register_utxo_agent(
        "http://localhost:8332",
        "rpcuser",
        "rpcpassword",
        "bc1exampleaddress",
    );

    orchestrator.run();
}

