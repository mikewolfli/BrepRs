use crate::geometry::bezier_curve2d::BezierCurve2D;
use crate::geometry::Point;

/// Bezier curve splitting (de Casteljau algorithm)
pub fn bezier_split(curve: &BezierCurve2D, t: f64) -> (BezierCurve2D, BezierCurve2D) {
    let n = curve.nb_poles() as usize;
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);
    let mut temp = curve.poles().to_vec();
    left.push(temp[0]);
    right.push(temp[n - 1]);
    for k in 1..n {
        let mut next = Vec::with_capacity(n - k);
        for i in 0..(n - k) {
            let p = Point::new(
                (1.0 - t) * temp[i].x() + t * temp[i + 1].x(),
                (1.0 - t) * temp[i].y() + t * temp[i + 1].y(),
                (1.0 - t) * temp[i].z() + t * temp[i + 1].z(),
            );
            next.push(p);
        }
        left.push(next[0]);
        right.push(next[next.len() - 1]);
        temp = next;
    }
    let right_rev: Vec<Point> = right.into_iter().rev().collect();
    (BezierCurve2D::new(left), BezierCurve2D::new(right_rev))
}

/// Bezier curve joining (endpoint alignment)
pub fn bezier_join(curves: &[&BezierCurve2D]) -> Option<BezierCurve2D> {
    if curves.is_empty() {
        return None;
    }
    let mut poles = Vec::new();
    for (i, curve) in curves.iter().enumerate() {
        let pts = curve.poles();
        if i == 0 {
            poles.extend_from_slice(pts);
        } else {
            // Remove duplicate first endpoint
            poles.extend_from_slice(&pts[1..]);
        }
    }
    Some(BezierCurve2D::new(poles))
}
