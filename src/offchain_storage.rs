// src/offchain_storage.rs

use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex;
use log::{info, error, debug};

// Custom error type for OffChain Storage operations
#[derive(Debug, thiserror::Error)]
pub enum OffChainStorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid hash format: {0}")]
    InvalidHashFormat(String),
    #[error("Payload not found for hash: {0}")]
    NotFound(String),
    #[error("Failed to create storage directory: {0}")]
    DirectoryCreationFailed(String),
}

/// Manages the storage and retrieval of large data payloads off-chain.
#[derive(Debug)]
pub struct OffChainStorageManager {
    storage_path: PathBuf,
}

impl OffChainStorageManager {
    /// Creates a new OffChainStorageManager instance.
    ///
    /// Creates the storage directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `base_data_dir` - The main data directory of the node.
    ///   The off-chain storage will be placed in a subdirectory named "offchain_storage".
    pub fn new(base_data_dir: &Path) -> Result<Self, OffChainStorageError> {
        let storage_path = base_data_dir.join("offchain_storage");
        info!("Initializing off-chain storage at: {:?}", storage_path);

        if !storage_path.exists() {
            debug!("Creating off-chain storage directory: {:?}", storage_path);
            fs::create_dir_all(&storage_path).map_err(|e| {
                error!("Failed to create off-chain storage directory {:?}: {}", storage_path, e);
                OffChainStorageError::DirectoryCreationFailed(e.to_string())
            })?;
        }

        Ok(OffChainStorageManager { storage_path })
    }

    /// Stores a data payload off-chain.
    ///
    /// Calculates the SHA-256 hash of the payload, uses the hex representation
    /// of the hash as the filename, and saves the payload to the storage directory.
    /// If a file with the same hash already exists, it's assumed the content is identical
    /// and the operation succeeds without rewriting.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice representing the data to store.
    ///
    /// # Returns
    ///
    /// * `Ok([u8; 32])` - The SHA-256 hash of the stored payload.
    /// * `Err(OffChainStorageError)` - If an I/O error occurs during hashing or saving.
    pub fn store_payload(&self, payload: &[u8]) -> Result<[u8; 32], OffChainStorageError> {
        // 1. Calculate hash
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let hash_result = hasher.finalize();
        let hash_array: [u8; 32] = hash_result.into();
        let hash_hex = hex::encode(hash_array);

        // 2. Determine file path
        let file_path = self.storage_path.join(&hash_hex);
        debug!("Storing payload with hash {} at {:?}", hash_hex, file_path);

        // 3. Check if file already exists (optimisation)
        if file_path.exists() {
            info!("Payload with hash {} already exists. Skipping write.", hash_hex);
            return Ok(hash_array);
        }

        // 4. Write payload to file
        let mut file = File::create(&file_path)?;
        file.write_all(payload)?;
        debug!("Successfully wrote {} bytes to {:?}", payload.len(), file_path);

        Ok(hash_array)
    }

    /// Retrieves a data payload from off-chain storage using its hash.
    ///
    /// # Arguments
    ///
    /// * `payload_hash` - The SHA-256 hash (byte array) of the payload to retrieve.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The retrieved payload data.
    /// * `Err(OffChainStorageError::NotFound)` - If no payload exists for the given hash.
    /// * `Err(OffChainStorageError::Io)` - If an I/O error occurs during reading.
    pub fn retrieve_payload(&self, payload_hash: &[u8; 32]) -> Result<Vec<u8>, OffChainStorageError> {
        let hash_hex = hex::encode(payload_hash);
        let file_path = self.storage_path.join(&hash_hex);
        debug!("Retrieving payload with hash {} from {:?}", hash_hex, file_path);

        if !file_path.exists() {
            warn!("Payload file not found for hash {}", hash_hex);
            return Err(OffChainStorageError::NotFound(hash_hex));
        }

        let mut file = File::open(&file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        debug!("Successfully read {} bytes for hash {}", buffer.len(), hash_hex);

        // Optional: Verify hash of retrieved data matches requested hash?
        // let mut hasher = Sha256::new();
        // hasher.update(&buffer);
        // let retrieved_hash: [u8; 32] = hasher.finalize().into();
        // if &retrieved_hash != payload_hash {
        //     error!("Hash mismatch for retrieved payload {}! Data corruption?", hash_hex);
        //     // Decide how to handle corruption - maybe delete the file?
        //     return Err(OffChainStorageError::InvalidHashFormat("Retrieved data hash mismatch".to_string()));
        // }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module
    use tempfile::tempdir;

    #[test]
    fn test_offchain_storage_new() {
        let base_dir = tempdir().unwrap();
        let storage_dir = base_dir.path().join("offchain_storage");
        assert!(!storage_dir.exists());

        let manager = OffChainStorageManager::new(base_dir.path());
        assert!(manager.is_ok());
        assert!(storage_dir.exists());
        assert!(storage_dir.is_dir());
    }

    #[test]
    fn test_offchain_store_and_retrieve() {
        let base_dir = tempdir().unwrap();
        let manager = OffChainStorageManager::new(base_dir.path()).unwrap();

        let payload1 = b"hello offchain world";
        let payload2 = b"another payload data";

        // Store first payload
        let hash1_result = manager.store_payload(payload1);
        assert!(hash1_result.is_ok());
        let hash1 = hash1_result.unwrap();

        // Verify file exists
        let hash1_hex = hex::encode(hash1);
        let file1_path = base_dir.path().join("offchain_storage").join(&hash1_hex);
        assert!(file1_path.exists());

        // Retrieve first payload
        let retrieved1_result = manager.retrieve_payload(&hash1);
        assert!(retrieved1_result.is_ok());
        assert_eq!(retrieved1_result.unwrap(), payload1);

        // Store second payload
        let hash2_result = manager.store_payload(payload2);
        assert!(hash2_result.is_ok());
        let hash2 = hash2_result.unwrap();
        assert_ne!(hash1, hash2);

        // Retrieve second payload
        let retrieved2_result = manager.retrieve_payload(&hash2);
        assert!(retrieved2_result.is_ok());
        assert_eq!(retrieved2_result.unwrap(), payload2);

        // Retrieve first payload again
        let retrieved1_again_result = manager.retrieve_payload(&hash1);
        assert!(retrieved1_again_result.is_ok());
        assert_eq!(retrieved1_again_result.unwrap(), payload1);
    }

    #[test]
    fn test_offchain_store_duplicate() {
        let base_dir = tempdir().unwrap();
        let manager = OffChainStorageManager::new(base_dir.path()).unwrap();
        let payload = b"duplicate data";

        // Store first time
        let hash1 = manager.store_payload(payload).unwrap();
        let file_path = base_dir.path().join("offchain_storage").join(hex::encode(hash1));
        let metadata1 = fs::metadata(&file_path).unwrap();
        let modified1 = metadata1.modified().unwrap();

        // Wait a bit to ensure modification time changes if rewritten
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Store second time
        let hash2 = manager.store_payload(payload).unwrap();
        let metadata2 = fs::metadata(&file_path).unwrap();
        let modified2 = metadata2.modified().unwrap();

        // Hashes should be the same, modification time should NOT change
        assert_eq!(hash1, hash2);
        assert_eq!(modified1, modified2, "File was modified on duplicate store");

        // Retrieve should still work
        assert_eq!(manager.retrieve_payload(&hash1).unwrap(), payload);
    }

    #[test]
    fn test_offchain_retrieve_not_found() {
        let base_dir = tempdir().unwrap();
        let manager = OffChainStorageManager::new(base_dir.path()).unwrap();
        let non_existent_hash = [99u8; 32];

        let result = manager.retrieve_payload(&non_existent_hash);
        assert!(result.is_err());
        match result.err().unwrap() {
            OffChainStorageError::NotFound(hash_hex) => {
                assert_eq!(hash_hex, hex::encode(non_existent_hash));
            }
            _ => panic!("Expected NotFound error"),
        }
    }
}

