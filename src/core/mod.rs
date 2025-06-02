use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;
// Corrected: Import StorageError as well
use crate::storage::{StorageManager, StorageError};
use crate::consensus; // Import consensus functions
use crate::mempool::Mempool; // Import Mempool
use std::path::Path;
use log::{info, error, debug};

// Constants
const MAX_TRANSACTIONS_PER_BLOCK: usize = 100; // Example limit
const MEMPOOL_MAX_SIZE: usize = 1000; // Example limit

// Define Hash as a fixed-size array for SHA-256
pub type Hash = [u8; 32];

// Placeholder for public key/address type
pub type Address = Vec<u8>;

// Represents a single transaction in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    sender: Address,
    receiver: Address,
    amount: u64,
    timestamp: u64,
    data_hash: Option<Vec<u8>>,
    data_size: Option<u64>,
    // signature: Vec<u8>,
    // nonce: u64,
}

impl Transaction {
    pub fn new_transfer(sender: Address, receiver: Address, amount: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            data_hash: None,
            data_size: None,
        }
    }

    pub fn new_storage(sender: Address, data_hash: Hash, data_size: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Transaction {
            sender,
            receiver: vec![],
            amount: 0,
            timestamp,
            data_hash: Some(data_hash.to_vec()),
            data_size: Some(data_size),
        }
    }

    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        // Use bincode for consistent hashing
        let serialized = bincode::serialize(self).expect("Failed to serialize transaction for hashing");
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

// Represents the header of a block
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
    pub height: u64,
}

impl BlockHeader {
    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        let serialized = bincode::serialize(self).expect("Failed to serialize block header for hashing");
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

// Represents a full block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(previous_hash: Hash, transactions: Vec<Transaction>, difficulty: u32, height: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let merkle_root = Self::calculate_merkle_root(&transactions);
        let nonce = 0; // Placeholder, will be found by miner

        let header = BlockHeader {
            previous_hash,
            merkle_root,
            timestamp,
            nonce,
            difficulty,
            height,
        };

        Block { header, transactions }
    }

    // Calculate the Merkle root from transaction hashes
    // TODO: Implement proper Merkle tree construction
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return [0u8; 32];
        }
        let mut hasher = Sha256::new();
        for tx in transactions {
            hasher.update(tx.calculate_hash());
        }
        hasher.finalize().into()
    }

    pub fn hash(&self) -> Hash {
        self.header.calculate_hash()
    }
}

// --- Blockchain Structure (Persistent) ---

/// Manages the blockchain state, interacting with StorageManager and Mempool.
#[derive(Debug)]
pub struct Blockchain {
    storage: StorageManager,
    mempool: Mempool,
    current_tip_hash: Option<Hash>,
    current_height: Option<u64>,
}

// Custom error type for Blockchain operations
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    // Corrected: Use StorageError from the storage module
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Block validation failed: {0}")]
    Validation(String),
    #[error("Consensus error: {0}")]
    Consensus(String),
    #[error("Initialization error: {0}")]
    Initialization(String),
    #[error("Block not found (hash: {0})")]
    BlockNotFoundByHash(String),
    #[error("Block not found (height: {0})")]
    BlockNotFoundByHeight(u64),
    #[error("Genesis block already exists")]
    GenesisAlreadyExists,
    #[error("Blockchain not initialized (no genesis block)")]
    NotInitialized,
    #[error("Mempool error: {0}")]
    Mempool(String),
}

impl Blockchain {
    /// Creates a new Blockchain instance, loading state from storage and initializing mempool.
pub fn new(storage_path: &Path) -> Result<Self, BlockchainError> {
        info!("Initializing blockchain from storage path: {:?}", storage_path);
        // Now StorageManager::new returns StorageError, which BlockchainError can handle via From
        let storage = StorageManager::new(storage_path)?;
        let mempool = Mempool::new(MEMPOOL_MAX_SIZE);

        // These methods now return StorageError, handled by '?'
        let current_tip_hash = storage.get_last_block_hash()?;
        let current_height = storage.get_chain_height()?;

        if current_tip_hash.is_some() != current_height.is_some() {
            error!("Inconsistent storage state: tip hash present ({}) but height present ({}).",
                   current_tip_hash.is_some(), current_height.is_some());
            return Err(BlockchainError::Initialization("Inconsistent tip hash and height in storage".to_string()));
        }

        if let Some(height) = current_height {
            info!("Loaded existing blockchain. Current height: {}, Tip hash: {}",
                   height, current_tip_hash.map(hex::encode).unwrap_or_default());
        } else {
            info!("No existing blockchain found in storage. Ready for genesis block.");
        }

        Ok(Blockchain {
            storage,
            mempool,
            current_tip_hash,
            current_height,
        })
    }

    /// Creates and saves the genesis block if the blockchain is empty.
pub fn initialize_genesis_if_needed(&mut self) -> Result<(), BlockchainError> {
        if self.current_height.is_some() {
            info!("Genesis block already exists.");
            return Ok(());
        }

        info!("Creating genesis block...");
        let genesis_transactions = Vec::new();
        let merkle_root = Block::calculate_merkle_root(&genesis_transactions);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let initial_difficulty = consensus::MIN_DIFFICULTY;

        let genesis_header = BlockHeader {
            previous_hash: [0u8; 32],
            merkle_root,
            timestamp,
            nonce: 0,
            difficulty: initial_difficulty,
            height: 0,
        };

        let genesis_block = Block {
            header: genesis_header,
            transactions: genesis_transactions,
        };
        let genesis_hash = genesis_block.hash();

        // save_block now returns StorageError, handled by '?'
        self.storage.save_block(&genesis_block)?;
        self.current_tip_hash = Some(genesis_hash);
        self.current_height = Some(0);
        info!("Genesis block created and saved. Hash: {}", hex::encode(genesis_hash));
        Ok(())
    }

    /// Adds a transaction to the mempool.
pub fn add_pending_transaction(&mut self, tx: Transaction) -> Result<bool, BlockchainError> {
        // TODO: Add validation against current blockchain state (e.g., sufficient funds)
        self.mempool.add_transaction(tx).map_err(BlockchainError::Mempool)
    }

    /// Retrieves a block by its hash from storage.
pub fn get_block_by_hash(&self, hash: &Hash) -> Result<Option<Block>, BlockchainError> {
        // get_block_by_hash now returns StorageError, handled by '?'
        Ok(self.storage.get_block_by_hash(hash)?)
    }

    /// Retrieves a block by its height from storage.
pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockchainError> {
        // get_block_by_height now returns StorageError, handled by '?'
        Ok(self.storage.get_block_by_height(height)?)
    }

    /// Returns the hash of the latest block (tip) in the chain.
pub fn get_last_block_hash(&self) -> Option<Hash> {
        // This doesn't return Result, no change needed
        self.current_tip_hash
    }

    /// Returns the height of the latest block (tip) in the chain.
pub fn get_chain_height(&self) -> Option<u64> {
        // This doesn't return Result, no change needed
        self.current_height
    }

    /// Validates and adds a new block to the blockchain.
pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let block_hash = block.hash();
        let header = &block.header;
        info!("Attempting to add block {} (Hash: {})...", header.height, hex::encode(block_hash));

        let current_height = self.current_height.ok_or(BlockchainError::NotInitialized)?;
        let current_tip_hash = self.current_tip_hash.ok_or(BlockchainError::NotInitialized)?;

        // --- Basic Validation ---
        let expected_height = current_height + 1;
        if header.height != expected_height {
            return Err(BlockchainError::Validation(format!(
                "Invalid block height. Expected: {}, Got: {}",
                expected_height, header.height
            )));
        }

        if header.previous_hash != current_tip_hash {
            return Err(BlockchainError::Validation(format!(
                "Invalid previous block hash. Expected: {}, Got: {}",
                hex::encode(current_tip_hash), hex::encode(header.previous_hash)
            )));
        }

        let calculated_merkle_root = Block::calculate_merkle_root(&block.transactions);
        if header.merkle_root != calculated_merkle_root {
            return Err(BlockchainError::Validation(format!(
                "Invalid Merkle root. Expected: {}, Got: {}",
                hex::encode(calculated_merkle_root), hex::encode(header.merkle_root)
            )));
        }

        // --- Consensus Validation ---
        if !consensus::verify_pow(&block_hash, header.difficulty) {
            return Err(BlockchainError::Consensus(format!(
                "Invalid Proof-of-Work. Hash {} does not meet difficulty {}",
                hex::encode(block_hash), header.difficulty
            )));
        }

        // calculate_next_difficulty needs access to storage, error handled by '?'
        let expected_difficulty = consensus::calculate_next_difficulty(current_height, &self.storage)
            .map_err(BlockchainError::Consensus)?;
        if header.difficulty != expected_difficulty {
            return Err(BlockchainError::Consensus(format!(
                "Incorrect difficulty for block {}. Expected: {}, Got: {}",
                header.height, expected_difficulty, header.difficulty
            )));
        }

        // TODO: Add transaction validation logic here (e.g., check signatures, balances)

        // --- Save Block --- 
        // save_block now returns StorageError, handled by '?'
        self.storage.save_block(&block)?;

        // --- Update Cache --- 
        self.current_tip_hash = Some(block_hash);
        self.current_height = Some(header.height);

        info!("Block {} added successfully. New height: {}, New tip: {}",
               header.height, self.current_height.unwrap(), hex::encode(self.current_tip_hash.unwrap()));

        Ok(())
    }

    /// Creates a new block candidate, mines it, and returns the mined block.
    /// Does NOT add the block to the chain automatically.
    pub fn mine_new_block(&mut self /*, _miner_address: Address */) -> Result<Block, BlockchainError> {
        let current_height = self.current_height.ok_or(BlockchainError::NotInitialized)?;
        let previous_hash = self.current_tip_hash.ok_or(BlockchainError::NotInitialized)?;
        let next_height = current_height + 1;

        info!("Attempting to mine block {}...", next_height);

        // 1. Get transactions from mempool
        let transactions = self.mempool.get_transactions(MAX_TRANSACTIONS_PER_BLOCK);
        debug!("Selected {} transactions from mempool for block {}.", transactions.len(), next_height);

        // TODO: Add Coinbase transaction rewarding the miner

        // 2. Calculate difficulty for the new block
        // calculate_next_difficulty needs access to storage, error handled by '?'
        let difficulty = consensus::calculate_next_difficulty(current_height, &self.storage)
            .map_err(BlockchainError::Consensus)?;
        debug!("Calculated difficulty for block {}: {}", next_height, difficulty);

        // 3. Create block template
        let mut block = Block::new(previous_hash, transactions, difficulty, next_height);

        // 4. Mine the block (find nonce)
        let start_time = SystemTime::now();
        let mined_hash = consensus::mine(&mut block.header, difficulty);
        let mining_duration = start_time.elapsed().unwrap_or_default();

        info!("Successfully mined block {} in {:?}. Hash: {}, Nonce: {}",
               next_height, mining_duration, hex::encode(mined_hash), block.header.nonce);

        Ok(block)
    }

    /// Processes a mined block: validates, adds to storage, and updates mempool.
    pub fn process_mined_block(&mut self, mined_block: Block) -> Result<(), BlockchainError> {
        let block_height = mined_block.header.height;
        let block_hash = mined_block.hash();
        info!("Processing mined block {} (Hash: {})...", block_height, hex::encode(block_hash));

        // 1. Validate and add the block to the chain (storage + cache update)
        // add_block internally handles StorageError now
        self.add_block(mined_block.clone())?;

        // 2. Remove included transactions from the mempool
        let tx_hashes: Vec<Hash> = mined_block.transactions.iter().map(|tx| tx.calculate_hash()).collect();
        self.mempool.remove_transactions(&tx_hashes);
        debug!("Removed {} transactions from mempool after adding block {}.", tx_hashes.len(), block_height);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module
    use tempfile::tempdir;
    use std::thread; // For sleep
    use std::time::Duration;
    use crate::storage::StorageError; // Import StorageError for tests

    // Helper to create a basic block for testing add_block
    fn create_test_block(prev_hash: Hash, height: u64, difficulty: u32, transactions: Vec<Transaction>) -> Block {
        let mut block = Block::new(prev_hash, transactions, difficulty, height);
        let _mined_hash = consensus::mine(&mut block.header, difficulty);
        block
    }

    #[test]
    fn blockchain_new_with_mempool() {
        let dir = tempdir().unwrap();
        let blockchain = Blockchain::new(dir.path());
        assert!(blockchain.is_ok());
        let bc = blockchain.unwrap();
        assert!(bc.current_tip_hash.is_none());
        assert!(bc.current_height.is_none());
        assert!(bc.mempool.is_empty());
        assert_eq!(bc.mempool.max_size, MEMPOOL_MAX_SIZE);
    }

    #[test]
    fn blockchain_add_pending_transaction() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 100);
        let tx2 = Transaction::new_transfer(vec![3], vec![4], 200);

        assert!(blockchain.add_pending_transaction(tx1.clone()).unwrap());
        assert!(!blockchain.mempool.is_empty());
        assert!(blockchain.add_pending_transaction(tx2.clone()).unwrap());
        // Try adding duplicate
        assert!(!blockchain.add_pending_transaction(tx1.clone()).unwrap());
    }

    #[test]
    fn blockchain_initialize_genesis() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        assert!(blockchain.current_height.is_none());

        let genesis_result = blockchain.initialize_genesis_if_needed();
        assert!(genesis_result.is_ok());
        assert_eq!(blockchain.get_chain_height(), Some(0));
        assert!(blockchain.get_last_block_hash().is_some());

        // Try initializing again
        let genesis_again_result = blockchain.initialize_genesis_if_needed();
        assert!(genesis_again_result.is_ok()); // Should be ok, just does nothing
        assert_eq!(blockchain.get_chain_height(), Some(0)); // Height remains 0
    }

    #[test]
    fn blockchain_add_block_valid() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let prev_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        let block1 = create_test_block(prev_hash, height + 1, difficulty, vec![]);
        let add_result = blockchain.add_block(block1);

        if let Err(e) = &add_result {
            eprintln!("Add block failed: {}", e);
        }
        assert!(add_result.is_ok());
        assert_eq!(blockchain.get_chain_height(), Some(1));
        assert_eq!(blockchain.get_last_block_hash(), Some(block1.hash()));
    }

    #[test]
    fn blockchain_add_block_invalid_height() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let prev_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        // Invalid height (should be 1)
        let block_invalid = create_test_block(prev_hash, height + 2, difficulty, vec![]);
        let add_result = blockchain.add_block(block_invalid);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Validation(msg) => assert!(msg.contains("Invalid block height")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn blockchain_add_block_invalid_prev_hash() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let _prev_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        // Invalid previous hash
        let block_invalid = create_test_block([1u8; 32], height + 1, difficulty, vec![]);
        let add_result = blockchain.add_block(block_invalid);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Validation(msg) => assert!(msg.contains("Invalid previous block hash")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn blockchain_mine_and_process_block() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        // Add some transactions to mempool
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 50);
        let tx2 = Transaction::new_transfer(vec![3], vec![4], 150);
        blockchain.add_pending_transaction(tx1.clone()).unwrap();
        blockchain.add_pending_transaction(tx2.clone()).unwrap();
        assert!(!blockchain.mempool.is_empty());

        // Mine a new block
        let mine_result = blockchain.mine_new_block();
        assert!(mine_result.is_ok());
        let mined_block = mine_result.unwrap();
        assert_eq!(mined_block.header.height, 1);
        assert_eq!(mined_block.transactions.len(), 2);
        assert!(mined_block.transactions.contains(&tx1));
        assert!(mined_block.transactions.contains(&tx2));

        // Process the mined block
        let process_result = blockchain.process_mined_block(mined_block.clone());
        assert!(process_result.is_ok());

        // Verify chain state
        assert_eq!(blockchain.get_chain_height(), Some(1));
        assert_eq!(blockchain.get_last_block_hash(), Some(mined_block.hash()));

        // Verify mempool is empty
        assert!(blockchain.mempool.is_empty());

        // Verify block is in storage
        let stored_block = blockchain.get_block_by_height(1).unwrap().unwrap();
        assert_eq!(stored_block.hash(), mined_block.hash());
    }
}

