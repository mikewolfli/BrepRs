use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MemoryManager {
    total_allocated: AtomicUsize,
    total_freed: AtomicUsize,
    active_objects: AtomicUsize,
}

impl MemoryManager {
    pub const fn new() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            total_freed: AtomicUsize::new(0),
            active_objects: AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self, size: usize) {
        self.total_allocated.fetch_add(size, Ordering::SeqCst);
        self.active_objects.fetch_add(1, Ordering::SeqCst);
    }

    pub fn free(&self, size: usize) {
        self.total_freed.fetch_add(size, Ordering::SeqCst);
        self.active_objects.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated.load(Ordering::SeqCst)
    }

    pub fn total_freed(&self) -> usize {
        self.total_freed.load(Ordering::SeqCst)
    }

    pub fn active_objects(&self) -> usize {
        self.active_objects.load(Ordering::SeqCst)
    }

    pub fn current_usage(&self) -> usize {
        self.total_allocated() - self.total_freed()
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

pub static MEMORY_MANAGER: MemoryManager = MemoryManager::new();

pub fn track_allocation<T>(obj: T) -> T {
    MEMORY_MANAGER.allocate(std::mem::size_of::<T>());
    obj
}

pub fn track_deallocation<T>(obj: T) {
    MEMORY_MANAGER.free(std::mem::size_of::<T>());
    std::mem::forget(obj);
}

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
        let stats = get_memory_stats();
        assert!(stats.total_allocated >= 0);
        assert!(stats.total_freed >= 0);
        assert!(stats.active_objects >= 0);
    }

    #[test]
    fn test_current_usage() {
        let initial_usage = MEMORY_MANAGER.current_usage();

        let _obj = track_allocation(vec![1, 2, 3]);
        let new_usage = MEMORY_MANAGER.current_usage();
        assert!(new_usage > initial_usage);
    }
}
