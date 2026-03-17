//! Point cloud sampling module
//! 
//! This module provides various sampling algorithms for point clouds,
//! including random sampling, uniform sampling, and Poisson disk sampling.

use super::PointCloud;
use crate::geometry::Point;

/// Random sampling
pub struct RandomSampling {
    /// Number of points to sample
    num_points: usize,
}

impl RandomSampling {
    /// Create a new random sampling
    pub fn new(num_points: usize) -> Self {
        Self {
            num_points,
        }
    }

    /// Apply the sampling to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let mut sampled = PointCloud::new();
        
        if cloud.len() <= self.num_points {
            return cloud.clone();
        }
        
        // Create a list of indices and shuffle them
        let mut indices: Vec<usize> = (0..cloud.len()).collect();
        rng.shuffle(&mut indices);
        
        // Take the first num_points indices
        for &i in &indices[..self.num_points] {
            let point = cloud.points()[i];
            
            if let Some(normals) = cloud.normals() {
                if let Some(colors) = cloud.colors() {
                    sampled.add_point_with_normal_and_color(point, normals[i], colors[i]);
                } else {
                    sampled.add_point_with_normal(point, normals[i]);
                }
            } else if let Some(colors) = cloud.colors() {
                sampled.add_point_with_color(point, colors[i]);
            } else {
                sampled.add_point(point);
            }
        }
        
        sampled
    }
}

/// Uniform sampling
pub struct UniformSampling {
    /// Sampling step
    step: f64,
}

impl UniformSampling {
    /// Create a new uniform sampling
    pub fn new(step: f64) -> Self {
        Self {
            step,
        }
    }

    /// Apply the sampling to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        use std::collections::HashSet;
        
        let mut sampled = PointCloud::new();
        let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();
        
        for (i, point) in cloud.points().iter().enumerate() {
            // Calculate grid cell
            let cell = (
                (point.x / self.step).floor() as i32,
                (point.y / self.step).floor() as i32,
                (point.z / self.step).floor() as i32,
            );
            
            // If this cell is not yet occupied, add the point
            if !grid.contains(&cell) {
                grid.insert(cell);
                
                if let Some(normals) = cloud.normals() {
                    if let Some(colors) = cloud.colors() {
                        sampled.add_point_with_normal_and_color(*point, normals[i], colors[i]);
                    } else {
                        sampled.add_point_with_normal(*point, normals[i]);
                    }
                } else if let Some(colors) = cloud.colors() {
                    sampled.add_point_with_color(*point, colors[i]);
                } else {
                    sampled.add_point(*point);
                }
            }
        }
        
        sampled
    }
}

/// Poisson disk sampling
pub struct PoissonDiskSampling {
    /// Radius of the disks
    radius: f64,
    /// Number of samples to generate
    num_samples: usize,
    /// Number of attempts per sample
    num_attempts: usize,
}

impl PoissonDiskSampling {
    /// Create a new Poisson disk sampling
    pub fn new(radius: f64, num_samples: usize, num_attempts: usize) -> Self {
        Self {
            radius,
            num_samples,
            num_attempts,
        }
    }

    /// Apply the sampling to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        use rand::Rng;
        use std::collections::{HashSet, VecDeque};
        
        let mut sampled = PointCloud::new();
        
        if cloud.is_empty() {
            return sampled;
        }
        
        // Calculate bounding box
        let (min, max) = cloud.bounding_box();
        
        // Create a grid for efficient neighbor search
        let cell_size = self.radius / 2.0_f64.sqrt();
        let grid_width = ((max.x - min.x) / cell_size).ceil() as i32;
        let grid_height = ((max.y - min.y) / cell_size).ceil() as i32;
        let grid_depth = ((max.z - min.z) / cell_size).ceil() as i32;
        
        let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();
        let mut points: Vec<Point> = Vec::new();
        let mut active_list: VecDeque<Point> = VecDeque::new();
        
        // Generate first point randomly
        let mut rng = rand::thread_rng();
        let first_point = Point::new(
            min.x + rng.gen_range(0.0..(max.x - min.x)),
            min.y + rng.gen_range(0.0..(max.y - min.y)),
            min.z + rng.gen_range(0.0..(max.z - min.z)),
        );
        
        // Add first point to grid and active list
        let cell = self.point_to_cell(&first_point, &min, cell_size);
        grid.insert(cell);
        points.push(first_point);
        active_list.push_back(first_point);
        
        // Generate samples
        while !active_list.is_empty() && points.len() < self.num_samples {
            let random_index = rng.gen_range(0..active_list.len());
            let center = active_list[random_index];
            let mut found = false;
            
            for _ in 0..self.num_attempts {
                // Generate random point in annulus [radius, 2*radius]
                let angle1 = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                let angle2 = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                let distance = self.radius + rng.gen_range(0.0..self.radius);
                
                let x = center.x + distance * angle1.cos() * angle2.cos();
                let y = center.y + distance * angle1.sin() * angle2.cos();
                let z = center.z + distance * angle2.sin();
                
                let candidate = Point::new(x, y, z);
                
                // Check if candidate is within bounds
                if candidate.x < min.x || candidate.x > max.x ||
                   candidate.y < min.y || candidate.y > max.y ||
                   candidate.z < min.z || candidate.z > max.z {
                    continue;
                }
                
                // Check if candidate is far enough from existing points
                let cell = self.point_to_cell(&candidate, &min, cell_size);
                let mut too_close = false;
                
                // Check neighboring cells
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            let neighbor_cell = (cell.0 + dx, cell.1 + dy, cell.2 + dz);
                            if grid.contains(&neighbor_cell) {
                                // Check all points in this cell
                                for point in &points {
                                    if candidate.distance(point) < self.radius {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if too_close {
                                    break;
                                }
                            }
                        }
                        if too_close {
                            break;
                        }
                    }
                    if too_close {
                        break;
                    }
                }
                
                if !too_close {
                    // Add candidate to grid, points, and active list
                    grid.insert(cell);
                    points.push(candidate);
                    active_list.push_back(candidate);
                    found = true;
                    
                    if points.len() >= self.num_samples {
                        break;
                    }
                }
            }
            
            if !found {
                active_list.remove(random_index);
            }
        }
        
        // Add sampled points to point cloud
        for point in points {
            sampled.add_point(point);
        }
        
        sampled
    }

    /// Convert a point to a grid cell
    fn point_to_cell(&self, point: &Point, min: &Point, cell_size: f64) -> (i32, i32, i32) {
        (
            ((point.x - min.x) / cell_size).floor() as i32,
            ((point.y - min.y) / cell_size).floor() as i32,
            ((point.z - min.z) / cell_size).floor() as i32,
        )
    }
}

/// Farthest point sampling
pub struct FarthestPointSampling {
    /// Number of points to sample
    num_points: usize,
}

impl FarthestPointSampling {
    /// Create a new farthest point sampling
    pub fn new(num_points: usize) -> Self {
        Self {
            num_points,
        }
    }

    /// Apply the sampling to a point cloud
    pub fn apply(&self, cloud: &PointCloud) -> PointCloud {
        let mut sampled = PointCloud::new();
        
        if cloud.len() <= self.num_points {
            return cloud.clone();
        }
        
        let mut selected = vec![false; cloud.len()];
        let mut distances = vec![f64::MAX; cloud.len()];
        
        // Select first point randomly
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let first_index = rng.gen_range(0..cloud.len());
        selected[first_index] = true;
        
        let first_point = cloud.points()[first_index];
        if let Some(normals) = cloud.normals() {
            if let Some(colors) = cloud.colors() {
                sampled.add_point_with_normal_and_color(first_point, normals[first_index], colors[first_index]);
            } else {
                sampled.add_point_with_normal(first_point, normals[first_index]);
            }
        } else if let Some(colors) = cloud.colors() {
            sampled.add_point_with_color(first_point, colors[first_index]);
        } else {
            sampled.add_point(first_point);
        }
        
        // Select remaining points
        for _ in 1..self.num_points {
            // Update distances to the nearest selected point
            for (i, point) in cloud.points().iter().enumerate() {
                if !selected[i] {
                    let mut min_dist = distances[i];
                    for (j, selected_point) in cloud.points().iter().enumerate() {
                        if selected[j] {
                            let dist = point.distance(selected_point);
                            if dist < min_dist {
                                min_dist = dist;
                            }
                        }
                    }
                    distances[i] = min_dist;
                }
            }
            
            // Find the point with the maximum distance
            let mut max_dist = 0.0;
            let mut max_index = 0;
            
            for (i, &dist) in distances.iter().enumerate() {
                if !selected[i] && dist > max_dist {
                    max_dist = dist;
                    max_index = i;
                }
            }
            
            // Select this point
            selected[max_index] = true;
            let point = cloud.points()[max_index];
            
            if let Some(normals) = cloud.normals() {
                if let Some(colors) = cloud.colors() {
                    sampled.add_point_with_normal_and_color(point, normals[max_index], colors[max_index]);
                } else {
                    sampled.add_point_with_normal(point, normals[max_index]);
                }
            } else if let Some(colors) = cloud.colors() {
                sampled.add_point_with_color(point, colors[max_index]);
            } else {
                sampled.add_point(point);
            }
        }
        
        sampled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_random_sampling() {
        let mut cloud = PointCloud::new();
        
        for i in 0..100 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        let sampler = RandomSampling::new(10);
        let sampled = sampler.apply(&cloud);
        
        assert_eq!(sampled.len(), 10);
    }

    #[test]
    fn test_uniform_sampling() {
        let mut cloud = PointCloud::new();
        
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    cloud.add_point(Point::new(i as f64 * 0.1, j as f64 * 0.1, k as f64 * 0.1));
                }
            }
        }
        
        let sampler = UniformSampling::new(0.5);
        let sampled = sampler.apply(&cloud);
        
        // Should significantly reduce the number of points
        assert!(sampled.len() < cloud.len());
    }

    #[test]
    fn test_farthest_point_sampling() {
        let mut cloud = PointCloud::new();
        
        for i in 0..100 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }
        
        let sampler = FarthestPointSampling::new(10);
        let sampled = sampler.apply(&cloud);
        
        assert_eq!(sampled.len(), 10);
    }
}
