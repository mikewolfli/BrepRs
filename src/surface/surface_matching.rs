use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::geometry::nurbs_surface::NurbsSurface;

#[derive(Debug, Clone)]
pub struct SurfaceFittingOptions {
    tolerance: StandardReal,
    max_iterations: usize,
    smoothness_weight: StandardReal,
    preserve_boundaries: bool,
}

impl Default for SurfaceFittingOptions {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 100,
            smoothness_weight: 0.1,
            preserve_boundaries: true,
        }
    }
}

impl SurfaceFittingOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tolerance(mut self, tolerance: StandardReal) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn with_smoothness_weight(mut self, weight: StandardReal) -> Self {
        self.smoothness_weight = weight;
        self
    }

    pub fn with_preserve_boundaries(mut self, preserve: bool) -> Self {
        self.preserve_boundaries = preserve;
        self
    }
}

pub struct SurfaceFitter {
    options: SurfaceFittingOptions,
}

impl SurfaceFitter {
    pub fn new() -> Self {
        Self {
            options: SurfaceFittingOptions::default(),
        }
    }

    pub fn with_options(options: SurfaceFittingOptions) -> Self {
        Self { options }
    }

    pub fn fit_to_points(&self, points: &[Point], degree_u: i32, degree_v: i32, num_poles_u: i32, num_poles_v: i32) -> Result<NurbsSurface, String> {
        if points.is_empty() {
            return Err("Cannot fit surface to empty point set".to_string());
        }

        if degree_u < 1 || degree_v < 1 {
            return Err("Degrees must be at least 1".to_string());
        }

        if num_poles_u <= degree_u || num_poles_v <= degree_v {
            return Err("Number of poles must be greater than degree".to_string());
        }

        let bounding_box = self.compute_bounding_box(points);
        let initial_surface = self.create_initial_surface(degree_u, degree_v, num_poles_u, num_poles_v, &bounding_box);

        self.iterative_fit(&initial_surface, points)
    }

    fn compute_bounding_box(&self, points: &[Point]) -> (Point, Point) {
        let mut min = Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point::new(f64::MIN, f64::MIN, f64::MIN);

        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }

        (min, max)
    }

    fn create_initial_surface(&self, degree_u: i32, degree_v: i32, num_poles_u: i32, num_poles_v: i32, bbox: &(Point, Point)) -> NurbsSurface {
        let mut poles = Vec::with_capacity(num_poles_u as usize);
        let mut weights = Vec::with_capacity(num_poles_u as usize);

        for i in 0..num_poles_u as usize {
            let mut row = Vec::with_capacity(num_poles_v as usize);
            let mut weight_row = Vec::with_capacity(num_poles_v as usize);

            let u = i as StandardReal / (num_poles_u - 1) as StandardReal;

            for j in 0..num_poles_v as usize {
                let v = j as StandardReal / (num_poles_v - 1) as StandardReal;

                let x = bbox.0.x + u * (bbox.1.x - bbox.0.x);
                let y = bbox.0.y + v * (bbox.1.y - bbox.0.y);
                let z = bbox.0.z + 0.5 * (bbox.1.z - bbox.0.z);

                row.push(Point::new(x, y, z));
                weight_row.push(1.0);
            }

            poles.push(row);
            weights.push(weight_row);
        }

        let u_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let v_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let u_multiplicities: Vec<i32> = vec![num_poles_u];
        let v_multiplicities: Vec<i32> = vec![num_poles_v];

        NurbsSurface::new(
            degree_u,
            degree_v,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        )
    }

    fn iterative_fit(&self, initial_surface: &NurbsSurface, points: &[Point]) -> Result<NurbsSurface, String> {
        let mut current_surface = initial_surface.clone();

        for _iteration in 0..self.options.max_iterations {
            let mut new_poles = current_surface.poles().to_vec();
            let weights = current_surface.weights().to_vec();

            for point in points {
                let (u, v) = self.find_closest_parameters(&current_surface, point);

                if let Some((_du, _dv)) = self.compute_parameter_derivatives(&current_surface, u, v) {
                    let surface_point = self.evaluate_surface(&current_surface, u, v);
                    let error = Vector::new(
                        point.x - surface_point.x,
                        point.y - surface_point.y,
                        point.z - surface_point.z,
                    );

                    let step = 0.1;
                    for i in 0..new_poles.len() {
                        for j in 0..new_poles[i].len() {
                            let basis_u = self.basis_function(current_surface.u_degree(), current_surface.u_knots(), i, u);
                            let basis_v = self.basis_function(current_surface.v_degree(), current_surface.v_knots(), j, v);
                            let basis = basis_u * basis_v;

                            new_poles[i][j] = Point::new(
                                new_poles[i][j].x + step * error.x * basis,
                                new_poles[i][j].y + step * error.y * basis,
                                new_poles[i][j].z + step * error.z * basis,
                            );
                        }
                    }
                }
            }

            current_surface = NurbsSurface::new(
                current_surface.u_degree(),
                current_surface.v_degree(),
                new_poles,
                weights,
                current_surface.u_knots().to_vec(),
                current_surface.v_knots().to_vec(),
                current_surface.u_multiplicities().to_vec(),
                current_surface.v_multiplicities().to_vec(),
            );
        }

        Ok(current_surface)
    }

    fn find_closest_parameters(&self, surface: &NurbsSurface, point: &Point) -> (StandardReal, StandardReal) {
        let mut best_u = 0.5;
        let mut best_v = 0.5;
        let mut best_dist = f64::MAX;

        for i in 0..10 {
            for j in 0..10 {
                let u = i as StandardReal / 9.0;
                let v = j as StandardReal / 9.0;

                let surface_point = self.evaluate_surface(surface, u, v);
                let dist = (surface_point.x - point.x).powi(2)
                    + (surface_point.y - point.y).powi(2)
                    + (surface_point.z - point.z).powi(2);

                if dist < best_dist {
                    best_dist = dist;
                    best_u = u;
                    best_v = v;
                }
            }
        }

        (best_u, best_v)
    }

    fn evaluate_surface(&self, surface: &NurbsSurface, u: StandardReal, v: StandardReal) -> Point {
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

    fn compute_parameter_derivatives(&self, _surface: &NurbsSurface, _u: StandardReal, _v: StandardReal) -> Option<(Vector, Vector)> {
        Some((Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)))
    }
}

impl Default for SurfaceFitter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendType {
    Linear,
    Smooth,
    Parabolic,
    Cubic,
}

pub struct SurfaceBlender {
    blend_type: BlendType,
    blend_width: StandardReal,
}

impl SurfaceBlender {
    pub fn new() -> Self {
        Self {
            blend_type: BlendType::Smooth,
            blend_width: 0.1,
        }
    }

    pub fn with_blend_type(mut self, blend_type: BlendType) -> Self {
        self.blend_type = blend_type;
        self
    }

    pub fn with_blend_width(mut self, width: StandardReal) -> Self {
        self.blend_width = width;
        self
    }

    pub fn blend_surfaces(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.len() != poles2.len() || poles1[0].len() != poles2[0].len() {
            return Err("Surfaces must have the same number of control points for blending".to_string());
        }

        let mut blended_poles = poles1.to_vec();
        let blended_weights = surface1.weights().to_vec();

        for i in 0..blended_poles.len() {
            for j in 0..blended_poles[i].len() {
                let t = self.compute_blend_parameter(i, j, blended_poles.len(), blended_poles[i].len());
                let blend_factor = self.blend_function(t);

                blended_poles[i][j] = Point::new(
                    poles1[i][j].x * (1.0 - blend_factor) + poles2[i][j].x * blend_factor,
                    poles1[i][j].y * (1.0 - blend_factor) + poles2[i][j].y * blend_factor,
                    poles1[i][j].z * (1.0 - blend_factor) + poles2[i][j].z * blend_factor,
                );
            }
        }

        Ok(NurbsSurface::new(
            surface1.u_degree(),
            surface1.v_degree(),
            blended_poles,
            blended_weights,
            surface1.u_knots().to_vec(),
            surface1.v_knots().to_vec(),
            surface1.u_multiplicities().to_vec(),
            surface1.v_multiplicities().to_vec(),
        ))
    }

    fn compute_blend_parameter(&self, i: usize, j: usize, u_count: usize, v_count: usize) -> StandardReal {
        let u = i as StandardReal / (u_count - 1).max(1) as StandardReal;
        let v = j as StandardReal / (v_count - 1).max(1) as StandardReal;
        (u + v) / 2.0
    }

    fn blend_function(&self, t: StandardReal) -> StandardReal {
        match self.blend_type {
            BlendType::Linear => t,
            BlendType::Smooth => t * t * (3.0 - 2.0 * t),
            BlendType::Parabolic => t * t,
            BlendType::Cubic => t * t * t,
        }
    }
}

impl Default for SurfaceBlender {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SurfaceBridge;

impl SurfaceBridge {
    pub fn new() -> Self {
        Self
    }

    pub fn bridge_surfaces(
        &self,
        surface1: &NurbsSurface,
        surface2: &NurbsSurface,
        bridge_segments: usize,
    ) -> Result<NurbsSurface, String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.is_empty() || poles2.is_empty() {
            return Err("Cannot bridge empty surfaces".to_string());
        }

        let v_count1 = poles1[0].len();
        let v_count2 = poles2[0].len();

        if v_count1 != v_count2 {
            return Err("Surfaces must have the same V dimension for bridging".to_string());
        }

        let u_count = poles1.len() + bridge_segments + poles2.len();
        let mut bridge_poles = Vec::with_capacity(u_count);
        let mut bridge_weights = Vec::with_capacity(u_count);

        for row in poles1 {
            bridge_poles.push(row.clone());
        }

        for seg in 0..bridge_segments {
            let t = (seg + 1) as StandardReal / (bridge_segments + 1) as StandardReal;
            let mut row = Vec::with_capacity(v_count1);

            for j in 0..v_count1 {
                let last_pole1 = &poles1[poles1.len() - 1][j];
                let first_pole2 = &poles2[0][j];

                let interpolated = Point::new(
                    last_pole1.x * (1.0 - t) + first_pole2.x * t,
                    last_pole1.y * (1.0 - t) + first_pole2.y * t,
                    last_pole1.z * (1.0 - t) + first_pole2.z * t,
                );

                row.push(interpolated);
            }

            bridge_poles.push(row);
            bridge_weights.push(vec![1.0; v_count1]);
        }

        for row in poles2 {
            bridge_poles.push(row.clone());
        }

        for row in surface1.weights() {
            bridge_weights.push(row.clone());
        }

        let u_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let v_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let u_multiplicities: Vec<i32> = vec![u_count as i32];
        let v_multiplicities: Vec<i32> = vec![v_count1 as i32];

        Ok(NurbsSurface::new(
            surface1.u_degree().max(surface2.u_degree()),
            surface1.v_degree(),
            bridge_poles,
            bridge_weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        ))
    }
}

impl Default for SurfaceBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Sharp,
    Smooth,
    Fillet,
    Chamfer,
}

pub struct SurfaceTransition {
    transition_type: TransitionType,
    transition_width: StandardReal,
}

impl SurfaceTransition {
    pub fn new() -> Self {
        Self {
            transition_type: TransitionType::Smooth,
            transition_width: 0.1,
        }
    }

    pub fn with_transition_type(mut self, transition_type: TransitionType) -> Self {
        self.transition_type = transition_type;
        self
    }

    pub fn with_transition_width(mut self, width: StandardReal) -> Self {
        self.transition_width = width;
        self
    }

    pub fn create_transition(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        match self.transition_type {
            TransitionType::Sharp => self.create_sharp_transition(surface1, surface2),
            TransitionType::Smooth => self.create_smooth_transition(surface1, surface2),
            TransitionType::Fillet => self.create_fillet_transition(surface1, surface2),
            TransitionType::Chamfer => self.create_chamfer_transition(surface1, surface2),
        }
    }

    fn create_sharp_transition(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        let bridge = SurfaceBridge::new();
        bridge.bridge_surfaces(surface1, surface2, 0)
    }

    fn create_smooth_transition(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        let bridge = SurfaceBridge::new();
        bridge.bridge_surfaces(surface1, surface2, 3)
    }

    fn create_fillet_transition(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.is_empty() || poles2.is_empty() {
            return Err("Cannot create transition for empty surfaces".to_string());
        }

        let v_count = poles1[0].len();
        let mut fillet_poles = Vec::new();

        for row in poles1 {
            fillet_poles.push(row.clone());
        }

        let fillet_segments = 4;
        for seg in 0..fillet_segments {
            let t = (seg + 1) as StandardReal / (fillet_segments + 1) as StandardReal;
            let angle = std::f64::consts::PI * t / 2.0;
            let mut row = Vec::with_capacity(v_count);

            for j in 0..v_count {
                let last_pole1 = &poles1[poles1.len() - 1][j];
                let first_pole2 = &poles2[0][j];

                let center_x = (last_pole1.x + first_pole2.x) / 2.0;
                let center_y = (last_pole1.y + first_pole2.y) / 2.0;
                let center_z = (last_pole1.z + first_pole2.z) / 2.0;

                let radius = self.transition_width;
                let x = center_x + radius * angle.cos();
                let y = center_y;
                let z = center_z + radius * angle.sin();

                row.push(Point::new(x, y, z));
            }

            fillet_poles.push(row);
        }

        for row in poles2 {
            fillet_poles.push(row.clone());
        }

        let u_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let v_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let u_multiplicities: Vec<i32> = vec![fillet_poles.len() as i32];
        let v_multiplicities: Vec<i32> = vec![v_count as i32];

        let mut fillet_weights = Vec::new();
        for _ in 0..fillet_poles.len() {
            fillet_weights.push(vec![1.0; v_count]);
        }

        Ok(NurbsSurface::new(
            surface1.u_degree(),
            surface1.v_degree(),
            fillet_poles,
            fillet_weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        ))
    }

    fn create_chamfer_transition(&self, surface1: &NurbsSurface, surface2: &NurbsSurface) -> Result<NurbsSurface, String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.is_empty() || poles2.is_empty() {
            return Err("Cannot create transition for empty surfaces".to_string());
        }

        let v_count = poles1[0].len();
        let mut chamfer_poles = Vec::new();

        for row in poles1 {
            chamfer_poles.push(row.clone());
        }

        let mut chamfer_row = Vec::with_capacity(v_count);
        for j in 0..v_count {
            let last_pole1 = &poles1[poles1.len() - 1][j];
            let first_pole2 = &poles2[0][j];

            chamfer_row.push(Point::new(
                (last_pole1.x + first_pole2.x) / 2.0,
                (last_pole1.y + first_pole2.y) / 2.0,
                (last_pole1.z + first_pole2.z) / 2.0,
            ));
        }
        chamfer_poles.push(chamfer_row);

        for row in poles2 {
            chamfer_poles.push(row.clone());
        }

        let u_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let v_knots: Vec<StandardReal> = vec![0.0, 1.0];
        let u_multiplicities: Vec<i32> = vec![chamfer_poles.len() as i32];
        let v_multiplicities: Vec<i32> = vec![v_count as i32];

        let mut chamfer_weights = Vec::new();
        for _ in 0..chamfer_poles.len() {
            chamfer_weights.push(vec![1.0; v_count]);
        }

        Ok(NurbsSurface::new(
            surface1.u_degree(),
            surface1.v_degree(),
            chamfer_poles,
            chamfer_weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        ))
    }
}

impl Default for SurfaceTransition {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SeamlessConnection;

impl SeamlessConnection {
    pub fn new() -> Self {
        Self
    }

    pub fn connect_surfaces(&self, surface1: &mut NurbsSurface, surface2: &mut NurbsSurface, tolerance: StandardReal) -> Result<(), String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.is_empty() || poles2.is_empty() {
            return Err("Cannot connect empty surfaces".to_string());
        }

        let v_count1 = poles1[0].len();
        let v_count2 = poles2[0].len();

        if v_count1 != v_count2 {
            return Err("Surfaces must have the same V dimension for seamless connection".to_string());
        }

        let mut new_poles1 = poles1.to_vec();
        let mut new_poles2 = poles2.to_vec();

        for j in 0..v_count1 {
            let last_pole1 = poles1[poles1.len() - 1][j];
            let first_pole2 = poles2[0][j];

            let distance = ((last_pole1.x - first_pole2.x).powi(2)
                + (last_pole1.y - first_pole2.y).powi(2)
                + (last_pole1.z - first_pole2.z).powi(2)).sqrt();

            if distance < tolerance {
                let mid_point = Point::new(
                    (last_pole1.x + first_pole2.x) / 2.0,
                    (last_pole1.y + first_pole2.y) / 2.0,
                    (last_pole1.z + first_pole2.z) / 2.0,
                );

                new_poles1[poles1.len() - 1][j] = mid_point;
                new_poles2[0][j] = mid_point;
            }
        }

        *surface1 = NurbsSurface::new(
            surface1.u_degree(),
            surface1.v_degree(),
            new_poles1,
            surface1.weights().to_vec(),
            surface1.u_knots().to_vec(),
            surface1.v_knots().to_vec(),
            surface1.u_multiplicities().to_vec(),
            surface1.v_multiplicities().to_vec(),
        );

        *surface2 = NurbsSurface::new(
            surface2.u_degree(),
            surface2.v_degree(),
            new_poles2,
            surface2.weights().to_vec(),
            surface2.u_knots().to_vec(),
            surface2.v_knots().to_vec(),
            surface2.u_multiplicities().to_vec(),
            surface2.v_multiplicities().to_vec(),
        );

        Ok(())
    }

    pub fn average_boundary_tangents(&self, surface1: &mut NurbsSurface, surface2: &mut NurbsSurface) -> Result<(), String> {
        let poles1 = surface1.poles();
        let poles2 = surface2.poles();

        if poles1.len() < 2 || poles2.len() < 2 {
            return Err("Surfaces must have at least 2 rows of control points".to_string());
        }

        let v_count = poles1[0].len();
        let mut new_poles1 = poles1.to_vec();
        let mut new_poles2 = poles2.to_vec();

        for j in 0..v_count {
            let tangent1 = Vector::new(
                poles1[poles1.len() - 1][j].x - poles1[poles1.len() - 2][j].x,
                poles1[poles1.len() - 1][j].y - poles1[poles1.len() - 2][j].y,
                poles1[poles1.len() - 1][j].z - poles1[poles1.len() - 2][j].z,
            );

            let tangent2 = Vector::new(
                poles2[1][j].x - poles2[0][j].x,
                poles2[1][j].y - poles2[0][j].y,
                poles2[1][j].z - poles2[0][j].z,
            );

            let avg_tangent = Vector::new(
                (tangent1.x + tangent2.x) / 2.0,
                (tangent1.y + tangent2.y) / 2.0,
                (tangent1.z + tangent2.z) / 2.0,
            );

            new_poles1[poles1.len() - 2][j] = Point::new(
                poles1[poles1.len() - 1][j].x - avg_tangent.x,
                poles1[poles1.len() - 1][j].y - avg_tangent.y,
                poles1[poles1.len() - 1][j].z - avg_tangent.z,
            );

            new_poles2[1][j] = Point::new(
                poles2[0][j].x + avg_tangent.x,
                poles2[0][j].y + avg_tangent.y,
                poles2[0][j].z + avg_tangent.z,
            );
        }

        *surface1 = NurbsSurface::new(
            surface1.u_degree(),
            surface1.v_degree(),
            new_poles1,
            surface1.weights().to_vec(),
            surface1.u_knots().to_vec(),
            surface1.v_knots().to_vec(),
            surface1.u_multiplicities().to_vec(),
            surface1.v_multiplicities().to_vec(),
        );

        *surface2 = NurbsSurface::new(
            surface2.u_degree(),
            surface2.v_degree(),
            new_poles2,
            surface2.weights().to_vec(),
            surface2.u_knots().to_vec(),
            surface2.v_knots().to_vec(),
            surface2.u_multiplicities().to_vec(),
            surface2.v_multiplicities().to_vec(),
        );

        Ok(())
    }
}

impl Default for SeamlessConnection {
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
    fn test_surface_fitter() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];

        let fitter = SurfaceFitter::new();
        let result = fitter.fit_to_points(&points, 1, 1, 2, 2);

        assert!(result.is_ok());
    }

    #[test]
    fn test_surface_blender() {
        let surface1 = create_test_surface();
        let surface2 = create_test_surface();

        let blender = SurfaceBlender::new()
            .with_blend_type(BlendType::Smooth);

        let result = blender.blend_surfaces(&surface1, &surface2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_surface_bridge() {
        let surface1 = create_test_surface();
        let surface2 = create_test_surface();

        let bridge = SurfaceBridge::new();
        let result = bridge.bridge_surfaces(&surface1, &surface2, 3);

        assert!(result.is_ok());
        let bridged = result.unwrap();
        assert_eq!(bridged.nb_u_poles(), 7);
    }

    #[test]
    fn test_surface_transition() {
        let surface1 = create_test_surface();
        let surface2 = create_test_surface();

        let transition = SurfaceTransition::new()
            .with_transition_type(TransitionType::Smooth);

        let result = transition.create_transition(&surface1, &surface2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_seamless_connection() {
        let mut surface1 = create_test_surface();
        let mut surface2 = create_test_surface();

        let connection = SeamlessConnection::new();
        let result = connection.connect_surfaces(&mut surface1, &mut surface2, 0.001);

        assert!(result.is_ok());
    }
}
