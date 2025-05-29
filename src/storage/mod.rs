//!
//! # Storage Module
//! Handles the persistence of blockchain data (blocks, etc.) to a local database.
//! Currently uses RocksDB as the underlying key-value store.

use crate::core::{Block, BlockHeader, Hash, Transaction};
use rocksdb::{Options, DB, WriteBatch};
use std::path::Path;
use std::sync::Arc;
use log::{error, info, warn};

// Define key prefixes for different data types in RocksDB
const PREFIX_BLOCK: u8 = b'b'; // Key: PREFIX_BLOCK + block_hash => Value: serialized_block
const PREFIX_HEADER: u8 = b'd'; // Key: PREFIX_HEADER + block_hash => Value: serialized_header (Optional optimization)
const PREFIX_HEIGHT_TO_HASH: u8 = b'h'; // Key: PREFIX_HEIGHT_TO_HASH + height (u64 BE) => Value: block_hash
const KEY_LAST_HASH: &[u8] = b"lh"; // Key: KEY_LAST_HASH => Value: last_block_hash
const KEY_CHAIN_HEIGHT: &[u8] = b"ch"; // Key: KEY_CHAIN_HEIGHT => Value: current_height (u64 BE)

/// Manages the interaction with the RocksDB database for blockchain storage.
#[derive(Clone)] // Clone is cheap due to Arc
pub struct StorageManager {
    db: Arc<DB>,
}

impl StorageManager {
    /// Opens or creates a RocksDB database at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the database directory.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the `StorageManager` or a `rocksdb::Error`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        // TODO: Consider adding Column Families for better organization
        // TODO: Tune RocksDB options (compression, caching, etc.)
        let db = DB::open(&opts, path)?;
        info!("RocksDB database opened successfully.");
        Ok(StorageManager { db: Arc::new(db) })
    }

    /// Saves a block to the database.
    /// This includes storing the block itself, updating the height-to-hash mapping,
    /// and updating the last hash and chain height.
    /// Uses a WriteBatch for atomicity.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to save.
    ///
    /// # Returns
    ///
    /// * An empty `Result` or a `Box<dyn std::error::Error>`.
    pub fn save_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let block_hash = block.hash();
        let block_height = block.header.height;
        let serialized_block = bincode::serialize(block)?;

        let mut batch = WriteBatch::default();

        // Store block by hash: b<hash> -> block_data
        let block_key = [&[PREFIX_BLOCK], block_hash.as_slice()].concat();
        batch.put(&block_key, &serialized_block);

        // Store height to hash mapping: h<height_be> -> hash
        let height_key = [&[PREFIX_HEIGHT_TO_HASH], &block_height.to_be_bytes()].concat();
        batch.put(&height_key, &block_hash);

        // Update last hash: lh -> hash
        batch.put(KEY_LAST_HASH, &block_hash);

        // Update chain height: ch -> height_be
        batch.put(KEY_CHAIN_HEIGHT, &block_height.to_be_bytes());

        self.db.write(batch)?; // Atomic write
        Ok(())
    }

    /// Retrieves a block from the database by its hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the block to retrieve.
    ///
    /// # Returns
    ///
    /// * An `Option<Block>` containing the deserialized block if found, or `None`.
    /// * Returns `Err` on database or deserialization errors.
    pub fn get_block_by_hash(&self, hash: &Hash) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let block_key = [&[PREFIX_BLOCK], hash.as_slice()].concat();
        match self.db.get(&block_key)? {
            Some(serialized_block) => {
                let block: Block = bincode::deserialize(&serialized_block)?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }

    /// Retrieves a block hash from the database by its height.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the block hash to retrieve.
    ///
    /// # Returns
    ///
    /// * An `Option<Hash>` containing the block hash if found, or `None`.
    /// * Returns `Err` on database errors.
    pub fn get_hash_by_height(&self, height: u64) -> Result<Option<Hash>, rocksdb::Error> {
        let height_key = [&[PREFIX_HEIGHT_TO_HASH], &height.to_be_bytes()].concat();
        match self.db.get(&height_key)? {
            Some(hash_vec) => {
                if hash_vec.len() == 32 {
                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&hash_vec);
                    Ok(Some(hash))
                } else {
                    // Data corruption or incorrect key schema?
                    error!("Invalid hash length found for height {}", height);
                    Ok(None) // Or return an error
                }
            }
            None => Ok(None),
        }
    }

    /// Retrieves a block from the database by its height.
    /// First looks up the hash by height, then retrieves the block by hash.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the block to retrieve.
    ///
    /// # Returns
    ///
    /// * An `Option<Block>` containing the deserialized block if found, or `None`.
    /// * Returns `Err` on database or deserialization errors.
    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        match self.get_hash_by_height(height)? {
            Some(hash) => self.get_block_by_hash(&hash),
            None => Ok(None),
        }
    }

    /// Retrieves the hash of the latest block in the main chain.
    ///
    /// # Returns
    ///
    /// * An `Option<Hash>` containing the last block hash if set, or `None`.
    /// * Returns `Err` on database errors.
    pub fn get_last_block_hash(&self) -> Result<Option<Hash>, rocksdb::Error> {
        match self.db.get(KEY_LAST_HASH)? {
            Some(hash_vec) => {
                if hash_vec.len() == 32 {
                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&hash_vec);
                    Ok(Some(hash))
                } else {
                    error!("Invalid last_hash length found in DB");
                    Ok(None)
                }
            }
            None => Ok(None), // Not set yet (e.g., empty DB)
        }
    }

    /// Retrieves the current height of the main chain.
    ///
    /// # Returns
    ///
    /// * An `Option<u64>` containing the chain height if set, or `None`.
    /// * Returns `Err` on database errors.
    pub fn get_chain_height(&self) -> Result<Option<u64>, rocksdb::Error> {
        match self.db.get(KEY_CHAIN_HEIGHT)? {
            Some(height_bytes) => {
                if height_bytes.len() == 8 {
                    Ok(Some(u64::from_be_bytes(height_bytes.try_into().unwrap())))
                } else {
                    error!("Invalid chain_height length found in DB");
                    Ok(None)
                }
            }
            None => Ok(None), // Not set yet
        }
    }

    // TODO: Add functions for storing/retrieving transactions by hash (optional index)
    // TODO: Add functions for managing blockchain state (e.g., UTXO set if applicable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Transaction;
    use tempfile::tempdir; // Use tempdir for isolated test databases

    // Helper to create a dummy block for testing
    fn create_test_block(height: u64, previous_hash: Hash, num_tx: usize) -> Block {
        let transactions = (0..num_tx)
            .map(|i| Transaction::new_transfer(vec![(i % 256) as u8], vec![], i as u64))
            .collect();
        Block::new(previous_hash, transactions, 10, height)
    }

    #[test]
    fn test_storage_new_open() {
        let dir = tempdir().unwrap();
        {
            let storage = StorageManager::new(dir.path()).expect("Failed to create/open DB");
            // DB should be open here
            info!("DB opened at {:?}", dir.path());
        } // Storage is dropped, DB should be closed
        // Reopen the same DB
        let storage = StorageManager::new(dir.path()).expect("Failed to reopen DB");
        info!("DB reopened successfully.");
        // Check initial state (should be empty)
        assert!(storage.get_last_block_hash().unwrap().is_none());
        assert!(storage.get_chain_height().unwrap().is_none());
    }

    #[test]
    fn test_save_and_get_block() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();

        let block0 = create_test_block(0, [0u8; 32], 0); // Genesis block
        let hash0 = block0.hash();
        storage.save_block(&block0).expect("Failed to save block 0");

        let block1 = create_test_block(1, hash0, 2);
        let hash1 = block1.hash();
        storage.save_block(&block1).expect("Failed to save block 1");

        // Test get_block_by_hash
        let retrieved_block0 = storage.get_block_by_hash(&hash0).unwrap().expect("Block 0 not found by hash");
        assert_eq!(retrieved_block0.header, block0.header);
        assert_eq!(retrieved_block0.transactions, block0.transactions);

        let retrieved_block1 = storage.get_block_by_hash(&hash1).unwrap().expect("Block 1 not found by hash");
        assert_eq!(retrieved_block1.header, block1.header);
        assert_eq!(retrieved_block1.transactions, block1.transactions);

        // Test get_block_by_height
        let retrieved_block0_h = storage.get_block_by_height(0).unwrap().expect("Block 0 not found by height");
        assert_eq!(retrieved_block0_h.header, block0.header);

        let retrieved_block1_h = storage.get_block_by_height(1).unwrap().expect("Block 1 not found by height");
        assert_eq!(retrieved_block1_h.header, block1.header);

        // Test get non-existent block
        assert!(storage.get_block_by_hash(&[99u8; 32]).unwrap().is_none());
        assert!(storage.get_block_by_height(2).unwrap().is_none());

        // Test metadata
        assert_eq!(storage.get_last_block_hash().unwrap(), Some(hash1));
        assert_eq!(storage.get_chain_height().unwrap(), Some(1));
    }

     #[test]
    fn test_get_hash_by_height() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();

        let block0 = create_test_block(0, [0u8; 32], 0);
        let hash0 = block0.hash();
        storage.save_block(&block0).unwrap();

        let block1 = create_test_block(1, hash0, 1);
        let hash1 = block1.hash();
        storage.save_block(&block1).unwrap();

        assert_eq!(storage.get_hash_by_height(0).unwrap(), Some(hash0));
        assert_eq!(storage.get_hash_by_height(1).unwrap(), Some(hash1));
        assert!(storage.get_hash_by_height(2).unwrap().is_none());
    }
}

