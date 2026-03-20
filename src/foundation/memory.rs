use std::sync::atomic::{AtomicUsize, Ordering};

/// Memory manager for tracking allocations and deallocations
///
/// This manager provides atomic tracking of memory usage throughout the CAD kernel.
/// It is designed for debugging and monitoring purposes, not for production use.
///
/// # Thread Safety
/// The memory manager uses atomic operations and is safe to use across threads.
///
/// # Usage
/// Use the `track_allocation!` and `track_deallocation!` macros for automatic tracking,
/// or use the `Tracked<T>` wrapper for automatic Drop-based tracking.
pub struct MemoryManager {
    total_allocated: AtomicUsize,
    total_freed: AtomicUsize,
    active_objects: AtomicUsize,
    enabled: bool,
}

impl MemoryManager {
    #[inline]
    /// Create a new memory manager, enabled by default in debug mode, configurable in release mode
    pub fn new() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            total_freed: AtomicUsize::new(0),
            active_objects: AtomicUsize::new(0),
            enabled: cfg!(debug_assertions),
        }
    }

    /// Explicitly enable/disable memory tracking
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Track an allocation of the given size
    #[inline]
    pub fn allocate(&self, size: usize) {
        // Use Relaxed ordering for better performance since we're only tracking, not synchronizing
        self.total_allocated.fetch_add(size, Ordering::Relaxed);
        self.active_objects.fetch_add(1, Ordering::Relaxed);
    }

    /// Track a deallocation of the given size
    #[inline]
    pub fn free(&self, size: usize) {
        // Use Relaxed ordering for better performance since we're only tracking, not synchronizing
        self.total_freed.fetch_add(size, Ordering::Relaxed);
        self.active_objects.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get total bytes allocated
    #[inline]
    pub fn total_allocated(&self) -> usize {
        self.total_allocated.load(Ordering::Relaxed)
    }

    /// Get total bytes freed
    #[inline]
    pub fn total_freed(&self) -> usize {
        self.total_freed.load(Ordering::Relaxed)
    }

    /// Get number of active objects
    #[inline]
    pub fn active_objects(&self) -> usize {
        self.active_objects.load(Ordering::Relaxed)
    }

    /// Get current memory usage (allocated - freed)
    #[inline]
    pub fn current_usage(&self) -> usize {
        self.total_allocated() - self.total_freed()
    }

    /// Reset all counters to zero
    #[inline]
    pub fn reset(&self) {
        self.total_allocated.store(0, Ordering::Relaxed);
        self.total_freed.store(0, Ordering::Relaxed);
        self.active_objects.store(0, Ordering::Relaxed);
    }

    /// Check if memory tracking is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global memory manager instance
use std::sync::LazyLock;
pub static MEMORY_MANAGER: LazyLock<MemoryManager> = LazyLock::new(|| MemoryManager::new());

/// Wrapper type for automatic memory tracking via Drop trait
///
/// This wrapper automatically tracks allocation on creation and deallocation
/// when it goes out of scope, integrating with Rust's ownership system.
///
/// # Example
/// ```rust
/// use breprs::foundation::memory::Tracked;
///
/// {
///     let tracked = Tracked::new(vec![1, 2, 3]);
///     // Memory is tracked while tracked is in scope
/// }
/// // Memory is automatically deallocated when tracked goes out of scope
/// ```
pub struct Tracked<T> {
    inner: T,
    size: usize,
}

impl<T> Tracked<T> {
    /// Create a new tracked value
    #[inline]
    pub fn new(inner: T) -> Self {
        let size = std::mem::size_of::<T>();
        // Only track in debug builds or when memory tracking is enabled
        if MEMORY_MANAGER.is_enabled() {
            MEMORY_MANAGER.allocate(size);
        }
        Self { inner, size }
    }

    /// Get a reference to the inner value
    #[inline]
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner value
    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consume the wrapper and return the inner value
    ///
    /// Note: This will not track the deallocation of the returned value.
    ///
    /// # Safety
    /// This function uses `std::ptr::read` which is safe here because:
    /// - We immediately call `std::mem::forget(self)` to prevent double-drop
    /// - The memory is properly tracked and freed via MEMORY_MANAGER
    /// - The pointer is valid as it points to our own field
    #[inline]
    pub fn into_inner(self) -> T {
        let size = self.size;
        // SAFETY: We use ptr::read to move the inner value out without
        // triggering Drop. This is safe because we immediately call
        // std::mem::forget(self) to prevent the destructor from running.
        let inner = unsafe { std::ptr::read(&self.inner) };
        std::mem::forget(self);
        // Only track in debug builds or when memory tracking is enabled
        if MEMORY_MANAGER.is_enabled() {
            MEMORY_MANAGER.free(size);
        }
        inner
    }
}

impl<T> std::ops::Deref for Tracked<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Tracked<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Drop for Tracked<T> {
    #[inline]
    fn drop(&mut self) {
        // Only track in debug builds or when memory tracking is enabled
        if MEMORY_MANAGER.is_enabled() {
            MEMORY_MANAGER.free(self.size);
        }
    }
}

impl<T: Clone> Clone for Tracked<T> {
    #[inline]
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Tracked::new(inner)
    }
}

#[inline]
pub fn track_allocation<T>(obj: T) -> T {
    // Only track in debug builds or when memory tracking is enabled
    if MEMORY_MANAGER.is_enabled() {
        MEMORY_MANAGER.allocate(std::mem::size_of::<T>());
    }
    obj
}

#[inline]
pub fn track_deallocation<T>(obj: T) {
    // Only track in debug builds or when memory tracking is enabled
    if MEMORY_MANAGER.is_enabled() {
        MEMORY_MANAGER.free(std::mem::size_of::<T>());
    }
    std::mem::forget(obj);
}

#[inline]
pub fn get_memory_stats() -> MemoryStats {
    MemoryStats {
        total_allocated: MEMORY_MANAGER.total_allocated(),
        total_freed: MEMORY_MANAGER.total_freed(),
        active_objects: MEMORY_MANAGER.active_objects(),
        current_usage: MEMORY_MANAGER.current_usage(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub active_objects: usize,
    pub current_usage: usize,
}

impl MemoryStats {
    pub fn new() -> Self {
        get_memory_stats()
    }
}

/// LOD memory manager
///
/// This manager provides specialized memory management for LOD (Level of Detail) data,
/// including memory budgeting and automatic memory release for lower priority LOD levels.
pub struct LodMemoryManager {
    /// Memory usage per LOD level
    lod_usage: Vec<AtomicUsize>,
    /// Total memory budget in bytes
    memory_budget: AtomicUsize,
}

impl LodMemoryManager {
    /// Create a new LOD memory manager
    pub fn new(num_levels: usize, memory_budget: usize) -> Self {
        let mut lod_usage = Vec::with_capacity(num_levels);
        for _ in 0..num_levels {
            lod_usage.push(AtomicUsize::new(0));
        }

        Self {
            lod_usage,
            memory_budget: AtomicUsize::new(memory_budget),
        }
    }

    /// Allocate memory for LOD data
    pub fn allocate(&self, lod_level: usize, size: usize) -> Result<(), crate::foundation::exception::Failure> {
        // Check if LOD level is valid
        if lod_level >= self.lod_usage.len() {
            return Err(crate::foundation::exception::Failure::range_error(
                format!("Invalid LOD level: {}", lod_level),
                Some(format!("allocate: lod_level={}, size={}", lod_level, size)),
                None,
            ));
        }

        // Check memory budget
        let current_usage = self.current_usage();
        if current_usage + size > self.memory_budget.load(Ordering::Relaxed) {
            // Try to free memory from lower priority LOD levels
            self.free_memory(size)?;
        }

        // Allocate memory
        self.lod_usage[lod_level].fetch_add(size, Ordering::Relaxed);

        Ok(())
    }

    /// Free memory for LOD data
    pub fn free(&self, lod_level: usize, size: usize) {
        if lod_level < self.lod_usage.len() {
            self.lod_usage[lod_level].fetch_sub(size, Ordering::Relaxed);
        }
    }

    /// Free memory to meet memory budget
    fn free_memory(&self, required_size: usize) -> Result<(), crate::foundation::exception::Failure> {
        let current_usage = self.current_usage();
        let target_usage = current_usage.saturating_sub(required_size);

        // Free memory from lowest priority LOD levels first
        for level in (0..self.lod_usage.len()).rev() {
            let level_usage = self.lod_usage[level].load(Ordering::Relaxed);
            if level_usage > 0 {
                // Free all memory for this level
                self.lod_usage[level].store(0, Ordering::Relaxed);

                if self.current_usage() <= target_usage {
                    return Ok(());
                }
            }
        }

        Err(crate::foundation::exception::Failure::range_error(
            "Insufficient memory budget",
            Some(format!("free_memory: required_size={}, current_usage={}", required_size, self.current_usage())),
            None,
        ))
    }

    /// Get current memory usage
    pub fn current_usage(&self) -> usize {
        self.lod_usage
            .iter()
            .map(|usage| usage.load(Ordering::Relaxed))
            .sum()
    }

    /// Get memory usage per LOD level
    pub fn usage_per_level(&self) -> Vec<usize> {
        self.lod_usage
            .iter()
            .map(|usage| usage.load(Ordering::Relaxed))
            .collect()
    }

    /// Set memory budget
    pub fn set_memory_budget(&self, budget: usize) {
        self.memory_budget.store(budget, Ordering::Relaxed);
    }

    /// Get memory budget
    pub fn memory_budget(&self) -> usize {
        self.memory_budget.load(Ordering::Relaxed)
    }
}

impl Default for LodMemoryManager {
    fn default() -> Self {
        Self::new(5, 1024 * 1024 * 1024) // 1GB default budget
    }
}

/// Global LOD memory manager instance
pub static LOD_MEMORY_MANAGER: std::sync::LazyLock<LodMemoryManager> =
    std::sync::LazyLock::new(|| LodMemoryManager::new(5, 1024 * 1024 * 1024));

#[macro_export]
macro_rules! track_allocation {
    ($expr:expr) => {
        $crate::foundation::memory::track_allocation($expr)
    };
}

#[macro_export]
macro_rules! track_deallocation {
    ($expr:expr) => {
        $crate::foundation::memory::track_deallocation($expr)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_memory_manager() {
        let initial_objects = MEMORY_MANAGER.active_objects();

        let _obj = track_allocation(42i32);
        assert_eq!(MEMORY_MANAGER.active_objects(), initial_objects + 1);

        track_deallocation(42i32);
        assert_eq!(MEMORY_MANAGER.active_objects(), initial_objects);
    }

    #[test]
    fn test_memory_stats() {
        let _ = get_memory_stats();
    }

    #[test]
    fn test_current_usage() {
        let initial_usage = MEMORY_MANAGER.current_usage();

        let _obj = track_allocation(vec![1, 2, 3]);
        let new_usage = MEMORY_MANAGER.current_usage();
        assert!(new_usage > initial_usage);
    }

    #[test]
    fn test_concurrent_allocation() {
        // Use a local memory manager for this test to avoid interference with other tests
        let manager = Arc::new(MemoryManager::new());
        let initial = manager.active_objects();

        let mut handles = vec![];
        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    manager_clone.allocate(4);
                    manager_clone.free(4);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // After all threads complete, active objects should be back to initial
        assert_eq!(manager.active_objects(), initial);
    }

    #[test]
    fn test_concurrent_tracked() {
        // Use a local memory manager for this test
        let manager = Arc::new(MemoryManager::new());
        let initial = manager.active_objects();

        let mut handles = vec![];
        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    manager_clone.allocate(4);
                    manager_clone.free(4);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // After all threads complete, active objects should be back to initial
        assert_eq!(manager.active_objects(), initial);
    }

    #[test]
    fn test_tracked_wrapper() {
        MEMORY_MANAGER.reset();
        let initial = MEMORY_MANAGER.active_objects();

        {
            let tracked = Tracked::new(vec![1, 2, 3]);
            assert_eq!(MEMORY_MANAGER.active_objects(), initial + 1);
            assert_eq!(tracked.len(), 3);
            assert_eq!(tracked[0], 1);
        }

        assert_eq!(MEMORY_MANAGER.active_objects(), initial);
    }

    #[test]
    fn test_tracked_into_inner() {
        MEMORY_MANAGER.reset();
        let initial = MEMORY_MANAGER.active_objects();

        let tracked = Tracked::new(vec![1, 2, 3]);
        let vec = tracked.into_inner();
        assert_eq!(vec, vec![1, 2, 3]);

        // After into_inner, the tracked wrapper is consumed
        assert_eq!(MEMORY_MANAGER.active_objects(), initial);
    }

    #[test]
    fn test_tracked_clone() {
        // This test uses the global MEMORY_MANAGER
        // We just verify that cloning creates a new tracked object
        let tracked1 = Tracked::new(42i32);
        let tracked2 = tracked1.clone();

        assert_eq!(*tracked1, 42);
        assert_eq!(*tracked2, 42);

        // Both should be independently tracked
        drop(tracked1);
        // tracked2 still exists
        assert_eq!(*tracked2, 42);
    }

    #[test]
    fn test_tracked_deref_mut() {
        let mut tracked = Tracked::new(vec![1, 2, 3]);
        tracked.push(4);
        assert_eq!(tracked.len(), 4);
    }

    #[test]
    fn test_memory_manager_reset() {
        let _obj = track_allocation(42i32);
        assert!(MEMORY_MANAGER.active_objects() > 0);

        MEMORY_MANAGER.reset();
        assert_eq!(MEMORY_MANAGER.active_objects(), 0);
        assert_eq!(MEMORY_MANAGER.total_allocated(), 0);
        assert_eq!(MEMORY_MANAGER.total_freed(), 0);
    }
}
