use crate::geometry::bezier_curve2d::BezierCurve2D;
use crate::geometry::Point;

/// Bezier curve boolean operation: intersection (find approximate intersection points)
pub fn bezier_intersect(
    curve1: &BezierCurve2D,
    curve2: &BezierCurve2D,
    tol: f64,
    samples: usize,
) -> Vec<(f64, f64, Point)> {
    let mut intersections = Vec::new();
    for i in 0..=samples {
        let t1 = i as f64 / samples as f64;
        let p1 = curve1.position(t1);
        for j in 0..=samples {
            let t2 = j as f64 / samples as f64;
            let p2 = curve2.position(t2);
            let dist =
                ((p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2) + (p1.z() - p2.z()).powi(2))
                    .sqrt();
            if dist < tol {
                intersections.push((t1, t2, p1));
            }
        }
    }
    intersections
}

/// Bezier curve union (merge control points, not a true geometric union)
pub fn bezier_union(curve1: &BezierCurve2D, curve2: &BezierCurve2D) -> Option<BezierCurve2D> {
    let mut poles = curve1.poles().to_vec();
    poles.extend_from_slice(curve2.poles());
    Some(BezierCurve2D::new(poles))
}

/// Bezier curve difference (remove overlapping control points)
pub fn bezier_difference(curve1: &BezierCurve2D, curve2: &BezierCurve2D) -> Option<BezierCurve2D> {
    let mut poles = curve1.poles().to_vec();
    for p in curve2.poles() {
        poles.retain(|x| x.x() != p.x() || x.y() != p.y() || x.z() != p.z());
    }
    if poles.is_empty() {
        None
    } else {
        Some(BezierCurve2D::new(poles))
    }
}
