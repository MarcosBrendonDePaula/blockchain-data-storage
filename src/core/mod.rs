use std::time::{SystemTime, UNIX_EPOCH};

// Placeholder for cryptographic hash type (e.g., using sha2 crate later)
type Hash = Vec<u8>;

// Placeholder for public key/address type
type Address = Vec<u8>;

// Represents a single transaction in the blockchain
#[derive(Debug, Clone)] // Added Clone
pub struct Transaction {
    sender: Address,       // Address of the sender
    receiver: Address,     // Address of the receiver (can be contract or user)
    amount: u64,           // Amount of native currency transferred (if any)
    timestamp: u64,        // Timestamp of transaction creation (Unix epoch seconds)
    data_hash: Option<Hash>, // Optional hash of the off-chain data being stored
    data_size: Option<u64>, // Optional size of the original off-chain data
    // signature: Vec<u8>, // Signature to verify sender authenticity (add later)
    // nonce: u64,         // Transaction nonce for replay protection (add later)
}

impl Transaction {
    // Basic constructor for a simple transfer (example)
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

    // Basic constructor for a data storage transaction (example)
    pub fn new_storage(sender: Address, data_hash: Hash, data_size: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Transaction {
            sender,
            receiver: vec![], // Or a specific contract address for storage
            amount: 0, // Storage might have associated fees, handled by miner
            timestamp,
            data_hash: Some(data_hash),
            data_size: Some(data_size),
        }
    }

    // Method to calculate transaction hash (placeholder)
    pub fn calculate_hash(&self) -> Hash {
        // In a real implementation, serialize the transaction fields
        // and hash the result using a cryptographic hash function.
        // For now, return a dummy hash.
        vec![0u8; 32] // Dummy 32-byte hash
    }
}

// Represents the header of a block
#[derive(Debug, Clone)] // Added Clone
pub struct BlockHeader {
    previous_hash: Hash, // Hash of the previous block in the chain
    merkle_root: Hash,   // Merkle root of all transactions in the block
    timestamp: u64,      // Timestamp of block creation (Unix epoch seconds)
    nonce: u64,          // Nonce found during Proof-of-Work mining
    difficulty: u32,     // Difficulty target for this block (adjust representation later)
    height: u64,         // Block height in the chain
}

impl BlockHeader {
    // Method to calculate block header hash (placeholder)
    pub fn calculate_hash(&self) -> Hash {
        // In a real implementation, serialize the header fields
        // and hash the result.
        vec![1u8; 32] // Dummy 32-byte hash
    }
}

// Represents a full block in the blockchain
#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    // Basic constructor (example)
    pub fn new(previous_hash: Hash, transactions: Vec<Transaction>, difficulty: u32, height: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Calculate Merkle root (placeholder)
        let merkle_root = Self::calculate_merkle_root(&transactions);

        // Nonce would be found by the miner
        let nonce = 0; // Placeholder

        let header = BlockHeader {
            previous_hash,
            merkle_root,
            timestamp,
            nonce, // This will be set by the mining process
            difficulty,
            height,
        };

        Block { header, transactions }
    }

    // Placeholder for Merkle root calculation
    fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return vec![0u8; 32]; // Or a specific empty hash
        }
        // In a real implementation, build the Merkle tree and get the root hash.
        // For now, hash the first transaction's hash as a dummy root.
        transactions[0].calculate_hash()
    }

    // Method to get block hash (hash of the header)
    pub fn hash(&self) -> Hash {
        self.header.calculate_hash()
    }
}

// Basic blockchain structure (in-memory for now)
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: u32, // Current network difficulty
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
        // Create the first block (genesis block)
        let genesis_block = Block::new(vec![0u8; 32], Vec::new(), self.difficulty, 0);
        // In a real PoW chain, the genesis block's nonce might be pre-calculated
        // or found via mining with a specific target.
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Basic validation could happen here
        self.pending_transactions.push(transaction);
    }

    pub fn get_last_block_hash(&self) -> Hash {
        self.chain.last().expect("Chain is empty").hash()
    }

    // Placeholder for mining process
    pub fn mine_pending_transactions(&mut self, miner_address: Address) {
        if self.pending_transactions.is_empty() {
            println!("No pending transactions to mine.");
            return;
        }

        let previous_hash = self.get_last_block_hash();
        let height = self.chain.len() as u64;

        // In a real scenario, a reward transaction for the miner would be added.
        // let reward_tx = Transaction::new_reward(miner_address, REWARD_AMOUNT);
        // let mut block_transactions = self.pending_transactions.clone();
        // block_transactions.insert(0, reward_tx); // Add reward tx

        let mut new_block = Block::new(previous_hash, self.pending_transactions.clone(), self.difficulty, height);

        // --- Proof-of-Work (Simplified Placeholder) ---
        // In reality, this loop would adjust the nonce and recalculate the header hash
        // until it meets the difficulty target.
        println!("Mining block {}...", height);
        // loop {
        //     let hash = new_block.header.calculate_hash();
        //     if Self::hash_meets_difficulty(&hash, self.difficulty) {
        //         println!("Block mined! Nonce: {}, Hash: {:?}", new_block.header.nonce, hash);
        //         break;
        //     }
        //     new_block.header.nonce += 1;
        //     new_block.header.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); // Update timestamp during mining
        // }
        // --- End PoW Placeholder ---

        // For this example, we assume the block is mined instantly with nonce 0
        println!("Block {} mined (placeholder).", height);

        self.chain.push(new_block);
        self.pending_transactions.clear();

        // Potentially adjust difficulty here based on mining time
    }

    // Placeholder for difficulty check
    // fn hash_meets_difficulty(hash: &Hash, difficulty: u32) -> bool {
    //     // Check if the hash starts with `difficulty` number of zero bits (simplified)
    //     let prefix = vec![0u8; (difficulty / 8) as usize];
    //     hash.starts_with(&prefix)
    // }

    // Basic validation (placeholder)
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if current block's hash is correct
            // if current_block.hash() != current_block.header.calculate_hash() {
            //     return false;
            // }

            // Check if previous_hash matches
            if current_block.header.previous_hash != previous_block.hash() {
                return false;
            }
            // Add more checks: PoW validation, transaction validation, etc.
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module

    #[test]
    fn it_works() {
        // Example usage
        let difficulty = 2; // Example difficulty
        let mut my_blockchain = Blockchain::new(difficulty);

        let sender_addr = vec![1];
        let receiver_addr = vec![2];
        let data_addr = vec![3];

        my_blockchain.add_transaction(Transaction::new_transfer(sender_addr.clone(), receiver_addr.clone(), 100));
        my_blockchain.add_transaction(Transaction::new_storage(data_addr.clone(), vec![5; 32], 1024));

        println!("Starting miner...");
        my_blockchain.mine_pending_transactions(vec![10]); // Miner's address

        println!("Blockchain valid: {}", my_blockchain.is_chain_valid());
        println!("Number of blocks: {}", my_blockchain.chain.len());

        my_blockchain.add_transaction(Transaction::new_transfer(receiver_addr.clone(), sender_addr.clone(), 50));
        my_blockchain.mine_pending_transactions(vec![11]);

        println!("Blockchain valid: {}", my_blockchain.is_chain_valid());
        println!("Number of blocks: {}", my_blockchain.chain.len());

        // Add basic assertion
        assert_eq!(my_blockchain.chain.len(), 3); // Genesis + 2 mined blocks
        assert!(my_blockchain.is_chain_valid());
    }
}

