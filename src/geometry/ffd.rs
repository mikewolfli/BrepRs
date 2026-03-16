use crate::api::traits::Transformable;
use crate::foundation::StandardReal;
use crate::geometry::{Point, Vector};

/// Free Form Deformation (FFD) for surface and shape deformation
/// Used for cell growth, tissue expansion, and soft body deformation
#[derive(Debug, Clone)]
pub struct FFD {
    /// Control points grid
    control_points: Vec<Vec<Vec<Point>>>,
    /// Grid dimensions (u, v, w)
    dimensions: (usize, usize, usize),
    /// Bounding box of the FFD volume
    bounding_box: (Point, Point),
}

impl FFD {
    /// Create a new FFD with given control points
    pub fn new(control_points: Vec<Vec<Vec<Point>>>) -> Self {
        let u_dim = control_points.len();
        let v_dim = if u_dim > 0 {
            control_points[0].len()
        } else {
            0
        };
        let w_dim = if v_dim > 0 && u_dim > 0 {
            control_points[0][0].len()
        } else {
            0
        };

        // Calculate bounding box
        let mut min_point = Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max_point = Point::new(f64::MIN, f64::MIN, f64::MIN);

        for u in 0..u_dim {
            for v in 0..v_dim {
                for w in 0..w_dim {
                    let point = control_points[u][v][w];
                    min_point.x = min_point.x.min(point.x);
                    min_point.y = min_point.y.min(point.y);
                    min_point.z = min_point.z.min(point.z);
                    max_point.x = max_point.x.max(point.x);
                    max_point.y = max_point.y.max(point.y);
                    max_point.z = max_point.z.max(point.z);
                }
            }
        }

        Self {
            control_points,
            dimensions: (u_dim, v_dim, w_dim),
            bounding_box: (min_point, max_point),
        }
    }

    /// Create a regular FFD grid around a bounding box
    pub fn create_regular_grid(
        min_point: Point,
        max_point: Point,
        u_divisions: usize,
        v_divisions: usize,
        w_divisions: usize,
    ) -> Self {
        let mut control_points = Vec::with_capacity(u_divisions + 1);

        for u in 0..=u_divisions {
            let mut v_points = Vec::with_capacity(v_divisions + 1);
            let u_param = u as StandardReal / u_divisions as StandardReal;

            for v in 0..=v_divisions {
                let mut w_points = Vec::with_capacity(w_divisions + 1);
                let v_param = v as StandardReal / v_divisions as StandardReal;

                for w in 0..=w_divisions {
                    let w_param = w as StandardReal / w_divisions as StandardReal;

                    let x = min_point.x + (max_point.x - min_point.x) * u_param;
                    let y = min_point.y + (max_point.y - min_point.y) * v_param;
                    let z = min_point.z + (max_point.z - min_point.z) * w_param;

                    w_points.push(Point::new(x, y, z));
                }

                v_points.push(w_points);
            }

            control_points.push(v_points);
        }

        Self::new(control_points)
    }

    /// Get the control points
    pub fn control_points(&self) -> &Vec<Vec<Vec<Point>>> {
        &self.control_points
    }

    /// Set a control point
    pub fn set_control_point(&mut self, u: usize, v: usize, w: usize, point: Point) {
        if u < self.dimensions.0 && v < self.dimensions.1 && w < self.dimensions.2 {
            self.control_points[u][v][w] = point;
        }
    }

    /// Deform a point using FFD
    pub fn deform_point(&self, point: &Point) -> Point {
        // Convert point to parametric coordinates (u, v, w) in [0, 1]^3
        let (u, v, w) = self.point_to_parametric(point);

        // Calculate B-spline basis functions
        let u_basis = self.calculate_basis_functions(u, self.dimensions.0 - 1);
        let v_basis = self.calculate_basis_functions(v, self.dimensions.1 - 1);
        let w_basis = self.calculate_basis_functions(w, self.dimensions.2 - 1);

        // Compute weighted sum of control points
        let mut deformed_point = Point::origin();

        for (i, u_weight) in u_basis.iter().enumerate() {
            for (j, v_weight) in v_basis.iter().enumerate() {
                for (k, w_weight) in w_basis.iter().enumerate() {
                    let weight = u_weight * v_weight * w_weight;
                    let control_point = self.control_points[i][j][k];
                    deformed_point = deformed_point + (control_point - Point::origin()) * weight;
                }
            }
        }

        deformed_point
    }

    /// Convert a point to parametric coordinates
    fn point_to_parametric(&self, point: &Point) -> (StandardReal, StandardReal, StandardReal) {
        let (min, max) = &self.bounding_box;

        let u = (point.x - min.x) / (max.x - min.x).max(1e-10);
        let v = (point.y - min.y) / (max.y - min.y).max(1e-10);
        let w = (point.z - min.z) / (max.z - min.z).max(1e-10);

        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0), w.clamp(0.0, 1.0))
    }

    /// Calculate B-spline basis functions for a parameter t and degree n
    fn calculate_basis_functions(&self, t: StandardReal, n: usize) -> Vec<StandardReal> {
        let mut basis = vec![0.0; n + 1];

        // Simple linear basis for now (can be extended to higher order)
        if n == 1 {
            // Linear basis
            basis[0] = 1.0 - t;
            basis[1] = t;
        } else if n == 2 {
            // Quadratic basis
            basis[0] = 0.5 * (1.0 - t) * (1.0 - t);
            basis[1] = 0.5 * (1.0 - t) * t * 2.0;
            basis[2] = 0.5 * t * t;
        } else if n == 3 {
            // Cubic basis
            basis[0] = (1.0 - t) * (1.0 - t) * (1.0 - t) / 6.0;
            basis[1] = (3.0 * t * t * t - 6.0 * t * t + 4.0) / 6.0;
            basis[2] = (-3.0 * t * t * t + 3.0 * t * t + 3.0 * t + 1.0) / 6.0;
            basis[3] = t * t * t / 6.0;
        } else {
            // Fallback to linear for higher dimensions
            for i in 0..=n {
                let i_float = i as StandardReal;
                let n_float = n as StandardReal;

                if t <= i_float / n_float {
                    basis[i] = 1.0;
                    break;
                } else if t >= (i_float + 1.0) / n_float {
                    basis[i] = 0.0;
                } else {
                    let local_t = (t - i_float / n_float) * n_float;
                    basis[i] = 1.0 - local_t;
                    if i < n {
                        basis[i + 1] = local_t;
                    }
                    break;
                }
            }
        }

        basis
    }

    /// Deform a vector using FFD (approximate)
    pub fn deform_vector(&self, point: &Point, vector: &Vector) -> Vector {
        // Deform two points along the vector and compute the difference
        let point1 = *point;
        let point2 = Point::new(
            point.x() + vector.x(),
            point.y() + vector.y(),
            point.z() + vector.z(),
        );

        let deformed_point1 = self.deform_point(&point1);
        let deformed_point2 = self.deform_point(&point2);

        deformed_point2 - deformed_point1
    }
}

/// Trait for objects that can be deformed by FFD
pub trait DeformableByFFD: Transformable {
    /// Deform the object using FFD
    fn deform_with_ffd(&self, ffd: &FFD) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffd_creation() {
        let min = Point::new(0.0, 0.0, 0.0);
        let max = Point::new(1.0, 1.0, 1.0);
        let ffd = FFD::create_regular_grid(min, max, 2, 2, 2);

        assert_eq!(ffd.dimensions, (3, 3, 3));
        assert_eq!(ffd.control_points[0][0][0], min);
        assert_eq!(ffd.control_points[2][2][2], max);
    }

    #[test]
    fn test_ffd_deformation() {
        let min = Point::new(0.0, 0.0, 0.0);
        let max = Point::new(1.0, 1.0, 1.0);
        let mut ffd = FFD::create_regular_grid(min, max, 1, 1, 1);

        // Move the top control point
        ffd.set_control_point(1, 1, 1, Point::new(1.0, 1.0, 2.0));

        // Test deformation at the center
        let center = Point::new(0.5, 0.5, 0.5);
        let deformed = ffd.deform_point(&center);

        // The center should move up by 0.5
        assert!((deformed.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ffd_basis_functions() {
        let min = Point::new(0.0, 0.0, 0.0);
        let max = Point::new(1.0, 1.0, 1.0);
        let ffd = FFD::create_regular_grid(min, max, 3, 3, 3);

        let basis = ffd.calculate_basis_functions(0.5, 3);
        assert_eq!(basis.len(), 4);

        // Sum of basis functions should be 1
        let sum: StandardReal = basis.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
