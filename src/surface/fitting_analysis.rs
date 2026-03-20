use crate::geometry::Point;
use crate::mesh::TriangleMesh;
use rand;
use std::collections::HashMap;

/// Surface fitting algorithm
pub enum SurfaceFittingAlgorithm {
    /// Least squares fitting
    LeastSquares,
    /// RANSAC algorithm
    RANSAC,
    /// Moving least squares
    MovingLeastSquares,
    /// Poisson surface reconstruction
    Poisson,
    /// Alpha shapes
    AlphaShapes,
    /// Ball pivoting
    BallPivoting,
}

/// Surface type
pub enum SurfaceType {
    /// Plane
    Plane,
    /// Sphere
    Sphere,
    /// Cylinder
    Cylinder,
    /// Cone
    Cone,
    /// Torus
    Torus,
    /// Bezier surface
    Bezier,
    /// NURBS surface
    NURBS,
    /// Triangle mesh
    TriangleMesh,
    /// Custom surface
    Custom(String),
}

/// Surface fitting settings
pub struct SurfaceFittingSettings {
    pub algorithm: SurfaceFittingAlgorithm,
    pub surface_type: SurfaceType,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub sample_size: usize,
    pub smoothing_factor: f64,
    pub curvature_weight: f64,
    pub normal_weight: f64,
    pub enable_outlier_removal: bool,
    pub outlier_threshold: f64,
}

impl Default for SurfaceFittingSettings {
    fn default() -> Self {
        Self {
            algorithm: SurfaceFittingAlgorithm::LeastSquares,
            surface_type: SurfaceType::TriangleMesh,
            max_iterations: 100,
            tolerance: 1e-6,
            sample_size: 1000,
            smoothing_factor: 0.1,
            curvature_weight: 0.5,
            normal_weight: 0.5,
            enable_outlier_removal: true,
            outlier_threshold: 2.0,
        }
    }
}

/// Surface fitting result
pub struct SurfaceFittingResult {
    pub surface: Option<Box<dyn Surface>>,
    pub mesh: Option<TriangleMesh>,
    pub fitting_error: f64,
    pub r_squared: f64,
    pub iterations: usize,
    pub time_ms: f64,
    pub points_used: usize,
    pub outliers_removed: usize,
}

/// Surface interface
pub trait Surface: Send + Sync {
    /// Evaluate surface at parameter (u, v)
    fn evaluate(&self, u: f64, v: f64) -> Point;
    /// Calculate normal at parameter (u, v)
    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector;
    /// Calculate curvature at parameter (u, v)
    fn curvature(&self, u: f64, v: f64) -> (f64, f64); // (principal curvature 1, principal curvature 2)
    /// Get surface type
    fn surface_type(&self) -> SurfaceType;
    /// Get surface parameters
    fn parameters(&self) -> HashMap<String, f64>;
    /// Convert to triangle mesh
    fn to_mesh(&self, resolution: usize) -> TriangleMesh;
}

/// Surface analyzer
pub struct SurfaceAnalyzer {
    pub settings: SurfaceFittingSettings,
}

impl SurfaceAnalyzer {
    /// Create a new surface analyzer
    pub fn new() -> Self {
        Self {
            settings: SurfaceFittingSettings::default(),
        }
    }

    /// Create a new surface analyzer with custom settings
    pub fn with_settings(settings: SurfaceFittingSettings) -> Self {
        Self { settings }
    }

    /// Fit surface to point cloud
    pub fn fit_surface(&self, points: &[Point]) -> SurfaceFittingResult {
        let start_time = std::time::Instant::now();

        // Remove outliers if enabled
        let (filtered_points, outliers_removed) = if self.settings.enable_outlier_removal {
            self.remove_outliers(points)
        } else {
            (points.to_vec(), 0)
        };

        // Choose fitting algorithm based on settings
        let result = match self.settings.algorithm {
            SurfaceFittingAlgorithm::LeastSquares => self.least_squares_fitting(&filtered_points),
            SurfaceFittingAlgorithm::RANSAC => self.ransac_fitting(&filtered_points),
            SurfaceFittingAlgorithm::MovingLeastSquares => {
                self.moving_least_squares_fitting(&filtered_points)
            }
            SurfaceFittingAlgorithm::Poisson => self.poisson_reconstruction(&filtered_points),
            SurfaceFittingAlgorithm::AlphaShapes => {
                self.alpha_shapes_reconstruction(&filtered_points)
            }
            SurfaceFittingAlgorithm::BallPivoting => {
                self.ball_pivoting_reconstruction(&filtered_points)
            }
        };

        let time_ms = start_time.elapsed().as_millis() as f64;

        SurfaceFittingResult {
            surface: result.0,
            mesh: result.1,
            fitting_error: result.2,
            r_squared: result.3,
            iterations: result.4,
            time_ms,
            points_used: filtered_points.len(),
            outliers_removed,
        }
    }

    /// Remove outliers from point cloud
    fn remove_outliers(&self, points: &[Point]) -> (Vec<Point>, usize) {
        if points.len() < 3 {
            return (points.to_vec(), 0);
        }

        // Calculate mean and standard deviation
        let mut mean = Point::new(0.0, 0.0, 0.0);
        for point in points {
            mean.x += point.x;
            mean.y += point.y;
            mean.z += point.z;
        }
        mean.x /= points.len() as f64;
        mean.y /= points.len() as f64;
        mean.z /= points.len() as f64;

        // Calculate standard deviation
        let mut std_dev = 0.0;
        for point in points {
            let distance = point.distance(&mean);
            std_dev += distance * distance;
        }
        std_dev = (std_dev / points.len() as f64).sqrt();

        // Remove outliers
        let threshold = self.settings.outlier_threshold * std_dev;
        let mut filtered_points = Vec::new();
        let mut outliers_removed = 0;

        for point in points {
            let distance = point.distance(&mean);
            if distance <= threshold {
                filtered_points.push(*point);
            } else {
                outliers_removed += 1;
            }
        }

        (filtered_points, outliers_removed)
    }

    /// Least squares fitting
    fn least_squares_fitting(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        if points.len() < 3 {
            return (None, None, 0.0, 0.0, 0);
        }

        let mut iterations = 0;
        let mut surface: Option<Box<dyn Surface>> = None;
        let mut fitting_error = 0.0;

        // Based on surface type, perform least squares fitting
        match self.settings.surface_type {
            SurfaceType::Plane => {
                // Plane fitting using SVD
                let (plane, error) = self.fit_plane(points);
                surface = Some(Box::new(plane));
                fitting_error = error;
                iterations = 1;
            }
            SurfaceType::Sphere => {
                // Sphere fitting
                if let Some((sphere, error, iters)) = self.fit_sphere(points) {
                    surface = Some(Box::new(sphere));
                    fitting_error = error;
                    iterations = iters;
                }
            }
            SurfaceType::Cylinder => {
                // Cylinder fitting using least squares
                let (cylinder, error) = self.fit_cylinder(points);
                surface = Some(Box::new(cylinder));
                fitting_error = error;
                iterations = 10;
            }
            _ => {
                // For other surface types, create a simple mesh
                let mesh = self.create_simple_mesh(points);
                return (None, Some(mesh), 0.0, 0.0, 1);
            }
        }

        let r_squared = if fitting_error > 0.0 {
            1.0 - fitting_error / points.len() as f64
        } else {
            1.0
        };

        (surface, None, fitting_error, r_squared, iterations)
    }

    /// Fit plane to points using least squares
    fn fit_plane(&self, points: &[Point]) -> (PlaneSurface, f64) {
        // Calculate centroid
        let mut centroid = Point::new(0.0, 0.0, 0.0);
        for point in points {
            centroid.x += point.x;
            centroid.y += point.y;
            centroid.z += point.z;
        }
        centroid.x /= points.len() as f64;
        centroid.y /= points.len() as f64;
        centroid.z /= points.len() as f64;

        // Calculate covariance matrix
        let mut cov = [[0.0; 3]; 3];
        for point in points {
            let dx = point.x - centroid.x;
            let dy = point.y - centroid.y;
            let dz = point.z - centroid.z;
            cov[0][0] += dx * dx;
            cov[0][1] += dx * dy;
            cov[0][2] += dx * dz;
            cov[1][1] += dy * dy;
            cov[1][2] += dy * dz;
            cov[2][2] += dz * dz;
        }
        cov[1][0] = cov[0][1];
        cov[2][0] = cov[0][2];
        cov[2][1] = cov[1][2];

        // Find eigenvector corresponding to smallest eigenvalue
        let (eigenvalues, eigenvectors) = self.calculate_eigen(cov);
        let mut min_eigenvalue = eigenvalues[0];
        let mut min_index = 0;
        for i in 1..3 {
            if eigenvalues[i] < min_eigenvalue {
                min_eigenvalue = eigenvalues[i];
                min_index = i;
            }
        }

        let normal = crate::geometry::Vector::new(
            eigenvectors[min_index][0],
            eigenvectors[min_index][1],
            eigenvectors[min_index][2],
        );

        // Calculate fitting error
        let mut error = 0.0;
        for point in points {
            let dist = self.point_plane_distance(point, &centroid, &normal);
            error += dist * dist;
        }

        (
            PlaneSurface {
                origin: centroid,
                normal,
            },
            error,
        )
    }

    /// Fit sphere to points using least squares
    fn fit_sphere(&self, points: &[Point]) -> Option<(SphereSurface, f64, usize)> {
        if points.len() < 4 {
            return None;
        }

        let mut center = Point::new(0.0, 0.0, 0.0);
        let mut radius = 1.0;
        let mut error = 0.0;
        let mut iterations = 0;

        // Initial guess: centroid as center
        for point in points {
            center.x += point.x;
            center.y += point.y;
            center.z += point.z;
        }
        center.x /= points.len() as f64;
        center.y /= points.len() as f64;
        center.z /= points.len() as f64;

        // Calculate initial radius
        for point in points {
            radius += point.distance(&center);
        }
        radius /= points.len() as f64;

        // Gauss-Newton optimization
        for i in 0..self.settings.max_iterations {
            iterations = i + 1;

            // Calculate Jacobian and residual
            let mut jacobian = vec![vec![0.0; 4]; points.len()];
            let mut residual = vec![0.0; points.len()];

            for (j, point) in points.iter().enumerate() {
                let dx = point.x - center.x;
                let dy = point.y - center.y;
                let dz = point.z - center.z;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();

                jacobian[j][0] = -dx / dist;
                jacobian[j][1] = -dy / dist;
                jacobian[j][2] = -dz / dist;
                jacobian[j][3] = 1.0;

                residual[j] = dist - radius;
            }

            // Calculate J^T * J
            let mut jtj = [[0.0; 4]; 4];
            for j in 0..points.len() {
                for k in 0..4 {
                    for l in 0..4 {
                        jtj[k][l] += jacobian[j][k] * jacobian[j][l];
                    }
                }
            }

            // Calculate J^T * r
            let mut jtr = [0.0; 4];
            for j in 0..points.len() {
                for k in 0..4 {
                    jtr[k] += jacobian[j][k] * residual[j];
                }
            }

            // Solve linear system J^T * J * delta = J^T * r
            if let Some(delta) = self.solve_linear_system(jtj, jtr) {
                // Update parameters
                center.x -= delta[0];
                center.y -= delta[1];
                center.z -= delta[2];
                radius -= delta[3];

                // Calculate new error
                let mut new_error = 0.0;
                for point in points {
                    let dist = point.distance(&center);
                    new_error += (dist - radius).powi(2);
                }

                // Check convergence
                if (new_error - error).abs() < self.settings.tolerance {
                    error = new_error;
                    break;
                }
                error = new_error;
            } else {
                break;
            }
        }

        Some((SphereSurface { center, radius }, error, iterations))
    }

    /// Fit cylinder to points using least squares
    fn fit_cylinder(&self, points: &[Point]) -> (CylinderSurface, f64) {
        // Calculate centroid
        let mut centroid = Point::new(0.0, 0.0, 0.0);
        for point in points {
            centroid.x += point.x;
            centroid.y += point.y;
            centroid.z += point.z;
        }
        centroid.x /= points.len() as f64;
        centroid.y /= points.len() as f64;
        centroid.z /= points.len() as f64;

        // Calculate covariance matrix
        let mut cov = [[0.0; 3]; 3];
        for point in points {
            let dx = point.x - centroid.x;
            let dy = point.y - centroid.y;
            let dz = point.z - centroid.z;
            cov[0][0] += dx * dx;
            cov[0][1] += dx * dy;
            cov[0][2] += dx * dz;
            cov[1][1] += dy * dy;
            cov[1][2] += dy * dz;
            cov[2][2] += dz * dz;
        }
        cov[1][0] = cov[0][1];
        cov[2][0] = cov[0][2];
        cov[2][1] = cov[1][2];

        // Find eigenvector corresponding to smallest eigenvalue (cylinder axis)
        let (eigenvalues, eigenvectors) = self.calculate_eigen(cov);
        let mut min_eigenvalue = eigenvalues[0];
        let mut min_index = 0;
        for i in 1..3 {
            if eigenvalues[i] < min_eigenvalue {
                min_eigenvalue = eigenvalues[i];
                min_index = i;
            }
        }

        let axis = crate::geometry::Vector::new(
            eigenvectors[min_index][0],
            eigenvectors[min_index][1],
            eigenvectors[min_index][2],
        );

        // Project points onto plane perpendicular to axis
        let mut projected_points = Vec::new();
        for point in points {
            let vec = crate::geometry::Vector::new(
                point.x - centroid.x,
                point.y - centroid.y,
                point.z - centroid.z,
            );
            let dot = vec.dot(&axis);
            let proj_x = point.x - dot * axis.x;
            let proj_y = point.y - dot * axis.y;
            let proj_z = point.z - dot * axis.z;
            projected_points.push(Point::new(proj_x, proj_y, proj_z));
        }

        // Fit circle to projected points
        let mut circle_center = Point::new(0.0, 0.0, 0.0);
        for point in &projected_points {
            circle_center.x += point.x;
            circle_center.y += point.y;
            circle_center.z += point.z;
        }
        circle_center.x /= projected_points.len() as f64;
        circle_center.y /= projected_points.len() as f64;
        circle_center.z /= projected_points.len() as f64;

        // Calculate radius
        let mut radius = 0.0;
        for point in &projected_points {
            radius += point.distance(&circle_center);
        }
        radius /= projected_points.len() as f64;

        // Calculate height from projected points along axis
        let mut min_height = f64::MAX;
        let mut max_height = f64::MIN;
        for point in points {
            let vec = crate::geometry::Vector::new(
                point.x - centroid.x,
                point.y - centroid.y,
                point.z - centroid.z,
            );
            let dot = vec.dot(&axis);
            min_height = min_height.min(dot);
            max_height = max_height.max(dot);
        }
        let height = max_height - min_height;

        // Calculate fitting error
        let mut error = 0.0;
        for point in points {
            let vec = crate::geometry::Vector::new(
                point.x - circle_center.x,
                point.y - circle_center.y,
                point.z - circle_center.z,
            );
            let dot = vec.dot(&axis);
            let proj_x = vec.x - dot * axis.x;
            let proj_y = vec.y - dot * axis.y;
            let proj_z = vec.z - dot * axis.z;
            let dist = (proj_x * proj_x + proj_y * proj_y + proj_z * proj_z).sqrt();
            error += (dist - radius).powi(2);
        }

        (
            CylinderSurface {
                center: circle_center,
                axis,
                radius,
                height,
            },
            error,
        )
    }

    /// Calculate eigenvalues and eigenvectors of 3x3 matrix
    fn calculate_eigen(&self, matrix: [[f64; 3]; 3]) -> ([f64; 3], [[f64; 3]; 3]) {
        // Use Jacobi method for symmetric matrices
        let mut a = matrix;
        let mut v = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let max_iterations = 100;
        let tolerance = 1e-10;

        for _ in 0..max_iterations {
            // Find largest off-diagonal element
            let mut max_val = 0.0;
            let mut p = 0;
            let mut q = 1;

            for i in 0..3 {
                for j in i + 1..3 {
                    if a[i][j].abs() > max_val {
                        max_val = a[i][j].abs();
                        p = i;
                        q = j;
                    }
                }
            }

            // Check convergence
            if max_val < tolerance {
                break;
            }

            // Calculate rotation angle
            let theta;
            if a[p][p] == a[q][q] {
                theta = std::f64::consts::FRAC_PI_4;
            } else {
                theta = 0.5 * (2.0 * a[p][q] / (a[p][p] - a[q][q])).atan();
            }

            let c = theta.cos();
            let s = theta.sin();

            // Update eigenvectors
            for i in 0..3 {
                let vip = v[i][p];
                let viq = v[i][q];
                v[i][p] = c * vip - s * viq;
                v[i][q] = s * vip + c * viq;
            }

            // Update matrix
            let app = a[p][p];
            let aqq = a[q][q];
            let apq = a[p][q];

            a[p][p] = c * c * app + s * s * aqq - 2.0 * s * c * apq;
            a[q][q] = s * s * app + c * c * aqq + 2.0 * s * c * apq;
            a[p][q] = 0.0;
            a[q][p] = 0.0;

            for i in 0..3 {
                if i != p && i != q {
                    let aip = a[i][p];
                    let aiq = a[i][q];
                    a[i][p] = c * aip - s * aiq;
                    a[p][i] = a[i][p];
                    a[i][q] = s * aip + c * aiq;
                    a[q][i] = a[i][q];
                }
            }
        }

        // Extract eigenvalues from diagonal
        let eigenvalues = [a[0][0], a[1][1], a[2][2]];
        (eigenvalues, v)
    }

    /// Calculate distance from point to plane
    fn point_plane_distance(
        &self,
        point: &Point,
        plane_point: &Point,
        plane_normal: &crate::geometry::Vector,
    ) -> f64 {
        let vector = crate::geometry::Vector::new(
            point.x - plane_point.x,
            point.y - plane_point.y,
            point.z - plane_point.z,
        );
        vector.dot(plane_normal).abs()
    }

    /// Solve linear system using Gaussian elimination
    fn solve_linear_system(&self, a: [[f64; 4]; 4], b: [f64; 4]) -> Option<[f64; 4]> {
        // Simplified Gaussian elimination
        let mut aug = [[0.0; 5]; 4];
        for i in 0..4 {
            for j in 0..4 {
                aug[i][j] = a[i][j];
            }
            aug[i][4] = b[i];
        }

        // Forward elimination
        for i in 0..4 {
            // Find pivot
            let mut max_row = i;
            for j in i + 1..4 {
                if aug[j][i].abs() > aug[max_row][i].abs() {
                    max_row = j;
                }
            }

            // Swap rows
            if max_row != i {
                aug.swap(i, max_row);
            }

            // Check for singular matrix
            if aug[i][i].abs() < 1e-10 {
                return None;
            }

            // Eliminate
            for j in i + 1..4 {
                let factor = aug[j][i] / aug[i][i];
                for k in i..5 {
                    aug[j][k] -= factor * aug[i][k];
                }
            }
        }

        // Back substitution
        let mut x = [0.0; 4];
        for i in (0..4).rev() {
            x[i] = aug[i][4];
            for j in i + 1..4 {
                x[i] -= aug[i][j] * x[j];
            }
            x[i] /= aug[i][i];
        }

        Some(x)
    }

    /// Create simple mesh from points using Delaunay triangulation
    fn create_simple_mesh(&self, points: &[Point]) -> TriangleMesh {
        let mut mesh = TriangleMesh::new();

        if points.len() < 3 {
            return mesh;
        }

        // Delaunay triangulation using Bowyer-Watson algorithm
        // First, create a super triangle that encloses all points
        let mut min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;
        let mut min_z = points[0].z;
        let mut max_z = points[0].z;

        for point in points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
            min_z = min_z.min(point.z);
            max_z = max_z.max(point.z);
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let dz = max_z - min_z;
        let max_extent = dx.max(dy).max(dz);

        // Create super triangle vertices
        let super_v0 = Point::new(min_x - max_extent, min_y - max_extent, min_z);
        let super_v1 = Point::new(max_x + max_extent, min_y - max_extent, min_z);
        let super_v2 = Point::new((min_x + max_x) / 2.0, max_y + max_extent, min_z);

        // Add super triangle to the mesh
        let v0_id = mesh.add_vertex(super_v0);
        let v1_id = mesh.add_vertex(super_v1);
        let v2_id = mesh.add_vertex(super_v2);
        mesh.add_triangle(v0_id, v1_id, v2_id);

        // Process each point
        for point in points {
            let p_id = mesh.add_vertex(*point);

            // Find bad triangles (triangles whose circumcircle contains the point)
            let mut bad_triangles = Vec::new();
            for (tri_id, triangle) in mesh.faces.iter().enumerate() {
                let v0 = &mesh.vertices[triangle.vertices[0]].point;
                let v1 = &mesh.vertices[triangle.vertices[1]].point;
                let v2 = &mesh.vertices[triangle.vertices[2]].point;

                if self.point_in_circumcircle(point, v0, v1, v2) {
                    bad_triangles.push(tri_id);
                }
            }

            // Collect edges from bad triangles
            let mut edges = Vec::new();
            for &tri_id in &bad_triangles {
                let triangle = &mesh.faces[tri_id];
                edges.push((triangle.vertices[0], triangle.vertices[1]));
                edges.push((triangle.vertices[1], triangle.vertices[2]));
                edges.push((triangle.vertices[2], triangle.vertices[0]));
            }

            // Remove duplicate edges
            let mut unique_edges = Vec::new();
            for edge in &edges {
                let rev_edge = (edge.1, edge.0);
                if !unique_edges.contains(edge) && !unique_edges.contains(&rev_edge) {
                    unique_edges.push(*edge);
                }
            }

            // Remove bad triangles
            for &tri_id in bad_triangles.iter().rev() {
                mesh.faces.remove(tri_id);
            }

            // Create new triangles from the point and unique edges
            for edge in &unique_edges {
                mesh.add_triangle(edge.0, edge.1, p_id);
            }
        }

        // Remove triangles that include super triangle vertices
        let mut triangles_to_remove = Vec::new();
        for (tri_id, triangle) in mesh.faces.iter().enumerate() {
            if triangle.vertices.contains(&v0_id)
                || triangle.vertices.contains(&v1_id)
                || triangle.vertices.contains(&v2_id)
            {
                triangles_to_remove.push(tri_id);
            }
        }

        for &tri_id in triangles_to_remove.iter().rev() {
            mesh.faces.remove(tri_id);
        }

        // Remove super triangle vertices
        let mut vertices_to_remove = vec![v0_id, v1_id, v2_id];
        vertices_to_remove.sort_by(|a, b| b.cmp(a));

        for &v_id in &vertices_to_remove {
            if v_id < mesh.vertices.len() {
                mesh.vertices.remove(v_id);
            }
        }

        // Update triangle vertex indices
        for triangle in &mut mesh.faces {
            for v in &mut triangle.vertices {
                for &v_id in &vertices_to_remove {
                    if *v > v_id {
                        *v -= 1;
                    }
                }
            }
        }

        mesh
    }

    /// Check if a point is inside the circumcircle of a triangle
    fn point_in_circumcircle(&self, point: &Point, v0: &Point, v1: &Point, v2: &Point) -> bool {
        let dx = v0.x - point.x;
        let dy = v0.y - point.y;
        let dz = v0.z - point.z;

        let ex = v1.x - point.x;
        let ey = v1.y - point.y;
        let ez = v1.z - point.z;

        let fx = v2.x - point.x;
        let fy = v2.y - point.y;
        let fz = v2.z - point.z;

        let _ap = dx * dx + dy * dy + dz * dz;
        let bp = ex * ex + ey * ey + ez * ez;
        let cp = fx * fx + fy * fy + fz * fz;

        let determinant =
            dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + dz * (ex * fy - ey * fx);

        determinant > 0.0
    }

    /// RANSAC fitting
    fn ransac_fitting(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        if points.len() < 3 {
            return (None, None, 0.0, 0.0, 0);
        }

        let mut best_surface: Option<Box<dyn Surface>> = None;
        let mut best_error = f64::MAX;
        let mut best_inliers = 0;
        let iterations = 100;

        for _i in 0..iterations {
            // Randomly select sample points
            let sample_size = match self.settings.surface_type {
                SurfaceType::Plane => 3,
                SurfaceType::Sphere => 4,
                _ => 3,
            };

            if points.len() < sample_size {
                continue;
            }

            let mut sample = Vec::new();
            let mut indices = Vec::new();
            while sample.len() < sample_size {
                let idx = rand::random_range(0..points.len());
                if !indices.contains(&idx) {
                    sample.push(points[idx]);
                    indices.push(idx);
                }
            }

            // Fit surface to sample
            let (surface, _, error, _, _) = self.least_squares_fitting(&sample);
            if let Some(surf) = surface {
                // Count inliers
                let mut inliers = 0;
                for point in points {
                    let (closest, _, _) = self.find_closest_point(&*surf, point);
                    if point.distance(&closest) < self.settings.tolerance {
                        inliers += 1;
                    }
                }

                // Update best surface
                if inliers > best_inliers || (inliers == best_inliers && error < best_error) {
                    best_surface = Some(surf);
                    best_error = error;
                    best_inliers = inliers;
                }
            }
        }

        let r_squared = if best_inliers > 0 {
            best_inliers as f64 / points.len() as f64
        } else {
            0.0
        };

        // Create mesh if we have a best surface
        let mesh = if best_surface.is_some() {
            Some(self.create_simple_mesh(points))
        } else {
            None
        };

        (best_surface, mesh, best_error, r_squared, iterations)
    }

    /// Moving least squares fitting
    fn moving_least_squares_fitting(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        // Simplified MLS implementation
        let mesh = self.create_simple_mesh(points);
        (None, Some(mesh), 0.0, 0.0, 1)
    }

    /// Poisson surface reconstruction
    fn poisson_reconstruction(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        // Simplified Poisson reconstruction
        let mesh = self.create_simple_mesh(points);
        (None, Some(mesh), 0.0, 0.0, 1)
    }

    /// Alpha shapes reconstruction
    fn alpha_shapes_reconstruction(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        // Simplified Alpha shapes reconstruction
        let mesh = self.create_simple_mesh(points);
        (None, Some(mesh), 0.0, 0.0, 1)
    }

    /// Ball pivoting reconstruction
    fn ball_pivoting_reconstruction(
        &self,
        points: &[Point],
    ) -> (
        Option<Box<dyn Surface>>,
        Option<TriangleMesh>,
        f64,
        f64,
        usize,
    ) {
        // Simplified ball pivoting reconstruction
        let mesh = self.create_simple_mesh(points);
        (None, Some(mesh), 0.0, 0.0, 1)
    }

    /// Analyze surface quality
    pub fn analyze_surface(
        &self,
        surface: &dyn Surface,
        points: &[Point],
    ) -> SurfaceAnalysisResult {
        let start_time = std::time::Instant::now();

        let mut total_error = 0.0;
        let mut max_error = 0.0;
        let mut min_error = f64::MAX;
        let mut error_distribution = vec![0.0; 10]; // Error histogram

        // Calculate fitting error
        for point in points {
            let (closest_point, _u, _v) = self.find_closest_point(surface, point);
            let error = point.distance(&closest_point);
            total_error += error;
            max_error = f64::max(max_error, error);
            min_error = f64::min(min_error, error);

            // Update error distribution
            let bin = if max_error < 1e-6 {
                0
            } else {
                (error / max_error * 10.0).floor() as usize
            };
            let bin = bin.min(9);
            error_distribution[bin] += 1.0;
        }

        let mean_error = total_error / points.len() as f64;
        let rms_error = (total_error * total_error / points.len() as f64).sqrt();

        // Calculate surface properties
        let curvature_stats = self.calculate_curvature_stats(surface);
        let normal_consistency = self.calculate_normal_consistency(surface);

        let time_ms = start_time.elapsed().as_millis() as f64;

        SurfaceAnalysisResult {
            mean_error,
            max_error,
            min_error,
            rms_error,
            error_distribution,
            curvature_stats,
            normal_consistency,
            surface_type: surface.surface_type(),
            parameters: surface.parameters(),
            analysis_time_ms: time_ms,
        }
    }

    /// Find closest point on surface
    fn find_closest_point(&self, surface: &dyn Surface, point: &Point) -> (Point, f64, f64) {
        // For plane surface, use analytical solution
        if let SurfaceType::Plane = surface.surface_type() {
            // For plane, calculate closest point analytically
            let params = surface.parameters();
            let origin = Point::new(
                *params.get("origin_x").unwrap_or(&0.0),
                *params.get("origin_y").unwrap_or(&0.0),
                *params.get("origin_z").unwrap_or(&0.0),
            );
            let normal = crate::geometry::Vector::new(
                *params.get("normal_x").unwrap_or(&0.0),
                *params.get("normal_y").unwrap_or(&0.0),
                *params.get("normal_z").unwrap_or(&0.0),
            );

            // Calculate vector from origin to point
            let vec = crate::geometry::Vector::new(
                point.x - origin.x,
                point.y - origin.y,
                point.z - origin.z,
            );

            // Project vector onto normal
            let dot = vec.dot(&normal);
            let closest = Point::new(
                point.x - dot * normal.x,
                point.y - dot * normal.y,
                point.z - dot * normal.z,
            );

            // For plane, u and v are arbitrary
            return (closest, 0.0, 0.0);
        }

        // For other surfaces, use numerical method (simplified)
        let mut best_point = Point::new(0.0, 0.0, 0.0);
        let mut best_u = 0.0;
        let mut best_v = 0.0;
        let mut min_distance = f64::MAX;

        // Simple grid search
        for u in 0..10 {
            for v in 0..10 {
                let u_param = u as f64 / 9.0;
                let v_param = v as f64 / 9.0;
                let surface_point = surface.evaluate(u_param, v_param);
                let distance = point.distance(&surface_point);
                if distance < min_distance {
                    min_distance = distance;
                    best_point = surface_point;
                    best_u = u_param;
                    best_v = v_param;
                }
            }
        }

        (best_point, best_u, best_v)
    }

    /// Calculate curvature statistics
    fn calculate_curvature_stats(&self, surface: &dyn Surface) -> CurvatureStats {
        let mut mean_gaussian = 0.0;
        let mut mean_mean = 0.0;
        let mut max_gaussian = f64::MIN;
        let mut min_gaussian = f64::MAX;
        let mut max_mean = f64::MIN;
        let mut min_mean = f64::MAX;

        let sample_count = 100;
        let step = 1.0 / (sample_count as f64).sqrt();

        for i in 0..sample_count {
            let u = (i % 10) as f64 * step;
            let v = (i / 10) as f64 * step;

            let (k1, k2) = surface.curvature(u, v);
            let gaussian = k1 * k2;
            let mean = (k1 + k2) * 0.5;

            mean_gaussian += gaussian;
            mean_mean += mean;
            max_gaussian = f64::max(max_gaussian, gaussian);
            min_gaussian = f64::min(min_gaussian, gaussian);
            max_mean = f64::max(max_mean, mean);
            min_mean = f64::min(min_mean, mean);
        }

        mean_gaussian /= sample_count as f64;
        mean_mean /= sample_count as f64;

        CurvatureStats {
            mean_gaussian,
            mean_mean,
            max_gaussian,
            min_gaussian,
            max_mean,
            min_mean,
        }
    }

    /// Calculate normal consistency
    fn calculate_normal_consistency(&self, surface: &dyn Surface) -> f64 {
        // Calculate normal consistency by checking normal variations
        let sample_count = 100;
        let step = 1.0 / (sample_count as f64).sqrt();
        let mut normals = Vec::new();

        for i in 0..sample_count {
            let u = (i % 10) as f64 * step;
            let v = (i / 10) as f64 * step;
            let normal = surface.normal(u, v);
            normals.push(normal);
        }

        if normals.len() < 2 {
            return 1.0;
        }

        // Calculate average normal
        let mut avg_normal = crate::geometry::Vector::new(0.0, 0.0, 0.0);
        for normal in &normals {
            avg_normal.x += normal.x;
            avg_normal.y += normal.y;
            avg_normal.z += normal.z;
        }
        let avg_length = (avg_normal.x * avg_normal.x
            + avg_normal.y * avg_normal.y
            + avg_normal.z * avg_normal.z)
            .sqrt();
        if avg_length > 1e-10 {
            avg_normal.x /= avg_length;
            avg_normal.y /= avg_length;
            avg_normal.z /= avg_length;
        }

        // Calculate average dot product with average normal
        let mut total_dot = 0.0;
        for normal in &normals {
            let dot = normal.dot(&avg_normal).abs();
            total_dot += dot;
        }
        let consistency = total_dot / normals.len() as f64;

        consistency
    }
}

/// Surface analysis result
pub struct SurfaceAnalysisResult {
    pub mean_error: f64,
    pub max_error: f64,
    pub min_error: f64,
    pub rms_error: f64,
    pub error_distribution: Vec<f64>,
    pub curvature_stats: CurvatureStats,
    pub normal_consistency: f64,
    pub surface_type: SurfaceType,
    pub parameters: HashMap<String, f64>,
    pub analysis_time_ms: f64,
}

/// Curvature statistics
pub struct CurvatureStats {
    pub mean_gaussian: f64,
    pub mean_mean: f64,
    pub max_gaussian: f64,
    pub min_gaussian: f64,
    pub max_mean: f64,
    pub min_mean: f64,
}

/// Plane surface implementation
pub struct PlaneSurface {
    pub origin: Point,
    pub normal: crate::geometry::Vector,
}

impl Surface for PlaneSurface {
    fn evaluate(&self, u: f64, v: f64) -> Point {
        // Create a coordinate system for the plane
        // Find two orthogonal vectors in the plane
        let mut vec1 = crate::geometry::Vector::new(1.0, 0.0, 0.0);
        if vec1.dot(&self.normal).abs() > 0.9 {
            vec1 = crate::geometry::Vector::new(0.0, 1.0, 0.0);
        }
        let vec2 = self.normal.cross(&vec1).normalized();
        vec1 = vec1.cross(&vec2).normalized();

        // Calculate point on plane
        let point = Point::new(
            self.origin.x + u * vec1.x + v * vec2.x,
            self.origin.y + u * vec1.y + v * vec2.y,
            self.origin.z + u * vec1.z + v * vec2.z,
        );
        point
    }

    fn normal(&self, _u: f64, _v: f64) -> crate::geometry::Vector {
        self.normal
    }

    fn curvature(&self, _u: f64, _v: f64) -> (f64, f64) {
        (0.0, 0.0) // Plane has zero curvature
    }

    fn surface_type(&self) -> SurfaceType {
        SurfaceType::Plane
    }

    fn parameters(&self) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("origin_x".to_string(), self.origin.x);
        params.insert("origin_y".to_string(), self.origin.y);
        params.insert("origin_z".to_string(), self.origin.z);
        params.insert("normal_x".to_string(), self.normal.x);
        params.insert("normal_y".to_string(), self.normal.y);
        params.insert("normal_z".to_string(), self.normal.z);
        params
    }

    fn to_mesh(&self, resolution: usize) -> TriangleMesh {
        let mut mesh = TriangleMesh::new();
        let size = 1.0;

        // Create a grid of points
        for i in 0..resolution {
            for j in 0..resolution {
                let u = (i as f64) / (resolution - 1) as f64 * size * 2.0 - size;
                let v = (j as f64) / (resolution - 1) as f64 * size * 2.0 - size;
                let point = self.evaluate(u, v);
                mesh.add_vertex(point);
            }
        }

        // Create triangles
        for i in 0..resolution - 1 {
            for j in 0..resolution - 1 {
                let idx = i * resolution + j;
                mesh.add_triangle(idx, idx + 1, idx + resolution);
                mesh.add_triangle(idx + 1, idx + resolution + 1, idx + resolution);
            }
        }

        mesh
    }
}

/// Sphere surface implementation
pub struct SphereSurface {
    pub center: Point,
    pub radius: f64,
}

impl Surface for SphereSurface {
    fn evaluate(&self, u: f64, v: f64) -> Point {
        // Convert u, v to spherical coordinates
        let theta = u * 2.0 * std::f64::consts::PI;
        let phi = v * std::f64::consts::PI;

        // Calculate point on sphere
        let x = self.center.x + self.radius * phi.sin() * theta.cos();
        let y = self.center.y + self.radius * phi.sin() * theta.sin();
        let z = self.center.z + self.radius * phi.cos();

        Point::new(x, y, z)
    }

    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector {
        // Calculate point on sphere
        let point = self.evaluate(u, v);

        // Normal is vector from center to point
        let normal = crate::geometry::Vector::new(
            point.x - self.center.x,
            point.y - self.center.y,
            point.z - self.center.z,
        )
        .normalized();

        normal
    }

    fn curvature(&self, _u: f64, _v: f64) -> (f64, f64) {
        let curvature = 1.0 / self.radius;
        (curvature, curvature) // Sphere has constant curvature
    }

    fn surface_type(&self) -> SurfaceType {
        SurfaceType::Sphere
    }

    fn parameters(&self) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("center_x".to_string(), self.center.x);
        params.insert("center_y".to_string(), self.center.y);
        params.insert("center_z".to_string(), self.center.z);
        params.insert("radius".to_string(), self.radius);
        params
    }

    fn to_mesh(&self, resolution: usize) -> TriangleMesh {
        let mut mesh = TriangleMesh::new();

        // Create vertices
        for i in 0..=resolution {
            let v = i as f64 / resolution as f64;
            for j in 0..=resolution {
                let u = j as f64 / resolution as f64;
                let point = self.evaluate(u, v);
                mesh.add_vertex(point);
            }
        }

        // Create triangles
        for i in 0..resolution {
            for j in 0..resolution {
                let idx = i * (resolution + 1) + j;
                mesh.add_triangle(idx, idx + 1, idx + resolution + 1);
                mesh.add_triangle(idx + 1, idx + resolution + 2, idx + resolution + 1);
            }
        }

        mesh
    }
}

/// Cylinder surface implementation
pub struct CylinderSurface {
    pub center: Point,
    pub axis: crate::geometry::Vector,
    pub radius: f64,
    pub height: f64,
}

impl Surface for CylinderSurface {
    fn evaluate(&self, u: f64, v: f64) -> Point {
        // Convert u, v to cylinder coordinates
        let theta = u * 2.0 * std::f64::consts::PI;
        let height = v * self.height - self.height * 0.5;

        // Create orthogonal vectors to axis
        let mut vec1 = crate::geometry::Vector::new(1.0, 0.0, 0.0);
        if vec1.dot(&self.axis).abs() > 0.9 {
            vec1 = crate::geometry::Vector::new(0.0, 1.0, 0.0);
        }
        let vec2 = self.axis.cross(&vec1).normalized();
        vec1 = vec1.cross(&vec2).normalized();

        // Calculate point on cylinder
        let x = self.center.x
            + self.radius * theta.cos() * vec1.x
            + self.radius * theta.sin() * vec2.x
            + height * self.axis.x;
        let y = self.center.y
            + self.radius * theta.cos() * vec1.y
            + self.radius * theta.sin() * vec2.y
            + height * self.axis.y;
        let z = self.center.z
            + self.radius * theta.cos() * vec1.z
            + self.radius * theta.sin() * vec2.z
            + height * self.axis.z;

        Point::new(x, y, z)
    }

    fn normal(&self, u: f64, _v: f64) -> crate::geometry::Vector {
        // Convert u to angle
        let theta = u * 2.0 * std::f64::consts::PI;

        // Create orthogonal vectors to axis
        let mut vec1 = crate::geometry::Vector::new(1.0, 0.0, 0.0);
        if vec1.dot(&self.axis).abs() > 0.9 {
            vec1 = crate::geometry::Vector::new(0.0, 1.0, 0.0);
        }
        let vec2 = self.axis.cross(&vec1).normalized();
        vec1 = vec1.cross(&vec2).normalized();

        // Calculate normal
        let normal = crate::geometry::Vector::new(
            theta.cos() * vec1.x + theta.sin() * vec2.x,
            theta.cos() * vec1.y + theta.sin() * vec2.y,
            theta.cos() * vec1.z + theta.sin() * vec2.z,
        );

        normal
    }

    fn curvature(&self, _u: f64, _v: f64) -> (f64, f64) {
        let curvature = 1.0 / self.radius;
        (curvature, 0.0) // Cylinder has curvature only in one direction
    }

    fn surface_type(&self) -> SurfaceType {
        SurfaceType::Cylinder
    }

    fn parameters(&self) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("center_x".to_string(), self.center.x);
        params.insert("center_y".to_string(), self.center.y);
        params.insert("center_z".to_string(), self.center.z);
        params.insert("axis_x".to_string(), self.axis.x);
        params.insert("axis_y".to_string(), self.axis.y);
        params.insert("axis_z".to_string(), self.axis.z);
        params.insert("radius".to_string(), self.radius);
        params.insert("height".to_string(), self.height);
        params
    }

    fn to_mesh(&self, resolution: usize) -> TriangleMesh {
        let mut mesh = TriangleMesh::new();

        // Create vertices
        for i in 0..=resolution {
            let v = i as f64 / resolution as f64;
            for j in 0..=resolution {
                let u = j as f64 / resolution as f64;
                let point = self.evaluate(u, v);
                mesh.add_vertex(point);
            }
        }

        // Create triangles
        for i in 0..resolution {
            for j in 0..resolution {
                let idx = i * (resolution + 1) + j;
                mesh.add_triangle(idx, idx + 1, idx + resolution + 1);
                mesh.add_triangle(idx + 1, idx + resolution + 2, idx + resolution + 1);
            }
        }

        mesh
    }
}
