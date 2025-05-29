use crate::core::{BlockHeader, Hash};
use sha2::{Sha256, Digest};
use hex;

// TODO: Define a more sophisticated difficulty representation (e.g., target value)
// For now, difficulty represents the number of leading zero bits required in the hash.

/// Verifies if a given hash meets the required difficulty target.
///
/// # Arguments
///
/// * `hash` - The hash to check.
/// * `difficulty` - The number of leading zero bits required.
///
/// # Returns
///
/// * `true` if the hash meets the difficulty target, `false` otherwise.
pub fn verify_pow(hash: &Hash, difficulty: u32) -> bool {
    if difficulty == 0 {
        return true; // No difficulty requirement
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
            // This case should ideally not happen if hash length is sufficient (e.g., 32 bytes)
            // but if the hash is shorter than required bytes+1, it fails.
            return false;
        }
    }

    true
}

/// Performs the Proof-of-Work mining process.
/// Finds a nonce such that the block header's hash meets the difficulty target.
///
/// # Arguments
///
/// * `header` - A mutable reference to the block header. The nonce will be updated.
/// * `difficulty` - The required difficulty (number of leading zero bits).
///
/// # Returns
///
/// * The calculated hash that meets the difficulty target.
pub fn mine(header: &mut BlockHeader, difficulty: u32) -> Hash {
    println!(
        "Mining block {} with difficulty {}... Target: {} leading zero bits",
        header.height,
        difficulty,
        difficulty
    );
    let start_time = std::time::Instant::now();
    loop {
        let hash = header.calculate_hash();
        if verify_pow(&hash, difficulty) {
            let duration = start_time.elapsed();
            println!(
                "Block {} mined successfully! Nonce: {}, Hash: {}, Time: {:?}",
                header.height,
                header.nonce,
                hex::encode(hash),
                duration
            );
            return hash;
        }
        // Increment nonce and try again
        header.nonce = header.nonce.checked_add(1).expect("Nonce overflow");
        // Optional: Update timestamp periodically during long mining?
        // header.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
}

// TODO: Implement difficulty adjustment algorithm
// pub fn adjust_difficulty(last_block_time: u64, current_time: u64, current_difficulty: u32) -> u32 {
//     // Placeholder logic
//     current_difficulty
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::BlockHeader;

    #[test]
    fn test_verify_pow_simple() {
        // Hash with 1 leading zero byte (8 zero bits)
        let hash1 = [0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash1, 8));
        assert!(!verify_pow(&hash1, 9));

        // Hash with 12 leading zero bits (0x0F...)
        let hash2 = [0x0F, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash2, 4)); // 0000....
        assert!(!verify_pow(&hash2, 5)); // 00001...

        // Hash with 16 leading zero bits
        let hash3 = [0x00, 0x00, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash3, 16));
        assert!(!verify_pow(&hash3, 17));

         // Hash with 17 leading zero bits (0x00, 0x00, 0x7F...)
        let hash4 = [0x00, 0x00, 0x7F, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        assert!(verify_pow(&hash4, 17)); // 00000000 00000000 0.......
        assert!(!verify_pow(&hash4, 18)); // 00000000 00000000 01......
    }

    #[test]
    fn test_mine_simple() {
        // Use a low difficulty for quick testing
        let difficulty = 8; // Require 1 leading zero byte
        let mut header = BlockHeader {
            previous_hash: [0u8; 32],
            merkle_root: [1u8; 32],
            timestamp: 1234567890,
            nonce: 0,
            difficulty, // Store difficulty in header? Or pass separately?
            height: 1,
        };

        let final_hash = mine(&mut header, difficulty);
        println!("Mining finished. Final Nonce: {}, Final Hash: {}", header.nonce, hex::encode(final_hash));

        // Verify the found hash meets the difficulty
        assert!(verify_pow(&final_hash, difficulty));

        // Verify that recalculating the hash with the found nonce yields the same result
        assert_eq!(header.calculate_hash(), final_hash);
    }

     #[test]
     #[ignore] // This test can take a while depending on the difficulty
     fn test_mine_higher_difficulty() {
         // Use a slightly higher difficulty
         let difficulty = 16; // Require 2 leading zero bytes
         let mut header = BlockHeader {
             previous_hash: [10u8; 32],
             merkle_root: [11u8; 32],
             timestamp: 1234567899,
             nonce: 0,
             difficulty,
             height: 2,
         };

         let final_hash = mine(&mut header, difficulty);
         println!("Mining (16 bits) finished. Final Nonce: {}, Final Hash: {}", header.nonce, hex::encode(final_hash));
         assert!(verify_pow(&final_hash, difficulty));
         assert_eq!(header.calculate_hash(), final_hash);
     }
}

