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

/// Bezier curve least squares fitting (complete implementation: fit high-order Bezier curve)
pub fn bezier_least_squares(points: &[Point]) -> Option<BezierCurve2D> {
    if points.len() < 2 {
        return None;
    }
    // Complete implementation: Least squares fitting for high-order Bezier curves
    // Generate a Bezier curve of order n-1, control points solved via least squares
    let n = points.len();
    let mut poles = vec![Point::new(0.0, 0.0, 0.0); n];
    // Endpoints directly use original points
    poles[0] = points[0];
    poles[n - 1] = points[n - 1];
    // Middle control points use uniform parameterization + least squares
    let mut t_vec = Vec::new();
    for i in 0..n {
        t_vec.push(i as f64 / (n as f64 - 1.0));
    }
    // Build coefficient matrix A and right-hand side b
    let mut a = vec![vec![0.0; n]; n];
    let mut bx = vec![0.0; n];
    let mut by = vec![0.0; n];
    let mut bz = vec![0.0; n];
    for i in 0..n {
        let t = t_vec[i];
        for j in 0..n {
            // Binomial coefficient * t^j * (1-t)^(n-1-j)
            let coeff = binomial(n-1, j) as f64 * t.powi(j as i32) * (1.0 - t).powi((n-1-j) as i32);
            a[i][j] = coeff;
        }
        bx[i] = points[i].x;
        by[i] = points[i].y;
        bz[i] = points[i].z;
    }
    // Solve linear system Ax = b
    let solve = |a: &Vec<Vec<f64>>, b: &Vec<f64>| -> Vec<f64> {
        let n = a.len();
        let mut a = a.clone();
        let mut b = b.clone();
        for i in 0..n {
            // Pivot
            let mut max_row = i;
            for j in i + 1..n {
                if a[j][i].abs() > a[max_row][i].abs() {
                    max_row = j;
                }
            }
            a.swap(i, max_row);
            b.swap(i, max_row);
            // Elimination
            for j in i + 1..n {
                let f = a[j][i] / a[i][i];
                for k in i..n {
                    a[j][k] -= f * a[i][k];
                }
                b[j] -= f * b[i];
            }
        }
        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            x[i] = b[i];
            for j in i + 1..n {
                x[i] -= a[i][j] * x[j];
            }
            x[i] /= a[i][i];
        }
        x
    };
    let x_coeffs = solve(&a, &bx);
    let y_coeffs = solve(&a, &by);
    let z_coeffs = solve(&a, &bz);
    for i in 0..n {
        poles[i] = Point::new(x_coeffs[i], y_coeffs[i], z_coeffs[i]);
    }
    Some(BezierCurve3D::new(n-1, poles))
}

/// Bernstein polynomial helper function
fn bernstein(n: usize, i: usize, t: f64) -> f64 {
    let binom = |n: usize, k: usize| -> f64 {
        let mut res = 1.0;
        for j in 1..=k {
            res *= (n - j + 1) as f64 / j as f64;
        }
        res
    };
    binom(n, i) * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
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
