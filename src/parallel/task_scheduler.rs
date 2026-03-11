/// Trait for parallel tasks
pub trait ParallelTask: Send + Sync {
    type Output: Send + Sync;
    fn execute(&self) -> Self::Output;
    fn priority(&self) -> TaskPriority;
}

/// Wrapper for tasks with priority and id
#[derive(Debug, Clone)]
pub struct TaskWrapper<T: ParallelTask> {
    pub task: T,
    pub id: usize,
    pub priority: TaskPriority,
}

impl<T: ParallelTask> TaskWrapper<T> {
    pub fn new(task: T, id: usize) -> Self {
        let priority = task.priority();
        Self { task, id, priority }
    }
}
/// Task Scheduler Module
// This module provides a comprehensive task scheduling system for parallel computation,
///
/// including work stealing, load balancing, and priority-based task execution.
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use super::{ParallelConfig, ParallelResult, ParallelStats};

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}
/// Priority-based task queue
pub struct PriorityTaskQueue<T: ParallelTask> {
    pub queues: Vec<Mutex<VecDeque<TaskWrapper<T>>>>,
    pub total_tasks: AtomicUsize,
    pub completed_tasks: AtomicUsize,
}

impl<T: ParallelTask> PriorityTaskQueue<T> {
    pub fn new() -> Self {
        let mut queues = Vec::with_capacity(5);
        for _ in 0..5 {
            queues.push(Mutex::new(VecDeque::new()));
        }
        Self {
            queues,
            total_tasks: AtomicUsize::new(0),
            completed_tasks: AtomicUsize::new(0),
        }
    }

    /// Submit a task to the queue
    pub fn submit(&self, task: T) -> usize {
        let wrapper = TaskWrapper::new(task, self.total_tasks.fetch_add(1, Ordering::SeqCst));
        let priority_idx = wrapper.priority as usize;

        if let Ok(mut queue) = self.queues[priority_idx].lock() {
            queue.push_back(wrapper);
        }

        self.total_tasks.load(Ordering::SeqCst)
    }

    /// Submit multiple tasks
    pub fn submit_batch(&self, tasks: Vec<T>) -> Vec<usize> {
        tasks.into_iter().map(|task| self.submit(task)).collect()
    }

    /// Get the next task (highest priority first)
    pub fn next_task(&self) -> Option<TaskWrapper<T>> {
        for queue in &self.queues {
            if let Ok(mut q) = queue.lock() {
                if let Some(task) = q.pop_front() {
                    return Some(task);
                }
            }
        }
        None
    }

    /// Mark a task as completed
    pub fn mark_completed(&self) {
        self.completed_tasks.fetch_add(1, Ordering::SeqCst);
    }

    /// Get the number of pending tasks
    pub fn pending_count(&self) -> usize {
        let total = self.total_tasks.load(Ordering::SeqCst);
        let completed = self.completed_tasks.load(Ordering::SeqCst);
        total.saturating_sub(completed)
    }

    /// Check if all tasks are completed
    pub fn is_empty(&self) -> bool {
        self.pending_count() == 0
    }
}

/// Work stealing task scheduler
pub struct WorkStealingScheduler<T: ParallelTask> {
    queue: Arc<PriorityTaskQueue<T>>,
    config: ParallelConfig,
    worker_count: usize,
}

impl<T: ParallelTask> WorkStealingScheduler<T> {
    pub fn new(config: ParallelConfig) -> Self {
        let worker_count = config
            .num_threads
            .unwrap_or_else(rayon::current_num_threads);

        Self {
            queue: Arc::new(PriorityTaskQueue::new()),
            config,
            worker_count,
        }
    }

    /// Submit a task
    pub fn submit(&self, task: T) -> usize {
        self.queue.submit(task)
    }

    /// Submit multiple tasks
    pub fn submit_batch(&self, tasks: Vec<T>) -> Vec<usize> {
        self.queue.submit_batch(tasks)
    }

    /// Execute all tasks and collect results
    pub fn execute_all(&self) -> ParallelResult<Vec<T::Output>> {
        let start = Instant::now();
        let queue = self.queue.clone();

        // Use Rayon for parallel execution
        let results: Vec<T::Output> = (0..self.worker_count)
            .into_par_iter()
            .flat_map(|_| {
                let mut local_results = Vec::new();
                let q = queue.clone();

                while let Some(wrapper) = q.next_task() {
                    let result = wrapper.task.execute();
                    local_results.push(result);
                    q.mark_completed();
                }

                local_results
            })
            .collect();

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(results.len())
            .with_threads_used(self.worker_count)
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Execute tasks with a limit on concurrent tasks
    pub fn execute_limited(&self, max_concurrent: usize) -> ParallelResult<Vec<T::Output>> {
        let start = Instant::now();
        let queue = self.queue.clone();

        // Use a simple mutex to limit concurrency
        let active_tasks = Arc::new(AtomicUsize::new(0));

        let results: Vec<T::Output> = (0..self.worker_count)
            .into_par_iter()
            .flat_map(|_| {
                let mut local_results = Vec::new();
                let q = queue.clone();
                let active = active_tasks.clone();

                while active.load(std::sync::atomic::Ordering::Relaxed) < max_concurrent {
                    match q.next_task() {
                        Some(wrapper) => {
                            active.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            let result = wrapper.task.execute();
                            local_results.push(result);
                            q.mark_completed();
                            active.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        None => break,
                    }
                }

                local_results
            })
            .collect();

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(results.len())
            .with_threads_used(self.worker_count.min(max_concurrent))
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }
}

/// Load balancer for distributing work across threads
pub struct LoadBalancer {
    config: ParallelConfig,
    thread_loads: Vec<AtomicUsize>,
}

impl LoadBalancer {
    pub fn new(config: ParallelConfig) -> Self {
        let num_threads = config
            .num_threads
            .unwrap_or_else(rayon::current_num_threads);
        let thread_loads: Vec<AtomicUsize> =
            (0..num_threads).map(|_| AtomicUsize::new(0)).collect();

        Self {
            config,
            thread_loads,
        }
    }

    /// Get the least loaded thread
    pub fn least_loaded_thread(&self) -> usize {
        let mut min_load = usize::MAX;
        let mut min_thread = 0;

        for (i, load) in self.thread_loads.iter().enumerate() {
            let current_load = load.load(Ordering::Relaxed);
            if current_load < min_load {
                min_load = current_load;
                min_thread = i;
            }
        }

        min_thread
    }

    /// Add load to a thread
    pub fn add_load(&self, thread_id: usize, weight: usize) {
        if thread_id < self.thread_loads.len() {
            self.thread_loads[thread_id].fetch_add(weight, Ordering::Relaxed);
        }
    }

    /// Remove load from a thread
    pub fn remove_load(&self, thread_id: usize, weight: usize) {
        if thread_id < self.thread_loads.len() {
            self.thread_loads[thread_id].fetch_sub(weight, Ordering::Relaxed);
        }
    }

    /// Get current load distribution
    pub fn load_distribution(&self) -> Vec<usize> {
        self.thread_loads
            .iter()
            .map(|load| load.load(Ordering::Relaxed))
            .collect()
    }

    /// Check if load is balanced
    pub fn is_balanced(&self, threshold: f64) -> bool {
        let loads = self.load_distribution();
        if loads.is_empty() {
            return true;
        }

        let max_load = *loads.iter().max().unwrap_or(&0);
        let min_load = *loads.iter().min().unwrap_or(&0);

        if max_load == 0 {
            return true;
        }

        let imbalance = (max_load - min_load) as f64 / max_load as f64;
        imbalance <= threshold
    }
}

/// Task dependency graph for managing task dependencies
pub struct TaskDependencyGraph<T: ParallelTask + Clone> {
    tasks: RwLock<Vec<(T, Vec<usize>)>>, // (task, dependencies)
    completed: RwLock<Vec<bool>>,
}

impl<T: ParallelTask + Clone> TaskDependencyGraph<T> {
    pub fn new() -> Self {
        Self {
            tasks: RwLock::new(Vec::new()),
            completed: RwLock::new(Vec::new()),
        }
    }

    /// Add a task with dependencies
    pub fn add_task(&self, task: T, dependencies: Vec<usize>) -> usize {
        let mut tasks = self.tasks.write().unwrap();
        let id = tasks.len();
        tasks.push((task, dependencies));

        let mut completed = self.completed.write().unwrap();
        completed.push(false);

        id
    }

    /// Mark a task as completed
    pub fn mark_completed(&self, task_id: usize) {
        let mut completed = self.completed.write().unwrap();
        if task_id < completed.len() {
            completed[task_id] = true;
        }
    }

    /// Get tasks that are ready to execute (all dependencies completed)
    pub fn ready_tasks(&self) -> Vec<(usize, T)> {
        let tasks = self.tasks.read().unwrap();
        let completed = self.completed.read().unwrap();

        let mut result = Vec::new();
        for (id, (task, deps)) in tasks.iter().enumerate() {
            if !completed[id] {
                let all_deps_completed = deps
                    .iter()
                    .all(|dep| completed.get(*dep).copied().unwrap_or(false));
                if all_deps_completed {
                    result.push((id, task.clone()));
                }
            }
        }
        result
    }

    /// Check if all tasks are completed
    pub fn all_completed(&self) -> bool {
        let completed = self.completed.read().unwrap();
        completed.iter().all(|&c| c)
    }
}

/// Parallel pipeline for streaming data processing
pub struct ParallelPipeline<T> {
    stages: Vec<Box<dyn Fn(T) -> T + Send + Sync>>,
    config: ParallelConfig,
}

impl<T: Send + Sync + Clone> ParallelPipeline<T> {
    pub fn new(config: ParallelConfig) -> Self {
        Self {
            stages: Vec::new(),
            config,
        }
    }

    /// Add a processing stage
    pub fn add_stage<F>(&mut self, stage: F)
    where
        F: Fn(T) -> T + Send + Sync + 'static,
    {
        self.stages.push(Box::new(stage));
    }

    /// Process data through the pipeline
    pub fn process(&self, data: Vec<T>) -> ParallelResult<Vec<T>> {
        let start = Instant::now();
        let results: Vec<T> = if data.len() >= self.config.min_parallel_size {
            data.into_par_iter()
                .map(|item| {
                    let mut result = item;
                    for stage in &self.stages {
                        result = stage(result);
                    }
                    result
                })
                .collect()
        } else {
            data.into_iter()
                .map(|item| {
                    let mut result = item;
                    for stage in &self.stages {
                        result = stage(result);
                    }
                    result
                })
                .collect()
        };
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(results.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }
}

/// Performance metrics collector
pub struct PerformanceMetrics {
    operation_times: Mutex<Vec<(String, Duration)>>,
    throughput: AtomicUsize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            operation_times: Mutex::new(Vec::new()),
            throughput: AtomicUsize::new(0),
        }
    }

    /// Record an operation time
    pub fn record_operation(&self, name: &str, duration: Duration) {
        if let Ok(mut times) = self.operation_times.lock() {
            times.push((name.to_string(), duration));
        }
    }

    /// Increment throughput counter
    pub fn increment_throughput(&self) {
        self.throughput.fetch_add(1, Ordering::Relaxed);
    }

    /// Get average operation time
    pub fn average_operation_time(&self, name: &str) -> Option<Duration> {
        if let Ok(times) = self.operation_times.lock() {
            let matching: Vec<_> = times
                .iter()
                .filter(|(n, _)| n == name)
                .map(|(_, d)| *d)
                .collect();

            if !matching.is_empty() {
                let total: Duration = matching.iter().sum();
                Some(total / matching.len() as u32)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get throughput
    pub fn throughput(&self) -> usize {
        self.throughput.load(Ordering::Relaxed)
    }

    /// Reset metrics
    pub fn reset(&self) {
        if let Ok(mut times) = self.operation_times.lock() {
            times.clear();
        }
        self.throughput.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestTask {
        value: usize,
    }

    impl ParallelTask for TestTask {
        type Output = usize;

        fn execute(&self) -> Self::Output {
            self.value * 2
        }

        fn priority(&self) -> TaskPriority {
            TaskPriority::Normal
        }
    }

    #[test]
    fn test_task_queue() {
        let queue = PriorityTaskQueue::<TestTask>::new();

        let id1 = queue.submit(TestTask { value: 1 });
        let id2 = queue.submit(TestTask { value: 2 });

        assert_eq!(queue.pending_count(), 2);

        if let Some(task) = queue.next_task() {
            let _ = task.task.execute();
            queue.mark_completed();
        }

        assert_eq!(queue.pending_count(), 1);
    }

    #[test]
    fn test_load_balancer() {
        let config = ParallelConfig::default();
        let balancer = LoadBalancer::new(config);

        let thread = balancer.least_loaded_thread();
        balancer.add_load(thread, 10);

        let distribution = balancer.load_distribution();
        assert_eq!(distribution[thread], 10);
    }
}
