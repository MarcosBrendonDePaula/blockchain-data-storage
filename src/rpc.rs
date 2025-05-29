// src/rpc.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use log::{info, error, warn};
use base64::{Engine as _, engine::general_purpose::STANDARD as base64_engine}; // For payload encoding

use crate::core::{Blockchain, Transaction, Block, BlockchainError, Hash};
use crate::storage::StorageError;
use crate::offchain_storage::{OffChainStorageManager, OffChainStorageError}; // Import offchain storage

// --- JSON-RPC Structures (Keep existing ones) ---

#[derive(Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<JsonRpcError>,
    id: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

// --- Specific Request Parameter Structures ---

// Combined structure for sending transactions (transfer or storage)
#[derive(Deserialize, Debug)]
struct SendTransactionParams {
    // For transfer
    sender: Option<Vec<u8>>,
    recipient: Option<Vec<u8>>,
    amount: Option<u64>,
    // For storage
    payload_base64: Option<String>, // Payload data encoded in base64
    // Common (or inferred)
    // We might need sender even for storage tx
}

#[derive(Deserialize, Debug)]
struct GetBlockByHeightParams {
    height: u64,
}

#[derive(Deserialize, Debug)]
struct GetBlockByHashParams {
    hash: String, // Hex-encoded hash
}

#[derive(Deserialize, Debug)]
struct GetOffchainDataParams {
    hash: String, // Hex-encoded hash of the payload
}

// --- Application State ---

// Holds the shared state for handlers
struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
    offchain_storage: Arc<OffChainStorageManager>, // Add offchain storage manager
}

// --- RPC Handler Function ---

#[post("/")]
async fn rpc_handler(req_body: web::Json<JsonRpcRequest>, data: web::Data<AppState>) -> impl Responder {
    let request_id = req_body.id.clone();
    let method = req_body.method.as_str();
    let params = req_body.params.clone();
    let blockchain_arc = data.blockchain.clone();
    let offchain_storage_arc = data.offchain_storage.clone(); // Clone Arc for offchain storage

    info!("RPC Request Received - Method: {}, ID: {:?}", method, request_id);

    let response = match method {
        "send_transaction" => handle_send_transaction(params, blockchain_arc, offchain_storage_arc).await,
        "get_chain_height" => handle_get_chain_height(blockchain_arc).await,
        "get_block_by_height" => handle_get_block_by_height(params, blockchain_arc).await,
        "get_block_by_hash" => handle_get_block_by_hash(params, blockchain_arc).await,
        "get_offchain_data" => handle_get_offchain_data(params, offchain_storage_arc).await, // Add new handler
        _ => {
            error!("Unsupported RPC method: {}", method);
            create_error_response::<()>(
                request_id,
                -32601,
                "Method not found".to_string(),
                None,
            )
        }
    };

    HttpResponse::Ok().json(response)
}

// --- Specific Method Handlers ---

async fn handle_send_transaction(
    params: serde_json::Value,
    blockchain: Arc<Mutex<Blockchain>>,
    offchain_storage: Arc<OffChainStorageManager>,
) -> JsonRpcResponse<String> { // Returns Tx Hash on success
    match serde_json::from_value::<SendTransactionParams>(params.clone()) { // Clone params for parsing
        Ok(parsed_params) => {
            let tx_result = if let Some(payload_base64) = parsed_params.payload_base64 {
                // --- Storage Transaction --- 
                info!("Processing send_transaction (storage type)");
                let sender = parsed_params.sender.ok_or_else(|| "Missing sender for storage transaction".to_string())?;
                match base64_engine.decode(payload_base64) {
                    Ok(payload_data) => {
                        let data_size = payload_data.len() as u64;
                        match offchain_storage.store_payload(&payload_data) {
                            Ok(payload_hash) => {
                                let tx = Transaction::new_storage(sender, payload_hash, data_size);
                                Ok(tx)
                            }
                            Err(e) => Err(format!("Failed to store offchain payload: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Invalid base64 payload data: {}", e)),
                }
            } else if let (Some(sender), Some(recipient), Some(amount)) = (parsed_params.sender, parsed_params.recipient, parsed_params.amount) {
                // --- Transfer Transaction --- 
                info!("Processing send_transaction (transfer type)");
                let tx = Transaction::new_transfer(sender, recipient, amount);
                Ok(tx)
            } else {
                Err("Invalid parameters: Provide either payload_base64 and sender, or sender, recipient, and amount.".to_string())
            };

            match tx_result {
                Ok(tx) => {
                    let tx_hash = tx.calculate_hash();
                    let tx_hash_hex = hex::encode(tx_hash);
                    match blockchain.lock().expect("Blockchain lock poisoned").add_pending_transaction(tx) {
                        Ok(added) => {
                            if added {
                                info!("Transaction {} added to mempool via RPC.", tx_hash_hex);
                            } else {
                                warn!("Transaction {} already exists in mempool (RPC submission).", tx_hash_hex);
                            }
                            create_success_response(None, tx_hash_hex) // Return hash
                        }
                        Err(e) => {
                            error!("Failed to add transaction {} via RPC: {}", tx_hash_hex, e);
                            create_error_response(None, -32000, format!("Failed to add transaction: {}", e), None)
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create transaction from RPC params: {}", e);
                    create_error_response(None, -32602, "Invalid params for transaction type".to_string(), Some(serde_json::json!(e)))
                }
            }
        }
        Err(e) => {
            error!("Failed to parse send_transaction params: {}", e);
            create_error_response(None, -32602, "Invalid params structure".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}

async fn handle_get_chain_height(
    blockchain: Arc<Mutex<Blockchain>>,
) -> JsonRpcResponse<Option<u64>> {
    let height = blockchain.lock().expect("Blockchain lock poisoned").get_chain_height();
    info!("Processing get_chain_height. Result: {:?}", height);
    create_success_response(None, height)
}

async fn handle_get_block_by_height(
    params: serde_json::Value,
    blockchain: Arc<Mutex<Blockchain>>,
) -> JsonRpcResponse<Option<Block>> {
    match serde_json::from_value::<GetBlockByHeightParams>(params) {
        Ok(parsed_params) => {
            let height = parsed_params.height;
            info!("Processing get_block_by_height for height: {}", height);
            match blockchain.lock().expect("Blockchain lock poisoned").get_block_by_height(height) {
                Ok(Some(block)) => create_success_response(None, Some(block)),
                Ok(None) => create_success_response(None, None),
                Err(e) => {
                    error!("Error fetching block by height {}: {}", height, e);
                    create_error_response(None, -32001, format!("Storage error: {}", e), None)
                }
            }
        }
        Err(e) => {
            error!("Failed to parse get_block_by_height params: {}", e);
            create_error_response(None, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}

async fn handle_get_block_by_hash(
    params: serde_json::Value,
    blockchain: Arc<Mutex<Blockchain>>,
) -> JsonRpcResponse<Option<Block>> {
    match serde_json::from_value::<GetBlockByHashParams>(params) {
        Ok(parsed_params) => {
            let hash_hex = parsed_params.hash;
            info!("Processing get_block_by_hash for hash: {}", hash_hex);
            match hex::decode(&hash_hex) {
                Ok(hash_bytes) => {
                    if hash_bytes.len() == 32 {
                        let mut hash_array = [0u8; 32];
                        hash_array.copy_from_slice(&hash_bytes);
                        match blockchain.lock().expect("Blockchain lock poisoned").get_block_by_hash(&hash_array) {
                            Ok(Some(block)) => create_success_response(None, Some(block)),
                            Ok(None) => create_success_response(None, None),
                            Err(e) => {
                                error!("Error fetching block by hash {}: {}", hash_hex, e);
                                create_error_response(None, -32001, format!("Storage error: {}", e), None)
                            }
                        }
                    } else {
                        create_error_response(None, -32602, "Invalid hash length".to_string(), None)
                    }
                }
                Err(_) => {
                    create_error_response(None, -32602, "Invalid hex string for hash".to_string(), None)
                }
            }
        }
        Err(e) => {
            error!("Failed to parse get_block_by_hash params: {}", e);
            create_error_response(None, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}

// New handler for retrieving off-chain data
async fn handle_get_offchain_data(
    params: serde_json::Value,
    offchain_storage: Arc<OffChainStorageManager>,
) -> JsonRpcResponse<Option<String>> { // Returns base64 encoded data
    match serde_json::from_value::<GetOffchainDataParams>(params) {
        Ok(parsed_params) => {
            let hash_hex = parsed_params.hash;
            info!("Processing get_offchain_data for hash: {}", hash_hex);
            match hex::decode(&hash_hex) {
                Ok(hash_bytes) => {
                    if hash_bytes.len() == 32 {
                        let mut hash_array = [0u8; 32];
                        hash_array.copy_from_slice(&hash_bytes);
                        match offchain_storage.retrieve_payload(&hash_array) {
                            Ok(payload_data) => {
                                let payload_base64 = base64_engine.encode(payload_data);
                                create_success_response(None, Some(payload_base64))
                            }
                            Err(OffChainStorageError::NotFound(_)) => {
                                // Data not found is not a JSON-RPC error, return null result
                                create_success_response::<Option<String>>(None, None)
                            }
                            Err(e) => {
                                error!("Error retrieving offchain data for hash {}: {}", hash_hex, e);
                                create_error_response(None, -32002, format!("Offchain storage error: {}", e), None)
                            }
                        }
                    } else {
                        create_error_response(None, -32602, "Invalid hash length".to_string(), None)
                    }
                }
                Err(_) => {
                    create_error_response(None, -32602, "Invalid hex string for hash".to_string(), None)
                }
            }
        }
        Err(e) => {
            error!("Failed to parse get_offchain_data params: {}", e);
            create_error_response(None, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}

// --- Helper Functions for Responses (Keep existing ones) ---

fn create_success_response<T: Serialize>(
    id: Option<serde_json::Value>,
    result: T,
) -> JsonRpcResponse<T> {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(result),
        error: None,
        id,
    }
}

fn create_error_response<T>(
    id: Option<serde_json::Value>,
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
) -> JsonRpcResponse<T> {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(JsonRpcError { code, message, data }),
        id,
    }
}

// --- Server Startup Function ---

/// Starts the JSON-RPC HTTP server.
pub async fn start_rpc_server(
    bind_address: String,
    blockchain: Arc<Mutex<Blockchain>>,
    offchain_storage: Arc<OffChainStorageManager>, // Add offchain storage manager
) -> std::io::Result<()> {
    info!("Starting RPC server on {}", bind_address);

    // Create AppState with both managers
    let app_state = web::Data::new(AppState {
        blockchain,
        offchain_storage,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    })
    .bind(bind_address)?
    .run()
    .await
}

