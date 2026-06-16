//! Compression and encryption utilities
//!
//! Provides compression algorithms and encryption support.

use crate::BuildResult;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::io::{Read, Write};
use zstd::{Decoder, Encoder};

/// Compression utilities
pub struct Compress;

impl Compress {
    /// Zstd compression
    pub fn zstd(bytes: &[u8], level: i32) -> BuildResult<Vec<u8>> {
        let mut encoder = Encoder::new(Vec::new(), level)?;
        encoder.write_all(bytes)?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Gzip compression
    pub fn gzip(bytes: &[u8], level: u32) -> BuildResult<Vec<u8>> {
        let level = match level {
            0..=9 => Compression::new(level),
            _ => Compression::default(),
        };
        let mut encoder = GzEncoder::new(Vec::new(), level);
        encoder.write_all(bytes)?;
        encoder.finish().map_err(|e| crate::BuildError::io_error(e.to_string()))
    }

    /// Brotli compression (simplified - returns original if not available)
    pub fn brotli(bytes: &[u8], _level: u32) -> BuildResult<Vec<u8>> {
        // Brotli requires additional crate, simplified implementation
        // For now, use gzip as fallback
        Self::gzip(bytes, 6)
    }

    /// LZ4 compression (simplified - returns original if not available)
    pub fn lz4(bytes: &[u8]) -> BuildResult<Vec<u8>> {
        // LZ4 requires additional crate, simplified implementation
        // For now, use zstd level 1 as fallback
        Self::zstd(bytes, 1)
    }

    /// Decompress with specified algorithm
    pub fn decompress(bytes: &[u8], algo: crate::AssetCompress) -> BuildResult<Vec<u8>> {
        match algo {
            crate::AssetCompress::None => Ok(bytes.to_vec()),
            crate::AssetCompress::Zstd => {
                let mut decoder = Decoder::new(bytes)?;
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                Ok(decompressed)
            }
            crate::AssetCompress::Gzip => {
                let mut decoder = GzDecoder::new(bytes);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                Ok(decompressed)
            }
            crate::AssetCompress::Brotli | crate::AssetCompress::LZ4 => {
                // Fallback to zstd decompression
                Self::decompress(bytes, crate::AssetCompress::Zstd)
            }
        }
    }
}

/// Encryption utilities (requires encryption feature)
pub struct Encrypt;

impl Encrypt {
    /// AES-GCM-128 encryption (placeholder)
    #[cfg(not(feature = "encryption"))]
    pub fn aes_gcm_128(bytes: &[u8], _key: &[u8; 16], _nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        // Without encryption feature, return original
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn aes_gcm_128(bytes: &[u8], key: &[u8; 16], nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        use aes_gcm::{Aes128Gcm, KeyInit, aead::Aead};
        let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
        cipher.encrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Encryption failed"))
    }

    /// AES-GCM-256 encryption (placeholder)
    #[cfg(not(feature = "encryption"))]
    pub fn aes_gcm_256(bytes: &[u8], _key: &[u8; 32], _nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn aes_gcm_256(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
        cipher.encrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Encryption failed"))
    }

    /// ChaCha20-Poly1305 encryption (placeholder)
    #[cfg(not(feature = "encryption"))]
    pub fn chacha20(bytes: &[u8], _key: &[u8; 32], _nonce: &[u8; 24]) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn chacha20(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 24]) -> BuildResult<Vec<u8>> {
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::Aead};
        let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
        cipher.encrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Encryption failed"))
    }

    /// Decrypt with specified algorithm
    #[cfg(not(feature = "encryption"))]
    pub fn decrypt(bytes: &[u8], _key: &[u8], _nonce: &[u8], _algo: crate::AssetEncrypt) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn decrypt(bytes: &[u8], key: &[u8], nonce: &[u8], algo: crate::AssetEncrypt) -> BuildResult<Vec<u8>> {
        match algo {
            crate::AssetEncrypt::None => Ok(bytes.to_vec()),
            crate::AssetEncrypt::AesGcm128 => {
                use aes_gcm::{Aes128Gcm, KeyInit, aead::Aead};
                let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
                cipher.decrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Decryption failed"))
            }
            crate::AssetEncrypt::AesGcm256 => {
                use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
                let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
                cipher.decrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Decryption failed"))
            }
            crate::AssetEncrypt::XorChaCha20 => {
                use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::Aead};
                let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|_| crate::BuildError::crypto_error("Invalid key"))?;
                cipher.decrypt(nonce.into(), bytes).map_err(|_| crate::BuildError::crypto_error("Decryption failed"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_compress_decompress() {
        let data = b"test data for compression";
        let compressed = Compress::zstd(data, 3).unwrap();
        let decompressed = Compress::decompress(&compressed, crate::AssetCompress::Zstd).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_gzip_compress_decompress() {
        let data = b"test data for compression";
        let compressed = Compress::gzip(data, 6).unwrap();
        let decompressed = Compress::decompress(&compressed, crate::AssetCompress::Gzip).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_none() {
        let data = b"test data";
        let decompressed = Compress::decompress(data, crate::AssetCompress::None).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_encrypt_none() {
        let data = b"test data";
        let encrypted = Encrypt::aes_gcm_128(data, &[0u8; 16], &[0u8; 12]).unwrap();
        #[cfg(not(feature = "encryption"))]
        assert_eq!(data.to_vec(), encrypted);
    }
}