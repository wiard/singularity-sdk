use bitcoincore_rpc::{Auth, Client, RpcApi, Error as RpcError};
use bitcoincore_rpc::bitcoin::{Address, Network};
use bitcoincore_rpc::jsonrpc::error::Error as JsonRpcError;
use crate::ech::orchestrator::Agent;
use std::str::FromStr;
use std::fmt;

/// UtxoAgent for monitoring UTXOs at a given address
pub struct UtxoAgent {
    rpc_client: Client,
    address: String,
}

impl UtxoAgent {
    /// Create a new UtxoAgent with RPC credentials
    pub fn new(
        rpc_url: &str,
        rpc_user: &str,
        rpc_password: &str,
        address: &str,
    ) -> Result<Self, RpcError> {
        let rpc_client = Client::new(
            rpc_url.to_string(),
            Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
        )?;

        Ok(UtxoAgent {
            rpc_client,
            address: address.to_string(),
        })
    }

    /// Fetch UTXOs for the specified address
    pub fn fetch_utxos(&self) -> Result<Vec<String>, RpcError> {
        // Convert String to Bitcoin Address
        let address = Address::from_str(&self.address).map_err(|_| {
            RpcError::JsonRpc(JsonRpcError::Rpc(bitcoincore_rpc::jsonrpc::error::RpcError {
                code: -32602,
                message: "Invalid address".to_string(),
                data: None,
            }))
        })?;

        // Validate for the main network
        if address.network != Network::Bitcoin {
            return Err(RpcError::JsonRpc(JsonRpcError::Rpc(bitcoincore_rpc::jsonrpc::error::RpcError {
                code: -32602,
                message: "Address not valid for Bitcoin mainnet".to_string(),
                data: None,
            })));
        }

        // Call list_unspent with correct arguments
        let unspent = self.rpc_client.list_unspent(
            None,  // Min confirmations
            None,  // Max confirmations
            Some(&[&address]), // Address list (borrowed)
            None,  // Include unsafe (optional)
            None   // Query options (optional)
        )?;

        Ok(unspent
            .into_iter()
            .map(|utxo| format!("{}:{}", utxo.txid, utxo.vout))
            .collect())
    }
}

impl Agent for UtxoAgent {
    fn execute(&self) {
        match self.fetch_utxos() {
            Ok(utxos) => println!("UTXOs for {}: {:?}", self.address, utxos),
            Err(e) => eprintln!("Failed to fetch UTXOs for {}: {}", self.address, e),
        }
    }
}

impl fmt::Display for UtxoAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UtxoAgent tracking address: {}", self.address)
    }
}

