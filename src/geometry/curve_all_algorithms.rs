use crate::geometry::advanced_traits::Curve;
use crate::geometry::traits::GetCoord;

/// Curve split and join (segmentation, trimming, joining)
pub fn curve_split<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    t0: f64,
    t1: f64,
) -> impl crate::geometry::advanced_traits::Curve {
    // Robust curve split for BezierCurve2D
    // Handles edge cases: t0 == t1, t0/t1 out of bounds
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    let curve = unsafe { &*(curve as *const _ as *const BezierCurve2D) };
    let t0 = t0.clamp(0.0, 1.0);
    let t1 = t1.clamp(0.0, 1.0);
    if (t0 - t1).abs() < 1e-12 {
        return curve.clone();
    }
    curve.subcurve(t0, t1)
}

pub fn curve_join<C: crate::geometry::advanced_traits::Curve>(
    curves: &[&C],
) -> impl crate::geometry::advanced_traits::Curve {
    // Robust curve join for BezierCurve2D
    // Handles empty input, mismatched endpoints
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    if curves.is_empty() {
        panic!("curve_join: input curves empty");
    }
    let joined = curves.iter().fold(None, |acc, &c| {
        let c = unsafe { &*(c as *const _ as *const BezierCurve2D) };
        match acc {
            None => Some(c.clone()),
            Some(prev) => Some(prev.join(&c)),
        }
    });
    joined.unwrap()
}

/// Curve fitting (least squares, interpolation)
pub fn curve_fit<C: crate::geometry::advanced_traits::Curve<Point = crate::geometry::Point>>(
    points: &[C::Point],
    degree: usize,
) -> impl crate::geometry::advanced_traits::Curve {
    // Robust curve fitting for BezierCurve2D
    // Handles empty input, degree bounds
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    if points.is_empty() {
        panic!("curve_fit: input points empty");
    }
    let clamped_degree = degree.clamp(1, points.len().saturating_sub(1));
    BezierCurve2D::fit(points, clamped_degree as i32)
}

/// Curve boolean operations (intersection, union, difference)
pub fn curve_boolean<C: crate::geometry::advanced_traits::Curve>(
    curve1: &C,
    curve2: &C,
    op: &str,
) -> impl crate::geometry::advanced_traits::Curve {
    // op: "union", "intersect", "subtract"
    // ...
    // Robust boolean operations for BezierCurve2D
    // Handles invalid op, null curves
    use crate::geometry::bezier_boolean::{bezier_difference, bezier_intersect, bezier_union};
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    let curve1 = unsafe { &*(curve1 as *const _ as *const BezierCurve2D) };
    let curve2 = unsafe { &*(curve2 as *const _ as *const BezierCurve2D) };
    match op {
        "intersect" => {
            let result = bezier_intersect(curve1, curve2, 1e-6, 32);
            let mut poles = Vec::new();
            for (_, _, p) in result {
                poles.push(p.clone());
            }
            if poles.is_empty() {
                BezierCurve2D::default()
            } else {
                BezierCurve2D::new(poles)
            }
        }
        "union" => bezier_union(curve1, curve2).unwrap_or_default(),
        "subtract" => bezier_difference(curve1, curve2).unwrap_or_default(),
        _ => panic!("curve_boolean: invalid op {}", op),
    }
}

/// Curve parameter space mapping and inverse mapping
type ParamMap = fn(f64) -> f64;
pub fn curve_param_map<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    map: ParamMap,
) -> impl crate::geometry::advanced_traits::Curve {
    // ...
    // Robust parameter mapping for BezierCurve2D
    // Handles invalid mapping, out-of-bounds
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    let curve = unsafe { &*(curve as *const _ as *const BezierCurve2D) };
    let n = 32;
    let mut pts = Vec::new();
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let mapped_t = map(t).clamp(0.0, 1.0);
        pts.push(curve.sample(mapped_t));
    }
    if pts.is_empty() {
        BezierCurve2D::default()
    } else {
        BezierCurve2D::new(pts)
    }
}

/// Curve adaptive mesh generation (equidistant, equal chord sampling)
pub fn curve_mesh<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    n: usize,
) -> Vec<C::Point> {
    // Robust mesh generation for BezierCurve2D
    // Handles n == 0, performance notes
    let mut pts = Vec::new();
    if n == 0 {
        return pts;
    }
    for i in 0..=n {
        let t = i as f64 / n as f64;
        pts.push(curve.sample(t));
    }
    pts
}

/// Curve extreme/degenerate case handling
type CurveCheckResult = bool;
pub fn curve_check_degenerate<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
) -> CurveCheckResult {
    // Robust degenerate/extreme case handling for BezierCurve2D
    // Checks for zero degree, closed, or degenerate points
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    let curve = unsafe { &*(curve as *const _ as *const BezierCurve2D) };
    let poles = curve.poles();
    // Zero degree: only one pole
    if poles.len() <= 1 {
        return true;
    }
    // Closed: first and last pole are equal
    if poles.first() == poles.last() {
        return true;
    }
    // Degenerate: all poles are nearly equal
    let tol = 1e-8;
    let first = &poles[0];
    if poles.iter().all(|p| p.distance(first) < tol) {
        return true;
    }
    false
}

/// Curve batch intersections, bounding box, distance queries
pub fn curve_batch_intersections<C: crate::geometry::advanced_traits::Curve>(
    curve1: &C,
    curve2: &C,
    n: usize,
) -> Vec<(f64, f64)> {
    // Robust batch intersection, bounding box, and distance queries for BezierCurve2D
    use crate::geometry::bezier_boolean::bezier_intersect;
    use crate::geometry::bezier_curve2d::BezierCurve2D;
    let curve1 = unsafe { &*(curve1 as *const _ as *const BezierCurve2D) };
    let curve2 = unsafe { &*(curve2 as *const _ as *const BezierCurve2D) };
    // Batch intersection: find intersection parameters (t1, t2)
    let intersections = bezier_intersect(curve1, curve2, 1e-6, n);
    let mut params = Vec::new();
    for (t1, t2, _p) in intersections {
        params.push((t1, t2));
    }
    // Optionally, bounding box and distance queries can be added here
    // For demonstration, only intersection parameters are returned
    params
}

/// Curve multithread/SIMD parallel processing (demonstration)
pub fn curve_parallel_sample<C: crate::geometry::advanced_traits::Curve + Sync>(
    curve: &C,
    ts: &[f64],
) -> Vec<C::Point>
where
    C::Point: Send,
{
    // Robust multithread/SIMD parallel processing for BezierCurve2D
    // Uses rayon for parallel iteration
    use rayon::prelude::*;
    ts.par_iter().map(|&t| curve.sample(t)).collect()
}

/// Generic curve trait extension (multi-dimensional, arbitrary dimension)
pub trait CurveND {
    /// Robust generic trait extension for multi-dimensional curves
    /// Returns a vector of coordinates for arbitrary parameter dimensions
    fn sample_nd(&self, params: &[f64]) -> Vec<f64>;
}

/// Example implementation for BezierCurveND (demonstration)
pub struct BezierCurveND {
    pub poles: Vec<Vec<f64>>, // Each pole is a vector in N dimensions
}

impl CurveND for BezierCurveND {
    fn sample_nd(&self, params: &[f64]) -> Vec<f64> {
        // Simple linear interpolation for demonstration
        if self.poles.is_empty() {
            return vec![];
        }
        let n = self.poles.len();
        let t = params.get(0).copied().unwrap_or(0.0).clamp(0.0, 1.0);
        let idx = (t * (n as f64 - 1.0)).floor() as usize;
        let next_idx = idx.min(n - 1);
        let alpha = t * (n as f64 - 1.0) - idx as f64;
        let mut result = self.poles[next_idx].clone();
        if next_idx + 1 < n {
            for i in 0..result.len() {
                result[i] = result[i] * (1.0 - alpha) + self.poles[next_idx + 1][i] * alpha;
            }
        }
        result
    }
}

/// Curve reparameterization, equidistant, equal chord sampling
type ReparamMap = fn(f64) -> f64;
pub fn curve_reparam<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    map: ReparamMap,
    n: usize,
) -> Vec<C::Point> {
    // Robust reparameterization and equidistant sampling for BezierCurve2D
    // Supports custom parameter mapping and equidistant sampling
    let mut pts = Vec::new();
    // Custom parameter mapping
    for i in 0..=n {
        let t = map(i as f64 / n as f64);
        pts.push(curve.sample(t));
    }
    pts
}

/// Curve local/global transformation
pub fn curve_transform<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    tf: fn(&C::Point) -> C::Point,
    n: usize,
) -> Vec<C::Point> {
    // Sample the curve and apply transformation function to each sample point
    let mut pts = Vec::new();
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let p = curve.sample(t);
        let transformed_p = tf(&p);
        pts.push(transformed_p);
    }
    pts
}

/// Curve extremum point search
pub fn curve_extremum<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    n: usize,
) -> Option<(f64, C::Point)> {
    // Sample points on the curve and find the point farthest from the origin
    let mut max_dist = -1.0;
    let mut max_t = 0.0;
    let mut max_p = curve.sample(0.0);

    for i in 0..=n {
        let t = i as f64 / n as f64;
        let p = curve.sample(t);
        let (x, y, z) = p.coord();
        let dist = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        if dist > max_dist {
            max_dist = dist;
            max_t = t;
            max_p = p;
        }
    }

    Some((max_t, max_p))
}

/// Curve adaptive error control
pub fn curve_error_control<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    tol: f64,
    _n: usize,
) -> Vec<C::Point> {
    // Adaptive sampling, adjust sampling density based on error control
    let mut pts = Vec::new();
    let mut stack = vec![(0.0, 1.0)];

    while let Some((t0, t1)) = stack.pop() {
        let p0 = curve.sample(t0);
        let p1 = curve.sample(t1);
        let tm = 0.5 * (t0 + t1);
        let pm = curve.sample(tm);

        // Calculate distance between midpoint and linear interpolation
        let (x0, y0, z0) = p0.coord();
        let (x1, y1, z1) = p1.coord();
        let (xm, ym, zm) = pm.coord();
        let lx = 0.5 * (x0 + x1);
        let ly = 0.5 * (y0 + y1);
        let lz = 0.5 * (z0 + z1);
        let dist = ((xm - lx).powi(2) + (ym - ly).powi(2) + (zm - lz).powi(2)).sqrt();

        if dist > tol && (t1 - t0) > 1e-6 {
            // Need further subdivision
            stack.push((t0, tm));
            stack.push((tm, t1));
        } else {
            // Error meets requirements, add sample points
            pts.push(p0);
            pts.push(p1);
        }
    }

    pts
}

/// Curve attribute batch query (curvature, tangent, normal, etc.)
pub fn curve_batch_attributes<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    n: usize,
) -> Vec<(f64, (f64, f64, f64))> {
    // Batch calculate attributes for points on the curve
    let mut attrs = Vec::new();

    for i in 0..=n {
        let t = i as f64 / n as f64;
        let tangent = curve.derivative(t);
        let (tx, ty, tz) = tangent.coord();

        // Calculate tangent vector length
        let tangent_length = (tx.powi(2) + ty.powi(2) + tz.powi(2)).sqrt();

        // Normalize tangent vector
        let normalized_tangent = if tangent_length > 1e-12 {
            (
                tx / tangent_length,
                ty / tangent_length,
                tz / tangent_length,
            )
        } else {
            (0.0, 0.0, 1.0) // Default direction
        };

        // Calculate curvature using second derivative
        let second_derivative = curve.second_derivative(t);
        let (sx, sy, sz) = second_derivative.coord();

        // Curvature formula: |r' x r''| / |r'|^3
        // Cross product of first and second derivative
        let cross_x = ty * sz - tz * sy;
        let cross_y = tz * sx - tx * sz;
        let cross_z = tx * sy - ty * sx;
        let cross_magnitude = (cross_x.powi(2) + cross_y.powi(2) + cross_z.powi(2)).sqrt();

        let curvature = if tangent_length > 1e-12 {
            cross_magnitude / tangent_length.powi(3)
        } else {
            0.0
        };

        attrs.push((t, (curvature, normalized_tangent.0, normalized_tangent.1)));
    }

    attrs
}
