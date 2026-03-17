use crate::geometry::projection_closest::curve_project_closest;
use crate::geometry::traits::GetCoord;

/// Compute distance between two points
fn distance<P: GetCoord>(p1: &P, p2: &P) -> f64 {
    let (x1, y1, z1) = p1.coord();
    let (x2, y2, z2) = p2.coord();
    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

/// Batch query for distance, intersection, bounding box, closest point
/// Assumes Curve/Surface trait implementation
pub fn curve_distance_batch<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    points: &[C::Point],
) -> Vec<f64> {
    points
        .iter()
        .map(|p| {
            // For each point, find closest point on curve
            if let Some((_, closest)) = curve_project_closest(curve, p, 0.5, 20, 1e-8) {
                // Compute distance from point to closest point
                distance(p, &closest)
            } else {
                // Return large value if closest point not found
                f64::MAX
            }
        })
        .collect()
}

pub fn curve_bbox<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    t0: f64,
    t1: f64,
    n: usize,
) -> ((f64, f64, f64), (f64, f64, f64)) {
    // Initialize bounding box
    let mut min = (f64::MAX, f64::MAX, f64::MAX);
    let mut max = (f64::MIN, f64::MIN, f64::MIN);

    // Sample points on curve
    for i in 0..=n {
        let t = t0 + (t1 - t0) * (i as f64) / (n as f64);
        let p = curve.sample(t);
        let (x, y, z) = p.coord();

        // Update bounding box
        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        min.2 = min.2.min(z);
        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
        max.2 = max.2.max(z);
    }

    (min, max)
}

/// Curve intersection batch query (sampling + distance check)
pub fn curve_intersections<C: crate::geometry::advanced_traits::Curve>(
    curve1: &C,
    curve2: &C,
    t0: f64,
    t1: f64,
    n: usize,
    tol: f64,
) -> Vec<(f64, f64)> {
    let mut intersections = Vec::new();

    // Sample first curve
    for i in 0..=n {
        let t = t0 + (t1 - t0) * (i as f64) / (n as f64);
        let p1 = curve1.sample(t);

        // For each sample point, find closest point on second curve
        if let Some((s, p2)) = curve_project_closest(curve2, &p1, 0.5, 20, 1e-8) {
            // Check if two points are close enough
            if distance(&p1, &p2) < tol {
                intersections.push((t, s));
            }
        }
    }

    // 完整实现：空间索引+容差判定去重
    use std::collections::HashSet;
    let mut unique_set = HashSet::new();
    let mut unique_intersections = Vec::new();
    for &(t, s) in &intersections {
        // 量化参数，避免浮点误差
        let key = ((t / tol).round() as i64, (s / tol).round() as i64);
        if unique_set.insert(key) {
            unique_intersections.push((t, s));
        }
    }
    unique_intersections
}
