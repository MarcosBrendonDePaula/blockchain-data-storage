// tests/rpc_api_test.rs

use actix_web::{test, web, App, http::StatusCode};
use serde_json::{json, Value};
use tempfile::tempdir;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

// Import necessary items from the main crate
use blockchain_data_storage::core::{Blockchain, Transaction, Block};
use blockchain_data_storage::storage::StorageManager;
use blockchain_data_storage::rpc::{start_rpc_server, rpc_handler, AppState}; // Assuming rpc_handler and AppState are pub

// Helper to setup a test blockchain instance
fn setup_test_blockchain() -> (Arc<Mutex<Blockchain>>, tempfile::TempDir) {
    let dir = tempdir().expect("Failed to create temp dir");
    let path = dir.path().to_path_buf();
    let storage = StorageManager::new(&path).expect("Failed to create storage");
    let mut blockchain = Blockchain::new(storage).expect("Failed to create blockchain");
    blockchain.initialize_genesis_if_needed().expect("Failed to init genesis");
    (Arc::new(Mutex::new(blockchain)), dir)
}

// Helper to create a JSON-RPC request body
fn create_rpc_request(method: &str, params: Value, id: u64) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": id
    })
}

#[actix_web::test]
async fn test_rpc_get_chain_height() {
    let (blockchain_arc, _temp_dir) = setup_test_blockchain();
    let app_state = web::Data::new(AppState { blockchain: blockchain_arc.clone() });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    ).await;

    let req_body = create_rpc_request("get_chain_height", json!({}), 1);
    let req = test::TestRequest::post().uri("/").set_json(&req_body).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["jsonrpc"], "2.0");
    assert_eq!(body["id"], 1);
    assert!(body["error"].is_null());
    assert_eq!(body["result"], 0); // Genesis block is height 0
}

#[actix_web::test]
async fn test_rpc_send_transaction_and_get_block() {
    let (blockchain_arc, _temp_dir) = setup_test_blockchain();
    let app_state = web::Data::new(AppState { blockchain: blockchain_arc.clone() });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    ).await;

    // 1. Send a transaction
    let tx = Transaction::new_transfer(vec![1], vec![2], 100);
    let tx_hash = tx.calculate_hash();
    let tx_hash_hex = hex::encode(tx_hash);
    let send_tx_params = json!({ "transaction": tx });
    let req_body_send = create_rpc_request("send_transaction", send_tx_params, 2);

    let req_send = test::TestRequest::post().uri("/").set_json(&req_body_send).to_request();
    let resp_send = test::call_service(&app, req_send).await;

    assert_eq!(resp_send.status(), StatusCode::OK);
    let body_send: Value = test::read_body_json(resp_send).await;
    assert_eq!(body_send["id"], 2);
    assert!(body_send["error"].is_null());
    assert_eq!(body_send["result"], tx_hash_hex);

    // 2. Mine the block containing the transaction (simulate node mining)
    let mined_block;
    {
        let mut bc = blockchain_arc.lock().unwrap();
        mined_block = bc.mine_new_block().expect("Mining failed in test");
        bc.process_mined_block(mined_block.clone()).expect("Processing mined block failed");
    }
    let block_hash_hex = hex::encode(mined_block.hash());

    // 3. Get block by height
    let get_block_height_params = json!({ "height": 1 });
    let req_body_get_h = create_rpc_request("get_block_by_height", get_block_height_params, 3);
    let req_get_h = test::TestRequest::post().uri("/").set_json(&req_body_get_h).to_request();
    let resp_get_h = test::call_service(&app, req_get_h).await;

    assert_eq!(resp_get_h.status(), StatusCode::OK);
    let body_get_h: Value = test::read_body_json(resp_get_h).await;
    assert_eq!(body_get_h["id"], 3);
    assert!(body_get_h["error"].is_null());
    assert!(body_get_h["result"].is_object());
    let block_from_rpc_h: Block = serde_json::from_value(body_get_h["result"].clone()).unwrap();
    assert_eq!(block_from_rpc_h.hash(), mined_block.hash());
    assert_eq!(block_from_rpc_h.transactions.len(), 1);
    assert_eq!(block_from_rpc_h.transactions[0].calculate_hash(), tx_hash);

    // 4. Get block by hash
    let get_block_hash_params = json!({ "hash": block_hash_hex });
    let req_body_get_hash = create_rpc_request("get_block_by_hash", get_block_hash_params, 4);
    let req_get_hash = test::TestRequest::post().uri("/").set_json(&req_body_get_hash).to_request();
    let resp_get_hash = test::call_service(&app, req_get_hash).await;

    assert_eq!(resp_get_hash.status(), StatusCode::OK);
    let body_get_hash: Value = test::read_body_json(resp_get_hash).await;
    assert_eq!(body_get_hash["id"], 4);
    assert!(body_get_hash["error"].is_null());
    assert!(body_get_hash["result"].is_object());
    let block_from_rpc_hash: Block = serde_json::from_value(body_get_hash["result"].clone()).unwrap();
    assert_eq!(block_from_rpc_hash.hash(), mined_block.hash());
}

#[actix_web::test]
async fn test_rpc_get_block_not_found() {
    let (blockchain_arc, _temp_dir) = setup_test_blockchain();
    let app_state = web::Data::new(AppState { blockchain: blockchain_arc.clone() });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    ).await;

    // Get block by height (non-existent)
    let get_block_height_params = json!({ "height": 999 });
    let req_body_get_h = create_rpc_request("get_block_by_height", get_block_height_params, 5);
    let req_get_h = test::TestRequest::post().uri("/").set_json(&req_body_get_h).to_request();
    let resp_get_h = test::call_service(&app, req_get_h).await;

    assert_eq!(resp_get_h.status(), StatusCode::OK);
    let body_get_h: Value = test::read_body_json(resp_get_h).await;
    assert_eq!(body_get_h["id"], 5);
    assert!(body_get_h["error"].is_null());
    assert!(body_get_h["result"].is_null()); // Correct response for not found

    // Get block by hash (non-existent)
    let non_existent_hash = hex::encode([99u8; 32]);
    let get_block_hash_params = json!({ "hash": non_existent_hash });
    let req_body_get_hash = create_rpc_request("get_block_by_hash", get_block_hash_params, 6);
    let req_get_hash = test::TestRequest::post().uri("/").set_json(&req_body_get_hash).to_request();
    let resp_get_hash = test::call_service(&app, req_get_hash).await;

    assert_eq!(resp_get_hash.status(), StatusCode::OK);
    let body_get_hash: Value = test::read_body_json(resp_get_hash).await;
    assert_eq!(body_get_hash["id"], 6);
    assert!(body_get_hash["error"].is_null());
    assert!(body_get_hash["result"].is_null()); // Correct response for not found
}

#[actix_web::test]
async fn test_rpc_invalid_method() {
    let (blockchain_arc, _temp_dir) = setup_test_blockchain();
    let app_state = web::Data::new(AppState { blockchain: blockchain_arc.clone() });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    ).await;

    let req_body = create_rpc_request("invalid_method_name", json!({}), 7);
    let req = test::TestRequest::post().uri("/").set_json(&req_body).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], 7);
    assert!(body["result"].is_null());
    assert!(body["error"].is_object());
    assert_eq!(body["error"]["code"], -32601);
    assert_eq!(body["error"]["message"], "Method not found");
}

#[actix_web::test]
async fn test_rpc_invalid_params() {
    let (blockchain_arc, _temp_dir) = setup_test_blockchain();
    let app_state = web::Data::new(AppState { blockchain: blockchain_arc.clone() });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(rpc_handler)
    ).await;

    // Send transaction with wrong param structure
    let invalid_params = json!({ "wrong_field": "some_value" });
    let req_body = create_rpc_request("send_transaction", invalid_params, 8);
    let req = test::TestRequest::post().uri("/").set_json(&req_body).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], 8);
    assert!(body["result"].is_null());
    assert!(body["error"].is_object());
    assert_eq!(body["error"]["code"], -32602);
    assert_eq!(body["error"]["message"], "Invalid params");
}

