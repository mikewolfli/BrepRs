use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::geometry::nurbs_surface::NurbsSurface;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuityLevel {
    G0,
    G1,
    G2,
    G3,
}

#[derive(Debug, Clone)]
pub struct ControlPointHandle {
    u_index: usize,
    v_index: usize,
}

impl ControlPointHandle {
    pub fn new(u_index: usize, v_index: usize) -> Self {
        Self { u_index, v_index }
    }

    pub fn u_index(&self) -> usize {
        self.u_index
    }

    pub fn v_index(&self) -> usize {
        self.v_index
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceEditResult {
    pub modified_surface: NurbsSurface,
    pub edit_history: Vec<ControlPointEdit>,
}

#[derive(Debug, Clone)]
pub struct ControlPointEdit {
    pub handle: ControlPointHandle,
    pub old_position: Point,
    pub new_position: Point,
    pub old_weight: StandardReal,
    pub new_weight: StandardReal,
}

pub struct FreeFormSurfaceEditor {
    surface: NurbsSurface,
    edit_history: Vec<ControlPointEdit>,
    smoothing_factor: StandardReal,
}

impl FreeFormSurfaceEditor {
    pub fn new(surface: NurbsSurface) -> Self {
        Self {
            surface,
            edit_history: Vec::new(),
            smoothing_factor: 0.5,
        }
    }

    pub fn surface(&self) -> &NurbsSurface {
        &self.surface
    }

    pub fn set_smoothing_factor(&mut self, factor: StandardReal) {
        self.smoothing_factor = factor.clamp(0.0, 1.0);
    }

    pub fn move_control_point(&mut self, handle: &ControlPointHandle, delta: Vector) -> Result<Point, String> {
        let poles = self.surface.poles();
        if handle.u_index >= poles.len() {
            return Err(format!("U index {} out of bounds", handle.u_index));
        }
        if handle.v_index >= poles[handle.u_index].len() {
            return Err(format!("V index {} out of bounds", handle.v_index));
        }

        let old_position = poles[handle.u_index][handle.v_index];
        let new_position = Point::new(
            old_position.x + delta.x,
            old_position.y + delta.y,
            old_position.z + delta.z,
        );

        let old_weight = self.surface.weights()[handle.u_index][handle.v_index];

        let edit = ControlPointEdit {
            handle: handle.clone(),
            old_position,
            new_position,
            old_weight,
            new_weight: old_weight,
        };
        self.edit_history.push(edit);

        self.set_control_point(handle, new_position)?;

        Ok(new_position)
    }

    pub fn set_control_point(&mut self, handle: &ControlPointHandle, position: Point) -> Result<(), String> {
        let poles = self.surface.poles();
        if handle.u_index >= poles.len() {
            return Err(format!("U index {} out of bounds", handle.u_index));
        }
        if handle.v_index >= poles[handle.u_index].len() {
            return Err(format!("V index {} out of bounds", handle.v_index));
        }

        let mut new_poles = poles.to_vec();
        new_poles[handle.u_index][handle.v_index] = position;
        self.surface = self.create_modified_surface(new_poles, self.surface.weights().to_vec());

        Ok(())
    }

    pub fn set_control_weight(&mut self, handle: &ControlPointHandle, weight: StandardReal) -> Result<(), String> {
        let weights = self.surface.weights();
        if handle.u_index >= weights.len() {
            return Err(format!("U index {} out of bounds", handle.u_index));
        }
        if handle.v_index >= weights[handle.u_index].len() {
            return Err(format!("V index {} out of bounds", handle.v_index));
        }

        let old_position = self.surface.poles()[handle.u_index][handle.v_index];
        let old_weight = weights[handle.u_index][handle.v_index];

        let edit = ControlPointEdit {
            handle: handle.clone(),
            old_position,
            new_position: old_position,
            old_weight,
            new_weight: weight,
        };
        self.edit_history.push(edit);

        let mut new_weights = weights.to_vec();
        new_weights[handle.u_index][handle.v_index] = weight;
        self.surface = self.create_modified_surface(self.surface.poles().to_vec(), new_weights);

        Ok(())
    }

    pub fn move_control_points(&mut self, handles: &[ControlPointHandle], deltas: &[Vector]) -> Result<Vec<Point>, String> {
        if handles.len() != deltas.len() {
            return Err("Handles and deltas must have the same length".to_string());
        }

        let mut new_positions = Vec::with_capacity(handles.len());
        let mut new_poles = self.surface.poles().to_vec();
        let new_weights = self.surface.weights().to_vec();

        for (handle, delta) in handles.iter().zip(deltas.iter()) {
            if handle.u_index >= new_poles.len() {
                return Err(format!("U index {} out of bounds", handle.u_index));
            }
            if handle.v_index >= new_poles[handle.u_index].len() {
                return Err(format!("V index {} out of bounds", handle.v_index));
            }

            let old_position = new_poles[handle.u_index][handle.v_index];
            let new_position = Point::new(
                old_position.x + delta.x,
                old_position.y + delta.y,
                old_position.z + delta.z,
            );

            let old_weight = new_weights[handle.u_index][handle.v_index];

            let edit = ControlPointEdit {
                handle: handle.clone(),
                old_position,
                new_position,
                old_weight,
                new_weight: old_weight,
            };
            self.edit_history.push(edit);

            new_poles[handle.u_index][handle.v_index] = new_position;
            new_positions.push(new_position);
        }

        self.surface = self.create_modified_surface(new_poles, new_weights);
        Ok(new_positions)
    }

    fn create_modified_surface(&self, poles: Vec<Vec<Point>>, weights: Vec<Vec<StandardReal>>) -> NurbsSurface {
        NurbsSurface::new(
            self.surface.u_degree(),
            self.surface.v_degree(),
            poles,
            weights,
            self.surface.u_knots().to_vec(),
            self.surface.v_knots().to_vec(),
            self.surface.u_multiplicities().to_vec(),
            self.surface.v_multiplicities().to_vec(),
        )
    }

    pub fn undo_last_edit(&mut self) -> Option<ControlPointEdit> {
        if let Some(edit) = self.edit_history.pop() {
            let mut new_poles = self.surface.poles().to_vec();
            let mut new_weights = self.surface.weights().to_vec();
            new_poles[edit.handle.u_index][edit.handle.v_index] = edit.old_position;
            new_weights[edit.handle.u_index][edit.handle.v_index] = edit.old_weight;
            self.surface = self.create_modified_surface(new_poles, new_weights);
            Some(edit)
        } else {
            None
        }
    }

    pub fn clear_history(&mut self) {
        self.edit_history.clear();
    }

    pub fn edit_history(&self) -> &[ControlPointEdit] {
        &self.edit_history
    }

    pub fn into_surface(self) -> NurbsSurface {
        self.surface
    }
}

pub struct SurfaceFairing {
    iterations: usize,
    lambda: StandardReal,
}

impl SurfaceFairing {
    pub fn new() -> Self {
        Self {
            iterations: 10,
            lambda: 0.5,
        }
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_lambda(mut self, lambda: StandardReal) -> Self {
        self.lambda = lambda.clamp(0.0, 1.0);
        self
    }

    pub fn fair(&self, surface: &NurbsSurface) -> NurbsSurface {
        let mut current_surface = surface.clone();

        for _ in 0..self.iterations {
            current_surface = self.fair_iteration(&current_surface);
        }

        current_surface
    }

    fn fair_iteration(&self, surface: &NurbsSurface) -> NurbsSurface {
        let poles = surface.poles();
        let weights = surface.weights();
        let mut new_poles = poles.to_vec();

        let u_count = poles.len();
        if u_count < 3 {
            return surface.clone();
        }

        for i in 1..(u_count - 1) {
            let v_count = poles[i].len();
            if v_count < 3 {
                continue;
            }

            for j in 1..(v_count - 1) {
                let neighbors_sum = Point::new(
                    poles[i - 1][j].x + poles[i + 1][j].x + poles[i][j - 1].x + poles[i][j + 1].x,
                    poles[i - 1][j].y + poles[i + 1][j].y + poles[i][j - 1].y + poles[i][j + 1].y,
                    poles[i - 1][j].z + poles[i + 1][j].z + poles[i][j - 1].z + poles[i][j + 1].z,
                );

                let laplacian = Vector::new(
                    (neighbors_sum.x / 4.0) - poles[i][j].x,
                    (neighbors_sum.y / 4.0) - poles[i][j].y,
                    (neighbors_sum.z / 4.0) - poles[i][j].z,
                );

                new_poles[i][j] = Point::new(
                    poles[i][j].x + self.lambda * laplacian.x,
                    poles[i][j].y + self.lambda * laplacian.y,
                    poles[i][j].z + self.lambda * laplacian.z,
                );
            }
        }

        NurbsSurface::new(
            surface.u_degree(),
            surface.v_degree(),
            new_poles,
            weights.to_vec(),
            surface.u_knots().to_vec(),
            surface.v_knots().to_vec(),
            surface.u_multiplicities().to_vec(),
            surface.v_multiplicities().to_vec(),
        )
    }
}

impl Default for SurfaceFairing {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SurfaceSmoother {
    iterations: usize,
    sigma: StandardReal,
}

impl SurfaceSmoother {
    pub fn new() -> Self {
        Self {
            iterations: 5,
            sigma: 1.0,
        }
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_sigma(mut self, sigma: StandardReal) -> Self {
        self.sigma = sigma.max(0.001);
        self
    }

    pub fn smooth(&self, surface: &NurbsSurface) -> NurbsSurface {
        let mut current_surface = surface.clone();

        for _ in 0..self.iterations {
            current_surface = self.gaussian_smooth(&current_surface);
        }

        current_surface
    }

    fn gaussian_smooth(&self, surface: &NurbsSurface) -> NurbsSurface {
        let poles = surface.poles();
        let weights = surface.weights();
        let mut new_poles = poles.to_vec();

        let u_count = poles.len();
        if u_count < 3 {
            return surface.clone();
        }

        for i in 0..u_count {
            let v_count = poles[i].len();
            if v_count < 3 {
                continue;
            }

            for j in 0..v_count {
                let mut sum_x = 0.0;
                let mut sum_y = 0.0;
                let mut sum_z = 0.0;
                let mut weight_sum = 0.0;

                for di in -1i32..=1 {
                    for dj in -1i32..=1 {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;

                        if ni >= 0 && (ni as usize) < u_count && nj >= 0 && (nj as usize) < v_count {
                            let ni = ni as usize;
                            let nj = nj as usize;

                            let dist_sq = (di * di + dj * dj) as StandardReal;
                            let gaussian_weight = (-dist_sq / (2.0 * self.sigma * self.sigma)).exp();

                            sum_x += poles[ni][nj].x * gaussian_weight;
                            sum_y += poles[ni][nj].y * gaussian_weight;
                            sum_z += poles[ni][nj].z * gaussian_weight;
                            weight_sum += gaussian_weight;
                        }
                    }
                }

                if weight_sum > 0.0 {
                    new_poles[i][j] = Point::new(
                        sum_x / weight_sum,
                        sum_y / weight_sum,
                        sum_z / weight_sum,
                    );
                }
            }
        }

        NurbsSurface::new(
            surface.u_degree(),
            surface.v_degree(),
            new_poles,
            weights.to_vec(),
            surface.u_knots().to_vec(),
            surface.v_knots().to_vec(),
            surface.u_multiplicities().to_vec(),
            surface.v_multiplicities().to_vec(),
        )
    }
}

impl Default for SurfaceSmoother {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ContinuityController;

impl ContinuityController {
    pub fn new() -> Self {
        Self
    }

    pub fn enforce_continuity(
        &self,
        surface: &NurbsSurface,
        boundary: BoundaryType,
        level: ContinuityLevel,
    ) -> NurbsSurface {
        match level {
            ContinuityLevel::G0 => self.enforce_g0(surface, boundary),
            ContinuityLevel::G1 => self.enforce_g1(surface, boundary),
            ContinuityLevel::G2 => self.enforce_g2(surface, boundary),
            ContinuityLevel::G3 => self.enforce_g3(surface, boundary),
        }
    }

    fn enforce_g0(&self, surface: &NurbsSurface, _boundary: BoundaryType) -> NurbsSurface {
        surface.clone()
    }

    fn enforce_g1(&self, surface: &NurbsSurface, boundary: BoundaryType) -> NurbsSurface {
        let poles = surface.poles();
        let weights = surface.weights();
        let mut new_poles = poles.to_vec();

        match boundary {
            BoundaryType::UMin => {
                if poles.len() >= 2 {
                    let v_count = poles[0].len();
                    for j in 0..v_count {
                        let tangent = Vector::new(
                            poles[1][j].x - poles[0][j].x,
                            poles[1][j].y - poles[0][j].y,
                            poles[1][j].z - poles[0][j].z,
                        );
                        new_poles[1][j] = Point::new(
                            poles[0][j].x + tangent.x * 0.5,
                            poles[0][j].y + tangent.y * 0.5,
                            poles[0][j].z + tangent.z * 0.5,
                        );
                    }
                }
            }
            BoundaryType::UMax => {
                let u_count = poles.len();
                if u_count >= 2 {
                    let v_count = poles[u_count - 1].len();
                    for j in 0..v_count {
                        let tangent = Vector::new(
                            poles[u_count - 2][j].x - poles[u_count - 1][j].x,
                            poles[u_count - 2][j].y - poles[u_count - 1][j].y,
                            poles[u_count - 2][j].z - poles[u_count - 1][j].z,
                        );
                        new_poles[u_count - 2][j] = Point::new(
                            poles[u_count - 1][j].x + tangent.x * 0.5,
                            poles[u_count - 1][j].y + tangent.y * 0.5,
                            poles[u_count - 1][j].z + tangent.z * 0.5,
                        );
                    }
                }
            }
            BoundaryType::VMin => {
                let u_count = poles.len();
                for i in 0..u_count {
                    if poles[i].len() >= 2 {
                        let tangent = Vector::new(
                            poles[i][1].x - poles[i][0].x,
                            poles[i][1].y - poles[i][0].y,
                            poles[i][1].z - poles[i][0].z,
                        );
                        new_poles[i][1] = Point::new(
                            poles[i][0].x + tangent.x * 0.5,
                            poles[i][0].y + tangent.y * 0.5,
                            poles[i][0].z + tangent.z * 0.5,
                        );
                    }
                }
            }
            BoundaryType::VMax => {
                let u_count = poles.len();
                for i in 0..u_count {
                    let v_count = poles[i].len();
                    if v_count >= 2 {
                        let tangent = Vector::new(
                            poles[i][v_count - 2].x - poles[i][v_count - 1].x,
                            poles[i][v_count - 2].y - poles[i][v_count - 1].y,
                            poles[i][v_count - 2].z - poles[i][v_count - 1].z,
                        );
                        new_poles[i][v_count - 2] = Point::new(
                            poles[i][v_count - 1].x + tangent.x * 0.5,
                            poles[i][v_count - 1].y + tangent.y * 0.5,
                            poles[i][v_count - 1].z + tangent.z * 0.5,
                        );
                    }
                }
            }
        }

        NurbsSurface::new(
            surface.u_degree(),
            surface.v_degree(),
            new_poles,
            weights.to_vec(),
            surface.u_knots().to_vec(),
            surface.v_knots().to_vec(),
            surface.u_multiplicities().to_vec(),
            surface.v_multiplicities().to_vec(),
        )
    }

    fn enforce_g2(&self, surface: &NurbsSurface, boundary: BoundaryType) -> NurbsSurface {
        let result = self.enforce_g1(surface, boundary);

        let poles = result.poles();
        let weights = result.weights();
        let mut new_poles = poles.to_vec();

        match boundary {
            BoundaryType::UMin => {
                if poles.len() >= 3 {
                    let v_count = poles[0].len();
                    for j in 0..v_count {
                        let d1 = Vector::new(
                            poles[1][j].x - poles[0][j].x,
                            poles[1][j].y - poles[0][j].y,
                            poles[1][j].z - poles[0][j].z,
                        );
                        let d2 = Vector::new(
                            poles[2][j].x - poles[1][j].x,
                            poles[2][j].y - poles[1][j].y,
                            poles[2][j].z - poles[1][j].z,
                        );
                        new_poles[2][j] = Point::new(
                            poles[1][j].x + d1.x * 0.5 + d2.x * 0.25,
                            poles[1][j].y + d1.y * 0.5 + d2.y * 0.25,
                            poles[1][j].z + d1.z * 0.5 + d2.z * 0.25,
                        );
                    }
                }
            }
            _ => {}
        }

        NurbsSurface::new(
            result.u_degree(),
            result.v_degree(),
            new_poles,
            weights.to_vec(),
            result.u_knots().to_vec(),
            result.v_knots().to_vec(),
            result.u_multiplicities().to_vec(),
            result.v_multiplicities().to_vec(),
        )
    }

    fn enforce_g3(&self, surface: &NurbsSurface, boundary: BoundaryType) -> NurbsSurface {
        let result = self.enforce_g2(surface, boundary);

        let poles = result.poles();
        let weights = result.weights();
        let mut new_poles = poles.to_vec();

        match boundary {
            BoundaryType::UMin => {
                if poles.len() >= 4 {
                    let v_count = poles[0].len();
                    for j in 0..v_count {
                        let d1 = Vector::new(
                            poles[1][j].x - poles[0][j].x,
                            poles[1][j].y - poles[0][j].y,
                            poles[1][j].z - poles[0][j].z,
                        );
                        let d2 = Vector::new(
                            poles[2][j].x - poles[1][j].x,
                            poles[2][j].y - poles[1][j].y,
                            poles[2][j].z - poles[1][j].z,
                        );
                        let d3 = Vector::new(
                            poles[3][j].x - poles[2][j].x,
                            poles[3][j].y - poles[2][j].y,
                            poles[3][j].z - poles[2][j].z,
                        );
                        new_poles[3][j] = Point::new(
                            poles[2][j].x + d2.x * 0.5 + d1.x * 0.125 + d3.x * 0.125,
                            poles[2][j].y + d2.y * 0.5 + d1.y * 0.125 + d3.y * 0.125,
                            poles[2][j].z + d2.z * 0.5 + d1.z * 0.125 + d3.z * 0.125,
                        );
                    }
                }
            }
            _ => {}
        }

        NurbsSurface::new(
            result.u_degree(),
            result.v_degree(),
            new_poles,
            weights.to_vec(),
            result.u_knots().to_vec(),
            result.v_knots().to_vec(),
            result.u_multiplicities().to_vec(),
            result.v_multiplicities().to_vec(),
        )
    }
}

impl Default for ContinuityController {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryType {
    UMin,
    UMax,
    VMin,
    VMax,
}

pub struct InteractiveDeformation {
    surface: NurbsSurface,
    deformation_region: DeformationRegion,
}

#[derive(Debug, Clone)]
pub struct DeformationRegion {
    u_min: usize,
    u_max: usize,
    v_min: usize,
    v_max: usize,
}

impl DeformationRegion {
    pub fn new(u_min: usize, u_max: usize, v_min: usize, v_max: usize) -> Self {
        Self {
            u_min,
            u_max,
            v_min,
            v_max,
        }
    }

    pub fn contains(&self, u: usize, v: usize) -> bool {
        u >= self.u_min && u <= self.u_max && v >= self.v_min && v <= self.v_max
    }

    pub fn u_range(&self) -> (usize, usize) {
        (self.u_min, self.u_max)
    }

    pub fn v_range(&self) -> (usize, usize) {
        (self.v_min, self.v_max)
    }
}

impl InteractiveDeformation {
    pub fn new(surface: NurbsSurface) -> Self {
        let u_max = (surface.poles().len() - 1).max(0);
        let v_max = if u_max > 0 {
            (surface.poles()[0].len() - 1).max(0)
        } else {
            0
        };

        Self {
            surface,
            deformation_region: DeformationRegion::new(0, u_max, 0, v_max),
        }
    }

    pub fn set_deformation_region(&mut self, region: DeformationRegion) {
        self.deformation_region = region;
    }

    pub fn deformation_region(&self) -> &DeformationRegion {
        &self.deformation_region
    }

    pub fn apply_push_deformation(&mut self, center_u: usize, center_v: usize, direction: Vector, strength: StandardReal, falloff: StandardReal) {
        let poles = self.surface.poles();
        let weights = self.surface.weights();
        let mut new_poles = poles.to_vec();

        for i in self.deformation_region.u_min..=self.deformation_region.u_max {
            for j in self.deformation_region.v_min..=self.deformation_region.v_max {
                let du = (i as StandardReal - center_u as StandardReal).abs();
                let dv = (j as StandardReal - center_v as StandardReal).abs();
                let distance = (du * du + dv * dv).sqrt();

                let influence = if falloff > 0.0 {
                    (-distance / falloff).exp()
                } else {
                    if distance < 1.0 { 1.0 } else { 0.0 }
                };

                let displacement = Vector::new(
                    direction.x * strength * influence,
                    direction.y * strength * influence,
                    direction.z * strength * influence,
                );

                new_poles[i][j] = Point::new(
                    poles[i][j].x + displacement.x,
                    poles[i][j].y + displacement.y,
                    poles[i][j].z + displacement.z,
                );
            }
        }

        self.surface = NurbsSurface::new(
            self.surface.u_degree(),
            self.surface.v_degree(),
            new_poles,
            weights.to_vec(),
            self.surface.u_knots().to_vec(),
            self.surface.v_knots().to_vec(),
            self.surface.u_multiplicities().to_vec(),
            self.surface.v_multiplicities().to_vec(),
        );
    }

    pub fn apply_twist_deformation(&mut self, axis_u: bool, angle: StandardReal, center_u: usize, center_v: usize) {
        let poles = self.surface.poles();
        let weights = self.surface.weights();
        let mut new_poles = poles.to_vec();

        for i in self.deformation_region.u_min..=self.deformation_region.u_max {
            for j in self.deformation_region.v_min..=self.deformation_region.v_max {
                let (offset, _center) = if axis_u {
                    ((j as StandardReal - center_v as StandardReal), center_v as StandardReal)
                } else {
                    ((i as StandardReal - center_u as StandardReal), center_u as StandardReal)
                };

                let normalized_offset = if self.deformation_region.u_max > self.deformation_region.u_min {
                    offset / ((self.deformation_region.u_max - self.deformation_region.u_min) as StandardReal)
                } else {
                    0.0
                };

                let twist_angle = angle * normalized_offset;
                let cos_a = twist_angle.cos();
                let sin_a = twist_angle.sin();

                let x = poles[i][j].x;
                let y = poles[i][j].y;

                new_poles[i][j] = Point::new(
                    x * cos_a - y * sin_a,
                    x * sin_a + y * cos_a,
                    poles[i][j].z,
                );
            }
        }

        self.surface = NurbsSurface::new(
            self.surface.u_degree(),
            self.surface.v_degree(),
            new_poles,
            weights.to_vec(),
            self.surface.u_knots().to_vec(),
            self.surface.v_knots().to_vec(),
            self.surface.u_multiplicities().to_vec(),
            self.surface.v_multiplicities().to_vec(),
        );
    }

    pub fn apply_bend_deformation(&mut self, bend_angle: StandardReal, axis_u: bool) {
        let poles = self.surface.poles();
        let weights = self.surface.weights();
        let mut new_poles = poles.to_vec();

        let range = if axis_u {
            self.deformation_region.u_max - self.deformation_region.u_min
        } else {
            self.deformation_region.v_max - self.deformation_region.v_min
        };

        if range == 0 {
            return;
        }

        for i in self.deformation_region.u_min..=self.deformation_region.u_max {
            for j in self.deformation_region.v_min..=self.deformation_region.v_max {
                let t = if axis_u {
                    (i - self.deformation_region.u_min) as StandardReal / range as StandardReal
                } else {
                    (j - self.deformation_region.v_min) as StandardReal / range as StandardReal
                };

                let angle = bend_angle * t;
                let radius = 1.0 / bend_angle.abs().max(0.001);

                let x = poles[i][j].x;
                let y = poles[i][j].y;
                let z = poles[i][j].z;

                let new_x = x * angle.cos() + radius * angle.sin();
                let new_y = y;
                let new_z = z * angle.cos() + radius * (1.0 - angle.cos());

                new_poles[i][j] = Point::new(new_x, new_y, new_z);
            }
        }

        self.surface = NurbsSurface::new(
            self.surface.u_degree(),
            self.surface.v_degree(),
            new_poles,
            weights.to_vec(),
            self.surface.u_knots().to_vec(),
            self.surface.v_knots().to_vec(),
            self.surface.u_multiplicities().to_vec(),
            self.surface.v_multiplicities().to_vec(),
        );
    }

    pub fn into_surface(self) -> NurbsSurface {
        self.surface
    }

    pub fn surface(&self) -> &NurbsSurface {
        &self.surface
    }
}

pub fn create_bezier_surface_editable(degree_u: usize, degree_v: usize) -> NurbsSurface {
    let mut poles = Vec::with_capacity(degree_u + 1);
    let mut weights = Vec::with_capacity(degree_u + 1);

    for i in 0..=degree_u {
        let mut row = Vec::with_capacity(degree_v + 1);
        let mut weight_row = Vec::with_capacity(degree_v + 1);

        for j in 0..=degree_v {
            let x = i as StandardReal / degree_u as StandardReal;
            let y = j as StandardReal / degree_v as StandardReal;
            row.push(Point::new(x, y, 0.0));
            weight_row.push(1.0);
        }

        poles.push(row);
        weights.push(weight_row);
    }

    let u_knots: Vec<StandardReal> = vec![0.0, 1.0];
    let v_knots: Vec<StandardReal> = vec![0.0, 1.0];
    let u_multiplicities: Vec<i32> = vec![(degree_u + 1) as i32];
    let v_multiplicities: Vec<i32> = vec![(degree_v + 1) as i32];

    NurbsSurface::new(
        degree_u as i32,
        degree_v as i32,
        poles,
        weights,
        u_knots,
        v_knots,
        u_multiplicities,
        v_multiplicities,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_editable_surface() {
        let surface = create_bezier_surface_editable(3, 3);
        assert_eq!(surface.u_degree(), 3);
        assert_eq!(surface.v_degree(), 3);
        assert_eq!(surface.nb_u_poles(), 4);
        assert_eq!(surface.nb_v_poles(), 4);
    }

    #[test]
    fn test_move_control_point() {
        let surface = create_bezier_surface_editable(3, 3);
        let mut editor = FreeFormSurfaceEditor::new(surface);

        let handle = ControlPointHandle::new(1, 1);
        let delta = Vector::new(0.0, 0.0, 1.0);

        let result = editor.move_control_point(&handle, delta);
        assert!(result.is_ok());

        let new_pos = result.unwrap();
        assert_eq!(new_pos.z, 1.0);
    }

    #[test]
    fn test_surface_fairing() {
        let surface = create_bezier_surface_editable(3, 3);
        let fairing = SurfaceFairing::new().with_iterations(5);
        let faired = fairing.fair(&surface);

        assert_eq!(faired.u_degree(), surface.u_degree());
        assert_eq!(faired.v_degree(), surface.v_degree());
    }

    #[test]
    fn test_surface_smoothing() {
        let surface = create_bezier_surface_editable(3, 3);
        let smoother = SurfaceSmoother::new().with_iterations(3);
        let smoothed = smoother.smooth(&surface);

        assert_eq!(smoothed.u_degree(), surface.u_degree());
        assert_eq!(smoothed.v_degree(), surface.v_degree());
    }

    #[test]
    fn test_interactive_deformation() {
        let surface = create_bezier_surface_editable(3, 3);
        let mut deformation = InteractiveDeformation::new(surface);

        deformation.apply_push_deformation(
            2, 2,
            Vector::new(0.0, 0.0, 1.0),
            0.5,
            1.0,
        );

        let result = deformation.into_surface();
        assert_eq!(result.u_degree(), 3);
    }
}
