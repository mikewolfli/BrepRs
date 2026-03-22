use crate::foundation::types::StandardReal;
use super::distributed::{DistributedCluster, NodeId, TaskId, TaskStatus, ResourceRequirements};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingStrategy {
    FirstFit,
    BestFit,
    RoundRobin,
    LeastLoaded,
    PriorityBased,
    ResourceAware,
}

pub struct TaskScheduler {
    strategy: SchedulingStrategy,
    round_robin_index: usize,
    #[allow(dead_code)]
    priority_queue: BinaryHeap<PriorityTask>,
}

#[derive(Debug, Clone, Eq)]
struct PriorityTask {
    priority: u8,
    #[allow(dead_code)]
    task_id: TaskId,
    created_order: u64,
}

impl PartialEq for PriorityTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.created_order == other.created_order
    }
}

impl Ord for PriorityTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.created_order.cmp(&other.created_order))
    }
}

impl PartialOrd for PriorityTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TaskScheduler {
    pub fn new(strategy: SchedulingStrategy) -> Self {
        Self {
            strategy,
            round_robin_index: 0,
            priority_queue: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        match self.strategy {
            SchedulingStrategy::FirstFit => self.schedule_first_fit(cluster),
            SchedulingStrategy::BestFit => self.schedule_best_fit(cluster),
            SchedulingStrategy::RoundRobin => self.schedule_round_robin(cluster),
            SchedulingStrategy::LeastLoaded => self.schedule_least_loaded(cluster),
            SchedulingStrategy::PriorityBased => self.schedule_priority_based(cluster),
            SchedulingStrategy::ResourceAware => self.schedule_resource_aware(cluster),
        }
    }

    fn schedule_first_fit(&self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        let pending: Vec<TaskId> = cluster.pending_tasks.iter().cloned().collect();

        for task_id in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let resources = task.required_resources;
                
                for (node_id, node) in &cluster.nodes {
                    if node.is_available() && self.node_has_resources(node, resources) {
                        assignments.push((task_id, *node_id));
                        break;
                    }
                }
            }
        }

        assignments
    }

    fn schedule_best_fit(&self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        let pending: Vec<TaskId> = cluster.pending_tasks.iter().cloned().collect();

        for task_id in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let resources = task.required_resources;
                let mut best_node: Option<(NodeId, StandardReal)> = None;

                for (node_id, node) in &cluster.nodes {
                    if node.is_available() && self.node_has_resources(node, resources) {
                        let fit_score = self.calculate_fit_score(node, resources);
                        match best_node {
                            None => best_node = Some((*node_id, fit_score)),
                            Some((_, best_score)) if fit_score < best_score => {
                                best_node = Some((*node_id, fit_score));
                            }
                            _ => {}
                        }
                    }
                }

                if let Some((node_id, _)) = best_node {
                    assignments.push((task_id, node_id));
                }
            }
        }

        assignments
    }

    fn schedule_round_robin(&mut self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        let pending: Vec<TaskId> = cluster.pending_tasks.iter().cloned().collect();
        let available_nodes: Vec<NodeId> = cluster
            .nodes
            .iter()
            .filter(|(_, n)| n.is_available())
            .map(|(id, _)| *id)
            .collect();

        if available_nodes.is_empty() {
            return assignments;
        }

        for task_id in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let node_id = available_nodes[self.round_robin_index % available_nodes.len()];
                self.round_robin_index = (self.round_robin_index + 1) % available_nodes.len();
                assignments.push((task_id, node_id));
            }
        }

        assignments
    }

    fn schedule_least_loaded(&self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        let pending: Vec<TaskId> = cluster.pending_tasks.iter().cloned().collect();

        let mut available_nodes: Vec<_> = cluster
            .nodes
            .iter()
            .filter(|(_, n)| n.is_available())
            .map(|(id, n)| (*id, n))
            .collect();

        available_nodes.sort_by(|a, b| {
            a.1.current_load.partial_cmp(&b.1.current_load).unwrap_or(Ordering::Equal)
        });

        for task_id in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let resources = task.required_resources;
                
                for (node_id, node) in &available_nodes {
                    if self.node_has_resources(node, resources) {
                        assignments.push((task_id, *node_id));
                        break;
                    }
                }
            }
        }

        assignments
    }

    fn schedule_priority_based(&mut self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        
        let mut pending: Vec<_> = cluster
            .pending_tasks
            .iter()
            .filter_map(|id| cluster.get_task(*id).map(|t| (*id, t.priority.value())))
            .collect();

        pending.sort_by(|a, b| b.1.cmp(&a.1));

        for (task_id, _) in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let resources = task.required_resources;
                
                if let Some((node_id, _)) = cluster
                    .nodes
                    .iter()
                    .find(|(_, n)| n.is_available() && self.node_has_resources(n, resources))
                {
                    assignments.push((task_id, *node_id));
                }
            }
        }

        assignments
    }

    fn schedule_resource_aware(&self, cluster: &mut DistributedCluster) -> Vec<(TaskId, NodeId)> {
        let mut assignments = Vec::new();
        let pending: Vec<TaskId> = cluster.pending_tasks.iter().cloned().collect();

        let mut node_scores: HashMap<NodeId, StandardReal> = HashMap::new();

        for (_task_id, task) in cluster.tasks.iter() {
            if task.status == TaskStatus::Running {
                if let Some(node_id) = task.assigned_node {
                    let score = node_scores.entry(node_id).or_insert(0.0);
                    *score += task.required_resources.estimated_duration_secs;
                }
            }
        }

        let mut available_nodes: Vec<_> = cluster
            .nodes
            .iter()
            .filter(|(_, n)| n.is_available())
            .map(|(id, n)| {
                let base_score = node_scores.get(id).copied().unwrap_or(0.0);
                (*id, n, base_score)
            })
            .collect();

        available_nodes.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal));

        for task_id in pending {
            if let Some(task) = cluster.get_task(task_id) {
                if !self.dependencies_satisfied(cluster, &task.dependencies) {
                    continue;
                }

                let resources = task.required_resources;
                
                for (node_id, node, _) in available_nodes.iter() {
                    if self.node_has_resources(node, resources) {
                        assignments.push((task_id, node_id.clone()));
                        break;
                    }
                }
            }
        }

        assignments
    }

    fn dependencies_satisfied(&self, cluster: &DistributedCluster, dependencies: &[TaskId]) -> bool {
        dependencies.iter().all(|dep_id| {
            cluster
                .get_task(*dep_id)
                .map(|t| t.status == TaskStatus::Completed)
                .unwrap_or(false)
        })
    }

    fn node_has_resources(&self, node: &super::distributed::NodeInfo, resources: ResourceRequirements) -> bool {
        node.cpu_cores >= resources.min_cpu_cores
            && node.memory_gb >= resources.min_memory_gb
            && node.gpu_count >= resources.min_gpu_count
    }

    fn calculate_fit_score(&self, node: &super::distributed::NodeInfo, resources: ResourceRequirements) -> StandardReal {
        let cpu_diff = node.cpu_cores as StandardReal - resources.min_cpu_cores as StandardReal;
        let mem_diff = node.memory_gb - resources.min_memory_gb;
        let gpu_diff = node.gpu_count as StandardReal - resources.min_gpu_count as StandardReal;
        
        cpu_diff + mem_diff + gpu_diff
    }

    pub fn strategy(&self) -> SchedulingStrategy {
        self.strategy
    }

    pub fn set_strategy(&mut self, strategy: SchedulingStrategy) {
        self.strategy = strategy;
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new(SchedulingStrategy::LeastLoaded)
    }
}

pub struct SchedulerStats {
    pub total_scheduled: u64,
    pub successful_assignments: u64,
    pub failed_assignments: u64,
    pub average_wait_time: StandardReal,
}

impl SchedulerStats {
    pub fn new() -> Self {
        Self {
            total_scheduled: 0,
            successful_assignments: 0,
            failed_assignments: 0,
            average_wait_time: 0.0,
        }
    }

    pub fn success_rate(&self) -> StandardReal {
        if self.total_scheduled == 0 {
            return 0.0;
        }
        self.successful_assignments as StandardReal / self.total_scheduled as StandardReal
    }
}

impl Default for SchedulerStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = TaskScheduler::new(SchedulingStrategy::FirstFit);
        assert_eq!(scheduler.strategy(), SchedulingStrategy::FirstFit);
    }

    #[test]
    fn test_first_fit_scheduling() {
        let mut cluster = DistributedCluster::new();
        let node_id = cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        let task_id = cluster.submit_task("Task 1".to_string());

        let mut scheduler = TaskScheduler::new(SchedulingStrategy::FirstFit);
        let assignments = scheduler.schedule(&mut cluster);

        assert!(!assignments.is_empty());
        assert_eq!(assignments[0].0, task_id);
        assert_eq!(assignments[0].1, node_id);
    }

    #[test]
    fn test_least_loaded_scheduling() {
        let mut cluster = DistributedCluster::new();
        cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        cluster.register_node("Node 2".to_string(), "localhost:8081".to_string());
        
        cluster.submit_task("Task 1".to_string());
        cluster.submit_task("Task 2".to_string());

        let mut scheduler = TaskScheduler::new(SchedulingStrategy::LeastLoaded);
        let assignments = scheduler.schedule(&mut cluster);

        assert!(!assignments.is_empty());
    }

    #[test]
    fn test_priority_based_scheduling() {
        use crate::cloud::distributed::TaskPriority;
        
        let mut cluster = DistributedCluster::new();
        cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        
        let _low_task = cluster.submit_task_with_config(
            "Low Priority".to_string(),
            TaskPriority::Low,
            ResourceRequirements::default(),
            vec![],
        );
        
        let high_task = cluster.submit_task_with_config(
            "High Priority".to_string(),
            TaskPriority::High,
            ResourceRequirements::default(),
            vec![],
        );

        let mut scheduler = TaskScheduler::new(SchedulingStrategy::PriorityBased);
        let assignments = scheduler.schedule(&mut cluster);

        assert!(!assignments.is_empty());
        assert_eq!(assignments[0].0, high_task);
    }

    #[test]
    fn test_scheduler_stats() {
        let mut stats = SchedulerStats::new();
        stats.total_scheduled = 100;
        stats.successful_assignments = 95;
        stats.failed_assignments = 5;

        assert_eq!(stats.success_rate(), 0.95);
    }
}
