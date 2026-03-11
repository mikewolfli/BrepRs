//! GPU Memory Compression Utilities
//!
//! This module provides compression utilities for GPU textures and buffers
//! to reduce memory footprint and improve performance.
//! Compatible with OpenCASCADE Open API design.

use crate::visualization::texture_stream::TextureFormat;

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    /// No compression
    None,
    /// BC1 (DXT1) - 4:1 ratio, no alpha
    Bc1,
    /// BC3 (DXT5) - 4:1 ratio, alpha
    Bc3,
    /// BC4 (ATI1) - 4:1 ratio, grayscale
    Bc4,
    /// BC5 (ATI2) - 4:1 ratio, 2-channel
    Bc5,
    /// BC6 - 6:1 ratio, float
    Bc6,
    /// BC7 - 6:1 ratio, float with alpha
    Bc7,
    /// RLE (Run-Length Encoding)
    Rle,
    /// LZ4 compression
    Lz4,
    /// Zstandard compression
    Zstd,
}

impl CompressionAlgorithm {
    /// Get compression ratio (original:compressed)
    #[inline]
    pub fn compression_ratio(&self) -> f32 {
        match self {
            CompressionAlgorithm::None => 1.0,
            CompressionAlgorithm::Bc1 => 4.0,
            CompressionAlgorithm::Bc3 => 4.0,
            CompressionAlgorithm::Bc4 => 4.0,
            CompressionAlgorithm::Bc5 => 4.0,
            CompressionAlgorithm::Bc6 => 6.0,
            CompressionAlgorithm::Bc7 => 6.0,
            CompressionAlgorithm::Rle => 2.0,  // Average
            CompressionAlgorithm::Lz4 => 2.5,  // Average
            CompressionAlgorithm::Zstd => 3.0, // Average
        }
    }

    /// Check if algorithm is lossless
    #[inline]
    pub fn is_lossless(&self) -> bool {
        matches!(
            self,
            CompressionAlgorithm::None
                | CompressionAlgorithm::Rle
                | CompressionAlgorithm::Lz4
                | CompressionAlgorithm::Zstd
        )
    }

    /// Check if algorithm is GPU-native
    #[inline]
    pub fn is_gpu_native(&self) -> bool {
        matches!(
            self,
            CompressionAlgorithm::Bc1
                | CompressionAlgorithm::Bc3
                | CompressionAlgorithm::Bc4
                | CompressionAlgorithm::Bc5
                | CompressionAlgorithm::Bc6
                | CompressionAlgorithm::Bc7
        )
    }

    /// Get best compression algorithm for format
    #[inline]
    pub fn best_for_format(format: TextureFormat) -> Self {
        match format {
            TextureFormat::Rgba8 | TextureFormat::Rgb8 => CompressionAlgorithm::Bc3,
            TextureFormat::Rgba16 | TextureFormat::Rgb16 => CompressionAlgorithm::Bc3,
            TextureFormat::Rgba32 => CompressionAlgorithm::Bc7,
            TextureFormat::Rgba16Float => CompressionAlgorithm::Bc6,
            TextureFormat::Rgba32Float => CompressionAlgorithm::Bc7,
            TextureFormat::R8 => CompressionAlgorithm::Bc4,
            TextureFormat::R16 => CompressionAlgorithm::Bc5,
            TextureFormat::Bc1 => CompressionAlgorithm::Bc1,
            TextureFormat::Bc3 => CompressionAlgorithm::Bc3,
            TextureFormat::Bc4 => CompressionAlgorithm::Bc4,
            TextureFormat::Bc5 => CompressionAlgorithm::Bc5,
            TextureFormat::Bc6 => CompressionAlgorithm::Bc6,
            TextureFormat::Bc7 => CompressionAlgorithm::Bc7,
        }
    }
}

/// Compression quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionQuality {
    /// Fast compression, lower quality
    Fast,
    /// Balanced compression
    Balanced,
    /// Slow compression, higher quality
    High,
}

impl Default for CompressionQuality {
    #[inline]
    fn default() -> Self {
        CompressionQuality::Balanced
    }
}

/// Compression statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct CompressionStats {
    /// Original size in bytes
    pub original_size: u64,
    /// Compressed size in bytes
    pub compressed_size: u64,
    /// Compression ratio
    pub ratio: f32,
    /// Compression time in milliseconds
    pub compression_time_ms: f32,
    /// Decompression time in milliseconds
    pub decompression_time_ms: f32,
}

impl CompressionStats {
    /// Create new compression statistics
    #[inline]
    pub fn new(original_size: u64, compressed_size: u64) -> Self {
        let ratio = if compressed_size > 0 {
            original_size as f32 / compressed_size as f32
        } else {
            1.0
        };

        Self {
            original_size,
            compressed_size,
            ratio,
            compression_time_ms: 0.0,
            decompression_time_ms: 0.0,
        }
    }

    /// Calculate memory savings in bytes
    #[inline]
    pub fn memory_savings(&self) -> u64 {
        self.original_size.saturating_sub(self.compressed_size)
    }

    /// Calculate memory savings percentage
    #[inline]
    pub fn savings_percentage(&self) -> f32 {
        if self.original_size > 0 {
            (self.memory_savings() as f32 / self.original_size as f32) * 100.0
        } else {
            0.0
        }
    }
}

/// GPU memory compressor
pub struct GpuMemoryCompressor {
    algorithm: CompressionAlgorithm,
    #[allow(dead_code)]
    quality: CompressionQuality,
}

impl GpuMemoryCompressor {
    /// Create a new compressor with specified algorithm
    #[inline]
    pub fn new(algorithm: CompressionAlgorithm, quality: CompressionQuality) -> Self {
        Self { algorithm, quality }
    }

    /// Create compressor for texture format
    #[inline]
    pub fn for_format(format: TextureFormat, quality: CompressionQuality) -> Self {
        Self::new(CompressionAlgorithm::best_for_format(format), quality)
    }

    /// Compress data
    #[inline]
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::Bc1 => self.compress_bc1(data),
            CompressionAlgorithm::Bc3 => self.compress_bc3(data),
            CompressionAlgorithm::Bc4 => self.compress_bc4(data),
            CompressionAlgorithm::Bc5 => self.compress_bc5(data),
            CompressionAlgorithm::Bc6 => self.compress_bc6(data),
            CompressionAlgorithm::Bc7 => self.compress_bc7(data),
            CompressionAlgorithm::Rle => self.compress_rle(data),
            CompressionAlgorithm::Lz4 => self.compress_lz4(data),
            CompressionAlgorithm::Zstd => self.compress_zstd(data),
        }
    }

    /// Decompress data
    #[inline]
    pub fn decompress(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(compressed.to_vec()),
            CompressionAlgorithm::Bc1 => self.decompress_bc1(compressed, original_size),
            CompressionAlgorithm::Bc3 => self.decompress_bc3(compressed, original_size),
            CompressionAlgorithm::Bc4 => self.decompress_bc4(compressed, original_size),
            CompressionAlgorithm::Bc5 => self.decompress_bc5(compressed, original_size),
            CompressionAlgorithm::Bc6 => self.decompress_bc6(compressed, original_size),
            CompressionAlgorithm::Bc7 => self.decompress_bc7(compressed, original_size),
            CompressionAlgorithm::Rle => self.decompress_rle(compressed, original_size),
            CompressionAlgorithm::Lz4 => self.decompress_lz4(compressed, original_size),
            CompressionAlgorithm::Zstd => self.decompress_zstd(compressed, original_size),
        }
    }

    /// Compress with statistics
    #[inline]
    pub fn compress_with_stats(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionStats), String> {
        let start = std::time::Instant::now();
        let compressed = self.compress(data)?;
        let compression_time = start.elapsed().as_secs_f64() * 1000.0;

        let mut stats = CompressionStats::new(data.len() as u64, compressed.len() as u64);
        stats.compression_time_ms = compression_time as f32;

        Ok((compressed, stats))
    }

    /// Decompress with statistics
    #[inline]
    pub fn decompress_with_stats(
        &self,
        compressed: &[u8],
        original_size: usize,
    ) -> Result<(Vec<u8>, CompressionStats), String> {
        let start = std::time::Instant::now();
        let decompressed = self.decompress(compressed, original_size)?;
        let decompression_time = start.elapsed().as_secs_f64() * 1000.0;

        let mut stats = CompressionStats::new(original_size as u64, compressed.len() as u64);
        stats.decompression_time_ms = decompression_time as f32;

        Ok((decompressed, stats))
    }

    // BC1 compression (DXT1) - 4x4 blocks, 8 bytes per block
    #[inline]
    fn compress_bc1(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC1 compresses 4x4 RGBA blocks to 8 bytes
        // This is a simplified implementation
        // Real implementation would use proper BC1 encoding

        let block_size = 16; // 4x4 pixels * 4 bytes
        let compressed_block_size = 8;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            // Simplified: just copy first 8 bytes
            // Real implementation would encode properly
            compressed.extend_from_slice(&block[..compressed_block_size]);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc1(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        // Simplified decompression
        const BLOCK_SIZE: usize = 16;
        const COMPRESSED_BLOCK_SIZE: usize = 8;

        if compressed.len() % COMPRESSED_BLOCK_SIZE != 0 {
            return Err("Compressed data size invalid".to_string());
        }

        let mut decompressed = Vec::with_capacity(original_size);

        for block in compressed.chunks_exact(COMPRESSED_BLOCK_SIZE) {
            // Simplified: expand to 16 bytes
            let mut expanded = vec![0u8; BLOCK_SIZE];
            expanded[..COMPRESSED_BLOCK_SIZE].copy_from_slice(block);
            decompressed.extend_from_slice(&expanded);
        }

        Ok(decompressed)
    }

    // BC3 compression (DXT5) - 4x4 blocks, 16 bytes per block
    #[inline]
    fn compress_bc3(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC3 compresses 4x4 RGBA blocks to 16 bytes
        let block_size = 16;
        let compressed_block_size = 16;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            // Simplified: copy all 16 bytes
            compressed.extend_from_slice(block);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc3(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        let _ = original_size;
        Ok(compressed.to_vec())
    }

    // BC4 compression (ATI1) - 4x4 blocks, 8 bytes per block
    #[inline]
    fn compress_bc4(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC4 compresses 4x4 grayscale blocks to 8 bytes
        let block_size = 16;
        let compressed_block_size = 8;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            // Simplified: copy first 8 bytes
            compressed.extend_from_slice(&block[..compressed_block_size]);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc4(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        const BLOCK_SIZE: usize = 16;
        const COMPRESSED_BLOCK_SIZE: usize = 8;

        let mut decompressed = Vec::with_capacity(original_size);

        for block in compressed.chunks_exact(COMPRESSED_BLOCK_SIZE) {
            let mut expanded = vec![0u8; BLOCK_SIZE];
            expanded[..COMPRESSED_BLOCK_SIZE].copy_from_slice(block);
            decompressed.extend_from_slice(&expanded);
        }

        Ok(decompressed)
    }

    // BC5 compression (ATI2) - 4x4 blocks, 16 bytes per block
    #[inline]
    fn compress_bc5(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC5 compresses 4x4 2-channel blocks to 16 bytes
        let block_size = 16;
        let compressed_block_size = 16;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            compressed.extend_from_slice(block);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc5(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        let _ = original_size;
        Ok(compressed.to_vec())
    }

    // BC6 compression - 4x4 blocks, 16 bytes per block (float)
    #[inline]
    fn compress_bc6(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC6 compresses 4x4 float blocks to 16 bytes
        let block_size = 16;
        let compressed_block_size = 16;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            compressed.extend_from_slice(block);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc6(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        let _ = original_size;
        Ok(compressed.to_vec())
    }

    // BC7 compression - 4x4 blocks, 16 bytes per block (float with alpha)
    #[inline]
    fn compress_bc7(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // BC7 compresses 4x4 float blocks to 16 bytes
        let block_size = 16;
        let compressed_block_size = 16;

        if data.len() % block_size != 0 {
            return Err("Data size must be multiple of 16 bytes".to_string());
        }

        let num_blocks = data.len() / block_size;
        let mut compressed = Vec::with_capacity(num_blocks * compressed_block_size);

        for block in data.chunks_exact(block_size) {
            compressed.extend_from_slice(block);
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_bc7(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        let _ = original_size;
        Ok(compressed.to_vec())
    }

    // RLE compression
    #[inline]
    fn compress_rle(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut compressed = Vec::new();
        let mut i = 0;
        let data_len = data.len();

        while i < data_len {
            let byte = data[i];
            let mut count = 1u8;

            loop {
                let count_usize = count as usize;
                if i + count_usize >= data_len || count >= 255 {
                    break;
                }
                if data[i + count_usize] != byte {
                    break;
                }
                count = count.saturating_add(1);
            }

            compressed.push(count);
            compressed.push(byte);
            i += count as usize;
        }

        Ok(compressed)
    }

    #[inline]
    fn decompress_rle(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        let mut decompressed = Vec::with_capacity(original_size);
        let mut i = 0;

        while i + 1 < compressed.len() {
            let count = compressed[i] as usize;
            let byte = compressed[i + 1];

            for _ in 0..count {
                decompressed.push(byte);
            }

            i += 2;
        }

        Ok(decompressed)
    }

    // LZ4 compression (placeholder)
    #[inline]
    fn compress_lz4(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Real LZ4 compression
        use lz4::block::compress;
        compress(data, None, false).map_err(|e| e.to_string())
    }

    #[inline]
    fn decompress_lz4(&self, compressed: &[u8], original_size: usize) -> Result<Vec<u8>, String> {
        // Real LZ4 decompression
        use lz4::block::decompress;
        decompress(compressed, Some(original_size.try_into().unwrap())).map_err(|e| e.to_string())
    }

    // Zstandard compression (placeholder)
    #[inline]
    fn compress_zstd(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Real Zstd compression
        use std::io::Cursor;
        use zstd::stream::encode_all;
        encode_all(Cursor::new(data), 3).map_err(|e| e.to_string())
    }

    #[inline]
    fn decompress_zstd(&self, compressed: &[u8], _original_size: usize) -> Result<Vec<u8>, String> {
        // Real Zstd decompression
        use std::io::Cursor;
        use zstd::stream::decode_all;
        decode_all(Cursor::new(compressed)).map_err(|e| e.to_string())
    }
}

impl Default for GpuMemoryCompressor {
    #[inline]
    fn default() -> Self {
        Self::new(CompressionAlgorithm::Bc3, CompressionQuality::Balanced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_algorithm() {
        assert_eq!(CompressionAlgorithm::Bc1.compression_ratio(), 4.0);
        assert_eq!(CompressionAlgorithm::Bc3.compression_ratio(), 4.0);
        assert_eq!(CompressionAlgorithm::Bc6.compression_ratio(), 6.0);
        assert_eq!(CompressionAlgorithm::Bc7.compression_ratio(), 6.0);

        assert!(CompressionAlgorithm::Bc1.is_gpu_native());
        assert!(CompressionAlgorithm::Rle.is_lossless());
        assert!(!CompressionAlgorithm::Bc3.is_lossless());
    }

    #[test]
    fn test_compressor_creation() {
        let compressor =
            GpuMemoryCompressor::new(CompressionAlgorithm::Bc3, CompressionQuality::Balanced);

        assert_eq!(compressor.algorithm, CompressionAlgorithm::Bc3);
        assert_eq!(compressor.quality, CompressionQuality::Balanced);
    }

    #[test]
    fn test_compressor_for_format() {
        let compressor =
            GpuMemoryCompressor::for_format(TextureFormat::Rgba8, CompressionQuality::Balanced);

        assert_eq!(compressor.algorithm, CompressionAlgorithm::Bc3);
    }

    #[test]
    fn test_rle_compression() {
        let compressor =
            GpuMemoryCompressor::new(CompressionAlgorithm::Rle, CompressionQuality::Balanced);

        let data = vec![1u8, 1, 1, 2, 2, 2, 2, 3];
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();

        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_bc1_compression() {
        let compressor =
            GpuMemoryCompressor::new(CompressionAlgorithm::Bc1, CompressionQuality::Balanced);

        // Create 4x4 block (16 bytes)
        let data = vec![255u8; 16];
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();

        assert_eq!(compressed.len(), 8); // 8 bytes for BC1 block
        assert_eq!(decompressed.len(), 16);
    }

    #[test]
    fn test_compression_stats() {
        let stats = CompressionStats::new(1000, 250);

        assert_eq!(stats.original_size, 1000);
        assert_eq!(stats.compressed_size, 250);
        assert_eq!(stats.ratio, 4.0);
        assert_eq!(stats.memory_savings(), 750);
        assert_eq!(stats.savings_percentage(), 75.0);
    }

    #[test]
    fn test_compress_with_stats() {
        let compressor =
            GpuMemoryCompressor::new(CompressionAlgorithm::Rle, CompressionQuality::Balanced);

        let data = vec![1u8, 1, 1, 2, 2, 2, 2, 3];
        let (compressed, stats) = compressor.compress_with_stats(&data).unwrap();

        assert_eq!(stats.original_size, data.len() as u64);
        assert_eq!(stats.compressed_size, compressed.len() as u64);
        assert!(stats.compression_time_ms >= 0.0);
    }
}
