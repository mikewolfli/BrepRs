use crate::foundation::types::StandardReal;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

impl Default for NodeId {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

impl Default for TaskId {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Available,
    Busy,
    Offline,
    Maintenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl TaskPriority {
    pub fn value(&self) -> u8 {
        match self {
            TaskPriority::Low => 1,
            TaskPriority::Normal => 5,
            TaskPriority::High => 10,
            TaskPriority::Critical => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: NodeId,
    pub name: String,
    pub address: String,
    pub status: NodeStatus,
    pub cpu_cores: u32,
    pub memory_gb: StandardReal,
    pub gpu_count: u32,
    pub current_load: StandardReal,
    pub last_heartbeat: Instant,
}

impl NodeInfo {
    pub fn new(id: NodeId, name: String, address: String) -> Self {
        Self {
            id,
            name,
            address,
            status: NodeStatus::Available,
            cpu_cores: 4,
            memory_gb: 16.0,
            gpu_count: 0,
            current_load: 0.0,
            last_heartbeat: Instant::now(),
        }
    }

    pub fn with_resources(mut self, cpu_cores: u32, memory_gb: StandardReal, gpu_count: u32) -> Self {
        self.cpu_cores = cpu_cores;
        self.memory_gb = memory_gb;
        self.gpu_count = gpu_count;
        self
    }

    pub fn is_available(&self) -> bool {
        self.status == NodeStatus::Available && self.current_load < 1.0
    }

    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    pub fn seconds_since_heartbeat(&self) -> StandardReal {
        self.last_heartbeat.elapsed().as_secs_f64()
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub assigned_node: Option<NodeId>,
    pub created_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
    pub progress: StandardReal,
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
    pub dependencies: Vec<TaskId>,
    pub required_resources: ResourceRequirements,
}

impl Task {
    pub fn new(id: TaskId, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            assigned_node: None,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
            progress: 0.0,
            result: None,
            error: None,
            dependencies: Vec::new(),
            required_resources: ResourceRequirements::default(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<TaskId>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_resources(mut self, resources: ResourceRequirements) -> Self {
        self.required_resources = resources;
        self
    }

    pub fn start(&mut self, node_id: NodeId) {
        self.status = TaskStatus::Running;
        self.assigned_node = Some(node_id);
        self.started_at = Some(Instant::now());
    }

    pub fn complete(&mut self, result: Vec<u8>) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Instant::now());
        self.result = Some(result);
        self.progress = 1.0;
    }

    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Instant::now());
        self.error = Some(error);
    }

    pub fn update_progress(&mut self, progress: StandardReal) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(start.elapsed()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceRequirements {
    pub min_cpu_cores: u32,
    pub min_memory_gb: StandardReal,
    pub min_gpu_count: u32,
    pub estimated_duration_secs: StandardReal,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            min_cpu_cores: 1,
            min_memory_gb: 1.0,
            min_gpu_count: 0,
            estimated_duration_secs: 60.0,
        }
    }
}

impl ResourceRequirements {
    pub fn new(min_cpu_cores: u32, min_memory_gb: StandardReal) -> Self {
        Self {
            min_cpu_cores,
            min_memory_gb,
            min_gpu_count: 0,
            estimated_duration_secs: 60.0,
        }
    }

    pub fn with_gpu(mut self, gpu_count: u32) -> Self {
        self.min_gpu_count = gpu_count;
        self
    }

    pub fn with_duration(mut self, secs: StandardReal) -> Self {
        self.estimated_duration_secs = secs;
        self
    }
}

pub struct DistributedCluster {
    pub nodes: HashMap<NodeId, NodeInfo>,
    pub tasks: HashMap<TaskId, Task>,
    pub pending_tasks: Vec<TaskId>,
    pub running_tasks: Vec<TaskId>,
    pub completed_tasks: Vec<TaskId>,
    pub next_task_id: u64,
    pub next_node_id: u64,
}

impl DistributedCluster {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            tasks: HashMap::new(),
            pending_tasks: Vec::new(),
            running_tasks: Vec::new(),
            completed_tasks: Vec::new(),
            next_task_id: 1,
            next_node_id: 1,
        }
    }

    pub fn register_node(&mut self, name: String, address: String) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = NodeInfo::new(id, name, address);
        self.nodes.insert(id, node);
        id
    }

    pub fn unregister_node(&mut self, node_id: NodeId) -> Option<NodeInfo> {
        self.nodes.remove(&node_id)
    }

    pub fn submit_task(&mut self, name: String) -> TaskId {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;

        let task = Task::new(id, name);
        self.tasks.insert(id, task);
        self.pending_tasks.push(id);
        id
    }

    pub fn submit_task_with_config(
        &mut self,
        name: String,
        priority: TaskPriority,
        resources: ResourceRequirements,
        dependencies: Vec<TaskId>,
    ) -> TaskId {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;

        let task = Task::new(id, name)
            .with_priority(priority)
            .with_resources(resources)
            .with_dependencies(dependencies);

        self.tasks.insert(id, task);
        self.pending_tasks.push(id);
        id
    }

    pub fn cancel_task(&mut self, task_id: TaskId) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            if task.status == TaskStatus::Pending || task.status == TaskStatus::Running {
                task.status = TaskStatus::Cancelled;
                task.completed_at = Some(Instant::now());
                return true;
            }
        }
        false
    }

    pub fn get_task(&self, task_id: TaskId) -> Option<&Task> {
        self.tasks.get(&task_id)
    }

    pub fn get_task_mut(&mut self, task_id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(&task_id)
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&NodeInfo> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut NodeInfo> {
        self.nodes.get_mut(&node_id)
    }

    pub fn available_nodes(&self) -> Vec<&NodeInfo> {
        self.nodes.values().filter(|n| n.is_available()).collect()
    }

    pub fn pending_tasks(&self) -> Vec<&Task> {
        self.pending_tasks
            .iter()
            .filter_map(|id| self.tasks.get(id))
            .collect()
    }

    pub fn running_tasks(&self) -> Vec<&Task> {
        self.running_tasks
            .iter()
            .filter_map(|id| self.tasks.get(id))
            .collect()
    }

    pub fn completed_tasks(&self) -> Vec<&Task> {
        self.completed_tasks
            .iter()
            .filter_map(|id| self.tasks.get(id))
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn cluster_load(&self) -> StandardReal {
        if self.nodes.is_empty() {
            return 0.0;
        }
        let total_load: StandardReal = self.nodes.values().map(|n| n.current_load).sum();
        total_load / self.nodes.len() as StandardReal
    }

    pub fn update_node_status(&mut self, node_id: NodeId, status: NodeStatus) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.status = status;
            node.update_heartbeat();
        }
    }

    pub fn update_node_load(&mut self, node_id: NodeId, load: StandardReal) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.current_load = load.clamp(0.0, 1.0);
        }
    }

    pub fn check_timeouts(&mut self, timeout_secs: StandardReal) -> Vec<NodeId> {
        let timed_out: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|(_, n)| n.seconds_since_heartbeat() > timeout_secs)
            .map(|(id, _)| *id)
            .collect();

        for id in &timed_out {
            if let Some(node) = self.nodes.get_mut(id) {
                node.status = NodeStatus::Offline;
            }
        }

        timed_out
    }
}

impl Default for DistributedCluster {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterStats {
    pub total_nodes: usize,
    pub available_nodes: usize,
    pub busy_nodes: usize,
    pub offline_nodes: usize,
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub average_load: StandardReal,
}

impl ClusterStats {
    pub fn from_cluster(cluster: &DistributedCluster) -> Self {
        let nodes: Vec<_> = cluster.nodes.values().collect();
        let tasks: Vec<_> = cluster.tasks.values().collect();

        Self {
            total_nodes: nodes.len(),
            available_nodes: nodes.iter().filter(|n| n.status == NodeStatus::Available).count(),
            busy_nodes: nodes.iter().filter(|n| n.status == NodeStatus::Busy).count(),
            offline_nodes: nodes.iter().filter(|n| n.status == NodeStatus::Offline).count(),
            total_tasks: tasks.len(),
            pending_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Pending).count(),
            running_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Running).count(),
            completed_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(),
            failed_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Failed).count(),
            average_load: cluster.cluster_load(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = NodeInfo::new(NodeId(1), "Test Node".to_string(), "192.168.1.1:8080".to_string())
            .with_resources(8, 32.0, 1);

        assert_eq!(node.id, NodeId(1));
        assert_eq!(node.cpu_cores, 8);
        assert!(node.is_available());
    }

    #[test]
    fn test_task_creation() {
        let task = Task::new(TaskId(1), "Test Task".to_string())
            .with_priority(TaskPriority::High);

        assert_eq!(task.id, TaskId(1));
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::High);
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new(TaskId(1), "Test".to_string());
        
        task.start(NodeId(1));
        assert_eq!(task.status, TaskStatus::Running);
        
        task.update_progress(0.5);
        assert_eq!(task.progress, 0.5);
        
        task.complete(vec![1, 2, 3]);
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.result.is_some());
    }

    #[test]
    fn test_cluster_operations() {
        let mut cluster = DistributedCluster::new();
        
        let _node_id = cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        assert_eq!(cluster.node_count(), 1);
        
        let task_id = cluster.submit_task("Test Task".to_string());
        assert_eq!(cluster.task_count(), 1);
        
        assert!(cluster.cancel_task(task_id));
        assert_eq!(cluster.get_task(task_id).unwrap().status, TaskStatus::Cancelled);
    }

    #[test]
    fn test_resource_requirements() {
        let resources = ResourceRequirements::new(4, 8.0)
            .with_gpu(1)
            .with_duration(120.0);

        assert_eq!(resources.min_cpu_cores, 4);
        assert_eq!(resources.min_memory_gb, 8.0);
        assert_eq!(resources.min_gpu_count, 1);
        assert_eq!(resources.estimated_duration_secs, 120.0);
    }

    #[test]
    fn test_cluster_stats() {
        let mut cluster = DistributedCluster::new();
        cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        cluster.submit_task("Task 1".to_string());

        let stats = ClusterStats::from_cluster(&cluster);
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.total_tasks, 1);
    }
}
