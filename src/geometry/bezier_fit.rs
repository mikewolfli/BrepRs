use crate::geometry::bezier_curve2d::BezierCurve2D;
use crate::geometry::Point;

/// Bezier curve interpolation fitting (generate Bezier curve from given points)
pub fn bezier_fit(points: &[Point]) -> Option<BezierCurve2D> {
    if points.is_empty() {
        return None;
    }
    // Use points directly as control points, weights are 1
    Some(BezierCurve2D::new(points.to_vec()))
}

/// Bezier curve least squares fitting (simple implementation: fit a line)
pub fn bezier_least_squares(points: &[Point]) -> Option<BezierCurve2D> {
    if points.len() < 2 {
        return None;
    }
    // Fit a line: use first and last points as control points
    let mut poles = Vec::new();
    poles.push(points[0]);
    poles.push(points[points.len() - 1]);
    Some(BezierCurve2D::new(poles))
}

/// Bezier spline fitting (piecewise Bezier joining)
pub fn bezier_spline_fit(points: &[Point], segment: usize) -> Option<BezierCurve2D> {
    if points.len() < 2 || segment < 1 {
        return None;
    }
    let mut curves = Vec::new();
    let seg_len = points.len() / segment;
    for i in 0..segment {
        let start = i * seg_len;
        let end = if i == segment - 1 {
            points.len()
        } else {
            (i + 1) * seg_len
        };
        if end - start >= 2 {
            curves.push(BezierCurve2D::new(points[start..end].to_vec()));
        }
    }
    // Join all piecewise Bezier segments
    let curve_refs: Vec<&BezierCurve2D> = curves.iter().collect();
    crate::geometry::bezier_split_join::bezier_join(&curve_refs)
}
