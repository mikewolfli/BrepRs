use crate::foundation::types::StandardReal;
use super::distributed::{DistributedCluster, NodeId, NodeStatus};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    Static,
    Dynamic,
    Adaptive,
    Predictive,
}

pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    load_history: HashMap<NodeId, Vec<StandardReal>>,
    history_size: usize,
    load_threshold_high: StandardReal,
    load_threshold_low: StandardReal,
    migration_threshold: StandardReal,
}

impl LoadBalancer {
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            strategy,
            load_history: HashMap::new(),
            history_size: 10,
            load_threshold_high: 0.8,
            load_threshold_low: 0.2,
            migration_threshold: 0.3,
        }
    }

    pub fn with_thresholds(mut self, high: StandardReal, low: StandardReal) -> Self {
        self.load_threshold_high = high;
        self.load_threshold_low = low;
        self
    }

    pub fn with_history_size(mut self, size: usize) -> Self {
        self.history_size = size;
        self
    }

    pub fn update_load(&mut self, node_id: NodeId, load: StandardReal) {
        let history = self.load_history.entry(node_id).or_insert_with(Vec::new);
        history.push(load);
        if history.len() > self.history_size {
            history.remove(0);
        }
    }

    pub fn balance(&self, cluster: &mut DistributedCluster) -> Vec<MigrationPlan> {
        match self.strategy {
            LoadBalancingStrategy::Static => self.static_balance(cluster),
            LoadBalancingStrategy::Dynamic => self.dynamic_balance(cluster),
            LoadBalancingStrategy::Adaptive => self.adaptive_balance(cluster),
            LoadBalancingStrategy::Predictive => self.predictive_balance(cluster),
        }
    }

    fn static_balance(&self, cluster: &DistributedCluster) -> Vec<MigrationPlan> {
        let _ = cluster;
        Vec::new()
    }

    fn dynamic_balance(&self, cluster: &DistributedCluster) -> Vec<MigrationPlan> {
        let mut migrations = Vec::new();
        
        let overloaded: Vec<_> = cluster
            .nodes
            .iter()
            .filter(|(_, n)| n.current_load > self.load_threshold_high)
            .collect();

        let underloaded: Vec<_> = cluster
            .nodes
            .iter()
            .filter(|(_, n)| n.current_load < self.load_threshold_low && n.status == NodeStatus::Available)
            .collect();

        for (over_id, over_node) in &overloaded {
            let load_diff = over_node.current_load - self.load_threshold_high;
            
            for (under_id, _) in &underloaded {
                let migration_amount = load_diff.min(self.migration_threshold);
                migrations.push(MigrationPlan {
                    from_node: **over_id,
                    to_node: **under_id,
                    load_amount: migration_amount,
                });
            }
        }

        migrations
    }

    fn adaptive_balance(&self, cluster: &DistributedCluster) -> Vec<MigrationPlan> {
        let mut migrations = Vec::new();

        let avg_load = cluster.cluster_load();
        
        for (node_id, node) in &cluster.nodes {
            let deviation = (node.current_load - avg_load).abs();
            
            if deviation > self.migration_threshold {
                let history = self.load_history.get(node_id);
                let trend = self.calculate_trend(history);
                
                if trend > 0.1 && node.current_load > avg_load {
                    if let Some((target_id, _)) = cluster
                        .nodes
                        .iter()
                        .find(|(_, n)| n.current_load < avg_load - self.migration_threshold)
                    {
                        migrations.push(MigrationPlan {
                            from_node: *node_id,
                            to_node: *target_id,
                            load_amount: deviation / 2.0,
                        });
                    }
                }
            }
        }

        migrations
    }

    fn predictive_balance(&self, cluster: &DistributedCluster) -> Vec<MigrationPlan> {
        let mut migrations = Vec::new();

        let mut predictions: HashMap<NodeId, StandardReal> = HashMap::new();

        for (node_id, _) in &cluster.nodes {
            let history = self.load_history.get(node_id);
            let predicted_load = self.predict_load(history);
            predictions.insert(*node_id, predicted_load);
        }

        let avg_predicted = if !predictions.is_empty() {
            predictions.values().sum::<StandardReal>() / predictions.len() as StandardReal
        } else {
            0.0
        };

        for (node_id, predicted) in &predictions {
            if *predicted > avg_predicted + self.migration_threshold {
                if let Some((target_id, _)) = predictions
                    .iter()
                    .find(|(_, p)| **p < avg_predicted - self.migration_threshold)
                {
                    migrations.push(MigrationPlan {
                        from_node: *node_id,
                        to_node: *target_id,
                        load_amount: (*predicted - avg_predicted) / 2.0,
                    });
                }
            }
        }

        migrations
    }

    fn calculate_trend(&self, history: Option<&Vec<StandardReal>>) -> StandardReal {
        match history {
            Some(h) if h.len() >= 2 => {
                let n = h.len();
                let sum_x: StandardReal = (0..n).map(|i| i as StandardReal).sum();
                let sum_y: StandardReal = h.iter().sum();
                let sum_xy: StandardReal = h
                    .iter()
                    .enumerate()
                    .map(|(i, y)| i as StandardReal * y)
                    .sum();
                let sum_xx: StandardReal = (0..n).map(|i| (i * i) as StandardReal).sum();

                let denominator = n as StandardReal * sum_xx - sum_x * sum_x;
                if denominator.abs() < 1e-10 {
                    return 0.0;
                }

                (n as StandardReal * sum_xy - sum_x * sum_y) / denominator
            }
            _ => 0.0,
        }
    }

    fn predict_load(&self, history: Option<&Vec<StandardReal>>) -> StandardReal {
        match history {
            Some(h) if !h.is_empty() => {
                let trend = self.calculate_trend(Some(h));
                let last = *h.last().unwrap();
                (last + trend).clamp(0.0, 1.0)
            }
            _ => 0.5,
        }
    }

    pub fn strategy(&self) -> LoadBalancingStrategy {
        self.strategy
    }

    pub fn set_strategy(&mut self, strategy: LoadBalancingStrategy) {
        self.strategy = strategy;
    }

    pub fn get_node_history(&self, node_id: NodeId) -> Option<&Vec<StandardReal>> {
        self.load_history.get(&node_id)
    }

    pub fn clear_history(&mut self) {
        self.load_history.clear();
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new(LoadBalancingStrategy::Dynamic)
    }
}

#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub load_amount: StandardReal,
}

impl MigrationPlan {
    pub fn new(from_node: NodeId, to_node: NodeId, load_amount: StandardReal) -> Self {
        Self {
            from_node,
            to_node,
            load_amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadBalancerStats {
    pub total_migrations: u64,
    pub successful_migrations: u64,
    pub failed_migrations: u64,
    pub average_load_before: StandardReal,
    pub average_load_after: StandardReal,
    pub load_variance_before: StandardReal,
    pub load_variance_after: StandardReal,
}

impl LoadBalancerStats {
    pub fn new() -> Self {
        Self {
            total_migrations: 0,
            successful_migrations: 0,
            failed_migrations: 0,
            average_load_before: 0.0,
            average_load_after: 0.0,
            load_variance_before: 0.0,
            load_variance_after: 0.0,
        }
    }

    pub fn migration_success_rate(&self) -> StandardReal {
        if self.total_migrations == 0 {
            return 0.0;
        }
        self.successful_migrations as StandardReal / self.total_migrations as StandardReal
    }

    pub fn load_improvement(&self) -> StandardReal {
        self.load_variance_before - self.load_variance_after
    }
}

impl Default for LoadBalancerStats {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_load_variance(cluster: &DistributedCluster) -> StandardReal {
    let loads: Vec<StandardReal> = cluster.nodes.values().map(|n| n.current_load).collect();
    
    if loads.is_empty() {
        return 0.0;
    }

    let mean = loads.iter().sum::<StandardReal>() / loads.len() as StandardReal;
    let variance = loads
        .iter()
        .map(|l| (l - mean).powi(2))
        .sum::<StandardReal>()
        / loads.len() as StandardReal;

    variance
}

pub fn calculate_load_std_dev(cluster: &DistributedCluster) -> StandardReal {
    calculate_load_variance(cluster).sqrt()
}

pub fn calculate_load_balance_score(cluster: &DistributedCluster) -> StandardReal {
    let variance = calculate_load_variance(cluster);
    1.0 / (1.0 + variance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_balancer_creation() {
        let balancer = LoadBalancer::new(LoadBalancingStrategy::Dynamic);
        assert_eq!(balancer.strategy(), LoadBalancingStrategy::Dynamic);
    }

    #[test]
    fn test_load_history() {
        let mut balancer = LoadBalancer::new(LoadBalancingStrategy::Dynamic);
        let node_id = NodeId(1);
        
        balancer.update_load(node_id, 0.5);
        balancer.update_load(node_id, 0.6);
        balancer.update_load(node_id, 0.7);

        let history = balancer.get_node_history(node_id).unwrap();
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_migration_plan() {
        let plan = MigrationPlan::new(NodeId(1), NodeId(2), 0.3);
        assert_eq!(plan.from_node, NodeId(1));
        assert_eq!(plan.to_node, NodeId(2));
        assert_eq!(plan.load_amount, 0.3);
    }

    #[test]
    fn test_load_balancer_stats() {
        let mut stats = LoadBalancerStats::new();
        stats.total_migrations = 10;
        stats.successful_migrations = 9;
        stats.failed_migrations = 1;

        assert_eq!(stats.migration_success_rate(), 0.9);
    }

    #[test]
    fn test_load_variance() {
        let mut cluster = DistributedCluster::new();
        cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        cluster.register_node("Node 2".to_string(), "localhost:8081".to_string());

        let variance = calculate_load_variance(&cluster);
        assert!(variance >= 0.0);
    }

    #[test]
    fn test_load_balance_score() {
        let mut cluster = DistributedCluster::new();
        cluster.register_node("Node 1".to_string(), "localhost:8080".to_string());
        cluster.register_node("Node 2".to_string(), "localhost:8081".to_string());

        let score = calculate_load_balance_score(&cluster);
        assert!(score > 0.0 && score <= 1.0);
    }
}
