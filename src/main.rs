mod ech;
mod agents;

use ech::orchestrator::Orchestrator;

fn main() {
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

