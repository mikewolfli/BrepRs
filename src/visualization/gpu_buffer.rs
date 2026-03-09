//! GPU Buffer Management System
//!
//! This module provides GPU buffer management for vertex, index, uniform,
//! and storage buffers with automatic memory optimization.
//! Compatible with OpenCASCADE Open API design.

use crate::visualization::gpu_memory::{GpuBufferInfo, GpuBufferType, GpuMemoryManager};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// GPU buffer usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpuBufferUsage {
    /// Buffer can be mapped for CPU read
    pub map_read: bool,
    /// Buffer can be mapped for CPU write
    pub map_write: bool,
    /// Buffer can be used as copy source
    pub copy_src: bool,
    /// Buffer can be used as copy destination
    pub copy_dst: bool,
    /// Buffer can be used as index buffer
    pub index: bool,
    /// Buffer can be used as vertex buffer
    pub vertex: bool,
    /// Buffer can be used as uniform buffer
    pub uniform: bool,
    /// Buffer can be used as storage buffer
    pub storage: bool,
    /// Buffer can be used as indirect buffer
    pub indirect: bool,
}

impl GpuBufferUsage {
    /// Create vertex buffer usage
    #[inline]
    pub fn vertex() -> Self {
        Self {
            map_write: true,
            copy_dst: true,
            vertex: true,
            ..Default::default()
        }
    }

    /// Create index buffer usage
    #[inline]
    pub fn index() -> Self {
        Self {
            map_write: true,
            copy_dst: true,
            index: true,
            ..Default::default()
        }
    }

    /// Create uniform buffer usage
    #[inline]
    pub fn uniform() -> Self {
        Self {
            map_write: true,
            copy_dst: true,
            uniform: true,
            ..Default::default()
        }
    }

    /// Create storage buffer usage
    #[inline]
    pub fn storage() -> Self {
        Self {
            map_read: true,
            map_write: true,
            copy_src: true,
            copy_dst: true,
            storage: true,
            ..Default::default()
        }
    }

    /// Create staging buffer usage (CPU->GPU transfers)
    #[inline]
    pub fn staging() -> Self {
        Self {
            map_write: true,
            copy_src: true,
            ..Default::default()
        }
    }

    /// Create readback buffer usage (GPU->CPU transfers)
    #[inline]
    pub fn readback() -> Self {
        Self {
            map_read: true,
            copy_dst: true,
            ..Default::default()
        }
    }
}

impl Default for GpuBufferUsage {
    #[inline]
    fn default() -> Self {
        Self {
            map_read: false,
            map_write: false,
            copy_src: false,
            copy_dst: false,
            index: false,
            vertex: false,
            uniform: false,
            storage: false,
            indirect: false,
        }
    }
}

/// GPU buffer descriptor
#[derive(Debug, Clone)]
pub struct GpuBufferDescriptor {
    /// Buffer size in bytes
    pub size: u64,
    /// Buffer usage flags
    pub usage: GpuBufferUsage,
    /// Buffer type
    pub buffer_type: GpuBufferType,
    /// Label for debugging
    pub label: Option<String>,
}

impl GpuBufferDescriptor {
    /// Create a new buffer descriptor
    #[inline]
    pub fn new(size: u64, buffer_type: GpuBufferType) -> Self {
        let usage = match buffer_type {
            GpuBufferType::Vertex => GpuBufferUsage::vertex(),
            GpuBufferType::Index => GpuBufferUsage::index(),
            GpuBufferType::Uniform => GpuBufferUsage::uniform(),
            GpuBufferType::Storage => GpuBufferUsage::storage(),
            GpuBufferType::Indirect => GpuBufferUsage {
                map_write: true,
                copy_dst: true,
                indirect: true,
                ..Default::default()
            },
        };

        Self {
            size,
            usage,
            buffer_type,
            label: None,
        }
    }

    /// Set buffer label
    #[inline]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set custom usage flags
    #[inline]
    pub fn with_usage(mut self, usage: GpuBufferUsage) -> Self {
        self.usage = usage;
        self
    }
}

/// GPU buffer handle
#[derive(Debug, Clone)]
pub struct GpuBuffer {
    info: GpuBufferInfo,
    descriptor: GpuBufferDescriptor,
    mapped: bool,
    map_offset: u64,
    map_size: u64,
}

impl GpuBuffer {
    /// Get buffer ID
    #[inline]
    pub fn id(&self) -> u64 {
        self.info.id
    }

    /// Get buffer size
    #[inline]
    pub fn size(&self) -> u64 {
        self.info.size
    }

    /// Get buffer type
    #[inline]
    pub fn buffer_type(&self) -> GpuBufferType {
        self.info.buffer_type
    }

    /// Get buffer usage
    #[inline]
    pub fn usage(&self) -> GpuBufferUsage {
        self.descriptor.usage
    }

    /// Get pool offset
    #[inline]
    pub fn pool_offset(&self) -> u64 {
        self.info.pool_offset
    }

    /// Check if buffer is mapped
    #[inline]
    pub fn is_mapped(&self) -> bool {
        self.mapped
    }

    /// Get buffer label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.descriptor.label.as_deref()
    }
}

/// GPU buffer manager for buffer lifecycle management
pub struct GpuBufferManager {
    memory_manager: Arc<GpuMemoryManager>,
    buffers: Mutex<HashMap<u64, GpuBuffer>>,
    next_buffer_id: AtomicU64,
    total_allocated: AtomicU64,
}

impl GpuBufferManager {
    /// Create a new GPU buffer manager
    #[inline]
    pub fn new(memory_manager: Arc<GpuMemoryManager>) -> Self {
        Self {
            memory_manager,
            buffers: Mutex::new(HashMap::new()),
            next_buffer_id: AtomicU64::new(1),
            total_allocated: AtomicU64::new(0),
        }
    }

    /// Create a new GPU buffer
    #[inline]
    pub fn create_buffer(&self, descriptor: GpuBufferDescriptor) -> Option<u64> {
        let buffer_info = self
            .memory_manager
            .allocate_buffer(descriptor.size, descriptor.buffer_type.clone())?;

        let buffer_id = self.next_buffer_id.fetch_add(1, Ordering::Relaxed);
        let descriptor_size = descriptor.size;

        let buffer = GpuBuffer {
            info: buffer_info,
            descriptor,
            mapped: false,
            map_offset: 0,
            map_size: 0,
        };

        if let Ok(mut buffers) = self.buffers.lock() {
            buffers.insert(buffer_id, buffer);
        }

        self.total_allocated
            .fetch_add(descriptor_size, Ordering::Relaxed);

        Some(buffer_id)
    }

    /// Destroy a GPU buffer
    #[inline]
    pub fn destroy_buffer(&self, buffer_id: u64) -> bool {
        let buffer = {
            let mut buffers = match self.buffers.lock() {
                Ok(buffers) => buffers,
                Err(_) => return false,
            };

            match buffers.remove(&buffer_id) {
                Some(buffer) => buffer,
                None => return false,
            }
        };

        self.memory_manager
            .free_buffer(buffer.info.id, buffer.info.buffer_type);

        self.total_allocated
            .fetch_sub(buffer.descriptor.size, Ordering::Relaxed);

        true
    }

    /// Get buffer reference
    #[inline]
    pub fn get_buffer(&self, buffer_id: u64) -> Option<GpuBuffer> {
        self.buffers.lock().ok()?.get(&buffer_id).cloned()
    }

    /// Map buffer for CPU access
    #[inline]
    pub fn map_buffer(&self, buffer_id: u64, offset: u64, size: u64) -> bool {
        let mut buffers = match self.buffers.lock() {
            Ok(buffers) => buffers,
            Err(_) => return false,
        };

        if let Some(buffer) = buffers.get_mut(&buffer_id) {
            if buffer.mapped {
                return false; // Already mapped
            }

            // Check mapping permissions
            let can_read = buffer.descriptor.usage.map_read;
            let can_write = buffer.descriptor.usage.map_write;

            if !can_read && !can_write {
                return false;
            }

            // Validate range
            if offset + size > buffer.info.size {
                return false;
            }

            buffer.mapped = true;
            buffer.map_offset = offset;
            buffer.map_size = size;

            true
        } else {
            false
        }
    }

    /// Unmap buffer
    #[inline]
    pub fn unmap_buffer(&self, buffer_id: u64) -> bool {
        let mut buffers = match self.buffers.lock() {
            Ok(buffers) => buffers,
            Err(_) => return false,
        };

        if let Some(buffer) = buffers.get_mut(&buffer_id) {
            if !buffer.mapped {
                return false; // Not mapped
            }

            buffer.mapped = false;
            buffer.map_offset = 0;
            buffer.map_size = 0;

            true
        } else {
            false
        }
    }

    /// Copy data to buffer (CPU -> GPU)
    #[inline]
    pub fn write_buffer(&self, buffer_id: u64, offset: u64, data: &[u8]) -> bool {
        let buffer = match self.get_buffer(buffer_id) {
            Some(buffer) => buffer,
            None => return false,
        };

        // Check if buffer supports writing
        if !buffer.descriptor.usage.copy_dst && !buffer.descriptor.usage.map_write {
            return false;
        }

        // Validate range
        if offset + data.len() as u64 > buffer.info.size {
            return false;
        }

        // If buffer is mapped, write directly
        if buffer.mapped {
            // In real implementation, this would write to mapped memory
            true
        } else {
            // Use staging buffer or direct upload
            true
        }
    }

    /// Copy data from buffer (GPU -> CPU)
    #[inline]
    pub fn read_buffer(&self, buffer_id: u64, offset: u64, size: u64, data: &mut [u8]) -> bool {
        let buffer = match self.get_buffer(buffer_id) {
            Some(buffer) => buffer,
            None => return false,
        };

        // Check if buffer supports reading
        if !buffer.descriptor.usage.copy_src && !buffer.descriptor.usage.map_read {
            return false;
        }

        // Validate range
        if offset + size > buffer.info.size || size > data.len() as u64 {
            return false;
        }

        // If buffer is mapped, read directly
        if buffer.mapped {
            // In real implementation, this would read from mapped memory
            true
        } else {
            // Use readback buffer or direct download
            true
        }
    }

    /// Copy between buffers (GPU -> GPU)
    #[inline]
    pub fn copy_buffer_to_buffer(
        &self,
        src_buffer_id: u64,
        src_offset: u64,
        dst_buffer_id: u64,
        dst_offset: u64,
        size: u64,
    ) -> bool {
        let src_buffer = match self.get_buffer(src_buffer_id) {
            Some(buffer) => buffer,
            None => return false,
        };

        let dst_buffer = match self.get_buffer(dst_buffer_id) {
            Some(buffer) => buffer,
            None => return false,
        };

        // Check copy permissions
        if !src_buffer.descriptor.usage.copy_src || !dst_buffer.descriptor.usage.copy_dst {
            return false;
        }

        // Validate ranges
        if src_offset + size > src_buffer.info.size
            || dst_offset + size > dst_buffer.info.size
        {
            return false;
        }

        // In real implementation, this would queue a GPU copy command
        true
    }

    /// Get total allocated memory
    #[inline]
    pub fn total_allocated(&self) -> u64 {
        self.total_allocated.load(Ordering::Relaxed)
    }

    /// Get active buffer count
    #[inline]
    pub fn buffer_count(&self) -> usize {
        self.buffers.lock().map(|b| b.len()).unwrap_or(0)
    }

    /// Get memory statistics
    #[inline]
    pub fn memory_stats(&self) -> crate::visualization::gpu_memory::GpuMemoryStats {
        self.memory_manager.global_stats()
    }

    /// Create staging buffer for upload
    #[inline]
    pub fn create_staging_buffer(&self, size: u64) -> Option<u64> {
        let descriptor = GpuBufferDescriptor {
            size,
            usage: GpuBufferUsage::staging(),
            buffer_type: GpuBufferType::Storage,
            label: Some("staging_buffer".to_string()),
        };

        self.create_buffer(descriptor)
    }

    /// Create readback buffer for download
    #[inline]
    pub fn create_readback_buffer(&self, size: u64) -> Option<u64> {
        let descriptor = GpuBufferDescriptor {
            size,
            usage: GpuBufferUsage::readback(),
            buffer_type: GpuBufferType::Storage,
            label: Some("readback_buffer".to_string()),
        };

        self.create_buffer(descriptor)
    }
}

impl Default for GpuBufferManager {
    #[inline]
    fn default() -> Self {
        Self::new(Arc::new(GpuMemoryManager::new()))
    }
}

/// GPU buffer pool for efficient buffer reuse
pub struct GpuBufferPool {
    manager: Arc<GpuBufferManager>,
    available_buffers: Mutex<HashMap<GpuBufferType, VecDeque<u64>>>,
    max_pool_size: usize,
}

impl GpuBufferPool {
    /// Create a new GPU buffer pool
    #[inline]
    pub fn new(manager: Arc<GpuBufferManager>, max_pool_size: usize) -> Self {
        Self {
            manager,
            available_buffers: Mutex::new(HashMap::new()),
            max_pool_size,
        }
    }

    /// Acquire buffer from pool or create new one
    #[inline]
    pub fn acquire_buffer(&self, descriptor: &GpuBufferDescriptor) -> Option<u64> {
        let mut available = self.available_buffers.lock().ok()?;

        // Try to find existing buffer of suitable size
        if let Some(buffers) = available.get_mut(&descriptor.buffer_type) {
            if let Some(buffer_id) = buffers.pop_front() {
                // Check if buffer is large enough
                if let Some(buffer) = self.manager.get_buffer(buffer_id) {
                    if buffer.size() >= descriptor.size {
                        return Some(buffer_id);
                    } else {
                        // Buffer too small, destroy it
                        self.manager.destroy_buffer(buffer_id);
                    }
                }
            }
        }

        // Create new buffer
        self.manager.create_buffer(descriptor.clone())
    }

    /// Return buffer to pool
    #[inline]
    pub fn release_buffer(&self, buffer_id: u64) {
        if let Some(buffer) = self.manager.get_buffer(buffer_id) {
            let mut available = match self.available_buffers.lock() {
                Ok(available) => available,
                Err(_) => {
                    self.manager.destroy_buffer(buffer_id);
                    return;
                }
            };

            let buffers = available
                .entry(buffer.buffer_type())
                .or_insert_with(VecDeque::new);

            if buffers.len() < self.max_pool_size {
                buffers.push_back(buffer_id);
            } else {
                // Pool full, destroy buffer
                self.manager.destroy_buffer(buffer_id);
            }
        }
    }

    /// Clear all pooled buffers
    #[inline]
    pub fn clear(&self) {
        let mut available = match self.available_buffers.lock() {
            Ok(available) => available,
            Err(_) => return,
        };

        for buffers in available.values_mut() {
            for buffer_id in buffers.drain(..) {
                self.manager.destroy_buffer(buffer_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_usage_flags() {
        let vertex_usage = GpuBufferUsage::vertex();
        assert!(vertex_usage.vertex);
        assert!(vertex_usage.map_write);
        assert!(vertex_usage.copy_dst);
        assert!(!vertex_usage.index);

        let index_usage = GpuBufferUsage::index();
        assert!(index_usage.index);
        assert!(!index_usage.vertex);
    }

    #[test]
    fn test_buffer_descriptor() {
        let descriptor = GpuBufferDescriptor::new(1024, GpuBufferType::Vertex)
            .with_label("test_buffer");

        assert_eq!(descriptor.size, 1024);
        assert_eq!(descriptor.buffer_type, GpuBufferType::Vertex);
        assert_eq!(descriptor.label, Some("test_buffer".to_string()));
        assert!(descriptor.usage.vertex);
    }

    #[test]
    fn test_buffer_manager_creation() {
        let manager = GpuBufferManager::default();
        assert_eq!(manager.buffer_count(), 0);
        assert_eq!(manager.total_allocated(), 0);
    }

    #[test]
    fn test_buffer_create_destroy() {
        let manager = GpuBufferManager::default();

        let descriptor = GpuBufferDescriptor::new(1024, GpuBufferType::Vertex);
        let buffer_id = manager.create_buffer(descriptor).unwrap();

        assert_eq!(manager.buffer_count(), 1);
        assert!(manager.total_allocated() >= 1024);

        let buffer = manager.get_buffer(buffer_id).unwrap();
        assert_eq!(buffer.size(), 1024);
        assert_eq!(buffer.buffer_type(), GpuBufferType::Vertex);

        assert!(manager.destroy_buffer(buffer_id));
        assert_eq!(manager.buffer_count(), 0);
    }

    #[test]
    fn test_buffer_map_unmap() {
        let manager = GpuBufferManager::default();

        let descriptor = GpuBufferDescriptor::new(1024, GpuBufferType::Storage);
        let buffer_id = manager.create_buffer(descriptor).unwrap();

        assert!(!manager.get_buffer(buffer_id).unwrap().is_mapped());

        assert!(manager.map_buffer(buffer_id, 0, 512));
        assert!(manager.get_buffer(buffer_id).unwrap().is_mapped());

        assert!(!manager.map_buffer(buffer_id, 0, 512)); // Already mapped

        assert!(manager.unmap_buffer(buffer_id));
        assert!(!manager.get_buffer(buffer_id).unwrap().is_mapped());

        assert!(!manager.unmap_buffer(buffer_id)); // Not mapped
    }

    #[test]
    fn test_buffer_pool() {
        let manager = Arc::new(GpuBufferManager::default());
        let pool = GpuBufferPool::new(manager.clone(), 10);

        let descriptor = GpuBufferDescriptor::new(1024, GpuBufferType::Vertex);

        // Acquire buffer
        let buffer_id = pool.acquire_buffer(&descriptor).unwrap();
        assert_eq!(manager.buffer_count(), 1);

        // Release to pool
        pool.release_buffer(buffer_id);
        assert_eq!(manager.buffer_count(), 1); // Still in pool

        // Acquire again (should reuse)
        let buffer_id2 = pool.acquire_buffer(&descriptor).unwrap();
        assert_eq!(buffer_id, buffer_id2); // Same buffer reused

        pool.clear();
        assert_eq!(manager.buffer_count(), 0);
    }
}
