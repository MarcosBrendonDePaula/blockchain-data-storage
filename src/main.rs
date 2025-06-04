// src/main.rs

use blockchain_data_storage::core::Blockchain;
use blockchain_data_storage::network;
use blockchain_data_storage::rpc; // Importar o módulo RPC
use blockchain_data_storage::offchain_storage::OffChainStorageManager; // Importar o gerenciador de armazenamento off-chain

use clap::Parser;
use log::{info, error};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::select;
use tokio::signal::ctrl_c;

/// Command-line arguments for the blockchain node.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to store blockchain data.
    #[arg(short, long, value_name = "DIR", default_value = ".blockchain_data")]
    data_dir: PathBuf,

    /// Endereço para o servidor RPC
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:8000")]
    rpc_addr: String,

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
    info!("RPC server address: {}", cli.rpc_addr);

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

    // Inicializar o gerenciador de armazenamento off-chain
    let offchain_storage_path = cli.data_dir.join("offchain_data");
    std::fs::create_dir_all(&offchain_storage_path)?;
    let offchain_storage = Arc::new(OffChainStorageManager::new(&offchain_storage_path)?);
    info!("Off-chain storage initialized at {:?}", offchain_storage_path);

    // Wrap Blockchain in Arc<Mutex> for safe sharing
    let blockchain_arc = Arc::new(Mutex::new(blockchain));
    info!("Blockchain state prepared for concurrent access.");

    // Iniciar o servidor RPC em uma thread separada (não em uma task do Tokio)
    let rpc_blockchain = blockchain_arc.clone();
    let rpc_offchain_storage = offchain_storage.clone();
    let rpc_addr = cli.rpc_addr.clone();
    
    // Usando uma thread std para o servidor RPC
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            info!("Starting RPC server on {}", rpc_addr);
            if let Err(e) = rpc::start_rpc_server(rpc_addr, rpc_blockchain, rpc_offchain_storage).await {
                error!("RPC server error: {}", e);
            }
        });
    });

    info!("Node initialization complete. Starting network loop...");

    // Executar o nó de rede com tratamento de sinal para encerramento
    select! {
        result = network::start_network_node(blockchain_arc) => {
            if let Err(e) = result {
                error!("Network node encountered a fatal error: {}", e);
                return Err(e);
            }
            info!("Network node loop exited gracefully.");
        }
        _ = ctrl_c() => {
            info!("Received shutdown signal. Stopping blockchain node...");
        }
    }

    Ok(())
}
