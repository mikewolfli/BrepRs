use crate::geometry::traits::GetCoord;

/// Curve adaptive sampling algorithm
/// Input curve object implementing Curve trait, output sampling point sequence
pub fn adaptive_curve_sampling<C: crate::geometry::advanced_traits::Curve>(
    curve: &C,
    t0: f64,
    t1: f64,
    tolerance: f64,
    max_depth: usize,
) -> Vec<C::Point> {
    fn recurse<C: crate::geometry::advanced_traits::Curve>(
        curve: &C,
        t0: f64,
        t1: f64,
        tolerance: f64,
        depth: usize,
        max_depth: usize,
        out: &mut Vec<C::Point>,
    ) {
        let p0 = curve.sample(t0);
        let p1 = curve.sample(t1);
        let tm = 0.5 * (t0 + t1);
        let pm = curve.sample(tm);

        // Calculate distance between midpoint and linear interpolation
        let dist = {
            let (x0, y0, z0) = p0.coord();
            let (x1, y1, z1) = p1.coord();
            let (xm, ym, zm) = pm.coord();
            let lx = 0.5 * (x0 + x1);
            let ly = 0.5 * (y0 + y1);
            let lz = 0.5 * (z0 + z1);
            ((xm - lx).powi(2) + (ym - ly).powi(2) + (zm - lz).powi(2)).sqrt()
        };

        if dist > tolerance && depth < max_depth {
            recurse(curve, t0, tm, tolerance, depth + 1, max_depth, out);
            recurse(curve, tm, t1, tolerance, depth + 1, max_depth, out);
        } else {
            out.push(p0);
            out.push(p1);
        }
    }

    let mut result = Vec::new();
    recurse(curve, t0, t1, tolerance, 0, max_depth, &mut result);
    result
}

/// Surface adaptive sampling algorithm (demonstration, can be extended to grid sampling)
pub fn adaptive_surface_sampling<S: crate::geometry::advanced_traits::Surface>(
    surface: &S,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
    tolerance: f64,
    max_depth: usize,
) -> Vec<S::Point> {
    fn recurse<S: crate::geometry::advanced_traits::Surface>(
        surface: &S,
        u0: f64,
        u1: f64,
        v0: f64,
        v1: f64,
        tolerance: f64,
        depth: usize,
        max_depth: usize,
        out: &mut Vec<S::Point>,
    ) {
        let p00 = surface.sample(u0, v0);
        let p01 = surface.sample(u0, v1);
        let p10 = surface.sample(u1, v0);
        let p11 = surface.sample(u1, v1);

        let um = 0.5 * (u0 + u1);
        let vm = 0.5 * (v0 + v1);
        let pm = surface.sample(um, vm);

        // Calculate distance between center point and bilinear interpolation
        let dist = {
            let (x00, y00, z00) = p00.coord();
            let (x01, y01, z01) = p01.coord();
            let (x10, y10, z10) = p10.coord();
            let (x11, y11, z11) = p11.coord();
            let (xm, ym, zm) = pm.coord();

            // Bilinear interpolation at center (u=0.5, v=0.5)
            // P(u,v) = (1-u)(1-v)P00 + u(1-v)P10 + (1-u)vP01 + uvP11
            // At center: P(0.5,0.5) = 0.25*P00 + 0.25*P10 + 0.25*P01 + 0.25*P11
            let lx = 0.25 * (x00 + x01 + x10 + x11);
            let ly = 0.25 * (y00 + y01 + y10 + y11);
            let lz = 0.25 * (z00 + z01 + z10 + z11);

            ((xm - lx).powi(2) + (ym - ly).powi(2) + (zm - lz).powi(2)).sqrt()
        };

        if dist > tolerance && depth < max_depth {
            // Recursively subdivide four sub-regions
            recurse(
                surface,
                u0,
                um,
                v0,
                vm,
                tolerance,
                depth + 1,
                max_depth,
                out,
            );
            recurse(
                surface,
                u0,
                um,
                vm,
                v1,
                tolerance,
                depth + 1,
                max_depth,
                out,
            );
            recurse(
                surface,
                um,
                u1,
                v0,
                vm,
                tolerance,
                depth + 1,
                max_depth,
                out,
            );
            recurse(
                surface,
                um,
                u1,
                vm,
                v1,
                tolerance,
                depth + 1,
                max_depth,
                out,
            );
        } else {
            // Add four corner points
            out.push(p00);
            out.push(p01);
            out.push(p10);
            out.push(p11);
        }
    }

    let mut result = Vec::new();
    recurse(
        surface,
        u0,
        u1,
        v0,
        v1,
        tolerance,
        0,
        max_depth,
        &mut result,
    );
    result
}
