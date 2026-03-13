use crate::geometry::traits::GetCoord;

/// Compute vector length
fn vector_length(v: &(f64, f64, f64)) -> f64 {
    (v.0.powi(2) + v.1.powi(2) + v.2.powi(2)).sqrt()
}

/// Compute cross product of two vectors
fn cross_product(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> (f64, f64, f64) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

/// Bezier curve higher-order derivatives and curvature analysis
/// Assumes Curve trait is implemented, Point supports coord()
pub fn bezier_curvature<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    t: f64,
) -> Option<f64> {
    // Compute first derivative (tangent vector)
    let first_deriv = curve.derivative(t);
    let (dx, dy, dz) = first_deriv.coord();
    let first_vec = (dx, dy, dz);

    // Compute second derivative
    // Note: Here we assume Curve trait has second derivative method, if not, use numerical differentiation
    // For compatibility, we use numerical differentiation to compute second derivative
    let h = 1e-6;
    let t_plus_h = (t + h).min(1.0);
    let t_minus_h = (t - h).max(0.0);

    let deriv_plus = curve.derivative(t_plus_h);
    let (dx_plus, dy_plus, dz_plus) = deriv_plus.coord();

    let deriv_minus = curve.derivative(t_minus_h);
    let (dx_minus, dy_minus, dz_minus) = deriv_minus.coord();

    // Numerical differentiation to compute second derivative
    let second_vec = (
        (dx_plus - dx_minus) / (2.0 * h),
        (dy_plus - dy_minus) / (2.0 * h),
        (dz_plus - dz_minus) / (2.0 * h),
    );

    // Compute cross product
    let cross = cross_product(&first_vec, &second_vec);
    let cross_length = vector_length(&cross);

    // Compute cube of first derivative length
    let first_length = vector_length(&first_vec);
    let first_length_cubed = first_length.powi(3);

    // Avoid division by zero
    if first_length_cubed < 1e-12 {
        return None;
    }

    // Compute curvature
    Some(cross_length / first_length_cubed)
}

/// NURBS curve higher-order derivatives and curvature analysis (demonstration, actual implementation needs to consider weights)
pub fn nurbs_curvature<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    t: f64,
) -> Option<f64> {
    // NURBS curve curvature calculation is similar to Bezier, but needs to consider weight effects
    // Here we use the same numerical method as Bezier, actual implementation needs to consider weights

    // Compute first derivative (tangent vector)
    let first_deriv = curve.derivative(t);
    let (dx, dy, dz) = first_deriv.coord();
    let first_vec = (dx, dy, dz);

    // Compute second derivative (numerical differentiation)
    let h = 1e-6;
    let t_plus_h = (t + h).min(1.0);
    let t_minus_h = (t - h).max(0.0);

    let deriv_plus = curve.derivative(t_plus_h);
    let (dx_plus, dy_plus, dz_plus) = deriv_plus.coord();

    let deriv_minus = curve.derivative(t_minus_h);
    let (dx_minus, dy_minus, dz_minus) = deriv_minus.coord();

    // Numerical differentiation to compute second derivative
    let second_vec = (
        (dx_plus - dx_minus) / (2.0 * h),
        (dy_plus - dy_minus) / (2.0 * h),
        (dz_plus - dz_minus) / (2.0 * h),
    );

    // Compute cross product
    let cross = cross_product(&first_vec, &second_vec);
    let cross_length = vector_length(&cross);

    // Compute cube of first derivative length
    let first_length = vector_length(&first_vec);
    let first_length_cubed = first_length.powi(3);

    // Avoid division by zero
    if first_length_cubed < 1e-12 {
        return None;
    }

    // Compute curvature
    Some(cross_length / first_length_cubed)
}
