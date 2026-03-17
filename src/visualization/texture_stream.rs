//! Texture Streaming System
//!
//! This module provides LOD-based texture management for efficient
//! GPU memory usage with automatic streaming and caching.
//! Compatible with OpenCASCADE Open API design.

use crate::visualization::gpu_memory::{GpuBufferInfo, GpuBufferType, GpuMemoryManager};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFormat {
    /// 8-bit RGBA
    Rgba8,
    /// 8-bit RGB
    Rgb8,
    /// 16-bit RGBA
    Rgba16,
    /// 16-bit RGB
    Rgb16,
    /// 32-bit RGBA
    Rgba32,
    /// 16-bit float RGBA
    Rgba16Float,
    /// 32-bit float RGBA
    Rgba32Float,
    /// 8-bit grayscale
    R8,
    /// 16-bit grayscale
    R16,
    /// BC1 compressed (DXT1)
    Bc1,
    /// BC3 compressed (DXT5)
    Bc3,
    /// BC4 compressed (ATI1)
    Bc4,
    /// BC5 compressed (ATI2)
    Bc5,
    /// BC6 compressed (BPTC float)
    Bc6,
    /// BC7 compressed (BPTC)
    Bc7,
}

impl TextureFormat {
    /// Get bytes per pixel (uncompressed)
    #[inline]
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            TextureFormat::Rgba8 => 4,
            TextureFormat::Rgb8 => 3,
            TextureFormat::Rgba16 => 8,
            TextureFormat::Rgb16 => 6,
            TextureFormat::Rgba32 => 16,
            TextureFormat::Rgba16Float => 8,
            TextureFormat::Rgba32Float => 16,
            TextureFormat::R8 => 1,
            TextureFormat::R16 => 2,
            TextureFormat::Bc1 => 8, // 4x4 block = 16 pixels
            TextureFormat::Bc3 => 16, // 4x4 block = 16 pixels
            TextureFormat::Bc4 => 8, // 4x4 block = 16 pixels
            TextureFormat::Bc5 => 16, // 4x4 block = 16 pixels
            TextureFormat::Bc6 => 16, // 4x4 block = 16 pixels
            TextureFormat::Bc7 => 16, // 4x4 block = 16 pixels
        }
    }

    /// Check if format is compressed
    #[inline]
    pub fn is_compressed(&self) -> bool {
        matches!(
            self,
            TextureFormat::Bc1
                | TextureFormat::Bc3
                | TextureFormat::Bc4
                | TextureFormat::Bc5
                | TextureFormat::Bc6
                | TextureFormat::Bc7
        )
    }

    /// Check if format is float
    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            TextureFormat::Rgba16Float
                | TextureFormat::Rgba32Float
                | TextureFormat::Bc6
        )
    }
}

/// Texture usage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureUsage {
    /// Sampled texture (shader read)
    Sampled,
    /// Storage texture (read/write)
    Storage,
    /// Render target (color attachment)
    RenderTarget,
    /// Depth/stencil attachment
    DepthStencil,
    /// Copy source
    CopySrc,
    /// Copy destination
    CopyDst,
}

/// Texture mip level
#[derive(Debug, Clone, Copy)]
pub struct MipLevel {
    /// Level index
    pub level: u32,
    /// Width in texels
    pub width: u32,
    /// Height in texels
    pub height: u32,
    /// Depth in texels (for 3D textures)
    pub depth: u32,
    /// Offset in bytes
    pub offset: u64,
    /// Size in bytes
    pub size: u64,
}

/// Texture LOD (Level of Detail)
#[derive(Debug, Clone, Copy)]
pub struct TextureLod {
    /// LOD level (0 = highest quality)
    pub level: u32,
    /// Minimum distance for this LOD
    pub min_distance: f32,
    /// Maximum distance for this LOD
    pub max_distance: f32,
    /// Mip level range for this LOD
    pub mip_start: u32,
    pub mip_end: u32,
}

/// Texture descriptor
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    /// Width in texels
    pub width: u32,
    /// Height in texels
    pub height: u32,
    /// Depth in texels (for 3D textures)
    pub depth: u32,
    /// Number of mip levels
    pub mip_levels: u32,
    /// Number of array layers
    pub array_layers: u32,
    /// Texture format
    pub format: TextureFormat,
    /// Texture usage
    pub usage: TextureUsage,
    /// Label for debugging
    pub label: Option<String>,
    /// Enable streaming
    pub enable_streaming: bool,
    /// Max LOD level for streaming
    pub max_streaming_lod: u32,
}

impl TextureDescriptor {
    /// Create a new 2D texture descriptor
    #[inline]
    pub fn new_2d(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            width,
            height,
            depth: 1,
            mip_levels: 1,
            array_layers: 1,
            format,
            usage: TextureUsage::Sampled,
            label: None,
            enable_streaming: false,
            max_streaming_lod: 4,
        }
    }

    /// Create a new 3D texture descriptor
    #[inline]
    pub fn new_3d(width: u32, height: u32, depth: u32, format: TextureFormat) -> Self {
        Self {
            width,
            height,
            depth,
            mip_levels: 1,
            array_layers: 1,
            format,
            usage: TextureUsage::Sampled,
            label: None,
            enable_streaming: false,
            max_streaming_lod: 4,
        }
    }

    /// Set texture label
    #[inline]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set texture usage
    #[inline]
    pub fn with_usage(mut self, usage: TextureUsage) -> Self {
        self.usage = usage;
        self
    }

    /// Enable streaming
    #[inline]
    pub fn with_streaming(mut self, enable: bool) -> Self {
        self.enable_streaming = enable;
        self
    }

    /// Set max streaming LOD
    #[inline]
    pub fn with_max_streaming_lod(mut self, max_lod: u32) -> Self {
        self.max_streaming_lod = max_lod;
        self
    }

    /// Calculate total size in bytes
    #[inline]
    pub fn total_size(&self) -> u64 {
        let mut total = 0u64;
        let bytes_per_pixel = self.format.bytes_per_pixel() as u64;

        for level in 0..self.mip_levels {
            let mip_width = (self.width >> level).max(1);
            let mip_height = (self.height >> level).max(1);
            let mip_depth = (self.depth >> level).max(1);
            let mip_size = mip_width as u64 * mip_height as u64 * mip_depth as u64 * bytes_per_pixel;
            total += mip_size;
        }

        total * self.array_layers as u64
    }

    /// Generate mip levels
    #[inline]
    pub fn generate_mips(&self) -> Vec<MipLevel> {
        let mut mips = Vec::new();
        let mut offset = 0u64;
        let bytes_per_pixel = self.format.bytes_per_pixel() as u64;

        for level in 0..self.mip_levels {
            let mip_width = (self.width >> level).max(1);
            let mip_height = (self.height >> level).max(1);
            let mip_depth = (self.depth >> level).max(1);
            let mip_size = mip_width as u64 * mip_height as u64 * mip_depth as u64 * bytes_per_pixel;

            mips.push(MipLevel {
                level,
                width: mip_width,
                height: mip_height,
                depth: mip_depth,
                offset,
                size: mip_size,
            });

            offset += mip_size;
        }

        mips
    }

    /// Generate LOD levels
    #[inline]
    pub fn generate_lods(&self) -> Vec<TextureLod> {
        let mut lods = Vec::new();
        let max_lod = self.max_streaming_lod.min(self.mip_levels);
        let mips_per_lod = self.mip_levels / max_lod.max(1);

        for lod in 0..max_lod {
            let mip_start = lod * mips_per_lod;
            let mip_end = ((lod + 1) * mips_per_lod).min(self.mip_levels);

            lods.push(TextureLod {
                level: lod,
                min_distance: lod as f32 * 10.0,
                max_distance: (lod + 1) as f32 * 10.0,
                mip_start,
                mip_end,
            });
        }

        lods
    }
}

/// Texture handle
#[derive(Debug, Clone)]
pub struct Texture {
    info: GpuBufferInfo,
    descriptor: TextureDescriptor,
    mips: Vec<MipLevel>,
    lods: Vec<TextureLod>,
    current_lod: u32,
    loaded_mips: Vec<bool>,
}

impl Texture {
    /// Get texture ID
    #[inline]
    pub fn id(&self) -> u64 {
        self.info.id
    }

    /// Get texture width
    #[inline]
    pub fn width(&self) -> u32 {
        self.descriptor.width
    }

    /// Get texture height
    #[inline]
    pub fn height(&self) -> u32 {
        self.descriptor.height
    }

    /// Get texture format
    #[inline]
    pub fn format(&self) -> TextureFormat {
        self.descriptor.format
    }

    /// Get current LOD level
    #[inline]
    pub fn current_lod(&self) -> u32 {
        self.current_lod
    }

    /// Set current LOD level
    #[inline]
    pub fn set_current_lod(&mut self, lod: u32) {
        self.current_lod = lod.min(self.lods.len() as u32 - 1);
    }

    /// Check if mip level is loaded
    #[inline]
    pub fn is_mip_loaded(&self, mip_level: u32) -> bool {
        self.loaded_mips.get(mip_level as usize).cloned().unwrap_or(false)
    }

    /// Mark mip level as loaded
    #[inline]
    pub fn mark_mip_loaded(&mut self, mip_level: u32) {
        let mip_idx = mip_level as usize;
        let len = self.loaded_mips.len();
        if mip_idx < len {
            self.loaded_mips[mip_idx] = true;
        }
    }

    /// Get LOD for distance
    #[inline]
    pub fn lod_for_distance(&self, distance: f32) -> u32 {
        for (i, lod) in self.lods.iter().enumerate() {
            if distance >= lod.min_distance && distance < lod.max_distance {
                return i as u32;
            }
        }
        self.lods.len() as u32 - 1
    }
}

/// Texture streaming cache entry
#[derive(Debug, Clone)]
struct TextureCacheEntry {
    texture_id: u64,
    lod_level: u32,
    last_used: u64,
    access_count: u64,
}

/// Texture streaming system
pub struct TextureStreamingSystem {
    memory_manager: Arc<GpuMemoryManager>,
    textures: Mutex<HashMap<u64, Texture>>,
    cache: Mutex<VecDeque<TextureCacheEntry>>,
    max_cache_size: usize,
    next_texture_id: AtomicU64,
    total_allocated: AtomicU64,
}

impl TextureStreamingSystem {
    /// Create a new texture streaming system
    #[inline]
    pub fn new(memory_manager: Arc<GpuMemoryManager>, max_cache_size: usize) -> Self {
        Self {
            memory_manager,
            textures: Mutex::new(HashMap::new()),
            cache: Mutex::new(VecDeque::new()),
            max_cache_size,
            next_texture_id: AtomicU64::new(1),
            total_allocated: AtomicU64::new(0),
        }
    }

    /// Create a new texture
    #[inline]
    pub fn create_texture(&self, descriptor: TextureDescriptor) -> Option<u64> {
        let total_size = descriptor.total_size();
        let buffer_info = self
            .memory_manager
            .allocate_buffer(total_size, GpuBufferType::Storage)?;

        let texture_id = self.next_texture_id.fetch_add(1, Ordering::Relaxed);
        let mips = descriptor.generate_mips();
        let lods = descriptor.generate_lods();
        let loaded_mips = vec![false; descriptor.mip_levels as usize];

        let texture = Texture {
            info: buffer_info,
            descriptor,
            mips,
            lods,
            current_lod: 0,
            loaded_mips,
        };

        if let Ok(mut textures) = self.textures.lock() {
            textures.insert(texture_id, texture);
        }

        self.total_allocated.fetch_add(total_size, Ordering::Relaxed);

        Some(texture_id)
    }

    /// Destroy a texture
    #[inline]
    pub fn destroy_texture(&self, texture_id: u64) -> bool {
        let texture = {
            let mut textures = match self.textures.lock() {
                Ok(textures) => textures,
                Err(_) => return false,
            };

            match textures.remove(&texture_id) {
                Some(texture) => texture,
                None => return false,
            }
        };

        self.memory_manager
            .free_buffer(texture.info.id, GpuBufferType::Storage);

        self.total_allocated
            .fetch_sub(texture.descriptor.total_size(), Ordering::Relaxed);

        // Remove from cache
        if let Ok(mut cache) = self.cache.lock() {
            cache.retain(|entry| entry.texture_id != texture_id);
        }

        true
    }

    /// Get texture reference
    #[inline]
    pub fn get_texture(&self, texture_id: u64) -> Option<Texture> {
        self.textures.lock().ok()?.get(&texture_id).cloned()
    }

    /// Update texture LOD based on distance
    #[inline]
    pub fn update_lod(&self, texture_id: u64, distance: f32) -> bool {
        let mut textures = match self.textures.lock() {
            Ok(textures) => textures,
            Err(_) => return false,
        };

        if let Some(texture) = textures.get_mut(&texture_id) {
            let target_lod = texture.lod_for_distance(distance);
            texture.set_current_lod(target_lod);
            true
        } else {
            false
        }
    }

    /// Load mip level for texture
    #[inline]
    pub fn load_mip_level(&self, texture_id: u64, mip_level: u32, data: &[u8]) -> bool {
        let mut textures = match self.textures.lock() {
            Ok(textures) => textures,
            Err(_) => return false,
        };

        if let Some(texture) = textures.get_mut(&texture_id) {
            if mip_level as usize >= texture.mips.len() {
                return false;
            }

            let mip_info = &texture.mips[mip_level as usize];
            if data.len() as u64 != mip_info.size {
                return false;
            }

            // In real implementation, this would upload data to GPU
            texture.mark_mip_loaded(mip_level);
            true
        } else {
            false
        }
    }

    /// Stream texture LODs
    #[inline]
    pub fn stream_lod(&self, texture_id: u64, target_lod: u32) -> bool {
        let texture = match self.get_texture(texture_id) {
            Some(texture) => texture,
            None => return false,
        };

        if target_lod >= texture.lods.len() as u32 {
            return false;
        }

        let lod = &texture.lods[target_lod as usize];

        // Load all mip levels for this LOD
        for mip_level in lod.mip_start..lod.mip_end {
            // In real implementation, this would load from disk or generate
            // For now, mark as loaded
            let _ = self.load_mip_level(texture_id, mip_level, &[]);
        }

        true
    }

    /// Evict least recently used textures
    #[inline]
    pub fn evict_lru(&self, count: usize) -> usize {
        let mut evicted = 0;

        if let Ok(mut cache) = self.cache.lock() {
            for _ in 0..count {
                if let Some(entry) = cache.pop_front() {
                    if self.destroy_texture(entry.texture_id) {
                        evicted += 1;
                    }
                } else {
                    break;
                }
            }
        }

        evicted
    }

    /// Update cache access
    #[inline]
    pub fn update_cache_access(&self, texture_id: u64, lod_level: u32) {
        if let Ok(mut cache) = self.cache.lock() {
            // Remove existing entry
            cache.retain(|entry| entry.texture_id != texture_id);

            // Add to back (most recently used)
            cache.push_back(TextureCacheEntry {
                texture_id,
                lod_level,
                last_used: 0, // Would be timestamp in real implementation
                access_count: 1,
            });

            // Enforce cache size limit
            while cache.len() > self.max_cache_size {
                if let Some(entry) = cache.pop_front() {
                    self.destroy_texture(entry.texture_id);
                }
            }
        }
    }

    /// Get total allocated memory
    #[inline]
    pub fn total_allocated(&self) -> u64 {
        self.total_allocated.load(Ordering::Relaxed)
    }

    /// Get texture count
    #[inline]
    pub fn texture_count(&self) -> usize {
        self.textures.lock().map(|t| t.len()).unwrap_or(0)
    }

    /// Get cache statistics
    #[inline]
    pub fn cache_stats(&self) -> (usize, usize) {
        match self.cache.lock() {
            Ok(cache) => (cache.len(), self.max_cache_size),
            Err(_) => (0, self.max_cache_size),
        }
    }
}

impl Default for TextureStreamingSystem {
    #[inline]
    fn default() -> Self {
        Self::new(Arc::new(GpuMemoryManager::new()), 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_format() {
        assert_eq!(TextureFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(TextureFormat::Rgb8.bytes_per_pixel(), 3);
        assert_eq!(TextureFormat::Rgba16.bytes_per_pixel(), 8);

        assert!(!TextureFormat::Rgba8.is_compressed());
        assert!(TextureFormat::Bc1.is_compressed());

        assert!(!TextureFormat::Rgba8.is_float());
        assert!(TextureFormat::Rgba16Float.is_float());
    }

    #[test]
    fn test_texture_descriptor() {
        let descriptor = TextureDescriptor::new_2d(512, 512, TextureFormat::Rgba8)
            .with_label("test_texture")
            .with_streaming(true);

        assert_eq!(descriptor.width, 512);
        assert_eq!(descriptor.height, 512);
        assert_eq!(descriptor.format, TextureFormat::Rgba8);
        assert!(descriptor.enable_streaming);
        assert_eq!(descriptor.label, Some("test_texture".to_string()));
    }

    #[test]
    fn test_mip_generation() {
        let descriptor = TextureDescriptor::new_2d(256, 256, TextureFormat::Rgba8)
            .with_max_streaming_lod(4);

        let mips = descriptor.generate_mips();
        assert_eq!(mips.len(), 9); // log2(256) + 1

        assert_eq!(mips[0].width, 256);
        assert_eq!(mips[0].height, 256);
        assert_eq!(mips[1].width, 128);
        assert_eq!(mips[1].height, 128);

        let lods = descriptor.generate_lods();
        assert_eq!(lods.len(), 4);
    }

    #[test]
    fn test_texture_streaming() {
        let system = TextureStreamingSystem::default();

        let descriptor = TextureDescriptor::new_2d(256, 256, TextureFormat::Rgba8)
            .with_streaming(true);

        let texture_id = system.create_texture(descriptor).unwrap();
        assert_eq!(system.texture_count(), 1);

        let texture = system.get_texture(texture_id).unwrap();
        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);

        // Test LOD update
        assert!(system.update_lod(texture_id, 5.0));
        let texture = system.get_texture(texture_id).unwrap();
        assert_eq!(texture.current_lod(), 0);

        assert!(system.update_lod(texture_id, 15.0));
        let texture = system.get_texture(texture_id).unwrap();
        assert_eq!(texture.current_lod(), 1);

        assert!(system.destroy_texture(texture_id));
        assert_eq!(system.texture_count(), 0);
    }

    #[test]
    fn test_cache_eviction() {
        let system = TextureStreamingSystem::default();

        let descriptor = TextureDescriptor::new_2d(128, 128, TextureFormat::Rgba8)
            .with_streaming(true);

        // Create multiple textures
        let mut texture_ids = Vec::new();
        for _ in 0..10 {
            let id = system.create_texture(descriptor.clone()).unwrap();
            texture_ids.push(id);
        }

        assert_eq!(system.texture_count(), 10);

        // Evict some textures
        let evicted = system.evict_lru(3);
        assert!(evicted > 0);
        assert!(system.texture_count() < 10);
    }
}
