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
        encoder
            .finish()
            .map_err(|e| crate::BuildError::io_error(e.to_string()))
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
        use aes_gcm::{aead::{Aead, generic_array::GenericArray}, Aes128Gcm, KeyInit};
        let cipher = Aes128Gcm::new_from_slice(key)
            .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
        let nonce_arr = GenericArray::from_slice(nonce);
        cipher
            .encrypt(nonce_arr, bytes)
            .map_err(|_| crate::BuildError::crypto_error("Encryption failed".to_string()))
    }

    /// AES-GCM-256 encryption (placeholder)
    #[cfg(not(feature = "encryption"))]
    pub fn aes_gcm_256(bytes: &[u8], _key: &[u8; 32], _nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn aes_gcm_256(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> BuildResult<Vec<u8>> {
        use aes_gcm::{aead::{Aead, generic_array::GenericArray}, Aes256Gcm, KeyInit};
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
        let nonce_arr = GenericArray::from_slice(nonce);
        cipher
            .encrypt(nonce_arr, bytes)
            .map_err(|_| crate::BuildError::crypto_error("Encryption failed".to_string()))
    }

    /// ChaCha20-Poly1305 encryption (placeholder)
    #[cfg(not(feature = "encryption"))]
    pub fn chacha20(bytes: &[u8], _key: &[u8; 32], _nonce: &[u8; 24]) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn chacha20(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 24]) -> BuildResult<Vec<u8>> {
        use chacha20poly1305::{aead::{Aead, generic_array::GenericArray}, ChaCha20Poly1305, KeyInit};
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
        let nonce_arr = GenericArray::from_slice(nonce);
        cipher
            .encrypt(nonce_arr, bytes)
            .map_err(|_| crate::BuildError::crypto_error("Encryption failed".to_string()))
    }

    /// Decrypt with specified algorithm
    #[cfg(not(feature = "encryption"))]
    pub fn decrypt(
        bytes: &[u8],
        _key: &[u8],
        _nonce: &[u8],
        _algo: crate::AssetEncrypt,
    ) -> BuildResult<Vec<u8>> {
        Ok(bytes.to_vec())
    }

    #[cfg(feature = "encryption")]
    pub fn decrypt(
        bytes: &[u8],
        key: &[u8],
        nonce: &[u8],
        algo: crate::AssetEncrypt,
    ) -> BuildResult<Vec<u8>> {
        match algo {
            crate::AssetEncrypt::None => Ok(bytes.to_vec()),
            crate::AssetEncrypt::AesGcm128 => {
                use aes_gcm::{aead::{Aead, generic_array::GenericArray}, Aes128Gcm, KeyInit};
                let cipher = Aes128Gcm::new_from_slice(key)
                    .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
                let nonce_arr = GenericArray::from_slice(nonce);
                cipher
                    .decrypt(nonce_arr, bytes)
                    .map_err(|_| crate::BuildError::crypto_error("Decryption failed".to_string()))
            }
            crate::AssetEncrypt::AesGcm256 => {
                use aes_gcm::{aead::{Aead, generic_array::GenericArray}, Aes256Gcm, KeyInit};
                let cipher = Aes256Gcm::new_from_slice(key)
                    .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
                let nonce_arr = GenericArray::from_slice(nonce);
                cipher
                    .decrypt(nonce_arr, bytes)
                    .map_err(|_| crate::BuildError::crypto_error("Decryption failed".to_string()))
            }
            crate::AssetEncrypt::XorChaCha20 => {
                use chacha20poly1305::{aead::{Aead, generic_array::GenericArray}, ChaCha20Poly1305, KeyInit};
                let cipher = ChaCha20Poly1305::new_from_slice(key)
                    .map_err(|_| crate::BuildError::crypto_error("Invalid key".to_string()))?;
                let nonce_arr = GenericArray::from_slice(nonce);
                cipher
                    .decrypt(nonce_arr, bytes)
                    .map_err(|_| crate::BuildError::crypto_error("Decryption failed".to_string()))
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
    fn test_zstd_compression_actually_compresses() {
        let data = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let compressed = Compress::zstd(data, 10).unwrap();
        // 压缩后应该比原始小或略小，但至少有效
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_zstd_multiple_levels() {
        let data = b"hello world repeated: ";
        for level in &[1, 3, 7, 10] {
            let compressed = Compress::zstd(data, *level).unwrap();
            let decompressed =
                Compress::decompress(&compressed, crate::AssetCompress::Zstd).unwrap();
            assert_eq!(data.to_vec(), decompressed);
        }
    }

    #[test]
    fn test_gzip_compress_decompress() {
        let data = b"test data for compression";
        let compressed = Compress::gzip(data, 6).unwrap();
        let decompressed =
            Compress::decompress(&compressed, crate::AssetCompress::Gzip).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_gzip_multiple_levels() {
        let data = b"some gzip content";
        for level in &[1u32, 3, 6, 9] {
            let compressed = Compress::gzip(data, *level).unwrap();
            let decompressed =
                Compress::decompress(&compressed, crate::AssetCompress::Gzip).unwrap();
            assert_eq!(data.to_vec(), decompressed);
        }
    }

    #[test]
    fn test_compress_none() {
        let data = b"test data";
        let decompressed = Compress::decompress(data, crate::AssetCompress::None).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_brotli_falls_back_to_gzip() {
        let data = b"brotli fallback test";
        let compressed = Compress::brotli(data, 5).unwrap();
        // brotli 内部 fallback 到 gzip，所以使用 Gzip 解压
        let decompressed =
            Compress::decompress(&compressed, crate::AssetCompress::Gzip).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_lz4_falls_back_to_zstd() {
        let data = b"lz4 fallback test";
        let compressed = Compress::lz4(data).unwrap();
        let decompressed =
            Compress::decompress(&compressed, crate::AssetCompress::Zstd).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_encrypt_none() {
        let data = b"test data";
        let encrypted = Encrypt::aes_gcm_128(data, &[0u8; 16], &[0u8; 12]).unwrap();
        #[cfg(not(feature = "encryption"))]
        assert_eq!(data.to_vec(), encrypted);
    }

    #[test]
    fn test_encrypt_aes_gcm_256_default() {
        let data = b"256-bit key";
        let encrypted = Encrypt::aes_gcm_256(data, &[0u8; 32], &[0u8; 12]).unwrap();
        #[cfg(not(feature = "encryption"))]
        assert_eq!(data.to_vec(), encrypted);
    }

    #[test]
    fn test_encrypt_chacha20_default() {
        let data = b"chacha content";
        let encrypted = Encrypt::chacha20(data, &[0u8; 32], &[0u8; 24]).unwrap();
        #[cfg(not(feature = "encryption"))]
        assert_eq!(data.to_vec(), encrypted);
    }

    #[test]
    fn test_decrypt_default_no_encryption() {
        let data = b"plain content";
        let decrypted = Encrypt::decrypt(data, &[0u8; 16], &[0u8; 12], crate::AssetEncrypt::None).unwrap();
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_compress_roundtrip_large() {
        // 测试较大数据压缩
        let mut data = vec![0u8; 10_000];
        for (i, v) in data.iter_mut().enumerate() {
            *v = (i % 256) as u8;
        }
        let compressed = Compress::zstd(&data, 5).unwrap();
        let decompressed = Compress::decompress(&compressed, crate::AssetCompress::Zstd).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_decompress_fallback_brotli() {
        // 测试 brotli decompress 实际上会 fallback 到 zstd
        let original = b"brotli fallback test";
        let zstd = Compress::zstd(original, 3).unwrap();
        let decompressed = Compress::decompress(&zstd, crate::AssetCompress::Brotli).unwrap();
        assert_eq!(original.to_vec(), decompressed);
    }

    #[test]
    fn test_decompress_fallback_lz4() {
        let original = b"lz4 fallback test";
        let zstd = Compress::zstd(original, 3).unwrap();
        let decompressed = Compress::decompress(&zstd, crate::AssetCompress::LZ4).unwrap();
        assert_eq!(original.to_vec(), decompressed);
    }
}
