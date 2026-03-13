//! Parallel Processing Module
//!
//! This module provides parallel processing capabilities for geometric algorithms
//! using the Rayon library. It enables multi-threaded execution of computationally
//! intensive operations like boolean operations, mesh generation, and shape analysis.
//!
//! # Examples
//!
//! ```
//! use breprs::parallel::ParallelShapeProcessor;
//! use breprs::topology::topods_solid::TopoDsSolid;
//!
//! let solids: Vec<TopoDsSolid> = vec![...];
//! let processor = ParallelShapeProcessor::new();
//!
//! // Process solids in parallel
//! let results = processor.process_solids(&solids, |solid| {
//!     // Perform some operation on each solid
//!     solid.compute_volume()
//! });
//! ```

#[cfg(feature = "rayon")]
pub mod boolean_ops;
#[cfg(feature = "rayon")]
pub mod mesh_gen;
#[cfg(feature = "rayon")]
pub mod shape_analysis;
#[cfg(feature = "rayon")]
pub mod utils;
#[cfg(feature = "rayon")]
pub mod task_scheduler;
#[cfg(feature = "rayon")]
pub mod geometry_algorithms;

#[cfg(feature = "rayon")]
pub use boolean_ops::*;
#[cfg(feature = "rayon")]
pub use mesh_gen::*;
#[cfg(feature = "rayon")]
pub use shape_analysis::*;
#[cfg(feature = "rayon")]
// pub use utils::*; // 禁用歧义导出，建议显式导出所需项
#[cfg(feature = "rayon")]
pub use task_scheduler::*;
#[cfg(feature = "rayon")]
pub use geometry_algorithms::*;

use crate::foundation::handle::Handle;
use crate::topology::{topods_shape::TopoDsShape, topods_solid::TopoDsSolid, ShapeType};

/// Configuration for parallel processing
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Number of threads to use (None = use all available)
    pub num_threads: Option<usize>,
    /// Minimum number of items to process in parallel
    pub min_parallel_size: usize,
    /// Whether to use work stealing
    pub use_work_stealing: bool,
    /// Chunk size for parallel iteration
    pub chunk_size: Option<usize>,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            num_threads: None,
            min_parallel_size: 100,
            use_work_stealing: true,
            chunk_size: None,
        }
    }
}

impl ParallelConfig {
    /// Create a new configuration with specific thread count
    pub fn with_threads(num_threads: usize) -> Self {
        Self {
            num_threads: Some(num_threads),
            ..Default::default()
        }
    }

    /// Set minimum parallel size
    pub fn with_min_size(mut self, size: usize) -> Self {
        self.min_parallel_size = size;
        self
    }

    /// Disable work stealing
    pub fn without_work_stealing(mut self) -> Self {
        self.use_work_stealing = false;
        self
    }

    /// Set chunk size
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = Some(size);
        self
    }

    /// Initialize the thread pool with this configuration
    #[cfg(feature = "rayon")]
    pub fn init_thread_pool(&self) -> Result<(), rayon::ThreadPoolBuildError> {
        let mut builder = rayon::ThreadPoolBuilder::new();

        if let Some(num_threads) = self.num_threads {
            builder = builder.num_threads(num_threads);
        }

        if !self.use_work_stealing {
            // Note: Rayon always uses work stealing, this is for documentation
        }

        builder.build_global()
    }
}

/// Statistics for parallel operations
#[derive(Debug, Clone, Default)]
pub struct ParallelStats {
    pub items_processed: usize,
    pub threads_used: usize,
    pub elapsed_time_ms: u64,
    pub speedup_vs_sequential: f64,
}

impl ParallelStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items_processed(mut self, count: usize) -> Self {
        self.items_processed = count;
        self
    }

    pub fn with_threads_used(mut self, count: usize) -> Self {
        self.threads_used = count;
        self
    }

    pub fn with_threads_processed(mut self, count: usize) -> Self {
        self.threads_used = count;
        self
    }

    pub fn with_elapsed_time_ms(mut self, ms: u64) -> Self {
        self.elapsed_time_ms = ms;
        self
    }

    pub fn with_speedup(mut self, speedup: f64) -> Self {
        self.speedup_vs_sequential = speedup;
        self
    }
}

/// Result of a parallel operation
#[derive(Debug, Clone)]
pub struct ParallelResult<T> {
    pub data: T,
    pub stats: ParallelStats,
}

impl<T> ParallelResult<T> {
    pub fn new(data: T, stats: ParallelStats) -> Self {
        Self { data, stats }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ParallelResult<U> {
        ParallelResult {
            data: f(self.data),
            stats: self.stats,
        }
    }
}

/// Parallel shape processor
#[cfg(feature = "rayon")]
pub struct ParallelShapeProcessor {
    config: ParallelConfig,
}

#[cfg(feature = "rayon")]
impl ParallelShapeProcessor {
    pub fn new() -> Self {
        Self {
            config: ParallelConfig::default(),
        }
    }

    pub fn with_config(config: ParallelConfig) -> Self {
        Self { config }
    }

    /// Process a collection of shapes in parallel
    pub fn process_shapes<T, F, R>(
        &self,
        shapes: &[Handle<TopoDsShape>],
        f: F,
    ) -> ParallelResult<Vec<R>>
    where
        F: Fn(&Handle<TopoDsShape>) -> R + Sync + Send,
        R: Send,
    {
        use rayon::prelude::*;
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<R> = if shapes.len() >= self.config.min_parallel_size {
            shapes.par_iter().map(f).collect()
        } else {
            shapes.iter().map(f).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Process solids in parallel
    pub fn process_solids<T, F, R>(&self, solids: &[TopoDsSolid], f: F) -> ParallelResult<Vec<R>>
    where
        F: Fn(&TopoDsSolid) -> R + Sync + Send,
        R: Send,
    {
        use rayon::prelude::*;
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<R> = if solids.len() >= self.config.min_parallel_size {
            solids.par_iter().map(f).collect()
        } else {
            solids.iter().map(f).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(solids.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Filter shapes in parallel
    pub fn filter_shapes<F>(
        &self,
        shapes: &[Handle<TopoDsShape>],
        f: F,
    ) -> ParallelResult<Vec<Handle<TopoDsShape>>>
    where
        F: Fn(&Handle<TopoDsShape>) -> bool + Sync + Send,
    {
        use rayon::prelude::*;
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<Handle<TopoDsShape>> = if shapes.len() >= self.config.min_parallel_size {
            shapes.par_iter().filter(|s| f(s)).cloned().collect()
        } else {
            shapes.iter().filter(|s| f(s)).cloned().collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Partition shapes by type in parallel
    pub fn partition_by_type(
        &self,
        shapes: &[Handle<TopoDsShape>],
    ) -> ParallelResult<Vec<(ShapeType, Vec<Handle<TopoDsShape>>)>> {
        use rayon::prelude::*;
        use std::collections::HashMap;
        use std::time::Instant;

        let start = Instant::now();

        // Group by shape type
        let mut groups: HashMap<ShapeType, Vec<Handle<TopoDsShape>>> = HashMap::new();

        if shapes.len() >= self.config.min_parallel_size {
            // Parallel grouping using fold/reduce pattern
            groups = shapes
                .par_iter()
                .fold(
                    || HashMap::new(),
                    |mut acc: HashMap<ShapeType, Vec<Handle<TopoDsShape>>>,
                     shape: &Handle<TopoDsShape>| {
                        let shape_type = shape.shape_type();
                        acc.entry(shape_type).or_default().push(shape.clone());
                        acc
                    },
                )
                .reduce(
                    || HashMap::new(),
                    |mut a: HashMap<ShapeType, Vec<Handle<TopoDsShape>>>,
                     b: HashMap<ShapeType, Vec<Handle<TopoDsShape>>>| {
                        for (k, v) in b {
                            a.entry(k).or_default().extend(v);
                        }
                        a
                    },
                );
        } else {
            for shape in shapes {
                let shape_type = shape.shape_type();
                groups.entry(shape_type).or_default().push(shape.clone());
            }
        }

        let result: Vec<(ShapeType, Vec<Handle<TopoDsShape>>)> = groups.into_iter().collect();

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(result, stats)
    }
}

#[cfg(feature = "rayon")]
impl Default for ParallelShapeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Parallel iterator extensions for shapes
#[cfg(feature = "rayon")]
pub trait ParallelShapeIterator: Iterator + Sized {
    /// Process items in parallel when collection is large enough
    fn par_process<F, R>(self, min_size: usize, f: F) -> Vec<R>
    where
        F: Fn(Self::Item) -> R + Sync + Send,
        R: Send,
        Self::Item: Send,
    {
        use rayon::prelude::*;

        let vec: Vec<Self::Item> = self.collect();
        if vec.len() >= min_size {
            vec.into_par_iter().map(f).collect()
        } else {
            vec.into_iter().map(f).collect()
        }
    }
}

#[cfg(feature = "rayon")]
impl<T: Iterator> ParallelShapeIterator for T {}

/// Benchmark sequential vs parallel execution
#[cfg(feature = "rayon")]
pub fn benchmark_parallel<T, F, R>(items: &[T], f: F) -> (Vec<R>, Vec<R>, f64)
where
    F: Fn(&T) -> R + Sync + Send + Clone,
    R: Send,
    T: Sync,
{
    use rayon::prelude::*;
    use std::time::Instant;

    // Sequential execution
    let start = Instant::now();
    let sequential: Vec<R> = items.iter().map(|item| f(item)).collect();
    let seq_time = start.elapsed();

    // Parallel execution
    let start = Instant::now();
    let parallel: Vec<R> = items.par_iter().map(|item| f(item)).collect();
    let par_time = start.elapsed();

    let speedup = seq_time.as_secs_f64() / par_time.as_secs_f64();

    (sequential, parallel, speedup)
}

/// Check if parallel execution would be beneficial
#[cfg(feature = "rayon")]
pub fn should_use_parallel(item_count: usize, min_size: usize) -> bool {
    item_count >= min_size && rayon::current_num_threads() > 1
}

/// Get current thread pool information
#[cfg(feature = "rayon")]
pub fn thread_pool_info() -> ThreadPoolInfo {
    ThreadPoolInfo {
        num_threads: rayon::current_num_threads(),
        max_threads: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
    }
}

/// Thread pool information
#[derive(Debug, Clone)]
pub struct ThreadPoolInfo {
    pub num_threads: usize,
    pub max_threads: usize,
}

impl ThreadPoolInfo {
    pub fn utilization_ratio(&self) -> f64 {
        self.num_threads as f64 / self.max_threads as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_config_default() {
        let config = ParallelConfig::default();
        assert!(config.num_threads.is_none());
        assert_eq!(config.min_parallel_size, 100);
        assert!(config.use_work_stealing);
    }

    #[test]
    fn test_parallel_config_with_threads() {
        let config = ParallelConfig::with_threads(4);
        assert_eq!(config.num_threads, Some(4));
    }

    #[test]
    fn test_parallel_stats() {
        let stats = ParallelStats::new()
            .with_items_processed(1000)
            .with_threads_used(4)
            .with_elapsed_time_ms(100)
            .with_speedup(3.5);

        assert_eq!(stats.items_processed, 1000);
        assert_eq!(stats.threads_used, 4);
        assert_eq!(stats.elapsed_time_ms, 100);
        assert!((stats.speedup_vs_sequential - 3.5).abs() < 1e-10);
    }

    #[test]
    fn test_should_use_parallel() {
        assert!(!should_use_parallel(50, 100));
        assert!(should_use_parallel(200, 100));
    }

    #[test]
    fn test_thread_pool_info() {
        let info = thread_pool_info();
        assert!(info.num_threads > 0);
        assert!(info.max_threads > 0);
        assert!(info.utilization_ratio() > 0.0);
    }
}
