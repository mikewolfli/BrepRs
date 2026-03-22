use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::geometry::nurbs_surface::NurbsSurface;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurvatureType {
    Gaussian,
    Mean,
    Maximum,
    Minimum,
}

#[derive(Debug, Clone)]
pub struct CurvatureInfo {
    pub gaussian: StandardReal,
    pub mean: StandardReal,
    pub maximum: StandardReal,
    pub minimum: StandardReal,
    pub principal_direction1: Vector,
    pub principal_direction2: Vector,
}

impl CurvatureInfo {
    pub fn new() -> Self {
        Self {
            gaussian: 0.0,
            mean: 0.0,
            maximum: 0.0,
            minimum: 0.0,
            principal_direction1: Vector::new(1.0, 0.0, 0.0),
            principal_direction2: Vector::new(0.0, 1.0, 0.0),
        }
    }

    pub fn from_principal_curvatures(k1: StandardReal, k2: StandardReal, d1: Vector, d2: Vector) -> Self {
        Self {
            gaussian: k1 * k2,
            mean: (k1 + k2) / 2.0,
            maximum: k1.max(k2),
            minimum: k1.min(k2),
            principal_direction1: d1,
            principal_direction2: d2,
        }
    }
}

impl Default for CurvatureInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SurfaceCurvatureAnalyzer {
    surface: NurbsSurface,
}

impl SurfaceCurvatureAnalyzer {
    pub fn new(surface: NurbsSurface) -> Self {
        Self { surface }
    }

    pub fn compute_curvature_at(&self, u: StandardReal, v: StandardReal) -> CurvatureInfo {
        let (du, dv) = self.compute_first_derivatives(u, v);
        let (duu, duv, dvv) = self.compute_second_derivatives(u, v);

        let e = du.dot(&du);
        let f = du.dot(&dv);
        let g = dv.dot(&dv);

        let l = duu.dot(&self.compute_normal(u, v));
        let m = duv.dot(&self.compute_normal(u, v));
        let n = dvv.dot(&self.compute_normal(u, v));

        let denom = e * g - f * f;
        if denom.abs() < 1e-10 {
            return CurvatureInfo::new();
        }

        let gaussian = (l * n - m * m) / denom;
        let mean = (e * n - 2.0 * f * m + g * l) / (2.0 * denom);

        let discriminant = mean * mean - gaussian;
        let (k1, k2) = if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();
            (mean + sqrt_disc, mean - sqrt_disc)
        } else {
            (mean, mean)
        };

        CurvatureInfo::from_principal_curvatures(
            k1,
            k2,
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    pub fn compute_curvature_map(&self, resolution: usize) -> Vec<Vec<CurvatureInfo>> {
        let mut map = Vec::with_capacity(resolution);

        for i in 0..resolution {
            let mut row = Vec::with_capacity(resolution);
            let u = i as StandardReal / (resolution - 1).max(1) as StandardReal;

            for j in 0..resolution {
                let v = j as StandardReal / (resolution - 1).max(1) as StandardReal;
                row.push(self.compute_curvature_at(u, v));
            }

            map.push(row);
        }

        map
    }

    pub fn compute_gaussian_curvature(&self, u: StandardReal, v: StandardReal) -> StandardReal {
        self.compute_curvature_at(u, v).gaussian
    }

    pub fn compute_mean_curvature(&self, u: StandardReal, v: StandardReal) -> StandardReal {
        self.compute_curvature_at(u, v).mean
    }

    fn compute_first_derivatives(&self, u: StandardReal, v: StandardReal) -> (Vector, Vector) {
        let h = 1e-5;

        let p_um = self.evaluate_at(u - h, v);
        let p_up = self.evaluate_at(u + h, v);
        let p_vm = self.evaluate_at(u, v - h);
        let p_vp = self.evaluate_at(u, v + h);

        let du = Vector::new(
            (p_up.x - p_um.x) / (2.0 * h),
            (p_up.y - p_um.y) / (2.0 * h),
            (p_up.z - p_um.z) / (2.0 * h),
        );

        let dv = Vector::new(
            (p_vp.x - p_vm.x) / (2.0 * h),
            (p_vp.y - p_vm.y) / (2.0 * h),
            (p_vp.z - p_vm.z) / (2.0 * h),
        );

        (du, dv)
    }

    fn compute_second_derivatives(&self, u: StandardReal, v: StandardReal) -> (Vector, Vector, Vector) {
        let h = 1e-5;

        let p_uu_m = self.evaluate_at(u - h, v);
        let p_uu_0 = self.evaluate_at(u, v);
        let p_uu_p = self.evaluate_at(u + h, v);

        let duu = Vector::new(
            (p_uu_p.x - 2.0 * p_uu_0.x + p_uu_m.x) / (h * h),
            (p_uu_p.y - 2.0 * p_uu_0.y + p_uu_m.y) / (h * h),
            (p_uu_p.z - 2.0 * p_uu_0.z + p_uu_m.z) / (h * h),
        );

        let p_vv_m = self.evaluate_at(u, v - h);
        let p_vv_p = self.evaluate_at(u, v + h);

        let dvv = Vector::new(
            (p_vv_p.x - 2.0 * p_uu_0.x + p_vv_m.x) / (h * h),
            (p_vv_p.y - 2.0 * p_uu_0.y + p_vv_m.y) / (h * h),
            (p_vv_p.z - 2.0 * p_uu_0.z + p_vv_m.z) / (h * h),
        );

        let p_um_vm = self.evaluate_at(u - h, v - h);
        let p_um_vp = self.evaluate_at(u - h, v + h);
        let p_up_vm = self.evaluate_at(u + h, v - h);
        let p_up_vp = self.evaluate_at(u + h, v + h);

        let duv = Vector::new(
            (p_up_vp.x - p_up_vm.x - p_um_vp.x + p_um_vm.x) / (4.0 * h * h),
            (p_up_vp.y - p_up_vm.y - p_um_vp.y + p_um_vm.y) / (4.0 * h * h),
            (p_up_vp.z - p_up_vm.z - p_um_vp.z + p_um_vm.z) / (4.0 * h * h),
        );

        (duu, duv, dvv)
    }

    fn compute_normal(&self, u: StandardReal, v: StandardReal) -> Vector {
        let (du, dv) = self.compute_first_derivatives(u, v);
        du.cross(&dv).normalized()
    }

    fn evaluate_at(&self, u: StandardReal, v: StandardReal) -> Point {
        let poles = self.surface.poles();
        let weights = self.surface.weights();

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;

        for i in 0..poles.len() {
            for j in 0..poles[i].len() {
                let basis_u = self.basis_function(self.surface.u_degree(), self.surface.u_knots(), i, u);
                let basis_v = self.basis_function(self.surface.v_degree(), self.surface.v_knots(), j, v);
                let basis = basis_u * basis_v * weights[i][j];

                x += poles[i][j].x * basis;
                y += poles[i][j].y * basis;
                z += poles[i][j].z * basis;
                w += basis;
            }
        }

        if w > 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::origin()
        }
    }

    fn basis_function(&self, degree: i32, knots: &[StandardReal], i: usize, u: StandardReal) -> StandardReal {
        if degree == 0 {
            if i < knots.len() - 1 && u >= knots[i] && u < knots[i + 1] {
                return 1.0;
            }
            return 0.0;
        }

        let denom1 = if (i + degree as usize) < knots.len() {
            knots[i + degree as usize] - knots[i]
        } else {
            0.0
        };

        let term1 = if denom1.abs() > 1e-10 {
            (u - knots[i]) / denom1 * self.basis_function(degree - 1, knots, i, u)
        } else {
            0.0
        };

        let denom2 = if (i + degree as usize + 1) < knots.len() {
            knots[i + degree as usize + 1] - knots[i + 1]
        } else {
            0.0
        };

        let term2 = if denom2.abs() > 1e-10 {
            (knots[i + degree as usize + 1] - u) / denom2 * self.basis_function(degree - 1, knots, i + 1, u)
        } else {
            0.0
        };

        term1 + term2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceQualityMetric {
    Area,
    Smoothness,
    Fairness,
    IsophoteDeviation,
    ReflectionLine,
}

#[derive(Debug, Clone)]
pub struct SurfaceQualityResult {
    pub metric: SurfaceQualityMetric,
    pub value: StandardReal,
    pub min_value: StandardReal,
    pub max_value: StandardReal,
    pub distribution: Vec<StandardReal>,
}

pub struct SurfaceQualityEvaluator {
    surface: NurbsSurface,
}

impl SurfaceQualityEvaluator {
    pub fn new(surface: NurbsSurface) -> Self {
        Self { surface }
    }

    pub fn evaluate(&self, metric: SurfaceQualityMetric, resolution: usize) -> SurfaceQualityResult {
        match metric {
            SurfaceQualityMetric::Area => self.evaluate_area(),
            SurfaceQualityMetric::Smoothness => self.evaluate_smoothness(resolution),
            SurfaceQualityMetric::Fairness => self.evaluate_fairness(resolution),
            SurfaceQualityMetric::IsophoteDeviation => self.evaluate_isophote_deviation(resolution),
            SurfaceQualityMetric::ReflectionLine => self.evaluate_reflection_lines(resolution),
        }
    }

    fn evaluate_area(&self) -> SurfaceQualityResult {
        let analyzer = SurfaceCurvatureAnalyzer::new(self.surface.clone());
        let resolution = 50;

        let mut total_area = 0.0;
        let du = 1.0 / resolution as StandardReal;
        let dv = 1.0 / resolution as StandardReal;

        for i in 0..resolution {
            for j in 0..resolution {
                let u = (i as StandardReal + 0.5) * du;
                let v = (j as StandardReal + 0.5) * dv;

                let (deriv_u, deriv_v) = analyzer.compute_first_derivatives(u, v);
                let cross = deriv_u.cross(&deriv_v);
                let area_element = cross.magnitude() * du * dv;

                total_area += area_element;
            }
        }

        SurfaceQualityResult {
            metric: SurfaceQualityMetric::Area,
            value: total_area,
            min_value: total_area,
            max_value: total_area,
            distribution: vec![total_area],
        }
    }

    fn evaluate_smoothness(&self, resolution: usize) -> SurfaceQualityResult {
        let analyzer = SurfaceCurvatureAnalyzer::new(self.surface.clone());
        let curvature_map = analyzer.compute_curvature_map(resolution);

        let mut values = Vec::new();
        let mut sum = 0.0;
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for row in &curvature_map {
            for info in row {
                let curvature_variation = (info.maximum - info.minimum).abs();
                values.push(curvature_variation);
                sum += curvature_variation;
                min_val = min_val.min(curvature_variation);
                max_val = max_val.max(curvature_variation);
            }
        }

        let avg = sum / values.len() as StandardReal;

        SurfaceQualityResult {
            metric: SurfaceQualityMetric::Smoothness,
            value: avg,
            min_value: min_val,
            max_value: max_val,
            distribution: values,
        }
    }

    fn evaluate_fairness(&self, resolution: usize) -> SurfaceQualityResult {
        let analyzer = SurfaceCurvatureAnalyzer::new(self.surface.clone());

        let mut values = Vec::new();
        let mut sum = 0.0;
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for i in 0..resolution {
            for j in 0..resolution {
                let u = i as StandardReal / (resolution - 1).max(1) as StandardReal;
                let v = j as StandardReal / (resolution - 1).max(1) as StandardReal;

                let curvature = analyzer.compute_curvature_at(u, v);
                let fairness = curvature.gaussian.powi(2) + curvature.mean.powi(2);

                values.push(fairness);
                sum += fairness;
                min_val = min_val.min(fairness);
                max_val = max_val.max(fairness);
            }
        }

        let avg = sum / values.len() as StandardReal;

        SurfaceQualityResult {
            metric: SurfaceQualityMetric::Fairness,
            value: avg,
            min_value: min_val,
            max_value: max_val,
            distribution: values,
        }
    }

    fn evaluate_isophote_deviation(&self, resolution: usize) -> SurfaceQualityResult {
        let analyzer = SurfaceCurvatureAnalyzer::new(self.surface.clone());
        let light_dir = Vector::new(0.0, 0.0, 1.0);

        let mut values = Vec::new();
        let mut sum = 0.0;
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for i in 0..resolution {
            let u = i as StandardReal / (resolution - 1).max(1) as StandardReal;
            let v_prev = if i > 0 {
                (i - 1) as StandardReal / (resolution - 1).max(1) as StandardReal
            } else {
                0.0
            };

            let normal_curr = analyzer.compute_normal(u, 0.5);
            let normal_prev = if i > 0 {
                analyzer.compute_normal(v_prev, 0.5)
            } else {
                normal_curr.clone()
            };

            let isophote_curr = normal_curr.dot(&light_dir).acos();
            let isophote_prev = normal_prev.dot(&light_dir).acos();
            let deviation = (isophote_curr - isophote_prev).abs();

            values.push(deviation);
            sum += deviation;
            min_val = min_val.min(deviation);
            max_val = max_val.max(deviation);
        }

        let avg = sum / values.len() as StandardReal;

        SurfaceQualityResult {
            metric: SurfaceQualityMetric::IsophoteDeviation,
            value: avg,
            min_value: min_val,
            max_value: max_val,
            distribution: values,
        }
    }

    fn evaluate_reflection_lines(&self, resolution: usize) -> SurfaceQualityResult {
        let analyzer = SurfaceCurvatureAnalyzer::new(self.surface.clone());
        let view_dir = Vector::new(0.0, 0.0, 1.0);

        let mut values = Vec::new();
        let mut sum = 0.0;
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for i in 0..resolution {
            for j in 0..resolution {
                let u = i as StandardReal / (resolution - 1).max(1) as StandardReal;
                let v = j as StandardReal / (resolution - 1).max(1) as StandardReal;

                let normal = analyzer.compute_normal(u, v);
                let reflection = Vector::new(
                    2.0 * normal.dot(&view_dir) * normal.x - view_dir.x,
                    2.0 * normal.dot(&view_dir) * normal.y - view_dir.y,
                    2.0 * normal.dot(&view_dir) * normal.z - view_dir.z,
                );

                let reflection_quality = reflection.magnitude();

                values.push(reflection_quality);
                sum += reflection_quality;
                min_val = min_val.min(reflection_quality);
                max_val = max_val.max(reflection_quality);
            }
        }

        let avg = sum / values.len() as StandardReal;

        SurfaceQualityResult {
            metric: SurfaceQualityMetric::ReflectionLine,
            value: avg,
            min_value: min_val,
            max_value: max_val,
            distribution: values,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuityType {
    Positional,
    Tangent,
    Curvature,
}

#[derive(Debug, Clone)]
pub struct ContinuityAnalysisResult {
    pub continuity_type: ContinuityType,
    pub is_continuous: bool,
    pub deviation: StandardReal,
    pub location: Point,
}

pub struct SurfaceContinuityAnalyzer;

impl SurfaceContinuityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_boundary_continuity(
        &self,
        surface1: &NurbsSurface,
        surface2: &NurbsSurface,
        continuity_type: ContinuityType,
        samples: usize,
    ) -> Vec<ContinuityAnalysisResult> {
        let mut results = Vec::new();

        for i in 0..samples {
            let v = i as StandardReal / (samples - 1).max(1) as StandardReal;

            let result = match continuity_type {
                ContinuityType::Positional => {
                    self.check_positional_continuity(surface1, surface2, v)
                }
                ContinuityType::Tangent => {
                    self.check_tangent_continuity(surface1, surface2, v)
                }
                ContinuityType::Curvature => {
                    self.check_curvature_continuity(surface1, surface2, v)
                }
            };

            results.push(result);
        }

        results
    }

    fn check_positional_continuity(
        &self,
        surface1: &NurbsSurface,
        surface2: &NurbsSurface,
        v: StandardReal,
    ) -> ContinuityAnalysisResult {
        let p1 = self.evaluate_boundary(surface1, v, true);
        let p2 = self.evaluate_boundary(surface2, v, false);

        let deviation = ((p1.x - p2.x).powi(2)
            + (p1.y - p2.y).powi(2)
            + (p1.z - p2.z).powi(2)).sqrt();

        ContinuityAnalysisResult {
            continuity_type: ContinuityType::Positional,
            is_continuous: deviation < 1e-6,
            deviation,
            location: Point::new(
                (p1.x + p2.x) / 2.0,
                (p1.y + p2.y) / 2.0,
                (p1.z + p2.z) / 2.0,
            ),
        }
    }

    fn check_tangent_continuity(
        &self,
        surface1: &NurbsSurface,
        surface2: &NurbsSurface,
        v: StandardReal,
    ) -> ContinuityAnalysisResult {
        let n1 = self.compute_boundary_normal(surface1, v, true);
        let n2 = self.compute_boundary_normal(surface2, v, false);

        let dot = n1.x * n2.x + n1.y * n2.y + n1.z * n2.z;
        let deviation = (1.0 - dot.abs()).max(0.0);

        let p1 = self.evaluate_boundary(surface1, v, true);

        ContinuityAnalysisResult {
            continuity_type: ContinuityType::Tangent,
            is_continuous: deviation < 0.01,
            deviation,
            location: p1,
        }
    }

    fn check_curvature_continuity(
        &self,
        surface1: &NurbsSurface,
        surface2: &NurbsSurface,
        v: StandardReal,
    ) -> ContinuityAnalysisResult {
        let analyzer1 = SurfaceCurvatureAnalyzer::new(surface1.clone());
        let analyzer2 = SurfaceCurvatureAnalyzer::new(surface2.clone());

        let c1 = analyzer1.compute_curvature_at(1.0, v);
        let c2 = analyzer2.compute_curvature_at(0.0, v);

        let deviation = ((c1.gaussian - c2.gaussian).powi(2)
            + (c1.mean - c2.mean).powi(2)).sqrt();

        let p1 = self.evaluate_boundary(surface1, v, true);

        ContinuityAnalysisResult {
            continuity_type: ContinuityType::Curvature,
            is_continuous: deviation < 0.1,
            deviation,
            location: p1,
        }
    }

    fn evaluate_boundary(&self, surface: &NurbsSurface, v: StandardReal, is_end: bool) -> Point {
        let _u = if is_end { 1.0 } else { 0.0 };
        let poles = surface.poles();
        let weights = surface.weights();

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;

        for i in 0..poles.len() {
            for j in 0..poles[i].len() {
                let basis_u = if is_end && i == poles.len() - 1 {
                    1.0
                } else if !is_end && i == 0 {
                    1.0
                } else {
                    0.0
                };

                let basis_v = self.basis_function(surface.v_degree(), surface.v_knots(), j, v);
                let basis = basis_u * basis_v * weights[i][j];

                x += poles[i][j].x * basis;
                y += poles[i][j].y * basis;
                z += poles[i][j].z * basis;
                w += basis;
            }
        }

        if w > 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::origin()
        }
    }

    fn compute_boundary_normal(&self, surface: &NurbsSurface, v: StandardReal, is_end: bool) -> Vector {
        let h = 1e-5;
        let u = if is_end { 1.0 - h } else { h };
        let u_next = if is_end { 1.0 } else { 2.0 * h };

        let p1 = self.evaluate_at(surface, u, v);
        let p2 = self.evaluate_at(surface, u_next, v);
        let p3 = self.evaluate_at(surface, u, v + h);

        let du = Vector::new(
            p2.x - p1.x,
            p2.y - p1.y,
            p2.z - p1.z,
        );

        let dv = Vector::new(
            p3.x - p1.x,
            p3.y - p1.y,
            p3.z - p1.z,
        );

        du.cross(&dv).normalized()
    }

    fn evaluate_at(&self, surface: &NurbsSurface, u: StandardReal, v: StandardReal) -> Point {
        let poles = surface.poles();
        let weights = surface.weights();

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;

        for i in 0..poles.len() {
            for j in 0..poles[i].len() {
                let basis_u = self.basis_function(surface.u_degree(), surface.u_knots(), i, u);
                let basis_v = self.basis_function(surface.v_degree(), surface.v_knots(), j, v);
                let basis = basis_u * basis_v * weights[i][j];

                x += poles[i][j].x * basis;
                y += poles[i][j].y * basis;
                z += poles[i][j].z * basis;
                w += basis;
            }
        }

        if w > 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::origin()
        }
    }

    fn basis_function(&self, degree: i32, knots: &[StandardReal], i: usize, u: StandardReal) -> StandardReal {
        if degree == 0 {
            if i < knots.len() - 1 && u >= knots[i] && u < knots[i + 1] {
                return 1.0;
            }
            return 0.0;
        }

        let denom1 = if (i + degree as usize) < knots.len() {
            knots[i + degree as usize] - knots[i]
        } else {
            0.0
        };

        let term1 = if denom1.abs() > 1e-10 {
            (u - knots[i]) / denom1 * self.basis_function(degree - 1, knots, i, u)
        } else {
            0.0
        };

        let denom2 = if (i + degree as usize + 1) < knots.len() {
            knots[i + degree as usize + 1] - knots[i + 1]
        } else {
            0.0
        };

        let term2 = if denom2.abs() > 1e-10 {
            (knots[i + degree as usize + 1] - u) / denom2 * self.basis_function(degree - 1, knots, i + 1, u)
        } else {
            0.0
        };

        term1 + term2
    }
}

impl Default for SurfaceContinuityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_surface() -> NurbsSurface {
        let poles = vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)],
            vec![Point::new(1.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let u_mults = vec![2];
        let v_mults = vec![2];

        NurbsSurface::new(1, 1, poles, weights, u_knots, v_knots, u_mults, v_mults)
    }

    #[test]
    fn test_curvature_analyzer() {
        let surface = create_test_surface();
        let analyzer = SurfaceCurvatureAnalyzer::new(surface);

        let curvature = analyzer.compute_curvature_at(0.5, 0.5);
        assert!(curvature.gaussian.is_finite());
        assert!(curvature.mean.is_finite());
    }

    #[test]
    fn test_curvature_map() {
        let surface = create_test_surface();
        let analyzer = SurfaceCurvatureAnalyzer::new(surface);

        let map = analyzer.compute_curvature_map(10);
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
    }

    #[test]
    fn test_quality_evaluator() {
        let surface = create_test_surface();
        let evaluator = SurfaceQualityEvaluator::new(surface);

        let result = evaluator.evaluate(SurfaceQualityMetric::Smoothness, 10);
        assert!(result.value.is_finite());
    }

    #[test]
    fn test_continuity_analyzer() {
        let surface1 = create_test_surface();
        let surface2 = create_test_surface();

        let analyzer = SurfaceContinuityAnalyzer::new();
        let results = analyzer.analyze_boundary_continuity(
            &surface1,
            &surface2,
            ContinuityType::Positional,
            10,
        );

        assert_eq!(results.len(), 10);
    }
}
