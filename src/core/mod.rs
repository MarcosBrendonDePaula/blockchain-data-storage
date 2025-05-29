use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;
use crate::storage::{StorageManager, StorageError}; // Import StorageManager and its Error type
use crate::consensus; // Import consensus functions
use std::path::Path;
use log::{info, error, warn};

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
        let serialized = bincode::serialize(self).expect("Failed to serialize transaction for hashing"); // Use bincode for hashing
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
        let serialized = bincode::serialize(self).expect("Failed to serialize block header for hashing"); // Use bincode for hashing
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

// Represents a full block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // Added Clone, PartialEq, Eq
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    // Note: This constructor might be less used now, as mining creates the block
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

/// Manages the blockchain state, interacting with the StorageManager for persistence.
#[derive(Debug)]
pub struct Blockchain {
    storage: StorageManager,
    // Cache the current tip hash and height for faster access
    current_tip_hash: Option<Hash>,
    current_height: Option<u64>,
    // TODO: Add mempool (pending transactions)
    // pending_transactions: Vec<Transaction>,
}

// Custom error type for Blockchain operations
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
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
}

impl Blockchain {
    /// Creates a new Blockchain instance, loading the current state from storage.
pub fn new(storage_path: &Path) -> Result<Self, BlockchainError> {
        info!("Initializing blockchain from storage path: {:?}", storage_path);
        let storage = StorageManager::new(storage_path)?;

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
            current_tip_hash,
            current_height,
            // pending_transactions: Vec::new(),
        })
    }

    /// Creates and saves the genesis block if the blockchain is empty.
pub fn initialize_genesis_if_needed(&mut self) -> Result<(), BlockchainError> {
        if self.current_height.is_some() {
            info!("Genesis block already exists.");
            return Ok(()); // Or return Err(BlockchainError::GenesisAlreadyExists)?
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

        self.storage.save_block(&genesis_block)?;
        self.current_tip_hash = Some(genesis_hash);
        self.current_height = Some(0);
        info!("Genesis block created and saved. Hash: {}", hex::encode(genesis_hash));
        Ok(())
    }

    /// Retrieves a block by its hash from storage.
pub fn get_block_by_hash(&self, hash: &Hash) -> Result<Option<Block>, BlockchainError> {
        Ok(self.storage.get_block_by_hash(hash)?)
    }

    /// Retrieves a block by its height from storage.
pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockchainError> {
        Ok(self.storage.get_block_by_height(height)?)
    }

    /// Returns the hash of the latest block (tip) in the chain.
pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.current_tip_hash
    }

    /// Returns the height of the latest block (tip) in the chain.
pub fn get_chain_height(&self) -> Option<u64> {
        self.current_height
    }

    /// Validates and adds a new block to the blockchain.
    ///
    /// Performs several checks:
    /// - Previous hash matches the current chain tip.
    /// - Block height is correct.
    /// - Merkle root is valid for the transactions.
    /// - Proof-of-Work is valid for the block's stated difficulty.
    /// - The block's stated difficulty is correct according to the adjustment algorithm.
    ///
    /// # Arguments
    ///
    /// * `block` - The `Block` to add.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the block is valid and successfully added.
    /// * `Err(BlockchainError)` if the block is invalid or a storage error occurs.
pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let block_hash = block.hash();
        let header = &block.header;
        info!("Attempting to add block {} (Hash: {})...", header.height, hex::encode(block_hash));

        // --- Basic Validation ---
        let expected_height = self.current_height.map_or(0, |h| h + 1);
        if header.height != expected_height {
            return Err(BlockchainError::Validation(format!(
                "Invalid block height. Expected: {}, Got: {}",
                expected_height, header.height
            )));
        }

        let expected_prev_hash = self.current_tip_hash.unwrap_or([0u8; 32]);
        if header.previous_hash != expected_prev_hash {
            return Err(BlockchainError::Validation(format!(
                "Invalid previous block hash. Expected: {}, Got: {}",
                hex::encode(expected_prev_hash), hex::encode(header.previous_hash)
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

        // 1. Verify Proof-of-Work against the difficulty stated in the header
        if !consensus::verify_pow(&block_hash, header.difficulty) {
            return Err(BlockchainError::Consensus(format!(
                "Invalid Proof-of-Work. Hash {} does not meet difficulty {}",
                hex::encode(block_hash), header.difficulty
            )));
        }

        // 2. Verify that the difficulty stated in the header is the correct one
        //    based on the difficulty adjustment algorithm.
        //    Skip this check for the genesis block.
        if header.height > 0 {
            let previous_height = header.height - 1;
            let expected_difficulty = consensus::calculate_next_difficulty(previous_height, &self.storage)
                .map_err(BlockchainError::Consensus)?;

            if header.difficulty != expected_difficulty {
                return Err(BlockchainError::Consensus(format!(
                    "Incorrect difficulty for block {}. Expected: {}, Got: {}",
                    header.height, expected_difficulty, header.difficulty
                )));
            }
        }
        // TODO: Add transaction validation logic here

        // --- Save Block --- 
        self.storage.save_block(&block)?;

        // --- Update Cache --- 
        self.current_tip_hash = Some(block_hash);
        self.current_height = Some(header.height);

        info!("Block {} added successfully. New height: {}, New tip: {}",
               header.height, self.current_height.unwrap(), hex::encode(self.current_tip_hash.unwrap()));

        Ok(())
    }

    // TODO: Add method for mining a new block (integrating mempool, consensus::mine, add_block)
    // pub fn mine_and_add_block(&mut self, miner_address: Address) -> Result<Block, BlockchainError> { ... }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module
    use tempfile::tempdir;
    use std::thread; // For sleep
    use std::time::Duration;

    // Helper to create a basic block for testing add_block
    fn create_test_block(prev_hash: Hash, height: u64, difficulty: u32, transactions: Vec<Transaction>) -> Block {
        let mut block = Block::new(prev_hash, transactions, difficulty, height);
        // Simulate mining to find a nonce (use actual consensus::mine)
        let _mined_hash = consensus::mine(&mut block.header, difficulty);
        block
    }

    #[test]
    fn blockchain_new_empty_storage() {
        let dir = tempdir().unwrap();
        let blockchain = Blockchain::new(dir.path());
        assert!(blockchain.is_ok());
        let bc = blockchain.unwrap();
        assert!(bc.current_tip_hash.is_none());
        assert!(bc.current_height.is_none());
    }

    #[test]
    fn blockchain_initialize_genesis() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        assert!(blockchain.initialize_genesis_if_needed().is_ok());

        // Verify state after genesis creation
        assert!(blockchain.current_tip_hash.is_some());
        assert_eq!(blockchain.current_height, Some(0));

        // Verify genesis block in storage
        let genesis_block_opt = blockchain.storage.get_block_by_height(0).unwrap();
        assert!(genesis_block_opt.is_some());
        let genesis_block = genesis_block_opt.unwrap();
        assert_eq!(genesis_block.header.height, 0);
        assert_eq!(genesis_block.header.previous_hash, [0u8; 32]);
        assert_eq!(blockchain.current_tip_hash.unwrap(), genesis_block.hash());

        // Try initializing again, should do nothing
        assert!(blockchain.initialize_genesis_if_needed().is_ok());
        assert_eq!(blockchain.current_height, Some(0)); // Height should remain 0
    }

    #[test]
    fn blockchain_add_block_valid() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let genesis_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        let block1 = create_test_block(genesis_hash, height + 1, difficulty, vec![tx]);
        let block1_hash = block1.hash();

        let add_result = blockchain.add_block(block1.clone());
        assert!(add_result.is_ok());

        assert_eq!(blockchain.get_chain_height(), Some(1));
        assert_eq!(blockchain.get_last_block_hash(), Some(block1_hash));

        // Verify block is in storage
        let stored_block = blockchain.get_block_by_height(1).unwrap().unwrap();
        assert_eq!(stored_block.hash(), block1_hash);
        assert_eq!(stored_block.header.height, 1);
    }

    #[test]
    fn blockchain_add_block_invalid_height() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let genesis_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        // Create block with wrong height (height + 2 instead of height + 1)
        let block_wrong_height = create_test_block(genesis_hash, height + 2, difficulty, vec![tx]);

        let add_result = blockchain.add_block(block_wrong_height);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Validation(msg) => assert!(msg.contains("Invalid block height")),
            _ => panic!("Expected Validation Error"),
        }
        assert_eq!(blockchain.get_chain_height(), Some(0)); // Height should not change
    }

     #[test]
    fn blockchain_add_block_invalid_prev_hash() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let _genesis_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        // Create block with wrong previous hash
        let wrong_prev_hash = [99u8; 32];
        let block_wrong_prev = create_test_block(wrong_prev_hash, height + 1, difficulty, vec![tx]);

        let add_result = blockchain.add_block(block_wrong_prev);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Validation(msg) => assert!(msg.contains("Invalid previous block hash")),
            _ => panic!("Expected Validation Error"),
        }
        assert_eq!(blockchain.get_chain_height(), Some(0));
    }

    #[test]
    fn blockchain_add_block_invalid_pow() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let genesis_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();

        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        let mut block_invalid_pow = Block::new(genesis_hash, vec![tx], difficulty, height + 1);
        // Don't mine it, just set a random nonce
        block_invalid_pow.header.nonce = 12345;

        let add_result = blockchain.add_block(block_invalid_pow);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Consensus(msg) => assert!(msg.contains("Invalid Proof-of-Work")),
            _ => panic!("Expected Consensus Error"),
        }
        assert_eq!(blockchain.get_chain_height(), Some(0));
    }

    #[test]
    fn blockchain_add_block_incorrect_difficulty() {
        let dir = tempdir().unwrap();
        let mut blockchain = Blockchain::new(dir.path()).unwrap();
        blockchain.initialize_genesis_if_needed().unwrap();

        let genesis_hash = blockchain.get_last_block_hash().unwrap();
        let height = blockchain.get_chain_height().unwrap();
        let correct_difficulty = consensus::calculate_next_difficulty(height, &blockchain.storage).unwrap();
        let incorrect_difficulty = correct_difficulty + 1; // Set wrong difficulty

        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        // Create block but mine it with the *incorrect* difficulty
        let block_wrong_diff = create_test_block(genesis_hash, height + 1, incorrect_difficulty, vec![tx]);

        let add_result = blockchain.add_block(block_wrong_diff);
        assert!(add_result.is_err());
        match add_result.err().unwrap() {
            BlockchainError::Consensus(msg) => assert!(msg.contains("Incorrect difficulty")),
            e => panic!("Expected Consensus Error for difficulty, got {:?}", e),
        }
        assert_eq!(blockchain.get_chain_height(), Some(0));
    }

    // --- Previous tests (need refactoring later) ---

    #[test]
    fn serialization_deserialization() {
        // Use bincode now
        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        let serialized = bincode::serialize(&tx).unwrap();
        let deserialized: Transaction = bincode::deserialize(&serialized).unwrap();
        assert_eq!(tx, deserialized);

        let block = Block::new([0u8; 32], vec![tx], 2, 1);
        let serialized_block = bincode::serialize(&block).unwrap();
        let deserialized_block: Block = bincode::deserialize(&serialized_block).unwrap();
        assert_eq!(block.header, deserialized_block.header);
        assert_eq!(block.transactions, deserialized_block.transactions);
    }

    #[test]
    fn hash_calculation_consistency() {
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 100);
        let tx2 = Transaction::new_transfer(vec![1], vec![2], 100); // Identical transaction
        let tx3 = Transaction::new_transfer(vec![3], vec![4], 200);

        let hash1 = tx1.calculate_hash();
        let hash2 = tx2.calculate_hash();
        let hash3 = tx3.calculate_hash();

        assert_eq!(hash1, hash2, "Identical transactions should have the same hash");
        assert_ne!(hash1, hash3, "Different transactions should have different hashes");

        let header1 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 0, difficulty: 1, height: 1 };
        let header2 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 0, difficulty: 1, height: 1 }; // Identical
        let header3 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 1, difficulty: 1, height: 1 }; // Different nonce

        let h_hash1 = header1.calculate_hash();
        let h_hash2 = header2.calculate_hash();
        let h_hash3 = header3.calculate_hash();

        assert_eq!(h_hash1, h_hash2, "Identical headers should have the same hash");
        assert_ne!(h_hash1, h_hash3, "Different headers should have different hashes");
    }
}

