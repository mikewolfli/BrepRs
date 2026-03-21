//! GPU Memory Management Module
//!
//! This module provides VRAM pool management, buffer allocation,
//! and memory optimization for GPU-based visualization.
//! Compatible with OpenCASCADE Open API design.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// GPU memory usage statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct GpuMemoryStats {
    /// Total VRAM allocated in bytes
    pub total_allocated: u64,
    /// Total VRAM used in bytes
    pub total_used: u64,
    /// Available VRAM in bytes
    pub available: u64,
    /// Number of active buffers
    pub buffer_count: u32,
    /// Number of active textures
    pub texture_count: u32,
    /// Memory pool fragmentation ratio (0.0 - 1.0)
    pub fragmentation_ratio: f32,
}

/// GPU buffer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuBufferType {
    /// Vertex buffer
    Vertex,
    /// Index buffer
    Index,
    /// Uniform buffer
    Uniform,
    /// Storage buffer
    Storage,
    /// Indirect buffer
    Indirect,
}

impl GpuBufferType {
    /// Get default alignment for buffer type
    #[inline]
    pub fn alignment(&self) -> u64 {
        match self {
            GpuBufferType::Vertex => 16,
            GpuBufferType::Index => 4,
            GpuBufferType::Uniform => 256,
            GpuBufferType::Storage => 16,
            GpuBufferType::Indirect => 4,
        }
    }
}

/// GPU memory pool configuration
#[derive(Debug, Clone, Copy)]
pub struct GpuMemoryPoolConfig {
    /// Maximum pool size in bytes
    pub max_pool_size: u64,
    /// Initial pool size in bytes
    pub initial_pool_size: u64,
    /// Allocation alignment in bytes
    pub alignment: u64,
    /// Enable memory defragmentation
    pub enable_defragmentation: bool,
    /// Defragmentation threshold (fragmentation ratio)
    pub defrag_threshold: f32,
}

impl Default for GpuMemoryPoolConfig {
    #[inline]
    fn default() -> Self {
        Self {
            max_pool_size: 2 * 1024 * 1024 * 1024, // 2GB default
            initial_pool_size: 256 * 1024 * 1024,  // 256MB initial
            alignment: 256,
            enable_defragmentation: true,
            defrag_threshold: 0.3,
        }
    }
}

/// Memory block in GPU pool
#[derive(Debug, Clone)]
struct MemoryBlock {
    offset: u64,
    size: u64,
    used: bool,
    buffer_id: Option<u64>,
}

impl MemoryBlock {
    #[inline]
    fn new(offset: u64, size: u64) -> Self {
        Self {
            offset,
            size,
            used: false,
            buffer_id: None,
        }
    }
}

/// GPU memory pool for efficient VRAM management
pub struct GpuMemoryPool {
    config: GpuMemoryPoolConfig,
    blocks: Vec<MemoryBlock>,
    total_allocated: AtomicU64,
    total_used: AtomicU64,
    next_buffer_id: AtomicU64,
    buffer_map: Mutex<HashMap<u64, GpuBufferInfo>>,
}

/// GPU buffer information
#[derive(Debug, Clone)]
pub struct GpuBufferInfo {
    pub id: u64,
    pub buffer_type: GpuBufferType,
    pub size: u64,
    pub offset: u64,
    pub pool_offset: u64,
}

impl GpuMemoryPool {
    /// Create a new GPU memory pool
    #[inline]
    pub fn new(config: GpuMemoryPoolConfig) -> Self {
        let initial_block = MemoryBlock::new(0, config.initial_pool_size);
        
        Self {
            config,
            blocks: vec![initial_block],
            total_allocated: AtomicU64::new(config.initial_pool_size),
            total_used: AtomicU64::new(0),
            next_buffer_id: AtomicU64::new(1),
            buffer_map: Mutex::new(HashMap::new()),
        }
    }

    /// Allocate GPU buffer
    #[inline]
    pub fn allocate_buffer(&mut self, size: u64, buffer_type: GpuBufferType) -> Option<GpuBufferInfo> {
        let aligned_size = self.align_size(size, buffer_type.alignment());
        
        // Find best-fit free block
        let block_index = self.find_best_fit_block(aligned_size)?;
        
        let buffer_id = self.next_buffer_id.fetch_add(1, Ordering::Relaxed);
        let mut blocks = self.blocks.clone();
        
        let pool_offset = blocks[block_index].offset;
        
        // Split block if there's remaining space
        if blocks[block_index].size > aligned_size + self.config.alignment {
            let new_block = MemoryBlock::new(
                blocks[block_index].offset + aligned_size,
                blocks[block_index].size - aligned_size,
            );
            blocks[block_index].size = aligned_size;
            blocks.insert(block_index + 1, new_block);
        }
        
        blocks[block_index].used = true;
        blocks[block_index].buffer_id = Some(buffer_id);
        
        self.total_used.fetch_add(aligned_size, Ordering::Relaxed);
        
        let buffer_info = GpuBufferInfo {
            id: buffer_id,
            buffer_type,
            size: aligned_size,
            offset: 0,
            pool_offset,
        };
        
        if let Ok(mut map) = self.buffer_map.lock() {
            map.insert(buffer_id, buffer_info.clone());
        }
        
        // Update the blocks
        self.blocks = blocks;
        
        Some(buffer_info)
    }

    /// Free GPU buffer
    #[inline]
    pub fn free_buffer(&mut self, buffer_id: u64) -> bool {
        let buffer_info = {
            let mut map = match self.buffer_map.lock() {
                Ok(map) => map,
                Err(_) => return false,
            };
            
            match map.remove(&buffer_id) {
                Some(info) => info,
                None => return false,
            }
        };
        
        // Mark block as free
        for block in &mut self.blocks.clone() {
            if block.buffer_id == Some(buffer_id) {
                block.used = false;
                block.buffer_id = None;
                break;
            }
        }
        
        self.total_used.fetch_sub(buffer_info.size, Ordering::Relaxed);
        
        // Coalesce adjacent free blocks
        self.coalesce_blocks();
        
        // Check if defragmentation is needed
        if self.config.enable_defragmentation {
            let fragmentation = self.fragmentation_ratio();
            if fragmentation > self.config.defrag_threshold {
                self.defragment();
            }
        }
        
        true
    }

    /// Get memory statistics
    #[inline]
    pub fn stats(&self) -> GpuMemoryStats {
        let total_allocated = self.total_allocated.load(Ordering::Relaxed);
        let total_used = self.total_used.load(Ordering::Relaxed);
        
        GpuMemoryStats {
            total_allocated,
            total_used,
            available: total_allocated.saturating_sub(total_used),
            buffer_count: self.buffer_count(),
            texture_count: 0, // Will be implemented with texture manager
            fragmentation_ratio: self.fragmentation_ratio(),
        }
    }

    /// Get buffer information
    #[inline]
    pub fn get_buffer_info(&self, buffer_id: u64) -> Option<GpuBufferInfo> {
        self.buffer_map.lock().ok()?.get(&buffer_id).cloned()
    }

    /// Check if pool needs expansion
    #[inline]
    pub fn needs_expansion(&self) -> bool {
        let total_allocated = self.total_allocated.load(Ordering::Relaxed);
        let total_used = self.total_used.load(Ordering::Relaxed);
        
        total_used > total_allocated * 8 / 10 // 80% threshold
    }

    /// Expand pool size
    #[inline]
    pub fn expand_pool(&mut self, additional_size: u64) -> bool {
        let current_size = self.total_allocated.load(Ordering::Relaxed);
        let new_size = current_size + additional_size;
        
        if new_size > self.config.max_pool_size {
            return false;
        }
        
        let new_block = MemoryBlock::new(current_size, additional_size);
        self.blocks.push(new_block);
        self.total_allocated.store(new_size, Ordering::Relaxed);
        
        true
    }

    /// Align size to specified alignment
    #[inline]
    fn align_size(&self, size: u64, alignment: u64) -> u64 {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Find best-fit free block
    #[inline]
    fn find_best_fit_block(&self, size: u64) -> Option<usize> {
        let mut best_index = None;
        let mut best_size = u64::MAX;
        
        for (index, block) in self.blocks.iter().enumerate() {
            if !block.used && block.size >= size && block.size < best_size {
                best_index = Some(index);
                best_size = block.size;
            }
        }
        
        best_index
    }

    /// Coalesce adjacent free blocks
    #[inline]
    fn coalesce_blocks(&mut self) {
        let mut i = 0;
        while i < self.blocks.len().saturating_sub(1) {
            if !self.blocks[i].used && !self.blocks[i + 1].used {
                self.blocks[i].size += self.blocks[i + 1].size;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    /// Calculate fragmentation ratio
    #[inline]
    fn fragmentation_ratio(&self) -> f32 {
        let total_free: u64 = self.blocks
            .iter()
            .filter(|b| !b.used)
            .map(|b| b.size)
            .sum();
        
        if total_free == 0 {
            return 0.0;
        }
        
        let largest_free = self.blocks
            .iter()
            .filter(|b| !b.used)
            .map(|b| b.size)
            .max()
            .unwrap_or(0);
        
        1.0 - (largest_free as f32 / total_free as f32)
    }

    /// Defragment memory pool
    #[inline]
    fn defragment(&mut self) {
        // Move all used blocks to the beginning
        // Reorganize blocks to reduce fragmentation
        let mut new_blocks = Vec::new();
        let mut current_offset = 0;
        
        // Collect and compact used blocks
        for block in &self.blocks {
            if block.used {
                let mut compacted_block = block.clone();
                compacted_block.offset = current_offset;
                new_blocks.push(compacted_block);
                current_offset += block.size;
            }
        }
        
        // Add remaining free space as one large block
        let total_allocated = self.total_allocated.load(Ordering::Relaxed);
        if current_offset < total_allocated {
            new_blocks.push(MemoryBlock::new(current_offset, total_allocated - current_offset));
        }
        
        // Update buffer_map with new offsets
        if let Ok(mut map) = self.buffer_map.lock() {
            for block in &new_blocks {
                if block.used && block.buffer_id.is_some() {
                    if let Some(buffer_id) = block.buffer_id {
                        if let Some(buffer_info) = map.get_mut(&buffer_id) {
                            buffer_info.pool_offset = block.offset;
                        }
                    }
                }
            }
        }
        
        // Update blocks
        self.blocks = new_blocks;
    }

    /// Get active buffer count
    #[inline]
    fn buffer_count(&self) -> u32 {
        self.blocks.iter().filter(|b| b.used).count() as u32
    }
}

impl Default for GpuMemoryPool {
    #[inline]
    fn default() -> Self {
        Self::new(GpuMemoryPoolConfig::default())
    }
}

/// GPU memory manager for global VRAM management
pub struct GpuMemoryManager {
    pools: Mutex<HashMap<GpuBufferType, Arc<Mutex<GpuMemoryPool>>>>,
}

impl GpuMemoryManager {
    /// Create a new GPU memory manager
    #[inline]
    pub fn new() -> Self {
        let mut pools = HashMap::new();
        
        // Create separate pools for different buffer types
        pools.insert(GpuBufferType::Vertex, Arc::new(Mutex::new(GpuMemoryPool::default())));
        pools.insert(GpuBufferType::Index, Arc::new(Mutex::new(GpuMemoryPool::default())));
        pools.insert(GpuBufferType::Uniform, Arc::new(Mutex::new(GpuMemoryPool::default())));
        pools.insert(GpuBufferType::Storage, Arc::new(Mutex::new(GpuMemoryPool::default())));
        pools.insert(GpuBufferType::Indirect, Arc::new(Mutex::new(GpuMemoryPool::default())));
        
        Self {
            pools: Mutex::new(pools),
        }
    }

    /// Allocate buffer from appropriate pool
    #[inline]
    pub fn allocate_buffer(&self, size: u64, buffer_type: GpuBufferType) -> Option<GpuBufferInfo> {
        let pools = self.pools.lock().ok()?;
        let pool = pools.get(&buffer_type)?;
        let mut pool_lock = pool.lock().ok()?;
        pool_lock.allocate_buffer(size, buffer_type)
    }

    /// Free buffer from pool
    #[inline]
    pub fn free_buffer(&self, buffer_id: u64, buffer_type: GpuBufferType) -> bool {
        let pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return false,
        };
        
        if let Some(pool) = pools.get(&buffer_type) {
            if let Ok(mut pool_lock) = pool.lock() {
                pool_lock.free_buffer(buffer_id)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get global memory statistics
    #[inline]
    pub fn global_stats(&self) -> GpuMemoryStats {
        let pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return GpuMemoryStats::default(),
        };
        
        let mut total_stats = GpuMemoryStats::default();
        
        for pool in pools.values() {
            if let Ok(pool_lock) = pool.lock() {
                let stats = pool_lock.stats();
                total_stats.total_allocated += stats.total_allocated;
                total_stats.total_used += stats.total_used;
                total_stats.buffer_count += stats.buffer_count;
            }
        }
        
        total_stats.available = total_stats.total_allocated - total_stats.total_used;
        
        if total_stats.total_allocated > 0 {
            total_stats.fragmentation_ratio = 
                total_stats.total_used as f32 / total_stats.total_allocated as f32;
        }
        
        total_stats
    }

    /// Get pool for specific buffer type
    #[inline]
    pub fn get_pool(&self, buffer_type: GpuBufferType) -> Option<Arc<Mutex<GpuMemoryPool>>> {
        self.pools.lock().ok()?.get(&buffer_type).cloned()
    }
}

impl Default for GpuMemoryManager {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_memory_pool_creation() {
        let config = GpuMemoryPoolConfig::default();
        let pool = GpuMemoryPool::new(config);
        
        let stats = pool.stats();
        assert_eq!(stats.total_allocated, config.initial_pool_size);
        assert_eq!(stats.total_used, 0);
    }

    #[test]
    fn test_buffer_allocation() {
        let mut pool = GpuMemoryPool::default();
        
        let buffer = pool.allocate_buffer(1024, GpuBufferType::Vertex);
        assert!(buffer.is_some());
        
        let buffer = buffer.unwrap();
        assert_eq!(buffer.size, 1024);
        assert_eq!(buffer.buffer_type, GpuBufferType::Vertex);
    }

    #[test]
    fn test_buffer_alignment() {
        let mut pool = GpuMemoryPool::default();
        
        // Uniform buffers require 256-byte alignment
        let buffer = pool.allocate_buffer(100, GpuBufferType::Uniform);
        assert!(buffer.is_some());
        
        let buffer = buffer.unwrap();
        assert!(buffer.size >= 256);
    }

    #[test]
    fn test_buffer_free() {
        let mut pool = GpuMemoryPool::default();
        
        let buffer = pool.allocate_buffer(1024, GpuBufferType::Vertex).unwrap();
        let buffer_id = buffer.id;
        
        assert!(pool.free_buffer(buffer_id));
        assert!(!pool.free_buffer(buffer_id)); // Already freed
    }

    #[test]
    fn test_memory_stats() {
        let mut pool = GpuMemoryPool::default();
        
        let _initial_stats = pool.stats();
        
        let buffer1 = pool.allocate_buffer(1024, GpuBufferType::Vertex).unwrap();
        let _buffer2 = pool.allocate_buffer(2048, GpuBufferType::Index).unwrap();
        
        let stats = pool.stats();
        assert_eq!(stats.buffer_count, 2);
        assert!(stats.total_used >= 1024 + 2048);
        
        pool.free_buffer(buffer1.id);
        
        let stats_after_free = pool.stats();
        assert_eq!(stats_after_free.buffer_count, 1);
    }

    #[test]
    fn test_gpu_memory_manager() {
        let manager = GpuMemoryManager::new();
        
        let buffer = manager.allocate_buffer(1024, GpuBufferType::Vertex);
        assert!(buffer.is_some());
        
        let stats = manager.global_stats();
        assert!(stats.buffer_count >= 1);
    }
}
