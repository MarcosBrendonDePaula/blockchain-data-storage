// Importações necessárias para os novos endpoints
use std::time::{SystemTime, UNIX_EPOCH};

// Adicionando estruturas para os novos endpoints
#[derive(Deserialize, Debug)]
struct GetBalanceParams {
    address: String, // Endereço da carteira em formato hexadecimal
}

#[derive(Deserialize, Debug)]
struct CreateTokenParams {
    creator_address: String, // Endereço do criador do token em formato hexadecimal
    token_name: String,      // Nome do token
    token_symbol: String,    // Símbolo do token (abreviação)
    initial_supply: u64,     // Suprimento inicial do token
}

// Implementação dos handlers para os novos endpoints
async fn handle_get_balance(
    params: serde_json::Value,
    blockchain: Arc<Mutex<Blockchain>>,
) -> JsonRpcResponse<serde_json::Value> {
    let request_id = None;
    match serde_json::from_value::<GetBalanceParams>(params) {
        Ok(parsed_params) => {
            let address_hex = parsed_params.address;
            info!("Processing get_balance for address: {}", address_hex);
            
            match hex::decode(&address_hex) {
                Ok(address_bytes) => {
                    // Aqui precisamos implementar a lógica para calcular o saldo
                    // Isso envolve percorrer as transações na blockchain
                    let bc_guard = blockchain.lock().expect("Blockchain lock poisoned");
                    
                    // Implementação temporária: retorna um saldo fixo para teste
                    // Em uma implementação real, percorreríamos todas as transações
                    // para calcular o saldo real do endereço
                    let balance = 1000; // Valor temporário para teste
                    
                    info!("Balance for address {}: {}", address_hex, balance);
                    create_success_response(request_id, serde_json::json!({ "balance": balance }))
                },
                Err(_) => {
                    error!("Invalid hex string for address: {}", address_hex);
                    create_error_response(request_id, -32602, "Invalid hex string for address".to_string(), None)
                }
            }
        },
        Err(e) => {
            error!("Failed to parse get_balance params: {}", e);
            create_error_response(request_id, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}

async fn handle_create_token(
    params: serde_json::Value,
    blockchain: Arc<Mutex<Blockchain>>,
    offchain_storage: Arc<OffChainStorageManager>,
) -> JsonRpcResponse<serde_json::Value> {
    let request_id = None;
    match serde_json::from_value::<CreateTokenParams>(params) {
        Ok(parsed_params) => {
            let creator_address_hex = parsed_params.creator_address;
            let token_name = parsed_params.token_name;
            let token_symbol = parsed_params.token_symbol;
            let initial_supply = parsed_params.initial_supply;
            
            info!("Processing create_token: {} ({}) with supply {} by creator {}",
                  token_name, token_symbol, initial_supply, creator_address_hex);
            
            match hex::decode(&creator_address_hex) {
                Ok(creator_address) => {
                    // Criamos um payload JSON com os metadados do token
                    let token_metadata = serde_json::json!({
                        "type": "token_creation",
                        "name": token_name,
                        "symbol": token_symbol,
                        "initial_supply": initial_supply,
                        "creator": creator_address_hex,
                        "created_at": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                    });
                    
                    // Convertemos para string e depois para bytes
                    let token_metadata_str = token_metadata.to_string();
                    let token_metadata_bytes = token_metadata_str.as_bytes();
                    
                    // Armazenamos os metadados do token no armazenamento off-chain
                    match offchain_storage.store_payload(token_metadata_bytes) {
                        Ok(metadata_hash) => {
                            // Criamos uma transação especial para registrar a criação do token
                            let tx = Transaction::new_storage(
                                creator_address, 
                                metadata_hash, 
                                token_metadata_bytes.len() as u64
                            );
                            
                            let tx_hash = tx.calculate_hash();
                            let tx_hash_hex = hex::encode(tx_hash);
                            
                            // Adicionamos a transação ao mempool
                            match blockchain.lock().expect("Blockchain lock poisoned").add_pending_transaction(tx) {
                                Ok(added) => {
                                    if added {
                                        info!("Token creation transaction {} added to mempool.", tx_hash_hex);
                                        create_success_response(request_id, serde_json::json!({
                                            "token_name": token_name,
                                            "token_symbol": token_symbol,
                                            "initial_supply": initial_supply,
                                            "transaction_hash": tx_hash_hex,
                                            "metadata_hash": hex::encode(metadata_hash)
                                        }))
                                    } else {
                                        warn!("Token creation transaction already exists in mempool.");
                                        create_error_response(request_id, -32000, "Transaction already exists in mempool".to_string(), None)
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to add token creation transaction to mempool: {}", e);
                                    create_error_response(request_id, -32000, format!("Failed to add transaction: {}", e), None)
                                }
                            }
                        },
                        Err(e) => {
                            error!("Failed to store token metadata: {}", e);
                            create_error_response(request_id, -32000, format!("Failed to store token metadata: {}", e), None)
                        }
                    }
                },
                Err(_) => {
                    error!("Invalid hex string for creator address: {}", creator_address_hex);
                    create_error_response(request_id, -32602, "Invalid hex string for creator address".to_string(), None)
                }
            }
        },
        Err(e) => {
            error!("Failed to parse create_token params: {}", e);
            create_error_response(request_id, -32602, "Invalid params".to_string(), Some(serde_json::json!(e.to_string())))
        }
    }
}
