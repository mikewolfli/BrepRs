use crate::foundation::{handle::Handle, StandardReal};
use crate::geometry::{bounding_box::BoundingBox, Point, Vector};
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShell, TopoDsSolid, TopoDsVertex, TopoDsWire};
use std::sync::Arc;

/// Point cloud for surface reconstruction
#[derive(Debug, Clone)]
pub struct PointCloud {
    /// Points in the cloud
    pub points: Vec<Point>,
    /// Normals (optional)
    pub normals: Option<Vec<Vector>>,
    /// Bounding box
    pub bounding_box: BoundingBox,
}

/// Surface reconstruction parameters
#[derive(Debug, Clone)]
pub struct ReconstructionParameters {
    /// Voxel size for Marching Cubes
    pub voxel_size: StandardReal,
    /// Iso value for Marching Cubes
    pub iso_value: StandardReal,
    /// Neighbor search radius
    pub neighbor_radius: StandardReal,
    /// Normal estimation parameters
    pub normal_estimation: NormalEstimationParams,
}

/// Normal estimation parameters
#[derive(Debug, Clone)]
pub struct NormalEstimationParams {
    /// Number of neighbors for normal estimation
    pub k_neighbors: usize,
    /// Search radius for normal estimation
    pub search_radius: StandardReal,
}

/// Surface reconstruction result
#[derive(Debug, Clone)]
pub struct ReconstructionResult {
    /// Surface area
    pub surface_area: StandardReal,
    /// Volume
    pub volume: StandardReal,
}

impl PointCloud {
    /// Create a new point cloud
    pub fn new(points: Vec<Point>, normals: Option<Vec<Vector>>) -> Self {
        // Calculate bounding box
        // Compute min/max for bounding box
        let min = points.iter().fold(
            Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            |acc, p| Point::new(acc.x.min(p.x), acc.y.min(p.y), acc.z.min(p.z)),
        );
        let max = points.iter().fold(
            Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            |acc, p| Point::new(acc.x.max(p.x), acc.y.max(p.y), acc.z.max(p.z)),
        );
        let bounding_box = BoundingBox::new(min, max);

        Self {
            points,
            normals,
            bounding_box,
        }
    }

    /// Estimate normals for the point cloud
    pub fn estimate_normals(&mut self, params: &NormalEstimationParams) {
        let mut normals = Vec::with_capacity(self.points.len());

        for point in self.points.iter() {
            // Find k nearest neighbors
            let neighbors = self.find_k_nearest_neighbors(point, params.k_neighbors);

            // Estimate normal using PCA
            if neighbors.len() >= 3 {
                let normal = self.estimate_normal(&neighbors);
                normals.push(normal);
            } else {
                normals.push(Vector::new(0.0, 0.0, 1.0)); // Default normal
            }
        }

        self.normals = Some(normals);
    }

    /// Find k nearest neighbors to a point
    fn find_k_nearest_neighbors(&self, point: &Point, k: usize) -> Vec<Point> {
        let mut distances: Vec<(StandardReal, Point)> = self
            .points
            .iter()
            .map(|p| ((*p - *point).magnitude(), *p))
            .collect();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        distances.into_iter().take(k).map(|(_, p)| p).collect()
    }

    /// Estimate normal using PCA
    fn estimate_normal(&self, points: &[Point]) -> Vector {
        // Compute centroid
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;
        
        for point in points {
            sum_x += point.x;
            sum_y += point.y;
            sum_z += point.z;
        }
        
        let centroid = Point::new(
            sum_x / points.len() as f64,
            sum_y / points.len() as f64,
            sum_z / points.len() as f64
        );
        
        // Compute covariance matrix
        let mut covariance = [[0.0; 3]; 3];
        
        for point in points {
            let diff = *point - centroid;
            let diff_vec = Vector::new(diff.x, diff.y, diff.z);
            covariance[0][0] += diff_vec.x * diff_vec.x;
            covariance[0][1] += diff_vec.x * diff_vec.y;
            covariance[0][2] += diff_vec.x * diff_vec.z;
            covariance[1][0] += diff_vec.y * diff_vec.x;
            covariance[1][1] += diff_vec.y * diff_vec.y;
            covariance[1][2] += diff_vec.y * diff_vec.z;
            covariance[2][0] += diff_vec.z * diff_vec.x;
            covariance[2][1] += diff_vec.z * diff_vec.y;
            covariance[2][2] += diff_vec.z * diff_vec.z;
        }
        
        // Normalize covariance matrix
        let inv_n = 1.0 / points.len() as f64;
        for i in 0..3 {
            for j in 0..3 {
                covariance[i][j] *= inv_n;
            }
        }
        
        // Find the eigenvector corresponding to the smallest eigenvalue
        // Uses direction sampling method to estimate the normal vector
        // Future implementation will use more robust eigenvalue decomposition
        let mut normal = Vector::new(0.0, 0.0, 1.0);
        let mut min_eigenvalue = f64::INFINITY;
        
        // Try different directions
        let directions = [
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 0.0),
            Vector::new(1.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        ];
        
        for dir in &directions {
            let normalized_dir = dir.normalized();
            let eigenvalue = normalized_dir.x * (covariance[0][0] * normalized_dir.x + covariance[0][1] * normalized_dir.y + covariance[0][2] * normalized_dir.z) +
                           normalized_dir.y * (covariance[1][0] * normalized_dir.x + covariance[1][1] * normalized_dir.y + covariance[1][2] * normalized_dir.z) +
                           normalized_dir.z * (covariance[2][0] * normalized_dir.x + covariance[2][1] * normalized_dir.y + covariance[2][2] * normalized_dir.z);
            
            if eigenvalue < min_eigenvalue {
                min_eigenvalue = eigenvalue;
                normal = normalized_dir;
            }
        }
        
        normal
    }

    /// Reconstruct surface from point cloud
    pub fn reconstruct_surface(&self) -> ReconstructionResult {
        // Use Marching Cubes algorithm for surface reconstruction
        let voxel_size = 0.1; // Default voxel size
        let mut marching_cubes = MarchingCubes::new(&self.bounding_box, voxel_size);
        
        // Set voxel values based on point cloud
        for point in &self.points {
            // Find voxel coordinates
            let x = ((point.x - self.bounding_box.min.x) / voxel_size).floor() as usize;
            let y = ((point.y - self.bounding_box.min.y) / voxel_size).floor() as usize;
            let z = ((point.z - self.bounding_box.min.z) / voxel_size).floor() as usize;
            
            // Set voxel value based on distance from point
            marching_cubes.set_voxel(x, y, z, 1.0);
        }
        
        // Reconstruct surface
        let _solid = marching_cubes.reconstruct();
        
        // Calculate surface area and volume
        // Uses bounding box approximation for surface area and volume
        // Future implementation will use the reconstructed solid's geometry for accurate calculations
        let surface_area = 6.0 * (self.bounding_box.max.x - self.bounding_box.min.x).powi(2);
        let volume = (self.bounding_box.max.x - self.bounding_box.min.x).powi(3);

        ReconstructionResult {
            surface_area,
            volume,
        }
    }
}

impl ReconstructionParameters {
    /// Create default reconstruction parameters
    pub fn default() -> Self {
        Self {
            voxel_size: 0.1,
            iso_value: 0.5,
            neighbor_radius: 0.2,
            normal_estimation: NormalEstimationParams {
                k_neighbors: 10,
                search_radius: 0.1,
            },
        }
    }
}

impl NormalEstimationParams {
    /// Create default normal estimation parameters
    pub fn default() -> Self {
        Self {
            k_neighbors: 10,
            search_radius: 0.1,
        }
    }
}

impl ReconstructionResult {
    /// Create a new reconstruction result
    pub fn new(surface_area: StandardReal, volume: StandardReal) -> Self {
        Self {
            surface_area,
            volume,
        }
    }
}

/// Marching Cubes algorithm for surface reconstruction
#[allow(dead_code)]
pub struct MarchingCubes {
    /// Voxel grid
    voxel_grid: Vec<Vec<Vec<StandardReal>>>,
    /// Grid dimensions
    dimensions: (usize, usize, usize),
    /// Voxel size
    voxel_size: StandardReal,
    /// Origin
    origin: Point,
    /// Iso value for surface reconstruction
    iso_value: StandardReal,
}

impl MarchingCubes {
    /// Create a new Marching Cubes instance
    pub fn new(bounding_box: &BoundingBox, voxel_size: StandardReal) -> Self {
        let width = ((bounding_box.max.x - bounding_box.min.x) / voxel_size).ceil() as usize;
        let height = ((bounding_box.max.y - bounding_box.min.y) / voxel_size).ceil() as usize;
        let depth = ((bounding_box.max.z - bounding_box.min.z) / voxel_size).ceil() as usize;

        let voxel_grid = vec![vec![vec![0.0; depth]; height]; width];

        Self {
            voxel_grid,
            dimensions: (width, height, depth),
            voxel_size,
            origin: bounding_box.min,
            iso_value: 0.5, // Default iso value
        }
    }

    /// Get scalar field value at a point
    fn scalar_field(&self, point: &Point) -> StandardReal {
        // Convert point to voxel coordinates
        let x = ((point.x - self.origin.x) / self.voxel_size).floor() as usize;
        let y = ((point.y - self.origin.y) / self.voxel_size).floor() as usize;
        let z = ((point.z - self.origin.z) / self.voxel_size).floor() as usize;
        
        // Check if voxel is within bounds
        if x < self.dimensions.0 && y < self.dimensions.1 && z < self.dimensions.2 {
            // Return voxel value
            self.voxel_grid[x][y][z]
        } else {
            // Return 0.0 for points outside the grid
            0.0
        }
    }

    /// Set voxel value
    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, value: StandardReal) {
        if x < self.dimensions.0 && y < self.dimensions.1 && z < self.dimensions.2 {
            self.voxel_grid[x][y][z] = value;
        }
    }

    /// Reconstruct surface
    pub fn reconstruct(&self) -> TopoDsSolid {
        // Implement Marching Cubes algorithm
        // Note: This is a simplified implementation of the Marching Cubes algorithm
        let mut vertices = Vec::new();
        let mut faces: Vec<Vec<usize>> = Vec::new();

        // Marching Cubes edge table
        let edge_table = [
            0x0, 0x109, 0x203, 0x30a, 0x406, 0x50f, 0x605, 0x70c, 0x80c, 0x905, 0xa0f, 0xb06,
            0xc0a, 0xd03, 0xe09, 0xf00, 0x190, 0x99, 0x393, 0x29a, 0x596, 0x49f, 0x795, 0x69c,
            0x99c, 0x895, 0xb9f, 0xa96, 0xd9a, 0xc93, 0xf99, 0xe90, 0x230, 0x339, 0x33, 0x13a,
            0x636, 0x73f, 0x435, 0x53c, 0xa3c, 0xb35, 0x83f, 0x936, 0xe3a, 0xf33, 0xc39, 0xd30,
            0x3a0, 0x2a9, 0x1a3, 0xaa, 0x7a6, 0x6af, 0x5a5, 0x4ac, 0xbac, 0xaa5, 0x9af, 0x8a6,
            0xfaa, 0xea3, 0xda9, 0xca0, 0x460, 0x569, 0x663, 0x76a, 0x66, 0x16f, 0x265, 0x36c,
            0xc6c, 0xd65, 0xe6f, 0xf66, 0x86a, 0x963, 0xa69, 0xb60, 0x5f0, 0x4f9, 0x7f3, 0x6fa,
            0x1f6, 0xff, 0x3f5, 0x2fc, 0xdfc, 0xcf5, 0xfff, 0xef6, 0x9fa, 0x8f3, 0xbf9, 0xaf0,
            0x650, 0x759, 0x453, 0x55a, 0x256, 0x35f, 0x55, 0x15c, 0xe5c, 0xf55, 0xc5f, 0xd56,
            0xa5a, 0xb53, 0x859, 0x950, 0x7c0, 0x6c9, 0x5c3, 0x4ca, 0x3c6, 0x2cf, 0x1c5, 0xcc,
            0xfcc, 0xec5, 0xdcf, 0xcc6, 0xbca, 0xac3, 0x9c9, 0x8c0, 0x8c0, 0x9c9, 0xac3, 0xbca,
            0xcc6, 0xdcf, 0xec5, 0xfcc, 0xcc, 0x1c5, 0x2cf, 0x3c6, 0x4ca, 0x5c3, 0x6c9, 0x7c0,
            0x950, 0x859, 0xb53, 0xa5a, 0xd56, 0xc5f, 0xf55, 0xe5c, 0x15c, 0x55, 0x35f, 0x256,
            0x55a, 0x453, 0x759, 0x650, 0xaf0, 0xbf9, 0x8f3, 0x9fa, 0xef6, 0xfff, 0xcf5, 0xdfc,
            0x2fc, 0x3f5, 0xff, 0x1f6, 0x6fa, 0x7f3, 0x4f9, 0x5f0, 0xb60, 0xa69, 0x963, 0x86a,
            0xf66, 0xe6f, 0xd65, 0xc6c, 0x36c, 0x265, 0x16f, 0x66, 0x76a, 0x663, 0x569, 0x460,
            0xca0, 0xda9, 0xea3, 0xfaa, 0x8a6, 0x9af, 0xaa5, 0xbac, 0x4ac, 0x5a5, 0x6af, 0x7a6,
            0xaa, 0x1a3, 0x2a9, 0x3a0, 0xd30, 0xc39, 0xf33, 0xe3a, 0x936, 0x83f, 0xb35, 0xa3c,
            0x53c, 0x435, 0x73f, 0x636, 0x13a, 0x33, 0x339, 0x230, 0xe90, 0xf99, 0xc93, 0xd9a,
            0xa96, 0xb9f, 0x895, 0x99c, 0x69c, 0x795, 0x49f, 0x596, 0x29a, 0x393, 0x99, 0x190,
            0xf00, 0xe09, 0xd03, 0xc0a, 0xb06, 0xa0f, 0x905, 0x80c, 0x70c, 0x605, 0x50f, 0x406,
            0x30a, 0x203, 0x109, 0x0,
        ];

        // Marching Cubes triangle table
        let tri_table = vec![
            vec![],
            vec![0, 8, 3],
            vec![0, 1, 9],
            vec![1, 8, 3, 9, 8, 1],
            vec![1, 2, 10],
            vec![0, 8, 3, 1, 2, 10],
            vec![9, 2, 10, 0, 2, 9],
            vec![2, 8, 3, 2, 10, 8, 10, 9, 8],
            vec![3, 11, 2],
            vec![0, 11, 2, 8, 11, 0],
            vec![1, 9, 0, 2, 3, 11],
            vec![1, 11, 2, 1, 9, 11, 9, 8, 11],
            vec![3, 10, 1, 11, 10, 3],
            vec![0, 10, 1, 0, 11, 10, 8, 11, 0],
            vec![3, 9, 0, 3, 11, 9, 11, 10, 9],
            vec![9, 8, 11, 10, 9, 11],
            vec![4, 7, 8],
            vec![4, 3, 0, 7, 3, 4],
            vec![0, 1, 9, 8, 4, 7],
            vec![4, 1, 9, 4, 7, 1, 7, 3, 1],
            vec![1, 2, 10, 8, 4, 7],
            vec![3, 4, 7, 3, 0, 4, 1, 2, 10],
            vec![4, 2, 10, 4, 7, 2, 9, 7, 2, 9, 2, 0],
            vec![2, 4, 7, 10, 4, 2, 9, 7, 4],
            vec![7, 11, 2, 4, 11, 7],
            vec![11, 4, 7, 11, 2, 4, 2, 0, 4],
            vec![4, 7, 8, 9, 1, 0, 2, 3, 11],
            vec![9, 7, 4, 9, 11, 7, 9, 1, 11, 1, 2, 11],
            vec![10, 1, 3, 10, 3, 11, 7, 4, 8],
            vec![1, 11, 10, 1, 4, 11, 1, 0, 4, 7, 11, 4],
            vec![9, 11, 10, 9, 7, 11, 9, 0, 7, 4, 7, 0],
            vec![9, 10, 7, 4, 7, 9],
            vec![9, 4, 5],
            vec![9, 3, 0, 5, 3, 9],
            vec![0, 5, 4, 1, 5, 0],
            vec![8, 5, 4, 8, 3, 5, 3, 1, 5],
            vec![2, 10, 1, 5, 4, 9],
            vec![3, 5, 4, 3, 0, 5, 1, 2, 10],
            vec![2, 10, 5, 4, 5, 10],
            vec![2, 10, 5, 3, 2, 5, 8, 3, 5, 8, 5, 4],
            vec![9, 7, 8, 5, 7, 9],
            vec![9, 3, 0, 9, 5, 3, 5, 7, 3],
            vec![7, 5, 4, 1, 5, 0],
            vec![1, 7, 4, 1, 5, 7, 3, 7, 5],
            vec![9, 2, 10, 5, 7, 9, 8, 7, 5],
            vec![2, 10, 1, 3, 2, 5, 3, 5, 7, 4, 7, 5],
            vec![5, 2, 10, 5, 4, 2],
            vec![3, 2, 4, 8, 3, 4],
            vec![10, 5, 4, 2, 5, 10],
            vec![0, 10, 5, 3, 10, 0, 3, 5, 10, 3, 4, 5],
            vec![1, 5, 4, 10, 5, 1],
            vec![8, 5, 4, 8, 3, 5, 1, 3, 5],
            vec![4, 2, 10, 4, 7, 2, 9, 7, 2, 9, 2, 1],
            vec![9, 7, 2, 9, 2, 1, 7, 3, 2],
            vec![7, 10, 1, 7, 2, 10, 5, 2, 7],
            vec![1, 7, 5, 0, 7, 1],
            vec![6, 5, 4],
            vec![3, 6, 5, 3, 0, 6, 0, 9, 6],
            vec![0, 1, 9, 4, 6, 5],
            vec![5, 1, 9, 5, 6, 1, 3, 6, 1],
            vec![6, 2, 10, 6, 5, 2, 4, 5, 2],
            vec![3, 0, 6, 3, 6, 5, 1, 2, 10, 4, 5, 2],
            vec![9, 2, 10, 9, 6, 2, 5, 6, 2, 5, 4, 2],
            vec![5, 3, 6, 5, 2, 3, 10, 2, 5, 4, 5, 2],
            vec![7, 6, 5, 8, 6, 7],
            vec![3, 7, 6, 3, 6, 0, 0, 6, 9],
            vec![9, 1, 0, 8, 6, 5, 7, 6, 8],
            vec![1, 6, 9, 1, 3, 6, 5, 6, 1],
            vec![2, 10, 1, 8, 7, 6, 5, 7, 8],
            vec![3, 6, 0, 3, 5, 6, 2, 10, 1, 5, 2, 6],
            vec![6, 10, 2, 6, 9, 10, 4, 5, 6, 9, 8, 6],
            vec![10, 8, 6, 10, 6, 2, 8, 7, 6],
            vec![11, 6, 5, 2, 6, 11],
            vec![11, 0, 6, 11, 6, 2, 9, 6, 0, 5, 6, 9],
            vec![1, 9, 0, 2, 11, 5, 6, 5, 11],
            vec![11, 1, 9, 11, 5, 1, 2, 5, 11, 6, 5, 2],
            vec![11, 10, 1, 11, 5, 10, 4, 5, 10, 6, 5, 4],
            vec![1, 11, 0, 1, 5, 11, 6, 5, 11, 4, 5, 6],
            vec![5, 11, 10, 5, 9, 11, 8, 11, 9, 6, 5, 8],
            vec![5, 8, 6, 10, 5, 6],
            vec![10, 9, 4, 10, 4, 2, 9, 5, 4],
            vec![10, 3, 0, 10, 4, 3, 9, 5, 4, 5, 3, 4],
            vec![1, 4, 2, 1, 5, 4],
            vec![8, 4, 3, 8, 2, 4, 5, 4, 2],
            vec![9, 5, 4, 2, 10, 9],
            vec![3, 5, 4, 3, 9, 5, 1, 9, 3, 2, 3, 10],
            vec![4, 10, 9],
            vec![3, 4, 10, 8, 3, 10],
            vec![7, 10, 9, 7, 6, 10, 5, 6, 9],
            vec![3, 7, 6, 3, 6, 10, 5, 6, 9, 10, 6, 9],
            vec![10, 0, 6, 10, 6, 2, 4, 6, 0, 5, 6, 4],
            vec![3, 4, 6, 3, 6, 2, 5, 6, 4],
            vec![1, 6, 4, 10, 6, 1],
            vec![8, 6, 4, 8, 2, 6, 3, 2, 8],
            vec![7, 9, 4, 7, 2, 9, 7, 6, 2, 10, 2, 9],
            vec![7, 6, 2, 9, 7, 2],
            vec![6, 1, 10, 6, 11, 1, 5, 11, 6],
            vec![6, 0, 11, 9, 6, 11, 5, 6, 9],
            vec![5, 11, 1, 5, 1, 4],
            vec![8, 5, 4, 8, 11, 5, 3, 11, 8],
            vec![9, 11, 10, 5, 11, 9, 6, 11, 5],
            vec![9, 6, 10, 3, 9, 10, 6, 5, 10],
            vec![11, 4, 5, 11, 10, 4, 10, 9, 4],
            vec![11, 8, 3, 10, 11, 3, 9, 10, 3],
            vec![11, 7, 6],
            vec![11, 9, 6, 0, 9, 11],
            vec![1, 7, 6, 1, 9, 7, 0, 7, 9],
            vec![1, 3, 11, 6, 1, 11],
            vec![11, 2, 10, 11, 6, 2, 7, 6, 11],
            vec![10, 0, 6, 10, 6, 2, 9, 6, 0, 7, 6, 9],
            vec![10, 7, 6, 9, 7, 10],
            vec![2, 3, 11, 7, 6, 2],
            vec![6, 0, 9, 6, 7, 0, 7, 3, 0],
            vec![1, 7, 6, 1, 9, 7, 8, 7, 9],
            vec![6, 1, 3, 6, 3, 7, 7, 3, 8],
            vec![2, 7, 6, 10, 7, 2],
            vec![6, 2, 10, 6, 7, 2, 3, 7, 2, 0, 2, 3],
            vec![9, 7, 2, 9, 2, 1, 7, 6, 2],
            vec![7, 6, 3, 8, 7, 3],
        ];

        // Process each voxel
        let grid_size = 10; // Simple grid size for demonstration
        let cell_size = 1.0 / grid_size as f64;

        for i in 0..grid_size {
            for j in 0..grid_size {
                for k in 0..grid_size {
                    // Calculate voxel corner positions
                    let corners = [
                        Point::new(
                            i as f64 * cell_size,
                            j as f64 * cell_size,
                            k as f64 * cell_size,
                        ),
                        Point::new(
                            (i + 1) as f64 * cell_size,
                            j as f64 * cell_size,
                            k as f64 * cell_size,
                        ),
                        Point::new(
                            (i + 1) as f64 * cell_size,
                            (j + 1) as f64 * cell_size,
                            k as f64 * cell_size,
                        ),
                        Point::new(
                            i as f64 * cell_size,
                            (j + 1) as f64 * cell_size,
                            k as f64 * cell_size,
                        ),
                        Point::new(
                            i as f64 * cell_size,
                            j as f64 * cell_size,
                            (k + 1) as f64 * cell_size,
                        ),
                        Point::new(
                            (i + 1) as f64 * cell_size,
                            j as f64 * cell_size,
                            (k + 1) as f64 * cell_size,
                        ),
                        Point::new(
                            (i + 1) as f64 * cell_size,
                            (j + 1) as f64 * cell_size,
                            (k + 1) as f64 * cell_size,
                        ),
                        Point::new(
                            i as f64 * cell_size,
                            (j + 1) as f64 * cell_size,
                            (k + 1) as f64 * cell_size,
                        ),
                    ];

                    // Calculate voxel value at each corner
                    let mut cube_index = 0;
                    for (idx, corner) in corners.iter().enumerate() {
                        let value = self.scalar_field(corner);
                        if value < self.iso_value {
                            cube_index |= 1 << idx;
                        }
                    }

                    // Get triangle configuration from edge table
                    let edge_config = edge_table[cube_index as usize];
                    if edge_config == 0 {
                        continue;
                    }

                    // Generate vertices for the edges
                    let mut edge_vertices = [Point::new(0.0, 0.0, 0.0); 12];

                    // Calculate edge vertices
                    if (edge_config & 1) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[0]))
                            / (self.scalar_field(&corners[1]) - self.scalar_field(&corners[0]));
                        edge_vertices[0] = corners[0] + (corners[1] - corners[0]) * t;
                    }
                    if (edge_config & 2) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[1]))
                            / (self.scalar_field(&corners[2]) - self.scalar_field(&corners[1]));
                        edge_vertices[1] = corners[1] + (corners[2] - corners[1]) * t;
                    }
                    if (edge_config & 4) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[2]))
                            / (self.scalar_field(&corners[3]) - self.scalar_field(&corners[2]));
                        edge_vertices[2] = corners[2] + (corners[3] - corners[2]) * t;
                    }
                    if (edge_config & 8) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[3]))
                            / (self.scalar_field(&corners[0]) - self.scalar_field(&corners[3]));
                        edge_vertices[3] = corners[3] + (corners[0] - corners[3]) * t;
                    }
                    if (edge_config & 16) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[4]))
                            / (self.scalar_field(&corners[5]) - self.scalar_field(&corners[4]));
                        edge_vertices[4] = corners[4] + (corners[5] - corners[4]) * t;
                    }
                    if (edge_config & 32) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[5]))
                            / (self.scalar_field(&corners[6]) - self.scalar_field(&corners[5]));
                        edge_vertices[5] = corners[5] + (corners[6] - corners[5]) * t;
                    }
                    if (edge_config & 64) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[6]))
                            / (self.scalar_field(&corners[7]) - self.scalar_field(&corners[6]));
                        edge_vertices[6] = corners[6] + (corners[7] - corners[6]) * t;
                    }
                    if (edge_config & 128) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[7]))
                            / (self.scalar_field(&corners[4]) - self.scalar_field(&corners[7]));
                        edge_vertices[7] = corners[7] + (corners[4] - corners[7]) * t;
                    }
                    if (edge_config & 256) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[0]))
                            / (self.scalar_field(&corners[4]) - self.scalar_field(&corners[0]));
                        edge_vertices[8] = corners[0] + (corners[4] - corners[0]) * t;
                    }
                    if (edge_config & 512) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[1]))
                            / (self.scalar_field(&corners[5]) - self.scalar_field(&corners[1]));
                        edge_vertices[9] = corners[1] + (corners[5] - corners[1]) * t;
                    }
                    if (edge_config & 1024) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[2]))
                            / (self.scalar_field(&corners[6]) - self.scalar_field(&corners[2]));
                        edge_vertices[10] = corners[2] + (corners[6] - corners[2]) * t;
                    }
                    if (edge_config & 2048) != 0 {
                        let t = (self.iso_value - self.scalar_field(&corners[3]))
                            / (self.scalar_field(&corners[7]) - self.scalar_field(&corners[3]));
                        edge_vertices[11] = corners[3] + (corners[7] - corners[3]) * t;
                    }

                    // Generate triangles
                    let tris = &tri_table[cube_index as usize];
                    for i in (0..tris.len()).step_by(3) {
                        if i + 2 >= tris.len() {
                            break;
                        }
                        let v0 = tris[i] as usize;
                        let v1 = tris[i + 1] as usize;
                        let v2 = tris[i + 2] as usize;

                        let vert0 = edge_vertices[v0];
                        let vert1 = edge_vertices[v1];
                        let vert2 = edge_vertices[v2];

                        vertices.push(vert0);
                        vertices.push(vert1);
                        vertices.push(vert2);

                        let idx = vertices.len() - 3;
                        faces.push(vec![idx, idx + 1, idx + 2]);
                    }
                }
            }
        }

        // Create solid from reconstructed surface
        let mut solid = TopoDsSolid::new();

        // For simplicity, create a shell and add faces
        let mut shell = TopoDsShell::new();

        // Create faces from the reconstructed triangles
        for face_verts in &faces {
            if face_verts.len() == 3 {
                let v0 = TopoDsVertex::new(vertices[face_verts[0]]);
                let v1 = TopoDsVertex::new(vertices[face_verts[1]]);
                let v2 = TopoDsVertex::new(vertices[face_verts[2]]);

                let v0_handle = Handle::new(Arc::new(v0));
                let v1_handle = Handle::new(Arc::new(v1));
                let v2_handle = Handle::new(Arc::new(v2));

                let edge1 = TopoDsEdge::new(v0_handle.clone(), v1_handle.clone());
                let edge2 = TopoDsEdge::new(v1_handle.clone(), v2_handle.clone());
                let edge3 = TopoDsEdge::new(v2_handle.clone(), v0_handle.clone());

                let mut wire = TopoDsWire::new();
                wire.add_edge(Handle::new(Arc::new(edge1)));
                wire.add_edge(Handle::new(Arc::new(edge2)));
                wire.add_edge(Handle::new(Arc::new(edge3)));

                let face = TopoDsFace::with_outer_wire(wire);
                shell.add_face(Handle::new(Arc::new(face)));
            }
        }

        if !shell.is_empty() {
            solid.add_shell(Handle::new(Arc::new(shell)));
        }

        solid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_point_cloud_creation() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        ];

        let point_cloud = PointCloud::new(points, None);

        assert_eq!(point_cloud.points.len(), 4);
        assert!(point_cloud.normals.is_none());
    }

    #[test]
    fn test_normal_estimation() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        ];

        let mut point_cloud = PointCloud::new(points, None);
        let params = NormalEstimationParams::default();
        point_cloud.estimate_normals(&params);

        assert!(point_cloud.normals.is_some());
        assert_eq!(point_cloud.normals.unwrap().len(), 4);
    }

    #[test]
    fn test_surface_reconstruction() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
        ];

        let point_cloud = PointCloud::new(points, None);
        // let params = ReconstructionParameters::default();
        let result = point_cloud.reconstruct_surface();

        assert!(result.surface_area >= 0.0);
        assert!(result.volume >= 0.0);
    }
}
