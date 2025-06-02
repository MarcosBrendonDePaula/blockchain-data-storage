// src/main.rs

use blockchain_data_storage::core::Blockchain;
use blockchain_data_storage::network;
// StorageManager is now handled internally by Blockchain::new
// use blockchain_data_storage::storage::StorageManager;

use clap::Parser;
use log::{info, error};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Command-line arguments for the blockchain node.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to store blockchain data.
    #[arg(short, long, value_name = "DIR", default_value = ".blockchain_data")]
    data_dir: PathBuf,

    // TODO: Add arguments for listen address, bootstrap peers, etc.
    // #[arg(short, long, value_name = "MULTIADDR")]
    // listen_address: Option<String>,

    // #[arg(short, long, value_name = "MULTIADDR")]
    // bootstrap_peer: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command-line arguments
    let cli = Cli::parse();
    info!("Starting blockchain node...");
    info!("Data directory: {:?}", cli.data_dir);

    // Storage Manager is initialized within Blockchain::new

    // Initialize Blockchain - Pass the data directory path directly
    let mut blockchain = match Blockchain::new(&cli.data_dir) { // Pass path
        Ok(bc) => {
            info!("Blockchain core initialized successfully.");
            bc
        }
        Err(e) => {
            error!("Failed to initialize blockchain core: {}", e);
            return Err(e.into());
        }
    };

    // Initialize Genesis Block if needed - Adjust match arms for Ok(())
    match blockchain.initialize_genesis_if_needed() {
        Ok(()) => info!("Genesis block checked/initialized successfully."), // Handle Ok(())
        Err(e) => {
            error!("Failed during genesis block check/initialization: {}", e);
            return Err(e.into());
        }
    }

    // Wrap Blockchain in Arc<Mutex> for safe sharing
    let blockchain_arc = Arc::new(Mutex::new(blockchain));
    info!("Blockchain state prepared for concurrent access.");

    info!("Node initialization complete. Starting network loop...");

    // Start the network node loop
    // This function runs indefinitely, handling network events.
    if let Err(e) = network::start_network_node(blockchain_arc).await {
        error!("Network node encountered a fatal error: {}", e);
        return Err(e);
    }

    // In theory, start_network_node should run forever or until a shutdown signal.
    // If it returns without error, it might indicate a planned shutdown.
    info!("Network node loop exited gracefully.");

    Ok(())
}

