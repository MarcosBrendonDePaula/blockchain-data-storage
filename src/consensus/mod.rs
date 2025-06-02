//!
//! # Consensus Module
//! Handles the blockchain's consensus mechanism, currently Proof-of-Work (PoW)
//! with dynamic difficulty adjustment.

use crate::core::{BlockHeader, Hash};
use crate::storage::StorageManager; // Import StorageManager
use hex;
use log::{info, warn};
use std::cmp::{max, min};

// --- Difficulty Adjustment Parameters ---

/// Target time for mining a block (e.g., 10 minutes).
pub const TARGET_BLOCK_TIME_SECS: u64 = 600;
/// Number of blocks after which difficulty is recalculated (e.g., 2016 blocks).
pub const ADJUSTMENT_INTERVAL_BLOCKS: u64 = 20; // Lowered for easier testing initially
/// Minimum difficulty (leading zero bits).
pub const MIN_DIFFICULTY: u32 = 4;
/// Maximum difficulty (leading zero bits) - prevents runaway difficulty.
pub const MAX_DIFFICULTY: u32 = 60; // Arbitrary limit, adjust as needed
/// Maximum factor by which difficulty can change in one adjustment (e.g., 4x).
pub const MAX_DIFFICULTY_CHANGE_FACTOR: f64 = 4.0;

// --- Proof-of-Work Functions ---

/// Verifies if a given hash meets the required difficulty target (leading zero bits).
///
/// The difficulty represents the number of leading zero bits required in the hash.
/// For example, a difficulty of 8 requires the hash to start with `0x00...`,
/// while a difficulty of 10 requires `0x00` followed by a byte starting with `00xxxxxx`.
///
/// # Arguments
///
/// * `hash` - The hash (`[u8; 32]`) to check.
/// * `difficulty` - The required number of leading zero bits.
///
/// # Returns
///
/// * `true` if the hash meets the difficulty target, `false` otherwise.
pub fn verify_pow(hash: &Hash, difficulty: u32) -> bool {
    if difficulty == 0 {
        return true;
    }
    let required_zero_bytes = (difficulty / 8) as usize;
    let required_zero_bits_in_next_byte = (difficulty % 8) as u8;

    // Check full zero bytes
    if !hash.iter().take(required_zero_bytes).all(|&byte| byte == 0) {
        return false;
    }

    // Check remaining bits in the next byte (if any)
    if required_zero_bits_in_next_byte > 0 {
        if let Some(&next_byte) = hash.get(required_zero_bytes) {
            // Create a mask for the required zero bits.
            // e.g., difficulty % 8 = 3 => mask = 11100000 (binary) = 0xE0
            let mask = 0xFFu8 << (8 - required_zero_bits_in_next_byte);
            // Check if the leading bits are zero by masking
            if (next_byte & mask) != 0 {
                return false;
            }
        } else {
            // Hash is too short to meet the difficulty requirement
            return false;
        }
    }
    true
}

/// Performs the Proof-of-Work mining process by iterating through nonces.
///
/// Finds a `nonce` for the given `BlockHeader` such that its SHA-256 hash
/// meets the specified `difficulty` target (number of leading zero bits).
/// The `difficulty` field within the `header` is also updated to the provided `difficulty`.
///
/// # Arguments
///
/// * `header` - A mutable reference to the `BlockHeader`. Its `nonce` will be modified
///              until a valid hash is found, and its `difficulty` field will be set.
/// * `difficulty` - The required difficulty (number of leading zero bits) for this block.
///
/// # Returns
///
/// * The valid `Hash` (`[u8; 32]`) that meets the difficulty target.
///
/// # Panics
///
/// * Panics if the nonce overflows `u64::MAX` before finding a solution.
pub fn mine(header: &mut BlockHeader, difficulty: u32) -> Hash {
    info!(
        "Mining block {} with difficulty {}...",
        header.height,
        difficulty
    );
    let start_time = std::time::Instant::now();
    header.difficulty = difficulty; // Set the difficulty used for mining this block

    loop {
        let hash = header.calculate_hash();
        if verify_pow(&hash, difficulty) {
            let duration = start_time.elapsed();
            info!(
                "Block {} mined! Nonce: {}, Hash: {}, Time: {:?}",
                header.height,
                header.nonce,
                hex::encode(hash),
                duration
            );
            return hash;
        }
        // Increment nonce and try again
        header.nonce = header.nonce.checked_add(1).expect("Nonce overflow during mining");
        // Optional: Update timestamp periodically if mining takes very long?
        // header.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
}

// --- Difficulty Adjustment Logic ---

/// Calculates the required difficulty for the *next* block based on the time taken for the previous interval.
///
/// The difficulty is adjusted every `ADJUSTMENT_INTERVAL_BLOCKS`. The calculation compares the actual time
/// taken to mine the last interval of blocks against the `TARGET_BLOCK_TIME_SECS` per block.
/// The adjustment is capped by `MAX_DIFFICULTY_CHANGE_FACTOR` to prevent extreme swings.
/// The difficulty is represented as the number of leading zero bits required in the block hash.
///
/// # Arguments
///
/// * `current_height` - The height of the *last* mined block (the one potentially triggering the adjustment).
/// * `storage` - A reference to the `StorageManager` used to fetch the headers of the current block
///               and the block at the start of the previous adjustment interval.
///
/// # Returns
///
/// * `Ok(u32)` - The calculated difficulty (leading zero bits) for the next block (`current_height + 1`).
/// * `Err(String)` - An error message if required blocks are not found in storage or other issues occur.
pub fn calculate_next_difficulty(current_height: u64, storage: &StorageManager) -> Result<u32, String> {
    // Fetch the header of the current (latest) block to get its difficulty and timestamp.
    let current_header = storage.get_block_by_height(current_height)
        .map_err(|e| format!("DB error getting current block {}: {}", current_height, e))?
        .ok_or_else(|| format!("Current block {} not found in storage for difficulty calc", current_height))?
        .header;

    let current_difficulty = current_header.difficulty;

    // Check if the *next* block marks the end of an adjustment interval.
    // The difficulty calculated now will apply to block `current_height + 1`.
    if (current_height + 1) % ADJUSTMENT_INTERVAL_BLOCKS != 0 {
        // Not an adjustment block, return the difficulty of the current block.
        return Ok(current_difficulty);
    }

    // Determine the height of the block that started the interval just completed.
    let interval_start_height = current_height.saturating_sub(ADJUSTMENT_INTERVAL_BLOCKS - 1);

    // Avoid adjusting based on the genesis block if the interval goes back that far.
    if interval_start_height == 0 {
        return Ok(current_difficulty);
    }

    // Fetch the header of the block at the start of the interval.
    let interval_start_header = storage.get_block_by_height(interval_start_height)
        .map_err(|e| format!("DB error getting interval start block {}: {}", interval_start_height, e))?
        .ok_or_else(|| format!("Interval start block {} not found for difficulty calc", interval_start_height))?
        .header;

    // Calculate the actual time elapsed during the interval.
    let actual_time_secs = current_header.timestamp.saturating_sub(interval_start_header.timestamp);

    // Avoid division by zero if timestamps are identical or invalid.
    if actual_time_secs == 0 {
        warn!(
            "Actual time for difficulty adjustment interval [{}, {}] is zero. Keeping difficulty {}.",
            interval_start_height, current_height, current_difficulty
        );
        // Consider a small increase in difficulty as a fallback?
        return Ok(max(MIN_DIFFICULTY, min(current_difficulty + 1, MAX_DIFFICULTY)));
    }

    // Calculate the target time for the interval.
    let target_time_secs = TARGET_BLOCK_TIME_SECS * ADJUSTMENT_INTERVAL_BLOCKS;

    // Calculate the adjustment factor: (target_time / actual_time).
    let mut adjustment_factor = target_time_secs as f64 / actual_time_secs as f64;
    info!(
        "Difficulty adjustment check at height {}: Interval [{}, {}], Actual time: {}s, Target time: {}s, Raw Factor: {:.4}",
        current_height + 1,
        interval_start_height,
        current_height,
        actual_time_secs,
        target_time_secs,
        adjustment_factor
    );

    // Clamp the adjustment factor to prevent excessive swings.
    adjustment_factor = adjustment_factor.clamp(1.0 / MAX_DIFFICULTY_CHANGE_FACTOR, MAX_DIFFICULTY_CHANGE_FACTOR);
    info!("Clamped Adjustment Factor: {:.4}", adjustment_factor);

    // Calculate the new difficulty.
    // If factor > 1.0 (blocks too fast), difficulty increases (more zero bits).
    // If factor < 1.0 (blocks too slow), difficulty decreases (fewer zero bits).
    let new_difficulty_float = current_difficulty as f64 * adjustment_factor;

    // Round to the nearest integer and apply absolute min/max limits.
    let mut new_difficulty = new_difficulty_float.round() as u32;
    new_difficulty = max(MIN_DIFFICULTY, min(new_difficulty, MAX_DIFFICULTY));

    info!(
        "Difficulty adjusted from {} to {} for block {}",
        current_difficulty,
        new_difficulty,
        current_height + 1
    );

    Ok(new_difficulty)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Block, BlockHeader, Transaction};
    use crate::storage::StorageManager;
    use tempfile::tempdir;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Helper to create a dummy block with specific timestamp and difficulty
    fn create_test_block_with_details(height: u64, previous_hash: Hash, timestamp: u64, difficulty: u32) -> Block {
        let transactions = vec![Transaction::new_transfer(vec![height as u8], vec![], 0)]; // Simple tx
        let mut header = BlockHeader {
            previous_hash,
            merkle_root: [0u8; 32], // Placeholder
            timestamp,
            nonce: 0, // Placeholder
            difficulty,
            height,
        };
        header.merkle_root = Block::calculate_merkle_root(&transactions);
        // We don't actually mine here, just create the block structure
        Block {
            header,
            transactions,
        }
    }

    #[test]
    fn test_verify_pow_simple() {
        let hash1 = [0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash1, 8));
        assert!(!verify_pow(&hash1, 9));
        let hash2 = [0x0F, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash2, 4));
        assert!(!verify_pow(&hash2, 5));
        let hash3 = [0x00, 0x00, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash3, 16));
        assert!(!verify_pow(&hash3, 17));
        let hash4 = [0x00, 0x00, 0x7F, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash4, 17));
        assert!(!verify_pow(&hash4, 18));
    }

    #[test]
    fn test_mine_simple() {
        let difficulty = 8;
        let mut header = BlockHeader {
            previous_hash: [0u8; 32],
            merkle_root: [1u8; 32],
            timestamp: 1234567890,
            nonce: 0,
            difficulty, // Difficulty is set before mining
            height: 1,
        };
        let final_hash = mine(&mut header, difficulty);
        assert!(verify_pow(&final_hash, difficulty));
        assert_eq!(header.calculate_hash(), final_hash);
    }

    // --- Difficulty Adjustment Tests ---

    #[test]
    fn test_difficulty_no_adjustment_before_interval() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Add blocks before the first adjustment interval
        for i in 0..(ADJUSTMENT_INTERVAL_BLOCKS - 1) {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
            timestamp += TARGET_BLOCK_TIME_SECS; // Simulate target time
        }

        // Calculate difficulty for the next block (which is *not* an adjustment block)
        let next_difficulty = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 2, &storage).unwrap();
        assert_eq!(next_difficulty, initial_difficulty);
    }

    #[test]
    fn test_difficulty_adjustment_target_time() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Add exactly ADJUSTMENT_INTERVAL_BLOCKS blocks, taking exactly the target time
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
            if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                 timestamp += TARGET_BLOCK_TIME_SECS; // Simulate target time
            }
        }

        // Calculate difficulty for the next block (height ADJUSTMENT_INTERVAL_BLOCKS)
        // Since time was exactly target, difficulty should remain the same
        let next_difficulty = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        assert_eq!(next_difficulty, initial_difficulty);
    }

    #[test]
    fn test_difficulty_adjustment_too_fast() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Add blocks, taking half the target time
        let time_per_block = TARGET_BLOCK_TIME_SECS / 2;
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block;
            }
        }

        // Calculate difficulty for the next block
        // Since blocks were too fast (factor ~2), difficulty should increase
        let next_difficulty = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        // Expected: initial_difficulty * 2 (approx, due to rounding)
        let expected_difficulty = (initial_difficulty as f64 * 2.0).round() as u32;
        assert_eq!(next_difficulty, max(MIN_DIFFICULTY, min(expected_difficulty, MAX_DIFFICULTY)));
    }

    #[test]
    fn test_difficulty_adjustment_too_slow() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Add blocks, taking double the target time
        let time_per_block = TARGET_BLOCK_TIME_SECS * 2;
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block;
            }
        }

        // Calculate difficulty for the next block
        // Since blocks were too slow (factor ~0.5), difficulty should decrease
        let next_difficulty = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        // Expected: initial_difficulty / 2 (approx, due to rounding)
        let expected_difficulty = (initial_difficulty as f64 * 0.5).round() as u32;
        assert_eq!(next_difficulty, max(MIN_DIFFICULTY, min(expected_difficulty, MAX_DIFFICULTY)));
    }

    #[test]
    fn test_difficulty_adjustment_limits() {
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Simulate extremely fast blocks (factor >> MAX_DIFFICULTY_CHANGE_FACTOR)
        let time_per_block = 1; // Very fast
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block;
            }
        }
        let next_difficulty_fast = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        // Should be limited by MAX_DIFFICULTY_CHANGE_FACTOR
        let max_expected = (initial_difficulty as f64 * MAX_DIFFICULTY_CHANGE_FACTOR).round() as u32;
        assert_eq!(next_difficulty_fast, max(MIN_DIFFICULTY, min(max_expected, MAX_DIFFICULTY)));

        // Reset and simulate extremely slow blocks
        let storage = StorageManager::new(tempdir().unwrap().path()).unwrap(); // Use a fresh DB instance for isolation
        last_hash = [0u8; 32];
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time_per_block_slow = TARGET_BLOCK_TIME_SECS * (MAX_DIFFICULTY_CHANGE_FACTOR as u64 * 2); // Very slow
         for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block_slow;
            }
        }
        let next_difficulty_slow = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        // Should be limited by 1 / MAX_DIFFICULTY_CHANGE_FACTOR
        let min_expected = (initial_difficulty as f64 / MAX_DIFFICULTY_CHANGE_FACTOR).round() as u32;
        assert_eq!(next_difficulty_slow, max(MIN_DIFFICULTY, min(min_expected, MAX_DIFFICULTY)));

        // Test MIN_DIFFICULTY limit
        let storage_min = StorageManager::new(tempdir().unwrap().path()).unwrap();
        let low_difficulty = MIN_DIFFICULTY;
        last_hash = [0u8; 32];
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, low_difficulty);
            last_hash = block.hash();
            storage_min.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block_slow; // Use slow time to trigger decrease
            }
        }
        let next_difficulty_min = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage_min).unwrap();
        assert_eq!(next_difficulty_min, MIN_DIFFICULTY);

        // Test MAX_DIFFICULTY limit
        let storage_max = StorageManager::new(tempdir().unwrap().path()).unwrap();
        let high_difficulty = MAX_DIFFICULTY;
        last_hash = [0u8; 32];
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, high_difficulty);
            last_hash = block.hash();
            storage_max.save_block(&block).unwrap();
             if i < ADJUSTMENT_INTERVAL_BLOCKS - 1 {
                timestamp += time_per_block; // Use fast time to trigger increase
            }
        }
        let next_difficulty_max = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage_max).unwrap();
        assert_eq!(next_difficulty_max, MAX_DIFFICULTY);
    }

    #[test]
    fn test_difficulty_adjustment_zero_actual_time() {
        // Test case where block timestamps are identical, leading to zero actual time
        let dir = tempdir().unwrap();
        let storage = StorageManager::new(dir.path()).unwrap();
        let initial_difficulty = 10;
        let mut last_hash = [0u8; 32];
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); // Same timestamp for all

        for i in 0..ADJUSTMENT_INTERVAL_BLOCKS {
            let block = create_test_block_with_details(i, last_hash, timestamp, initial_difficulty);
            last_hash = block.hash();
            storage.save_block(&block).unwrap();
        }

        // Should return a slightly increased difficulty (or initial) instead of erroring/panicking
        let next_difficulty = calculate_next_difficulty(ADJUSTMENT_INTERVAL_BLOCKS - 1, &storage).unwrap();
        assert_eq!(next_difficulty, max(MIN_DIFFICULTY, min(initial_difficulty + 1, MAX_DIFFICULTY)));
    }
}

