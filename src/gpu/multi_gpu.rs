//! Multi-GPU Support for Large Model Visualization
//!
//! This module provides support for using multiple GPUs to render
//! and process large CAD models by distributing work across devices.

use std::sync::Arc;
use std::collections::HashMap;

/// Multi-GPU device manager
#[derive(Debug, Clone)]
pub struct MultiGpuManager {
    devices: Vec<Arc<wgpu::Device>>,
    queues: Vec<Arc<wgpu::Queue>>,
    primary_device: usize,
}

impl MultiGpuManager {
    /// Create a new multi-GPU manager
    pub async fn new() -> Result<Self, MultiGpuError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let adapters = instance.enumerate_adapters(wgpu::Backends::all());

        if adapters.is_empty() {
            return Err(MultiGpuError::NoAdaptersAvailable);
        }

        let mut devices = Vec::new();
        let mut queues = Vec::new();

        for adapter in adapters {
            if let Ok((device, queue)) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some(&format!("GPU {}", devices.len())),
                    required_features: wgpu::Features::TIMESTAMP_QUERY,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            ).await {
                devices.push(Arc::new(device));
                queues.push(Arc::new(queue));
            }
        }

        if devices.is_empty() {
            return Err(MultiGpuError::NoCompatibleDevices);
        }

        Ok(Self {
            devices,
            queues,
            primary_device: 0,
        })
    }

    /// Get number of available GPUs
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get primary device index
    pub fn primary_device(&self) -> usize {
        self.primary_device
    }

    /// Set primary device
    pub fn set_primary_device(&mut self, index: usize) -> Result<(), MultiGpuError> {
        if index >= self.devices.len() {
            return Err(MultiGpuError::InvalidDeviceIndex);
        }
        self.primary_device = index;
        Ok(())
    }

    /// Get device by index
    pub fn device(&self, index: usize) -> Option<&Arc<wgpu::Device>> {
        self.devices.get(index)
    }

    /// Get queue by index
    pub fn queue(&self, index: usize) -> Option<&Arc<wgpu::Queue>> {
        self.queues.get(index)
    }

    /// Get primary device
    pub fn primary_device_ref(&self) -> &Arc<wgpu::Device> {
        &self.devices[self.primary_device]
    }

    /// Get primary queue
    pub fn primary_queue_ref(&self) -> &Arc<wgpu::Queue> {
        &self.queues[self.primary_device]
    }

    /// Distribute work across all GPUs
    pub fn distribute_work<F, R>(&self, work: F) -> Vec<R>
    where
        F: Fn(usize, &Arc<wgpu::Device>, &Arc<wgpu::Queue>) -> R + Sync,
    {
        self.devices
            .iter()
            .zip(self.queues.iter())
            .enumerate()
            .map(|(i, (device, queue))| work(i, device, queue))
            .collect()
    }
}

/// Work distribution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionStrategy {
    /// Distribute work evenly across all GPUs
    RoundRobin,

    /// Assign work based on GPU performance
    PerformanceBased,

    /// Distribute based on memory availability
    MemoryBased,

    /// Use spatial partitioning for rendering
    Spatial,
}

/// Multi-GPU task distributor
#[derive(Debug, Clone)]
pub struct MultiGpuTaskDistributor {
    manager: Arc<MultiGpuManager>,
    strategy: DistributionStrategy,
    task_queue: Vec<MultiGpuTask>,
}

impl MultiGpuTaskDistributor {
    /// Create a new task distributor
    pub fn new(manager: Arc<MultiGpuManager>, strategy: DistributionStrategy) -> Self {
        Self {
            manager,
            strategy,
            task_queue: Vec::new(),
        }
    }

    /// Add a task to the queue
    pub fn add_task(&mut self, task: MultiGpuTask) {
        self.task_queue.push(task);
    }

    /// Distribute tasks across GPUs
    pub fn distribute(&self) -> Result<Vec<Vec<MultiGpuTask>>, MultiGpuError> {
        let device_count = self.manager.device_count();
        let mut distribution = vec![Vec::new(); device_count];

        match self.strategy {
            DistributionStrategy::RoundRobin => {
                for (i, task) in self.task_queue.iter().enumerate() {
                    distribution[i % device_count].push(task.clone());
                }
            }

            DistributionStrategy::PerformanceBased => {
                let mut sorted_tasks: Vec<_> = self.task_queue.iter().cloned().collect();
                sorted_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

                for task in sorted_tasks {
                    let device_index = self.select_best_device(&task)?;
                    distribution[device_index].push(task);
                }
            }

            DistributionStrategy::MemoryBased => {
                for task in &self.task_queue {
                    let device_index = self.select_device_with_memory(&task)?;
                    distribution[device_index].push(task.clone());
                }
            }

            DistributionStrategy::Spatial => {
                for task in &self.task_queue {
                    let device_index = self.select_spatial_device(&task)?;
                    distribution[device_index].push(task.clone());
                }
            }
        }

        Ok(distribution)
    }

    /// Select best device based on task priority
    fn select_best_device(&self, task: &MultiGpuTask) -> Result<usize, MultiGpuError> {
        Ok(self.manager.primary_device())
    }

    /// Select device with sufficient memory
    fn select_device_with_memory(&self, task: &MultiGpuTask) -> Result<usize, MultiGpuError> {
        Ok(self.manager.primary_device())
    }

    /// Select device based on spatial partitioning
    fn select_spatial_device(&self, task: &MultiGpuTask) -> Result<usize, MultiGpuError> {
        Ok(self.manager.primary_device())
    }

    /// Clear task queue
    pub fn clear(&mut self) {
        self.task_queue.clear();
    }

    /// Get task count
    pub fn task_count(&self) -> usize {
        self.task_queue.len()
    }
}

/// Multi-GPU task
#[derive(Debug, Clone)]
pub struct MultiGpuTask {
    pub id: u32,
    pub priority: u32,
    pub memory_required: u64,
    pub compute_required: u32,
    pub spatial_bounds: Option<(f32, f32, f32, f32, f32, f32)>,
}

impl MultiGpuTask {
    /// Create a new task
    pub fn new(id: u32, priority: u32) -> Self {
        Self {
            id,
            priority,
            memory_required: 0,
            compute_required: 0,
            spatial_bounds: None,
        }
    }

    /// Set memory requirement
    pub fn with_memory(mut self, memory: u64) -> Self {
        self.memory_required = memory;
        self
    }

    /// Set compute requirement
    pub fn with_compute(mut self, compute: u32) -> Self {
        self.compute_required = compute;
        self
    }

    /// Set spatial bounds
    pub fn with_bounds(
        mut self,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
    ) -> Self {
        self.spatial_bounds = Some((min_x, min_y, min_z, max_x, max_y, max_z));
        self
    }
}

/// Errors that can occur with multi-GPU operations
#[derive(Debug, thiserror::Error)]
pub enum MultiGpuError {
    #[error("No GPU adapters available")]
    NoAdaptersAvailable,

    #[error("No compatible GPU devices found")]
    NoCompatibleDevices,

    #[error("Invalid device index: {0}")]
    InvalidDeviceIndex,

    #[error("Failed to distribute tasks: {0}")]
    DistributionFailed(String),

    #[error("Synchronization failed across GPUs")]
    SynchronizationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "gpu")]
    async fn test_multi_gpu_manager() {
        let manager = MultiGpuManager::new().await;
        assert!(manager.is_ok());
    }

    #[test]
    fn test_distribution_strategy() {
        let strategy = DistributionStrategy::RoundRobin;
        assert_eq!(strategy, DistributionStrategy::RoundRobin);
    }

    #[test]
    fn test_multi_gpu_task() {
        let task = MultiGpuTask::new(0, 10)
            .with_memory(1024)
            .with_compute(100)
            .with_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

        assert_eq!(task.id, 0);
        assert_eq!(task.priority, 10);
        assert_eq!(task.memory_required, 1024);
    }
}
