//! Point cloud filtering module
//! 
//! This module provides various filtering algorithms for point clouds,
//! including statistical outlier removal, radius outlier removal, and more.

use super::PointCloud;
use crate::geometry::{Point, Vector};

/// Statistical outlier removal filter
pub struct StatisticalOutlierRemoval {
    /// Number of nearest neighbors to consider
    k: usize,
    /// Standard deviation multiplier
    std_dev_mul: f64,
}

impl StatisticalOutlierRemoval {
    /// Create a new statistical outlier removal filter
    pub fn new(k: usize, std_dev_mul: f64) -> Self {
        Self {
            k,
            std_dev_mul,
        }
    }

    /// Apply the filter to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        if cloud.len() < self.k {
            return cloud.clone();
        }
        
        // Calculate distances to k nearest neighbors for each point
        let mut distances = Vec::new();
        for (i, point) in cloud.points().iter().enumerate() {
            let mut dists = Vec::new();
            for (j, other_point) in cloud.points().iter().enumerate() {
                if i != j {
                    dists.push(point.distance(other_point));
                }
            }
            
            // Sort distances and take the first k
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let k_distances = &dists[..self.k];
            
            // Calculate average distance
            let avg_distance = k_distances.iter().sum::<f64>() / self.k as f64;
            distances.push(avg_distance);
        }
        
        // Calculate mean and standard deviation of average distances
        let mean = distances.iter().sum::<f64>() / distances.len() as f64;
        let variance = distances.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / distances.len() as f64;
        let std_dev = variance.sqrt();
        
        // Threshold for outlier detection
        let threshold = mean + self.std_dev_mul * std_dev;
        
        // Create filtered point cloud
        let mut filtered = PointCloud::new();
        
        for (i, point) in cloud.points().iter().enumerate() {
            if distances[i] < threshold {
                if let Some(normals) = cloud.normals() {
                    if let Some(colors) = cloud.colors() {
                        filtered.add_point_with_normal_and_color(*point, normals[i], colors[i]);
                    } else {
                        filtered.add_point_with_normal(*point, normals[i]);
                    }
                } else if let Some(colors) = cloud.colors() {
                    filtered.add_point_with_color(*point, colors[i]);
                } else {
                    filtered.add_point(*point);
                }
            }
        }
        
        filtered
    }
}

/// Radius outlier removal filter
pub struct RadiusOutlierRemoval {
    /// Radius for neighbor search
    radius: f64,
    /// Minimum number of neighbors required
    min_neighbors: usize,
}

impl RadiusOutlierRemoval {
    /// Create a new radius outlier removal filter
    pub fn new(radius: f64, min_neighbors: usize) -> Self {
        Self {
            radius,
            min_neighbors,
        }
    }

    /// Apply the filter to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        let mut filtered = PointCloud::new();
        
        for (i, point) in cloud.points().iter().enumerate() {
            let mut neighbor_count = 0;
            
            for (j, other_point) in cloud.points().iter().enumerate() {
                if i != j && point.distance(other_point) < self.radius {
                    neighbor_count += 1;
                    if neighbor_count >= self.min_neighbors {
                        break;
                    }
                }
            }
            
            if neighbor_count >= self.min_neighbors {
                if let Some(normals) = cloud.normals() {
                    if let Some(colors) = cloud.colors() {
                        filtered.add_point_with_normal_and_color(*point, normals[i], colors[i]);
                    } else {
                        filtered.add_point_with_normal(*point, normals[i]);
                    }
                } else if let Some(colors) = cloud.colors() {
                    filtered.add_point_with_color(*point, colors[i]);
                } else {
                    filtered.add_point(*point);
                }
            }
        }
        
        filtered
    }
}

/// Voxel grid filter
pub struct VoxelGrid {
    /// Voxel size
    voxel_size: f64,
}

impl VoxelGrid {
    /// Create a new voxel grid filter
    pub fn new(voxel_size: f64) -> Self {
        Self {
            voxel_size,
        }
    }

    /// Apply the filter to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        use std::collections::HashMap;
        
        let mut voxel_map: HashMap<(i32, i32, i32), Vec<(Point, Option<Vector>, Option<(u8, u8, u8)>)>> = HashMap::new();
        
        // Assign points to voxels
        for (i, point) in cloud.points().iter().enumerate() {
            let voxel_key = (
                (point.x / self.voxel_size).floor() as i32,
                (point.y / self.voxel_size).floor() as i32,
                (point.z / self.voxel_size).floor() as i32,
            );
            
            let normal = cloud.normals().map(|n| n[i]);
            let color = cloud.colors().map(|c| c[i]);
            
            voxel_map.entry(voxel_key)
                .or_insert_with(Vec::new)
                .push((*point, normal, color));
        }
        
        // Create filtered point cloud by averaging points in each voxel
        let mut filtered = PointCloud::new();
        
        for points in voxel_map.values() {
            if !points.is_empty() {
                // Calculate centroid
                let (sum_x, sum_y, sum_z) = points.iter()
                    .fold((0.0, 0.0, 0.0), |(sx, sy, sz), (p, _, _)| (sx + p.x, sy + p.y, sz + p.z));
                let count = points.len() as f64;
                let centroid = Point::new(sum_x / count, sum_y / count, sum_z / count);
                
                // Calculate average normal if available
                let normal = if points[0].1.is_some() {
                    let avg_normal = points.iter()
                        .fold(Vector::zero(), |sum, (_, n, _)| sum + n.unwrap())
                        .normalized();
                    Some(avg_normal)
                } else {
                    None
                };
                
                // Calculate average color if available
                let color = if points[0].2.is_some() {
                    let avg_color = points.iter()
                        .fold((0, 0, 0), |sum, (_, _, c)| {
                            let c = c.unwrap();
                            (sum.0 + c.0 as u32, sum.1 + c.1 as u32, sum.2 + c.2 as u32)
                        });
                    Some((
                        (avg_color.0 / points.len() as u32) as u8,
                        (avg_color.1 / points.len() as u32) as u8,
                        (avg_color.2 / points.len() as u32) as u8,
                    ))
                } else {
                    None
                };
                
                // Add the centroid to the filtered cloud
                match (normal, color) {
                    (Some(n), Some(c)) => filtered.add_point_with_normal_and_color(centroid, n, c),
                    (Some(n), None) => filtered.add_point_with_normal(centroid, n),
                    (None, Some(c)) => filtered.add_point_with_color(centroid, c),
                    (None, None) => filtered.add_point(centroid),
                }
            }
        }
        
        filtered
    }
}

/// Pass-through filter
pub struct PassThrough {
    /// Axis to filter along
    axis: char,
    /// Minimum value
    min: f64,
    /// Maximum value
    max: f64,
}

impl PassThrough {
    /// Create a new pass-through filter
    pub fn new(axis: char, min: f64, max: f64) -> Self {
        Self {
            axis: axis.to_lowercase().next().unwrap(),
            min,
            max,
        }
    }

    /// Apply the filter to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        let mut filtered = PointCloud::new();
        
        for (i, point) in cloud.points().iter().enumerate() {
            let value = match self.axis {
                'x' => point.x,
                'y' => point.y,
                'z' => point.z,
                _ => point.x,
            };
            
            if value >= self.min && value <= self.max {
                if let Some(normals) = cloud.normals() {
                    if let Some(colors) = cloud.colors() {
                        filtered.add_point_with_normal_and_color(*point, normals[i], colors[i]);
                    } else {
                        filtered.add_point_with_normal(*point, normals[i]);
                    }
                } else if let Some(colors) = cloud.colors() {
                    filtered.add_point_with_color(*point, colors[i]);
                } else {
                    filtered.add_point(*point);
                }
            }
        }
        
        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_statistical_outlier_removal() {
        let mut cloud = PointCloud::new();
        
        // Add inliers
        for i in 0..100 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        // Add outliers
        cloud.add_point(Point::new(1000.0, 1000.0, 1000.0));
        cloud.add_point(Point::new(-1000.0, -1000.0, -1000.0));
        
        let filter = StatisticalOutlierRemoval::new(5, 1.0);
        let filtered = filter.apply(&cloud);
        
        // Should remove the two outliers
        assert_eq!(filtered.len(), 100);
    }

    #[test]
    fn test_radius_outlier_removal() {
        let mut cloud = PointCloud::new();
        
        // Add a cluster of points
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    cloud.add_point(Point::new(i as f64, j as f64, k as f64));
                }
            }
        }
        
        // Add an isolated point
        cloud.add_point(Point::new(100.0, 100.0, 100.0));
        
        let filter = RadiusOutlierRemoval::new(2.0, 5);
        let filtered = filter.apply(&cloud);
        
        // Should remove the isolated point
        assert_eq!(filtered.len(), 1000);
    }

    #[test]
    fn test_voxel_grid() {
        let mut cloud = PointCloud::new();
        
        // Add points in a grid
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    cloud.add_point(Point::new(i as f64 * 0.1, j as f64 * 0.1, k as f64 * 0.1));
                }
            }
        }
        
        let filter = VoxelGrid::new(0.5);
        let filtered = filter.apply(&cloud);
        
        // Should significantly reduce the number of points
        assert!(filtered.len() < cloud.len());
    }

    #[test]
    fn test_pass_through() {
        let mut cloud = PointCloud::new();
        
        // Add points with various x values
        for i in 0..20 {
            cloud.add_point(Point::new(i as f64, 0.0, 0.0));
        }
        
        let filter = PassThrough::new('x', 5.0, 15.0);
        let filtered = filter.apply(&cloud);
        
        // Should keep points with x between 5 and 15
        assert_eq!(filtered.len(), 11); // 5,6,7,8,9,10,11,12,13,14,15
    }
}
