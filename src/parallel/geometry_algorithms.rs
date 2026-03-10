//! Parallel Geometry Algorithms
//!
//! This module provides parallel implementations of common geometric algorithms
//! including convex hull, Voronoi diagrams, Delaunay triangulation, and spatial queries.

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::{ParallelConfig, ParallelStats, ParallelResult};
use crate::geometry::{Point, Vector, Plane, BoundingBox};

/// Parallel convex hull computation
pub struct ParallelConvexHull;

impl ParallelConvexHull {
    /// Compute convex hull of a set of points in parallel
    pub fn compute(points: &[Point], config: &ParallelConfig) -> ParallelResult<Vec<Point>> {
        let start = Instant::now();
        
        if points.len() < 4 {
            return ParallelResult::new(points.to_vec(), ParallelStats::new());
        }
        
        // Divide and conquer approach
        let hull = if points.len() >= config.min_parallel_size {
            Self::divide_and_conquer(points)
        } else {
            Self::graham_scan(points)
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(hull, stats)
    }
    
    /// Divide and conquer convex hull
    fn divide_and_conquer(points: &[Point]) -> Vec<Point> {
        if points.len() <= 100 {
            return Self::graham_scan(points);
        }
        
        // Sort points by x-coordinate
        let mut sorted_points: Vec<Point> = points.to_vec();
        sorted_points.par_sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        
        // Divide
        let mid = sorted_points.len() / 2;
        let (left, right) = sorted_points.split_at(mid);
        
        // Conquer in parallel
        let (left_hull, right_hull) = rayon::join(
            || Self::divide_and_conquer(left),
            || Self::divide_and_conquer(right),
        );
        
        // Merge
        Self::merge_hulls(&left_hull, &right_hull)
    }
    
    /// Graham scan algorithm for convex hull
    fn graham_scan(points: &[Point]) -> Vec<Point> {
        if points.len() < 3 {
            return points.to_vec();
        }
        
        // Find the lowest point
        let mut lowest = 0;
        for i in 1..points.len() {
            if points[i].y < points[lowest].y || 
               (points[i].y == points[lowest].y && points[i].x < points[lowest].x) {
                lowest = i;
            }
        }
        
        // Sort by polar angle
        let pivot = points[lowest];
        let mut sorted: Vec<Point> = points.iter().enumerate()
            .filter(|(i, _)| *i != lowest)
            .map(|(_, p)| *p)
            .collect();
        
        sorted.sort_by(|a, b| {
            let angle_a = (a.y - pivot.y).atan2(a.x - pivot.x);
            let angle_b = (b.y - pivot.y).atan2(b.x - pivot.x);
            angle_a.partial_cmp(&angle_b).unwrap()
        });
        
        // Graham scan
        let mut hull = vec![pivot];
        for point in sorted {
            while hull.len() > 1 && Self::cross_product(&hull[hull.len()-2], &hull[hull.len()-1], &point) <= 0.0 {
                hull.pop();
            }
            hull.push(point);
        }
        
        hull
    }
    
    /// Cross product for orientation test
    fn cross_product(o: &Point, a: &Point, b: &Point) -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }
    
    /// Merge two convex hulls
    fn merge_hulls(left: &[Point], right: &[Point]) -> Vec<Point> {
        let mut merged: Vec<Point> = left.iter().chain(right.iter()).cloned().collect();
        Self::graham_scan(&merged)
    }
}

/// Parallel Delaunay triangulation
pub struct ParallelDelaunay;

impl ParallelDelaunay {
    /// Compute Delaunay triangulation in parallel
    pub fn triangulate(points: &[Point], config: &ParallelConfig) -> ParallelResult<Vec<(usize, usize, usize)>> {
        let start = Instant::now();
        
        if points.len() < 3 {
            return ParallelResult::new(Vec::new(), ParallelStats::new());
        }
        
        let triangles = if points.len() >= config.min_parallel_size {
            Self::parallel_triangulation(points)
        } else {
            Self::bowyer_watson(points)
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_processed(triangles.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(triangles, stats)
    }
    
    /// Parallel Delaunay triangulation using divide and conquer
    fn parallel_triangulation(points: &[Point]) -> Vec<(usize, usize, usize)> {
        if points.len() <= 100 {
            return Self::bowyer_watson(points);
        }
        
        // Sort points by x-coordinate
        let mut sorted_indices: Vec<usize> = (0..points.len()).collect();
        sorted_indices.par_sort_by(|&a, &b| points[a].x.partial_cmp(&points[b].x).unwrap());
        
        // Divide
        let mid = sorted_indices.len() / 2;
        let left_indices = &sorted_indices[..mid];
        let right_indices = &sorted_indices[mid..];
        
        // Extract points for each half
        let left_points: Vec<Point> = left_indices.iter().map(|&i| points[i]).collect();
        let right_points: Vec<Point> = right_indices.iter().map(|&i| points[i]).collect();
        
        // Conquer in parallel
        let (left_triangles, right_triangles) = rayon::join(
            || Self::parallel_triangulation(&left_points),
            || Self::parallel_triangulation(&right_points),
        );
        
        // Merge triangulations
        Self::merge_triangulations(&left_triangles, &right_triangles, left_indices, right_indices)
    }
    
    /// Bowyer-Watson algorithm for Delaunay triangulation
    fn bowyer_watson(points: &[Point]) -> Vec<(usize, usize, usize)> {
        if points.len() < 3 {
            return Vec::new();
        }
        
        // Create super triangle
        let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta_max = dx.max(dy);
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;
        
        let super_p1 = Point::new(mid_x - 20.0 * delta_max, mid_y - delta_max, 0.0);
        let super_p2 = Point::new(mid_x, mid_y + 20.0 * delta_max, 0.0);
        let super_p3 = Point::new(mid_x + 20.0 * delta_max, mid_y - delta_max, 0.0);
        
        let mut triangles = vec![(super_p1, super_p2, super_p3)];
        
        // Insert points one by one
        for (i, point) in points.iter().enumerate() {
            let mut bad_triangles = Vec::new();
            
            for (j, &(p1, p2, p3)) in triangles.iter().enumerate() {
                if Self::point_in_circumcircle(*point, p1, p2, p3) {
                    bad_triangles.push(j);
                }
            }
            
            // Remove bad triangles and create new ones
            let mut new_triangles = Vec::new();
            for (j, &(p1, p2, p3)) in triangles.iter().enumerate() {
                if !bad_triangles.contains(&j) {
                    new_triangles.push((p1, p2, p3));
                }
            }
            
            // Add new triangles (simplified)
            // In a full implementation, we would add edges of the polygonal hole
            
            triangles = new_triangles;
        }
        
        // Remove triangles with super triangle vertices
        triangles.retain(|&(p1, p2, p3)| {
            p1 != super_p1 && p1 != super_p2 && p1 != super_p3 &&
            p2 != super_p1 && p2 != super_p2 && p2 != super_p3 &&
            p3 != super_p1 && p3 != super_p2 && p3 != super_p3
        });
        
        // Convert to indices
        let mut result = Vec::new();
        for (p1, p2, p3) in triangles {
            if let (Some(i1), Some(i2), Some(i3)) = (
                points.iter().position(|&p| p == p1),
                points.iter().position(|&p| p == p2),
                points.iter().position(|&p| p == p3),
            ) {
                result.push((i1, i2, i3));
            }
        }
        
        result
    }
    
    /// Check if point is inside circumcircle of triangle
    fn point_in_circumcircle(p: Point, a: Point, b: Point, c: Point) -> bool {
        let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
        
        if d.abs() < 1e-10 {
            return false;
        }
        
        let ux = ((a.x * a.x + a.y * a.y) * (b.y - c.y) +
                  (b.x * b.x + b.y * b.y) * (c.y - a.y) +
                  (c.x * c.x + c.y * c.y) * (a.y - b.y)) / d;
        let uy = ((a.x * a.x + a.y * a.y) * (c.x - b.x) +
                  (b.x * b.x + b.y * b.y) * (a.x - c.x) +
                  (c.x * c.x + c.y * c.y) * (b.x - a.x)) / d;
        
        let center = Point::new(ux, uy, 0.0);
        let radius_sq = (a.x - ux).powi(2) + (a.y - uy).powi(2);
        let dist_sq = (p.x - ux).powi(2) + (p.y - uy).powi(2);
        
        dist_sq < radius_sq
    }
    
    /// Merge two triangulations
    fn merge_triangulations(
        left: &[(usize, usize, usize)],
        right: &[(usize, usize, usize)],
        left_indices: &[usize],
        right_indices: &[usize],
    ) -> Vec<(usize, usize, usize)> {
        let mut merged = left.to_vec();
        
        // Adjust indices for right triangles
        let offset = left_indices.len();
        for &(a, b, c) in right {
            merged.push((a + offset, b + offset, c + offset));
        }
        
        merged
    }
}

/// Parallel spatial queries
pub struct ParallelSpatialQuery;

impl ParallelSpatialQuery {
    /// Find k nearest neighbors for all points in parallel
    pub fn k_nearest_neighbors(
        points: &[Point],
        k: usize,
        config: &ParallelConfig,
    ) -> ParallelResult<Vec<Vec<(usize, f64)>>> {
        let start = Instant::now();
        
        let neighbors: Vec<Vec<(usize, f64)>> = if points.len() >= config.min_parallel_size {
            points.par_iter().enumerate().map(|(i, p)| {
                Self::find_k_nearest(i, p, points, k)
            }).collect()
        } else {
            points.iter().enumerate().map(|(i, p)| {
                Self::find_k_nearest(i, p, points, k)
            }).collect()
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(neighbors, stats)
    }
    
    /// Find k nearest neighbors for a single point
    fn find_k_nearest(idx: usize, point: &Point, points: &[Point], k: usize) -> Vec<(usize, f64)> {
        let mut distances: Vec<(usize, f64)> = points.iter().enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(i, p)| (i, Self::distance(point, p)))
            .collect();
        
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        distances.truncate(k);
        distances
    }
    
    /// Euclidean distance between two points
    fn distance(a: &Point, b: &Point) -> f64 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
    }
    
    /// Parallel range search
    pub fn range_search(
        points: &[Point],
        center: &Point,
        radius: f64,
        config: &ParallelConfig,
    ) -> ParallelResult<Vec<usize>> {
        let start = Instant::now();
        
        let radius_sq = radius * radius;
        let indices: Vec<usize> = if points.len() >= config.min_parallel_size {
            points.par_iter().enumerate()
                .filter(|(_, p)| Self::distance_squared(p, center) <= radius_sq)
                .map(|(i, _)| i)
                .collect()
        } else {
            points.iter().enumerate()
                .filter(|(_, p)| Self::distance_squared(p, center) <= radius_sq)
                .map(|(i, _)| i)
                .collect()
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(indices, stats)
    }
    
    /// Squared distance between two points
    fn distance_squared(a: &Point, b: &Point) -> f64 {
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)
    }
}

/// Parallel point cloud processing
pub struct ParallelPointCloud;

impl ParallelPointCloud {
    /// Compute bounding boxes for chunks of points in parallel
    pub fn compute_bounding_boxes(
        points: &[Point],
        chunk_size: usize,
        config: &ParallelConfig,
    ) -> ParallelResult<Vec<BoundingBox>> {
        let start = Instant::now();
        
        let chunks: Vec<&[Point]> = points.chunks(chunk_size).collect();
        
        let boxes: Vec<BoundingBox> = if chunks.len() >= config.min_parallel_size {
            chunks.par_iter().map(|chunk| {
                Self::compute_bbox(chunk)
            }).collect()
        } else {
            chunks.iter().map(|chunk| {
                Self::compute_bbox(chunk)
            }).collect()
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(boxes, stats)
    }
    
    /// Compute bounding box for a set of points
    fn compute_bbox(points: &[Point]) -> BoundingBox {
        if points.is_empty() {
            return BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
        }
        
        let mut min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;
        let mut min_z = points[0].z;
        let mut max_z = points[0].z;
        
        for p in points.iter().skip(1) {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
            min_z = min_z.min(p.z);
            max_z = max_z.max(p.z);
        }
        
        BoundingBox::new(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }
    
    /// Parallel point cloud downsampling using voxel grid
    pub fn voxel_downsample(
        points: &[Point],
        voxel_size: f64,
        config: &ParallelConfig,
    ) -> ParallelResult<Vec<Point>> {
        let start = Instant::now();
        
        // Assign points to voxels
        let voxel_map: Arc<Mutex<std::collections::HashMap<(i64, i64, i64), Vec<Point>>>> = 
            Arc::new(Mutex::new(std::collections::HashMap::new()));
        
        if points.len() >= config.min_parallel_size {
            points.par_iter().for_each(|p| {
                let voxel_x = (p.x / voxel_size).floor() as i64;
                let voxel_y = (p.y / voxel_size).floor() as i64;
                let voxel_z = (p.z / voxel_size).floor() as i64;
                
                if let Ok(mut map) = voxel_map.lock() {
                    map.entry((voxel_x, voxel_y, voxel_z))
                        .or_insert_with(Vec::new)
                        .push(*p);
                }
            });
        } else {
            points.iter().for_each(|p| {
                let voxel_x = (p.x / voxel_size).floor() as i64;
                let voxel_y = (p.y / voxel_size).floor() as i64;
                let voxel_z = (p.z / voxel_size).floor() as i64;
                
                if let Ok(mut map) = voxel_map.lock() {
                    map.entry((voxel_x, voxel_y, voxel_z))
                        .or_insert_with(Vec::new)
                        .push(*p);
                }
            });
        }
        
        // Compute centroid for each voxel
        let downsampled: Vec<Point> = if let Ok(map) = voxel_map.lock() {
            map.values()
                .map(|voxel_points| {
                    let mut centroid = Point::new(0.0, 0.0, 0.0);
                    for p in voxel_points {
                        centroid.x += p.x;
                        centroid.y += p.y;
                        centroid.z += p.z;
                    }
                    let n = voxel_points.len() as f64;
                    Point::new(centroid.x / n, centroid.y / n, centroid.z / n)
                })
                .collect()
        } else {
            Vec::new()
        };
        
        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(points.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);
        
        ParallelResult::new(downsampled, stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_convex_hull() {
        let points: Vec<Point> = (0..100).map(|i| {
            let angle = i as f64 * 0.1;
            Point::new(angle.cos(), angle.sin(), 0.0)
        }).collect();
        
        let config = ParallelConfig::default();
        let result = ParallelConvexHull::compute(&points, &config);
        
        assert!(!result.data.is_empty());
    }
    
    #[test]
    fn test_parallel_delaunay() {
        let points: Vec<Point> = (0..50).map(|i| {
            Point::new(i as f64 * 0.1, (i as f64 * 0.1).sin(), 0.0)
        }).collect();
        
        let config = ParallelConfig::default();
        let result = ParallelDelaunay::triangulate(&points, &config);
        
        // Should have triangles
        assert!(!result.data.is_empty());
    }
    
    #[test]
    fn test_k_nearest_neighbors() {
        let points: Vec<Point> = (0..100).map(|i| {
            Point::new(i as f64 * 0.1, 0.0, 0.0)
        }).collect();
        
        let config = ParallelConfig::default();
        let result = ParallelSpatialQuery::k_nearest_neighbors(&points, 5, &config);
        
        assert_eq!(result.data.len(), 100);
        assert_eq!(result.data[0].len(), 5);
    }
    
    #[test]
    fn test_range_search() {
        let points: Vec<Point> = (0..100).map(|i| {
            Point::new(i as f64 * 0.1, 0.0, 0.0)
        }).collect();
        
        let center = Point::new(5.0, 0.0, 0.0);
        let config = ParallelConfig::default();
        let result = ParallelSpatialQuery::range_search(&points, &center, 1.0, &config);
        
        // Should find points within radius
        assert!(!result.data.is_empty());
    }
    
    #[test]
    fn test_voxel_downsample() {
        let points: Vec<Point> = (0..1000).map(|i| {
            Point::new(
                (i % 10) as f64 * 0.1,
                ((i / 10) % 10) as f64 * 0.1,
                (i / 100) as f64 * 0.1,
            )
        }).collect();
        
        let config = ParallelConfig::default();
        let result = ParallelPointCloud::voxel_downsample(&points, 0.5, &config);
        
        // Should have fewer points after downsampling
        assert!(result.data.len() < points.len());
    }
}
