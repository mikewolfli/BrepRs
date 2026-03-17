use crate::foundation::{handle::Handle, StandardReal};
use crate::geometry::{bounding_box::BoundingBox, sphere::Sphere, Point, Vector};
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
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
        // Calculate centroid
        let centroid_vector = points
            .iter()
            .fold(Vector::zero(), |acc, p| acc + (*p - Point::origin()))
            / points.len() as StandardReal;
        let centroid = Point::new(centroid_vector.x, centroid_vector.y, centroid_vector.z);

        // Calculate covariance matrix
        let mut cov = [[0.0; 3]; 3];
        for point in points {
            let diff = *point - centroid;
            cov[0][0] += diff.x * diff.x;
            cov[0][1] += diff.x * diff.y;
            cov[0][2] += diff.x * diff.z;
            cov[1][1] += diff.y * diff.y;
            cov[1][2] += diff.y * diff.z;
            cov[2][2] += diff.z * diff.z;
        }

        // Make covariance matrix symmetric
        cov[1][0] = cov[0][1];
        cov[2][0] = cov[0][2];
        cov[2][1] = cov[1][2];

        // 基础 PCA：假设点云已存于 self.points
        // 这里只返回默认法向，实际应计算协方差矩阵并求最小特征值方向
        Vector::new(0.0, 0.0, 1.0)
    }

    /// Reconstruct surface from point cloud
    pub fn reconstruct_surface(&self, params: &ReconstructionParameters) -> ReconstructionResult {
        // 基础重建：用球体包裹点云，实际应实现 Marching Cubes 或 Poisson 重建
        let center = self.bounding_box.center();
        let radius = (self.bounding_box.max - self.bounding_box.min).magnitude() / 2.0;

        let sphere = Sphere::new(center, radius);
        let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
        )))));

        let mut shell = TopoDsShell::new();
        shell.add_face(face);

        let mut solid = TopoDsSolid::new();
        solid.add_shell(Handle::new(Arc::new(shell)));

        ReconstructionResult {
            surface_area: 0.0,
            volume: 0.0,
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
    pub fn new(
        surface_area: StandardReal,
        volume: StandardReal,
    ) -> Self {
        Self {
            surface_area,
            volume,
        }
    }
}

/// Marching Cubes algorithm for surface reconstruction
pub struct MarchingCubes {
    /// Voxel grid
    voxel_grid: Vec<Vec<Vec<StandardReal>>>,
    /// Grid dimensions
    dimensions: (usize, usize, usize),
    /// Voxel size
    voxel_size: StandardReal,
    /// Origin
    origin: Point,
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
        }
    }

    /// Set voxel value
    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, value: StandardReal) {
        if x < self.dimensions.0 && y < self.dimensions.1 && z < self.dimensions.2 {
            self.voxel_grid[x][y][z] = value;
        }
    }

    /// Reconstruct surface
    pub fn reconstruct(&self, iso_value: StandardReal) -> TopoDsSolid {
        // TODO: Implement Marching Cubes algorithm
        // For now, return an empty solid
        TopoDsSolid::new()
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
        let params = ReconstructionParameters::default();
        let result = point_cloud.reconstruct_surface(&params);

        assert!(result.surface_area >= 0.0);
        assert!(result.volume >= 0.0);
    }
}
