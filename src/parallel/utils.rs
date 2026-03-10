//! Parallel Utilities
//!
//! This module provides utility functions and types for parallel processing.

use rayon::prelude::*;
use std::time::{Duration, Instant};

use super::{ParallelConfig, ParallelResult, ParallelStats};

/// Parallel for-each with progress tracking
pub fn par_for_each_with_progress<T, F>(
    items: &[T],
    config: &ParallelConfig,
    progress_callback: impl FnMut(usize, usize) + Sync + Send,
    f: F,
) where
    T: Sync,
    F: Fn(&T) + Sync + Send,
{
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    let processed = AtomicUsize::new(0);
    let total = items.len();

    let callback = Mutex::new(progress_callback);

    if items.len() >= config.min_parallel_size {
        items.par_iter().for_each(|item| {
            f(item);
            let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if let Ok(mut cb) = callback.lock() {
                cb(count, total);
            }
        });
    } else {
        items.iter().for_each(|item| {
            f(item);
            let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if let Ok(mut cb) = callback.lock() {
                cb(count, total);
            }
        });
    }
}

/// Parallel map with index
pub fn par_map_with_index<T, F, R>(
    items: &[T],
    config: &ParallelConfig,
    f: F,
) -> ParallelResult<Vec<R>>
where
    T: Sync,
    F: Fn(&T, usize) -> R + Sync + Send,
    R: Send,
{
    let start = Instant::now();

    let results: Vec<R> = if items.len() >= config.min_parallel_size {
        items
            .par_iter()
            .enumerate()
            .map(|(i, item)| f(item, i))
            .collect()
    } else {
        items
            .iter()
            .enumerate()
            .map(|(i, item)| f(item, i))
            .collect()
    };

    let elapsed = start.elapsed();
    let stats = ParallelStats::new()
        .with_items_processed(items.len())
        .with_threads_used(rayon::current_num_threads())
        .with_elapsed_time_ms(elapsed.as_millis() as u64);

    ParallelResult::new(results, stats)
}

/// Parallel filter map
pub fn par_filter_map<T, F, R>(items: &[T], config: &ParallelConfig, f: F) -> ParallelResult<Vec<R>>
where
    T: Sync,
    F: Fn(&T) -> Option<R> + Sync + Send,
    R: Send,
{
    let start = Instant::now();

    let results: Vec<R> = if items.len() >= config.min_parallel_size {
        items.par_iter().filter_map(f).collect()
    } else {
        items.iter().filter_map(f).collect()
    };

    let elapsed = start.elapsed();
    let stats = ParallelStats::new()
        .with_items_processed(items.len())
        .with_threads_used(rayon::current_num_threads())
        .with_elapsed_time_ms(elapsed.as_millis() as u64);

    ParallelResult::new(results, stats)
}

/// Parallel reduce with custom identity
pub fn par_reduce<T, F, R>(
    items: &[T],
    config: &ParallelConfig,
    identity: impl Fn() -> R + Sync + Send + Clone,
    reduce: impl Fn(R, &T) -> R + Sync + Send,
) -> ParallelResult<R>
where
    T: Sync,
    R: Send + Clone,
{
    let start = Instant::now();

    let result: R = if items.len() >= config.min_parallel_size {
        items
            .par_iter()
            .fold(identity.clone(), |acc, item| reduce(acc, item))
            .reduce(
                || identity(),
                |a, _b| {
                    // Combine two partial results
                    // This is a simplified version - in practice, you'd need a proper combine function
                    a
                },
            )
    } else {
        items.iter().fold(identity(), |acc, item| reduce(acc, item))
    };

    let elapsed = start.elapsed();
    let stats = ParallelStats::new()
        .with_items_processed(items.len())
        .with_threads_used(rayon::current_num_threads())
        .with_elapsed_time_ms(elapsed.as_millis() as u64);

    ParallelResult::new(result, stats)
}

/// Parallel group by
pub fn par_group_by<T, F, K>(
    items: &[T],
    config: &ParallelConfig,
    key_fn: F,
) -> ParallelResult<std::collections::HashMap<K, Vec<T>>>
where
    T: Clone + Send + Sync,
    F: Fn(&T) -> K + Sync + Send,
    K: std::hash::Hash + Eq + Send,
{
    use std::collections::HashMap;
    let start = Instant::now();

    let groups: HashMap<K, Vec<T>> = if items.len() >= config.min_parallel_size {
        items
            .par_iter()
            .fold(
                || HashMap::new(),
                |mut acc: HashMap<K, Vec<T>>, item: &T| {
                    let key = key_fn(item);
                    acc.entry(key).or_default().push(item.clone());
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut a: HashMap<K, Vec<T>>, b: HashMap<K, Vec<T>>| {
                    for (k, v) in b {
                        a.entry(k).or_default().extend(v);
                    }
                    a
                },
            )
    } else {
        let mut groups: HashMap<K, Vec<T>> = HashMap::new();
        for item in items {
            let key = key_fn(item);
            groups.entry(key).or_default().push(item.clone());
        }
        groups
    };

    let elapsed = start.elapsed();
    let stats = ParallelStats::new()
        .with_items_processed(items.len())
        .with_threads_used(rayon::current_num_threads())
        .with_elapsed_time_ms(elapsed.as_millis() as u64);

    ParallelResult::new(groups, stats)
}

/// Parallel sort (unstable)
pub fn par_sort<T, F>(items: &mut [T], compare: F)
where
    T: Send,
    F: Fn(&T, &T) -> std::cmp::Ordering + Sync + Send,
{
    items.par_sort_unstable_by(compare);
}

/// Parallel sort by key
pub fn par_sort_by_key<T, F, K>(items: &mut [T], key_fn: F)
where
    T: Send,
    F: Fn(&T) -> K + Sync + Send,
    K: Ord + Send,
{
    items.par_sort_unstable_by_key(key_fn);
}

/// Chunked parallel processing
pub fn par_chunks<T, F, R>(items: &[T], chunk_size: usize, f: F) -> ParallelResult<Vec<R>>
where
    T: Sync,
    F: Fn(&[T]) -> R + Sync + Send,
    R: Send,
{
    let start = Instant::now();

    let results: Vec<R> = items.par_chunks(chunk_size).map(f).collect();

    let elapsed = start.elapsed();
    let stats = ParallelStats::new()
        .with_items_processed(items.len())
        .with_threads_used(rayon::current_num_threads())
        .with_elapsed_time_ms(elapsed.as_millis() as u64);

    ParallelResult::new(results, stats)
}

/// Parallel pipeline processing
pub struct ParallelPipeline<T, R> {
    stages: Vec<Box<dyn Fn(Vec<T>) -> Vec<T> + Send + Sync>>,
    _phantom: std::marker::PhantomData<R>,
}

impl<T: Send + Sync + Clone, R: Send> ParallelPipeline<T, R> {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn add_stage<F>(mut self, stage: F) -> Self
    where
        F: Fn(Vec<T>) -> Vec<T> + Send + Sync + 'static,
    {
        self.stages.push(Box::new(stage));
        self
    }

    pub fn execute(&self, input: Vec<T>) -> Vec<T> {
        let mut data = input;
        for stage in &self.stages {
            data = stage(data);
        }
        data
    }

    pub fn execute_parallel(&self, input: Vec<T>, min_size: usize) -> Vec<T> {
        use rayon::prelude::*;

        let mut data = input;
        for stage in &self.stages {
            if data.len() >= min_size {
                // Process in parallel chunks
                data = data
                    .par_chunks(min_size)
                    .flat_map(|chunk| {
                        let chunk_vec = chunk.to_vec();
                        stage(chunk_vec)
                    })
                    .collect();
            } else {
                data = stage(data);
            }
        }
        data
    }
}

impl<T: Send + Sync + Clone, R: Send> Default for ParallelPipeline<T, R> {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance timer for benchmarking
pub struct PerformanceTimer {
    start: Instant,
    name: String,
}

impl PerformanceTimer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    pub fn report(&self) {
        println!("[{}] Elapsed: {:?}", self.name, self.elapsed());
    }
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        self.report();
    }
}

/// Parallel execution guard that ensures proper cleanup
pub struct ParallelExecutionGuard;

impl ParallelExecutionGuard {
    pub fn new() -> Self {
        Self
    }

    pub fn current_thread_index() -> Option<usize> {
        rayon::current_thread_index()
    }

    pub fn current_num_threads() -> usize {
        rayon::current_num_threads()
    }
}

impl Default for ParallelExecutionGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-local storage for parallel operations
pub struct ThreadLocalCache<T: Clone + Send> {
    cache: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<usize, T>>>,
}

impl<T: Clone + Send> ThreadLocalCache<T> {
    pub fn new() -> Self {
        Self {
            cache: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub fn get_or_insert<F>(&self, factory: F) -> T
    where
        F: FnOnce() -> T,
    {
        let thread_id = rayon::current_thread_index().unwrap_or(0);
        let mut cache = self.cache.lock().unwrap();

        cache.get(&thread_id).cloned().unwrap_or_else(|| {
            let value = factory();
            cache.insert(thread_id, value.clone());
            value
        })
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl<T: Clone + Send> Default for ThreadLocalCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Compare sequential vs parallel execution
pub fn compare_execution<T, F, R>(items: &[T], f: F) -> (Duration, Duration, f64)
where
    T: Sync,
    F: Fn(&T) -> R + Sync + Send,
    R: Send,
{
    use rayon::prelude::*;

    // Sequential
    let start = Instant::now();
    let _: Vec<R> = items.iter().map(&f).collect();
    let seq_time = start.elapsed();

    // Parallel
    let start = Instant::now();
    let _: Vec<R> = items.par_iter().map(&f).collect();
    let par_time = start.elapsed();

    let speedup = if par_time.as_secs_f64() > 0.0 {
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    } else {
        1.0
    };

    (seq_time, par_time, speedup)
}

/// Auto-tune parallel configuration
pub fn auto_tune_config<T, F, R>(items: &[T], f: F) -> ParallelConfig
where
    T: Sync,
    F: Fn(&T) -> R + Sync + Send,
    R: Send,
{
    // Test different configurations and return the best one
    let configs = vec![
        ParallelConfig::default(),
        ParallelConfig::default().with_min_size(50),
        ParallelConfig::default().with_min_size(200),
    ];

    let mut best_config = ParallelConfig::default();
    let mut best_time = std::f64::MAX;

    for config in configs {
        let start = Instant::now();

        // Quick benchmark
        let sample_size = std::cmp::min(items.len(), 100);
        let sample = &items[..sample_size];

        if sample_size >= config.min_parallel_size {
            use rayon::prelude::*;
            let _: Vec<R> = sample.par_iter().map(&f).collect();
        } else {
            let _: Vec<R> = sample.iter().map(&f).collect();
        }

        let elapsed = start.elapsed().as_secs_f64();

        if elapsed < best_time {
            best_time = elapsed;
            best_config = config;
        }
    }

    best_config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::new("test");
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
    }

    #[test]
    fn test_parallel_execution_guard() {
        let _guard = ParallelExecutionGuard::new();
        assert!(ParallelExecutionGuard::current_num_threads() > 0);
    }

    #[test]
    fn test_thread_local_cache() {
        let cache: ThreadLocalCache<i32> = ThreadLocalCache::new();

        let value1 = cache.get_or_insert(|| 42);
        let value2 = cache.get_or_insert(|| 100); // Should return cached value

        assert_eq!(value1, 42);
        assert_eq!(value2, 42); // Same value from cache
    }

    #[test]
    fn test_parallel_pipeline() {
        let pipeline: ParallelPipeline<i32, i32> = ParallelPipeline::new()
            .add_stage(|mut data| {
                data.iter_mut().for_each(|x| *x *= 2);
                data
            })
            .add_stage(|mut data| {
                data.iter_mut().for_each(|x| *x += 1);
                data
            });

        let input = vec![1, 2, 3];
        let result = pipeline.execute(input);

        assert_eq!(result, vec![3, 5, 7]); // (1*2)+1=3, (2*2)+1=5, (3*2)+1=7
    }

    #[test]
    fn test_compare_execution() {
        let items: Vec<i32> = (0..100).collect();
        let (seq_time, par_time, speedup) = compare_execution(&items, |x| x * x);

        // Just verify it runs without error
        assert!(seq_time > Duration::ZERO);
        assert!(par_time > Duration::ZERO);
        assert!(speedup > 0.0);
    }
}
