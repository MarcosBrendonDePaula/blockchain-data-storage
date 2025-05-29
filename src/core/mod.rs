use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;

// Define Hash as a fixed-size array for SHA-256
pub type Hash = [u8; 32];

// Placeholder for public key/address type
// Using Vec<u8> which is serializable
pub type Address = Vec<u8>;

// Represents a single transaction in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // Added PartialEq, Eq for testing
pub struct Transaction {
    sender: Address,
    receiver: Address,
    amount: u64,
    timestamp: u64,
    data_hash: Option<Vec<u8>>, // Use Vec<u8> for Option<Hash> compatibility with serde
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
            data_hash: Some(data_hash.to_vec()), // Convert Hash to Vec<u8> for storage
            data_size: Some(data_size),
        }
    }

    // Calculate the SHA-256 hash of the transaction
    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        // Use JSON serialization for hashing for simplicity, though a canonical binary format is better
        let serialized = serde_json::to_vec(self).expect("Failed to serialize transaction for hashing");
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

// Represents the header of a block
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // Added PartialEq, Eq
pub struct BlockHeader {
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
    pub height: u64,
}

impl BlockHeader {
    // Calculate the SHA-256 hash of the block header
    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_vec(self).expect("Failed to serialize block header for hashing");
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

// Represents a full block in the blockchain
#[derive(Serialize, Deserialize, Debug)]
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
    // Simple placeholder: hash of concatenated tx hashes
    // TODO: Implement proper Merkle tree construction
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return [0u8; 32]; // Return fixed-size array
        }
        let mut hasher = Sha256::new();
        for tx in transactions {
            hasher.update(tx.calculate_hash());
        }
        hasher.finalize().into()
    }

    // Get the block hash (hash of the header)
    pub fn hash(&self) -> Hash {
        self.header.calculate_hash()
    }
}

// Basic blockchain structure (in-memory for now)
#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: u32,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_transactions = Vec::new();
        // Create a header manually for genesis
        let merkle_root = Block::calculate_merkle_root(&genesis_transactions);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let genesis_header = BlockHeader {
            previous_hash: [0u8; 32],
            merkle_root,
            timestamp,
            nonce: 0, // Typically pre-defined or mined
            difficulty: self.difficulty,
            height: 0,
        };
        let genesis_block = Block {
            header: genesis_header,
            transactions: genesis_transactions,
        };
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.chain.last().map(|block| block.hash())
    }

    pub fn mine_pending_transactions(&mut self, _miner_address: Address) {
        if self.pending_transactions.is_empty() {
            println!("No pending transactions to mine.");
            return;
        }

        let previous_hash = self.get_last_block_hash().unwrap_or([0u8; 32]);
        let height = self.chain.len() as u64;

        let mut new_block = Block::new(previous_hash, self.pending_transactions.clone(), self.difficulty, height);

        // --- Proof-of-Work (Simplified Placeholder) ---
        println!("Mining block {}...", height);
        // PoW logic (finding nonce) will be in consensus module.
        // For now, calculate the hash with nonce 0.
        new_block.header.nonce = 0; // Placeholder
        let _block_hash = new_block.hash(); // Calculate hash with nonce 0
        // --- End PoW Placeholder ---

        println!("Block {} mined (placeholder) with hash: {}", height, hex::encode(_block_hash));

        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn is_chain_valid(&self) -> bool {
        if self.chain.is_empty() {
            return true;
        }

        // Check genesis block hash (optional)
        // if self.chain[0].hash() != self.chain[0].header.calculate_hash() {
        //     println!("Genesis block hash mismatch");
        //     return false;
        // }

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if current block's hash is correct
            if current_block.hash() != current_block.header.calculate_hash() {
                println!("Block {} hash mismatch", current_block.header.height);
                return false;
            }

            // Check if previous_hash matches
            if current_block.header.previous_hash != previous_block.hash() {
                 println!("Block {} previous hash mismatch. Expected: {}, Got: {}",
                          current_block.header.height,
                          hex::encode(previous_block.hash()),
                          hex::encode(current_block.header.previous_hash));
                return false;
            }

            // Check Merkle root
            let calculated_merkle_root = Block::calculate_merkle_root(&current_block.transactions);
            if current_block.header.merkle_root != calculated_merkle_root {
                println!("Block {} Merkle root mismatch. Expected: {}, Got: {}",
                         current_block.header.height,
                         hex::encode(calculated_merkle_root),
                         hex::encode(current_block.header.merkle_root));
                return false;
            }

            // TODO: Add PoW validation (consensus module)
            // TODO: Add transaction validation within the block
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module

    #[test]
    fn basic_blockchain_operations() {
        let difficulty = 2;
        let mut my_blockchain = Blockchain::new(difficulty);

        let sender_addr = vec![1];
        let receiver_addr = vec![2];
        let data_addr = vec![3];

        my_blockchain.add_transaction(Transaction::new_transfer(sender_addr.clone(), receiver_addr.clone(), 100));
        my_blockchain.add_transaction(Transaction::new_storage(data_addr.clone(), [5u8; 32], 1024));
        assert_eq!(my_blockchain.pending_transactions.len(), 2);

        println!("Mining first block...");
        my_blockchain.mine_pending_transactions(vec![10]);
        assert_eq!(my_blockchain.chain.len(), 2);
        assert!(my_blockchain.pending_transactions.is_empty());
        assert!(my_blockchain.is_chain_valid(), "Chain became invalid after first mining");

        my_blockchain.add_transaction(Transaction::new_transfer(receiver_addr.clone(), sender_addr.clone(), 50));
        assert_eq!(my_blockchain.pending_transactions.len(), 1);

        println!("Mining second block...");
        my_blockchain.mine_pending_transactions(vec![11]);
        assert_eq!(my_blockchain.chain.len(), 3);
        assert!(my_blockchain.pending_transactions.is_empty());
        assert!(my_blockchain.is_chain_valid(), "Chain became invalid after second mining");
    }

    #[test]
    fn serialization_deserialization() {
        let tx = Transaction::new_transfer(vec![1], vec![2], 100);
        let serialized = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tx, deserialized);

        let block = Block::new([0u8; 32], vec![tx], 2, 1);
        let serialized_block = serde_json::to_string(&block).unwrap();
        let deserialized_block: Block = serde_json::from_str(&serialized_block).unwrap();
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

        println!("TX1 Hash: {}", hex::encode(hash1));
        println!("TX2 Hash: {}", hex::encode(hash2));
        println!("TX3 Hash: {}", hex::encode(hash3));

        assert_eq!(hash1, hash2, "Identical transactions should have the same hash");
        assert_ne!(hash1, hash3, "Different transactions should have different hashes");

        let header1 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 0, difficulty: 1, height: 1 };
        let header2 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 0, difficulty: 1, height: 1 }; // Identical
        let header3 = BlockHeader { previous_hash: [0u8; 32], merkle_root: [1u8; 32], timestamp: 123, nonce: 1, difficulty: 1, height: 1 }; // Different nonce

        let h_hash1 = header1.calculate_hash();
        let h_hash2 = header2.calculate_hash();
        let h_hash3 = header3.calculate_hash();

        println!("Header1 Hash: {}", hex::encode(h_hash1));
        println!("Header2 Hash: {}", hex::encode(h_hash2));
        println!("Header3 Hash: {}", hex::encode(h_hash3));

        assert_eq!(h_hash1, h_hash2, "Identical headers should have the same hash");
        assert_ne!(h_hash1, h_hash3, "Different headers should have different hashes");
    }
}

