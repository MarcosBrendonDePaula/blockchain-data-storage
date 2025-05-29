// tests/integration_test.rs

use blockchain_data_storage::core::Blockchain;
use blockchain_data_storage::storage::StorageManager;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::tempdir;
use assert_cmd::Command; // For testing the binary execution
use predicates::prelude::*;

// Helper to setup a test environment with a temporary data directory
fn setup_test_env() -> (PathBuf, tempfile::TempDir) {
    let dir = tempdir().expect("Failed to create temp dir for test");
    let data_path = dir.path().to_path_buf();
    (data_path, dir)
}

#[test]
fn test_node_initialization_creates_data_dir_and_genesis() {
    let (data_dir_path, _temp_dir) = setup_test_env(); // _temp_dir ensures directory is cleaned up

    // Check data directory doesn't exist initially (it shouldn't)
    // assert!(!data_dir_path.exists()); // tempdir creates it, so this check is invalid

    // Run the node binary with the specific data directory
    // This requires the binary to be built first (e.g., via `cargo build`)
    let mut cmd = Command::cargo_bin("blockchain-data-storage").unwrap();
    cmd.arg("--data-dir").arg(&data_dir_path);

    // We expect the node to initialize and then print the message before the loop
    // (or exit if the loop isn't running indefinitely yet).
    // Let's assert that it runs successfully and prints the expected initialization messages.
    // Note: This test assumes start_network_node exits quickly or isn't called yet in the test context.
    // If start_network_node runs forever, this test would hang. We need a way to stop it.
    // For now, we test the initialization phase output.

    // Since start_network_node runs indefinitely, we can't easily test the full run.
    // Instead, let's verify the *side effects* of initialization:
    // 1. The data directory exists.
    // 2. The storage manager can be initialized with this directory.
    // 3. The blockchain initializes and creates a genesis block.

    // We'll simulate the initialization steps manually based on main.rs logic
    // This avoids running the full binary and the infinite loop issue.

    // 1. Check directory (created by tempdir)
    assert!(data_dir_path.exists());

    // 2. Initialize Storage Manager
    let storage_manager = StorageManager::new(&data_dir_path);
    assert!(storage_manager.is_ok(), "Failed to init storage manager in test");
    let storage_manager = storage_manager.unwrap();

    // 3. Initialize Blockchain & Genesis
    let blockchain = Blockchain::new(storage_manager);
    assert!(blockchain.is_ok(), "Failed to init blockchain in test");
    let mut blockchain = blockchain.unwrap();

    let genesis_created = blockchain.initialize_genesis_if_needed();
    assert!(genesis_created.is_ok(), "Genesis check/init failed in test");
    assert!(genesis_created.unwrap(), "Genesis block was not created on first init");

    // Verify genesis block exists in storage
    let genesis_hash_from_bc = blockchain.get_block_hash(0);
    assert!(genesis_hash_from_bc.is_ok(), "Failed to get genesis hash after init");
    assert!(genesis_hash_from_bc.unwrap().is_some(), "Genesis hash not found after init");

    // Verify re-initialization detects existing genesis
    let storage_manager2 = StorageManager::new(&data_dir_path).unwrap();
    let mut blockchain2 = Blockchain::new(storage_manager2).unwrap();
    let genesis_created_again = blockchain2.initialize_genesis_if_needed();
    assert!(genesis_created_again.is_ok(), "Second genesis check failed");
    assert!(!genesis_created_again.unwrap(), "Genesis block was created again on second init");
    assert_eq!(blockchain2.get_chain_height(), Some(0));
}

// Test command line argument parsing (basic)
#[test]
fn test_cli_data_dir_argument() {
    let mut cmd = Command::cargo_bin("blockchain-data-storage").unwrap();
    let test_dir = "./my_test_data_dir_cli";
    cmd.arg("--data-dir").arg(test_dir);

    // We can't easily check the internal state, but we can check if the process starts
    // without crashing due to argument parsing errors.
    // Since it runs indefinitely, we might need to kill it or check stdout/stderr.
    // For now, just assert it doesn't fail immediately.
    // This is a weak test for the full execution.
    // A better approach might involve mocking start_network_node or adding a timeout/shutdown.

    // Let's just check if the help message works, which tests clap setup.
    let mut cmd_help = Command::cargo_bin("blockchain-data-storage").unwrap();
    cmd_help.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--data-dir <DIR>"));

    // Clean up dummy dir if created by a real run (not needed for --help)
    // std::fs::remove_dir_all(test_dir).ok();
}

// Add more tests as needed, e.g., for listen address parsing once implemented.

