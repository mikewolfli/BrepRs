use crate::geometry::traits::GetCoord;

/// Compute vector between two points
fn point_to_vector<P: GetCoord>(p1: &P, p2: &P) -> (f64, f64, f64) {
    let (x1, y1, z1) = p1.coord();
    let (x2, y2, z2) = p2.coord();
    (x2 - x1, y2 - y1, z2 - z1)
}

/// Compute dot product of two vectors
fn dot_product(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

/// Curve projection and closest point search (Newton-Raphson method demonstration)
pub fn curve_project_closest<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    point: &C::Point,
    t0: f64,
    max_iter: usize,
    tol: f64,
) -> Option<(f64, C::Point)> {
    let mut t = t0;

    for _ in 0..max_iter {
        // Compute point and first derivative on curve
        let p = curve.sample(t);
        let dp = curve.derivative(t);

        // Compute vector from curve point to target point
        let vec = point_to_vector(&p, point);
        let (dx, dy, dz) = dp.coord();
        let deriv_vec = (dx, dy, dz);

        // Compute numerator and denominator for Newton iteration
        let f = dot_product(&vec, &deriv_vec);
        let df = dot_product(&deriv_vec, &deriv_vec);

        // Avoid division by zero
        if df.abs() < 1e-12 {
            break;
        }

        // Compute next parameter value
        let t_next = t - f / df;

        // Ensure parameter stays within valid range
        let t_next = t_next.clamp(0.0, 1.0);

        // Check convergence condition
        if (t_next - t).abs() < tol {
            t = t_next;
            break;
        }

        t = t_next;
    }

    // Compute final closest point
    let closest = curve.sample(t);
    Some((t, closest))
}

/// Surface projection and closest point search (bivariate Newton-Raphson method)
pub fn surface_project_closest<S: crate::geometry::advanced_traits::Surface>(
    surface: &S,
    point: &S::Point,
    u0: f64,
    v0: f64,
    max_iter: usize,
    tol: f64,
) -> Option<((f64, f64), S::Point)> {
    let mut u = u0;
    let mut v = v0;

    for _ in 0..max_iter {
        // Compute point and partial derivatives on surface
        let p = surface.sample(u, v);
        // Surface trait's derivative method returns (u-direction derivative, v-direction derivative)
        let (du, dv) = surface.derivative(u, v);

        // Compute vector from surface point to target point
        let vec = point_to_vector(&p, point);

        // Get coordinates from Vector
        let (dux, duy, duz) = du.coord();
        let (dvx, dvy, dvz) = dv.coord();
        let du_vec = (dux, duy, duz);
        let dv_vec = (dvx, dvy, dvz);

        // Compute coefficient matrix for Newton iteration
        let a = dot_product(&du_vec, &du_vec);
        let b = dot_product(&du_vec, &dv_vec);
        let c = dot_product(&dv_vec, &dv_vec);
        let d = dot_product(&vec, &du_vec);
        let e = dot_product(&vec, &dv_vec);

        // Compute determinant
        let det = a * c - b * b;

        // Avoid division by zero
        if det.abs() < 1e-12 {
            break;
        }

        // Compute parameter updates
        let delta_u = (c * d - b * e) / det;
        let delta_v = (a * e - b * d) / det;

        // Compute next parameter values
        let u_next = (u - delta_u).clamp(0.0, 1.0);
        let v_next = (v - delta_v).clamp(0.0, 1.0);

        // Check convergence condition
        if (u_next - u).abs() < tol && (v_next - v).abs() < tol {
            u = u_next;
            v = v_next;
            break;
        }

        u = u_next;
        v = v_next;
    }

    // Compute final closest point
    let closest = surface.sample(u, v);
    Some(((u, v), closest))
}
