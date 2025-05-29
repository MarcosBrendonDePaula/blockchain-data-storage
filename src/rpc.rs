// src/rpc.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use log::{info, error, warn};

use crate::core::{Blockchain, Transaction, Block, BlockchainError};
use crate::storage::StorageError; // Assuming StorageError exists and is public

// --- JSON-RPC Structures ---

#[derive(Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value, // Use Value for flexibility initially
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

#[derive(Deserialize, Debug)]
struct SendTransactionParams {
    transaction: Transaction, // Assuming Transaction implements Deserialize
}

#[derive(Deserialize, Debug)]
struct GetBlockByHeightParams {
    height: u64,
}

#[derive(Deserialize, Debug)]
struct GetBlockByHashParams {
    hash: String, // Hex-encoded hash
}

// --- Application State ---

// Holds the shared blockchain state for handlers
struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
}

// --- RPC Handler Function ---

// Generic handler for all JSON-RPC requests
#[post("/")]
async fn rpc_handler(req_body: web::Json<JsonRpcRequest>, data: web::Data<AppState>) -> impl Responder {
    let request_id = req_body.id.clone();
    let method = req_body.method.as_str();
    let params = req_body.params.clone();
    let blockchain_arc = data.blockchain.clone();

    info!("RPC Request Received - Method: {}, ID: {:?}", method, request_id);

    let response = match method {
        "send_transaction" => handle_send_transaction(params, blockchain_arc).await,
        "get_chain_height" => handle_get_chain_height(blockchain_arc).await,
        "get_block_by_height" => handle_get_block_by_height(params, blockchain_arc).await,
        "get_block_by_hash" => handle_get_block_by_hash(params, blockchain_arc).await,
        // Add other method handlers here
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
) -> JsonRpcResponse<String> { // Returns Tx Hash on success
    match serde_json::from_value::<SendTransactionParams>(params) {
        Ok(parsed_params) => {
            let tx = parsed_params.transaction;
            let tx_hash = tx.calculate_hash();
            let tx_hash_hex = hex::encode(tx_hash);
            info!("Processing send_transaction for hash: {}", tx_hash_hex);

            match blockchain.lock().expect("Blockchain lock poisoned").add_pending_transaction(tx) {
                Ok(added) => {
                    if added {
                        info!("Transaction {} added to mempool via RPC.", tx_hash_hex);
                        // TODO: Propagate transaction via network service?
                        create_success_response(None, tx_hash_hex) // Return hash
                    } else {
                        warn!("Transaction {} already exists in mempool (RPC submission).", tx_hash_hex);
                        // Still return success as the tx is known
                        create_success_response(None, tx_hash_hex)
                    }
                }
                Err(e) => {
                    error!("Failed to add transaction {} via RPC: {}", tx_hash_hex, e);
                    create_error_response(None, -32000, format!("Failed to add transaction: {}", e), None)
                }
            }
        }
        Err(e) => {
            error!("Failed to parse send_transaction params: {}", e);
            create_error_response(None, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
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
                Ok(None) => create_success_response(None, None), // Block not found is not an error
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
                            Ok(None) => create_success_response(None, None), // Not found is not an error
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

// --- Helper Functions for Responses ---

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
) -> std::io::Result<()> {
    info!("Starting RPC server on {}", bind_address);

    let app_state = web::Data::new(AppState {
        blockchain: blockchain.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
            // Potentially add middleware for logging, CORS, etc.
    })
    .bind(bind_address)?
    .run()
    .await
}

