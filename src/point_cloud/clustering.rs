//! Point cloud clustering module
//! 
//! This module provides various clustering algorithms for point clouds,
//! including K-means, DBSCAN, and hierarchical clustering.

use super::PointCloud;
use crate::geometry::Point;

/// K-means clustering
pub struct KMeans {
    /// Number of clusters
    k: usize,
    /// Maximum number of iterations
    max_iterations: usize,
    /// Tolerance for convergence
    #[allow(dead_code)]
    tolerance: f64,
}

impl KMeans {
    /// Create a new K-means clustering
    pub fn new(k: usize, max_iterations: usize, tolerance: f64) -> Self {
        Self {
            k,
            max_iterations,
            tolerance,
        }
    }

    /// Apply the clustering to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> (PointCloud, Vec<usize>) {
        use rand::Rng;
        
        if cloud.len() < self.k {
            // Not enough points for k clusters
            let labeled_cloud = cloud.clone();
            let labels: Vec<usize> = (0..cloud.len()).collect();
            return (labeled_cloud, labels);
        }
        
        let mut rng = rand::rng();
        
        // Initialize centroids randomly
        let mut centroids: Vec<Point> = Vec::new();
        let mut used_indices = std::collections::HashSet::new();
        
        while centroids.len() < self.k {
            let index = rng.random_range(0..cloud.len());
            if !used_indices.contains(&index) {
                centroids.push(cloud.points()[index]);
                used_indices.insert(index);
            }
        }
        
        let mut labels: Vec<usize> = vec![0; cloud.len()];
        let mut new_labels: Vec<usize> = vec![0; cloud.len()];
        
        for _ in 0..self.max_iterations {
            // Assign points to clusters
            for (i, point) in cloud.points().iter().enumerate() {
                let mut min_distance = f64::MAX;
                let mut closest_centroid = 0;
                
                for (j, centroid) in centroids.iter().enumerate() {
                    let distance = point.distance(centroid);
                    if distance < min_distance {
                        min_distance = distance;
                        closest_centroid = j;
                    }
                }
                
                new_labels[i] = closest_centroid;
            }
            
            // Check for convergence
            let mut converged = true;
            for (i, &label) in new_labels.iter().enumerate() {
                if label != labels[i] {
                    converged = false;
                    break;
                }
            }
            
            if converged {
                break;
            }
            
            // Update centroids
            labels = new_labels.clone();
            
            for j in 0..self.k {
                let mut sum = Point::origin();
                let mut count = 0;
                
                for (i, &label) in labels.iter().enumerate() {
                    if label == j {
                        sum += cloud.points()[i];
                        count += 1;
                    }
                }
                
                if count > 0 {
                    let sum_vec = sum - Point::origin();
                    let avg_vec = sum_vec / count as f64;
                    centroids[j] = Point::origin() + avg_vec;
                }
            }
        }
        
        // Create labeled point cloud (assign colors based on cluster)
        let mut labeled_cloud = PointCloud::new();
        let colors = self.generate_cluster_colors();
        
        for (i, point) in cloud.points().iter().enumerate() {
            let cluster = labels[i];
            let color = colors[cluster % colors.len()];
            
            if let Some(normals) = cloud.normals() {
                labeled_cloud.add_point_with_normal_and_color(*point, normals[i], color);
            } else {
                labeled_cloud.add_point_with_color(*point, color);
            }
        }
        
        (labeled_cloud, labels)
    }

    /// Generate colors for clusters
    fn generate_cluster_colors(&self) -> Vec<(u8, u8, u8)> {
        let colors = vec![
            (255, 0, 0),     // Red
            (0, 255, 0),     // Green
            (0, 0, 255),     // Blue
            (255, 255, 0),   // Yellow
            (255, 0, 255),   // Magenta
            (0, 255, 255),   // Cyan
            (128, 0, 0),     // Maroon
            (0, 128, 0),     // Dark Green
            (0, 0, 128),     // Navy
            (128, 128, 0),   // Olive
            (128, 0, 128),   // Purple
            (0, 128, 128),   // Teal
        ];
        colors
    }
}

/// DBSCAN clustering
pub struct DBSCAN {
    /// Epsilon radius
    epsilon: f64,
    /// Minimum number of points in a cluster
    min_points: usize,
}

impl DBSCAN {
    /// Create a new DBSCAN clustering
    pub fn new(epsilon: f64, min_points: usize) -> Self {
        Self {
            epsilon,
            min_points,
        }
    }

    /// Apply the clustering to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> (PointCloud, Vec<usize>) {
        let mut labels: Vec<usize> = vec![usize::MAX; cloud.len()]; // usize::MAX for unassigned
        let mut cluster_id = 0;
        
        for i in 0..cloud.len() {
            if labels[i] != usize::MAX {
                continue;
            }
            
            // Find neighbors
            let neighbors = self.find_neighbors(cloud, i);
            
            if neighbors.len() < self.min_points {
                // Mark as noise
                labels[i] = usize::MAX - 1;
            } else {
                // Start a new cluster
                self.expand_cluster(cloud, &mut labels, i, neighbors, &mut cluster_id);
            }
        }
        
        // Create labeled point cloud
        let mut labeled_cloud = PointCloud::new();
        let colors = self.generate_cluster_colors();
        
        for (i, point) in cloud.points().iter().enumerate() {
            let cluster = labels[i];
            let color = if cluster == usize::MAX - 1 {
                (128, 128, 128) // Gray for noise
            } else {
                colors[cluster % colors.len()]
            };
            
            if let Some(normals) = cloud.normals() {
                labeled_cloud.add_point_with_normal_and_color(*point, normals[i], color);
            } else {
                labeled_cloud.add_point_with_color(*point, color);
            }
        }
        
        (labeled_cloud, labels)
    }

    /// Find neighbors of a point
    fn find_neighbors(&self, cloud: &PointCloud, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let point = cloud.points()[index];
        
        for (i, other_point) in cloud.points().iter().enumerate() {
            if i != index && point.distance(other_point) <= self.epsilon {
                neighbors.push(i);
            }
        }
        
        neighbors
    }

    /// Expand a cluster
    fn expand_cluster(&self, cloud: &PointCloud, labels: &mut Vec<usize>, index: usize, neighbors: Vec<usize>, cluster_id: &mut usize) {
        labels[index] = *cluster_id;
        
        let mut queue = std::collections::VecDeque::from(neighbors);
        
        while let Some(current) = queue.pop_front() {
            if labels[current] == usize::MAX {
                labels[current] = *cluster_id;
                
                let current_neighbors = self.find_neighbors(cloud, current);
                if current_neighbors.len() >= self.min_points {
                    queue.extend(current_neighbors);
                }
            } else if labels[current] == usize::MAX - 1 {
                labels[current] = *cluster_id;
            }
        }
        
        *cluster_id += 1;
    }

    /// Generate colors for clusters
    fn generate_cluster_colors(&self) -> Vec<(u8, u8, u8)> {
        let colors = vec![
            (255, 0, 0),     // Red
            (0, 255, 0),     // Green
            (0, 0, 255),     // Blue
            (255, 255, 0),   // Yellow
            (255, 0, 255),   // Magenta
            (0, 255, 255),   // Cyan
            (128, 0, 0),     // Maroon
            (0, 128, 0),     // Dark Green
            (0, 0, 128),     // Navy
            (128, 128, 0),   // Olive
            (128, 0, 128),   // Purple
            (0, 128, 128),   // Teal
        ];
        colors
    }
}

/// Hierarchical clustering
pub struct HierarchicalClustering {
    /// Number of clusters
    k: usize,
}

impl HierarchicalClustering {
    /// Create a new hierarchical clustering
    pub fn new(k: usize) -> Self {
        Self {
            k,
        }
    }

    /// Apply the clustering to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> (PointCloud, Vec<usize>) {
        if cloud.len() <= self.k {
            // Not enough points for k clusters
            let labeled_cloud = cloud.clone();
            let labels: Vec<usize> = (0..cloud.len()).collect();
            return (labeled_cloud, labels);
        }
        
        // Initialize each point as its own cluster
        let mut clusters: Vec<Vec<usize>> = (0..cloud.len()).map(|i| vec![i]).collect();
        
        // Calculate distance matrix
        let mut distances = Vec::new();
        for i in 0..cloud.len() {
            for j in i+1..cloud.len() {
                let distance = cloud.points()[i].distance(&cloud.points()[j]);
                distances.push((distance, i, j));
            }
        }
        
        // Sort distances
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        // Merge clusters until we have k clusters
        while clusters.len() > self.k {
            // Find the closest pair of clusters
            let mut min_distance = f64::MAX;
            let mut closest_pair = (0, 1);
            
            for i in 0..clusters.len() {
                for j in i+1..clusters.len() {
                    let distance = self.cluster_distance(cloud, &clusters[i], &clusters[j]);
                    if distance < min_distance {
                        min_distance = distance;
                        closest_pair = (i, j);
                    }
                }
            }
            
            // Merge the closest pair
            let (i, j) = closest_pair;
            let mut merged = clusters[i].clone();
            merged.extend(&clusters[j]);
            
            // Remove the old clusters
            if i < j {
                clusters.remove(j);
                clusters.remove(i);
            } else {
                clusters.remove(i);
                clusters.remove(j);
            }
            
            // Add the merged cluster
            clusters.push(merged);
        }
        
        // Assign labels
        let mut labels: Vec<usize> = vec![0; cloud.len()];
        for (cluster_id, cluster) in clusters.iter().enumerate() {
            for &point_id in cluster {
                labels[point_id] = cluster_id;
            }
        }
        
        // Create labeled point cloud
        let mut labeled_cloud = PointCloud::new();
        let colors = self.generate_cluster_colors();
        
        for (i, point) in cloud.points().iter().enumerate() {
            let cluster = labels[i];
            let color = colors[cluster % colors.len()];
            
            if let Some(normals) = cloud.normals() {
                labeled_cloud.add_point_with_normal_and_color(*point, normals[i], color);
            } else {
                labeled_cloud.add_point_with_color(*point, color);
            }
        }
        
        (labeled_cloud, labels)
    }

    /// Calculate distance between two clusters
    fn cluster_distance(&self, cloud: &PointCloud, cluster1: &[usize], cluster2: &[usize]) -> f64 {
        // Use average linkage
        let mut sum = 0.0;
        let mut count = 0;
        
        for &i in cluster1 {
            for &j in cluster2 {
                sum += cloud.points()[i].distance(&cloud.points()[j]);
                count += 1;
            }
        }
        
        sum / count as f64
    }

    /// Generate colors for clusters
    fn generate_cluster_colors(&self) -> Vec<(u8, u8, u8)> {
        let colors = vec![
            (255, 0, 0),     // Red
            (0, 255, 0),     // Green
            (0, 0, 255),     // Blue
            (255, 255, 0),   // Yellow
            (255, 0, 255),   // Magenta
            (0, 255, 255),   // Cyan
            (128, 0, 0),     // Maroon
            (0, 128, 0),     // Dark Green
            (0, 0, 128),     // Navy
            (128, 128, 0),   // Olive
            (128, 0, 128),   // Purple
            (0, 128, 128),   // Teal
        ];
        colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_kmeans_clustering() {
        let mut cloud = PointCloud::new();
        
        // Add points in two clusters
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64 + 100.0, i as f64 + 100.0, i as f64 + 100.0));
        }
        
        let kmeans = KMeans::new(2, 100, 1e-5);
        let (labeled_cloud, labels) = kmeans.apply(&cloud);
        
        assert_eq!(labeled_cloud.len(), 100);
        assert_eq!(labels.len(), 100);
    }

    #[test]
    fn test_dbscan_clustering() {
        let mut cloud = PointCloud::new();
        
        // Add points in two clusters
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64 + 100.0, i as f64 + 100.0, i as f64 + 100.0));
        }
        
        let dbscan = DBSCAN::new(10.0, 5);
        let (labeled_cloud, labels) = dbscan.apply(&cloud);
        
        assert_eq!(labeled_cloud.len(), 100);
        assert_eq!(labels.len(), 100);
    }

    #[test]
    fn test_hierarchical_clustering() {
        let mut cloud = PointCloud::new();
        
        // Add points in two clusters
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        for i in 0..50 {
            cloud.add_point(Point::new(i as f64 + 100.0, i as f64 + 100.0, i as f64 + 100.0));
        }
        
        let hierarchical = HierarchicalClustering::new(2);
        let (labeled_cloud, labels) = hierarchical.apply(&cloud);
        
        assert_eq!(labeled_cloud.len(), 100);
        assert_eq!(labels.len(), 100);
    }
}
