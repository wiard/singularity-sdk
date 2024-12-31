use crate::agents::{bitmap_agent::BitmapAgent, utxo_agent::UtxoAgent};
use std::fmt;

/// Define the Agent trait for dynamic execution of different agents
pub trait Agent: fmt::Display {
    fn execute(&self);
}

/// Orchestrator to manage and run multiple agents
pub struct Orchestrator {
    agents: Vec<Box<dyn Agent>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            agents: Vec::new(),
        }
    }

    /// Register BitmapAgent
    pub fn register_bitmap_agent(
        &mut self,
        rpc_url: &str,
        rpc_user: &str,
        rpc_password: &str,
    ) {
        match BitmapAgent::new(rpc_url, rpc_user, rpc_password) {
            Ok(agent) => {
                println!("Registered BitmapAgent: {}", agent);
                self.agents.push(Box::new(agent));
            }
            Err(e) => eprintln!("Failed to register BitmapAgent: {}", e),
        }
    }

    /// Register UtxoAgent
    pub fn register_utxo_agent(
        &mut self,
        rpc_url: &str,
        rpc_user: &str,
        rpc_password: &str,
        address: &str,
    ) {
        match UtxoAgent::new(rpc_url, rpc_user, rpc_password, address) {
            Ok(agent) => {
                println!("Registered UtxoAgent: {}", agent);
                self.agents.push(Box::new(agent));
            }
            Err(e) => eprintln!("Failed to register UtxoAgent: {}", e),
        }
    }

    /// Run all registered agents
    pub fn run(&self) {
        for agent in &self.agents {
            agent.execute();
        }
    }
}

