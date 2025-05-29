use crate::core::{Transaction, Hash};
use std::collections::{HashMap, VecDeque};
use log::debug;

/// Manages pending transactions that have not yet been included in a block.
#[derive(Debug)]
pub struct Mempool {
    /// Stores transactions keyed by their hash for quick lookup and deduplication.
    transactions: HashMap<Hash, Transaction>,
    /// Maintains the order in which transactions arrived (FIFO for now).
    order: VecDeque<Hash>,
    /// Maximum number of transactions allowed in the mempool.
    max_size: usize,
}

impl Mempool {
    /// Creates a new Mempool with a specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Mempool {
            transactions: HashMap::new(),
            order: VecDeque::new(),
            max_size,
        }
    }

    /// Adds a transaction to the mempool if valid and space permits.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to add.
    ///
    /// # Returns
    ///
    /// * `Ok(bool)` - Returns `Ok(true)` if the transaction was added, `Ok(false)` if it already existed.
    /// * `Err(String)` - If the transaction is invalid or the mempool is full.
pub fn add_transaction(&mut self, tx: Transaction) -> Result<bool, String> {
        // TODO: Add more sophisticated validation (e.g., signature verification, balance checks)
        let tx_hash = tx.calculate_hash();

        if self.transactions.contains_key(&tx_hash) {
            debug!("Transaction {} already exists in mempool.", hex::encode(tx_hash));
            return Ok(false); // Indicate transaction already present
        }

        if self.transactions.len() >= self.max_size {
            // Option 1: Reject new transaction (simple approach)
            // return Err("Mempool is full".to_string());

            // Option 2: Evict the oldest transaction
            if let Some(oldest_hash) = self.order.pop_front() {
                self.transactions.remove(&oldest_hash);
                debug!("Mempool full. Evicted oldest transaction: {}", hex::encode(oldest_hash));
            } else {
                 // Should not happen if len >= max_size and max_size > 0
                 return Err("Mempool full, but failed to evict oldest transaction".to_string());
            }
        }

        debug!("Adding transaction {} to mempool.", hex::encode(tx_hash));
        self.transactions.insert(tx_hash, tx);
        self.order.push_back(tx_hash);

        Ok(true) // Indicate transaction was added
    }

    /// Retrieves a batch of transactions from the mempool.
    ///
    /// Returns up to `max_count` transactions, prioritizing older ones (FIFO).
    ///
    /// # Arguments
    ///
    /// * `max_count` - The maximum number of transactions to retrieve.
    ///
    /// # Returns
    ///
    /// * A vector containing the retrieved transactions.
pub fn get_transactions(&self, max_count: usize) -> Vec<Transaction> {
        self.order
            .iter()
            .take(max_count)
            .filter_map(|hash| self.transactions.get(hash).cloned()) // Clone transactions
            .collect()
    }

    /// Removes a list of transactions from the mempool, typically after they've been included in a block.
    ///
    /// # Arguments
    ///
    /// * `tx_hashes` - A slice of transaction hashes to remove.
pub fn remove_transactions(&mut self, tx_hashes: &[Hash]) {
        let mut removed_count = 0;
        for tx_hash in tx_hashes {
            if self.transactions.remove(tx_hash).is_some() {
                // Also remove from the order queue (less efficient, but necessary)
                self.order.retain(|h| h != tx_hash);
                removed_count += 1;
            }
        }
        if removed_count > 0 {
             debug!("Removed {} transactions from mempool.", removed_count);
        }
    }

    /// Returns the current number of transactions in the mempool.
    pub fn size(&self) -> usize {
        self.transactions.len()
    }

    /// Checks if the mempool is empty.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Transaction;

    #[test]
    fn mempool_add_and_get() {
        let mut mempool = Mempool::new(10);
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 100);
        let tx1_hash = tx1.calculate_hash();
        let tx2 = Transaction::new_transfer(vec![3], vec![4], 200);
        let tx2_hash = tx2.calculate_hash();

        assert!(mempool.add_transaction(tx1.clone()).unwrap());
        assert_eq!(mempool.size(), 1);
        assert!(mempool.add_transaction(tx2.clone()).unwrap());
        assert_eq!(mempool.size(), 2);

        // Try adding duplicate
        assert!(!mempool.add_transaction(tx1.clone()).unwrap());
        assert_eq!(mempool.size(), 2);

        // Get transactions
        let retrieved = mempool.get_transactions(5);
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].calculate_hash(), tx1_hash); // FIFO order
        assert_eq!(retrieved[1].calculate_hash(), tx2_hash);

        let retrieved_limited = mempool.get_transactions(1);
        assert_eq!(retrieved_limited.len(), 1);
        assert_eq!(retrieved_limited[0].calculate_hash(), tx1_hash);
    }

    #[test]
    fn mempool_remove() {
        let mut mempool = Mempool::new(10);
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 100);
        let tx1_hash = tx1.calculate_hash();
        let tx2 = Transaction::new_transfer(vec![3], vec![4], 200);
        let tx2_hash = tx2.calculate_hash();

        mempool.add_transaction(tx1.clone()).unwrap();
        mempool.add_transaction(tx2.clone()).unwrap();
        assert_eq!(mempool.size(), 2);

        // Remove tx1
        mempool.remove_transactions(&[tx1_hash]);
        assert_eq!(mempool.size(), 1);
        assert!(mempool.transactions.get(&tx1_hash).is_none());
        assert!(mempool.transactions.get(&tx2_hash).is_some());
        assert_eq!(mempool.order.len(), 1);
        assert_eq!(mempool.order[0], tx2_hash);

        // Remove tx2
        mempool.remove_transactions(&[tx2_hash]);
        assert_eq!(mempool.size(), 0);
        assert!(mempool.is_empty());
        assert!(mempool.order.is_empty());

        // Remove non-existent
        mempool.remove_transactions(&[[0u8; 32]]);
        assert_eq!(mempool.size(), 0);
    }

    #[test]
    fn mempool_max_size_eviction() {
        let mut mempool = Mempool::new(2);
        let tx1 = Transaction::new_transfer(vec![1], vec![2], 100);
        let tx1_hash = tx1.calculate_hash();
        let tx2 = Transaction::new_transfer(vec![3], vec![4], 200);
        let tx2_hash = tx2.calculate_hash();
        let tx3 = Transaction::new_transfer(vec![5], vec![6], 300);
        let tx3_hash = tx3.calculate_hash();

        mempool.add_transaction(tx1.clone()).unwrap();
        mempool.add_transaction(tx2.clone()).unwrap();
        assert_eq!(mempool.size(), 2);

        // Add tx3, should evict tx1 (oldest)
        assert!(mempool.add_transaction(tx3.clone()).unwrap());
        assert_eq!(mempool.size(), 2);
        assert!(mempool.transactions.get(&tx1_hash).is_none()); // tx1 evicted
        assert!(mempool.transactions.get(&tx2_hash).is_some());
        assert!(mempool.transactions.get(&tx3_hash).is_some());
        assert_eq!(mempool.order.len(), 2);
        assert_eq!(mempool.order[0], tx2_hash); // tx2 is now oldest
        assert_eq!(mempool.order[1], tx3_hash);
    }
}

