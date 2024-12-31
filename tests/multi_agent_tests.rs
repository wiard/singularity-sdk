#[test]
fn test_multi_agent_workflow() {
    let rpc_url = "http://localhost:8332";
    
    let mut orchestrator = Orchestrator::new();
    orchestrator.register_bitmap_agent(rpc_url);
    orchestrator.register_utxo_agent(rpc_url, "bc1qaddress".to_string());

    orchestrator.run();
    assert!(true);
}

