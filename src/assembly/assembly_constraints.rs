use std::collections::HashMap;
use crate::geometry::{Point, Vector};
use crate::foundation::types::StandardReal;
use super::assembly_manager::{ComponentId, Assembly};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintId(pub u64);

impl ConstraintId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for ConstraintId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    Coincident,
    Parallel,
    Perpendicular,
    Tangent,
    Concentric,
    Distance,
    Angle,
    Fix,
    Symmetric,
}

#[derive(Debug, Clone)]
pub enum GeometryReference {
    Vertex(ComponentId, usize),
    Edge(ComponentId, usize),
    Face(ComponentId, usize),
    Axis(ComponentId, Vector),
    Plane(ComponentId, Point, Vector),
    Origin(ComponentId),
}

impl GeometryReference {
    pub fn component_id(&self) -> ComponentId {
        match self {
            GeometryReference::Vertex(id, _) => *id,
            GeometryReference::Edge(id, _) => *id,
            GeometryReference::Face(id, _) => *id,
            GeometryReference::Axis(id, _) => *id,
            GeometryReference::Plane(id, _, _) => *id,
            GeometryReference::Origin(id) => *id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    id: ConstraintId,
    constraint_type: ConstraintType,
    first_geometry: GeometryReference,
    second_geometry: GeometryReference,
    value: Option<StandardReal>,
    offset: Vector,
    enabled: bool,
    name: String,
}

impl Constraint {
    pub fn new(
        constraint_type: ConstraintType,
        first_geometry: GeometryReference,
        second_geometry: GeometryReference,
    ) -> Self {
        Self {
            id: ConstraintId::new(),
            constraint_type,
            first_geometry,
            second_geometry,
            value: None,
            offset: Vector::zero(),
            enabled: true,
            name: String::new(),
        }
    }

    pub fn with_value(mut self, value: StandardReal) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn id(&self) -> ConstraintId {
        self.id
    }

    pub fn constraint_type(&self) -> ConstraintType {
        self.constraint_type
    }

    pub fn first_geometry(&self) -> &GeometryReference {
        &self.first_geometry
    }

    pub fn second_geometry(&self) -> &GeometryReference {
        &self.second_geometry
    }

    pub fn value(&self) -> Option<StandardReal> {
        self.value
    }

    pub fn offset(&self) -> &Vector {
        &self.offset
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintStatus {
    Satisfied,
    Violated,
    OverConstrained,
    UnderConstrained,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct ConstraintResult {
    pub status: ConstraintStatus,
    pub error: StandardReal,
    pub message: String,
}

impl ConstraintResult {
    pub fn satisfied() -> Self {
        Self {
            status: ConstraintStatus::Satisfied,
            error: 0.0,
            message: String::new(),
        }
    }

    pub fn violated(error: StandardReal, message: String) -> Self {
        Self {
            status: ConstraintStatus::Violated,
            error,
            message,
        }
    }

    pub fn conflict(message: String) -> Self {
        Self {
            status: ConstraintStatus::Conflict,
            error: 0.0,
            message,
        }
    }
}

pub struct ConstraintSolver {
    constraints: HashMap<ConstraintId, Constraint>,
    tolerance: StandardReal,
    max_iterations: usize,
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self {
            constraints: HashMap::new(),
            tolerance: 1e-6,
            max_iterations: 100,
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> ConstraintId {
        let id = constraint.id;
        self.constraints.insert(id, constraint);
        id
    }

    pub fn remove_constraint(&mut self, id: ConstraintId) -> Option<Constraint> {
        self.constraints.remove(&id)
    }

    pub fn get_constraint(&self, id: ConstraintId) -> Option<&Constraint> {
        self.constraints.get(&id)
    }

    pub fn get_constraint_mut(&mut self, id: ConstraintId) -> Option<&mut Constraint> {
        self.constraints.get_mut(&id)
    }

    pub fn constraints(&self) -> &HashMap<ConstraintId, Constraint> {
        &self.constraints
    }

    pub fn set_tolerance(&mut self, tolerance: StandardReal) {
        self.tolerance = tolerance;
    }

    pub fn set_max_iterations(&mut self, iterations: usize) {
        self.max_iterations = iterations;
    }

    pub fn solve(&self, assembly: &Assembly) -> HashMap<ConstraintId, ConstraintResult> {
        let mut results = HashMap::new();

        for (id, constraint) in &self.constraints {
            if constraint.enabled {
                let result = self.evaluate_constraint(constraint, assembly);
                results.insert(*id, result);
            } else {
                results.insert(*id, ConstraintResult::satisfied());
            }
        }

        results
    }

    fn evaluate_constraint(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        match constraint.constraint_type {
            ConstraintType::Coincident => self.evaluate_coincident(constraint, assembly),
            ConstraintType::Parallel => self.evaluate_parallel(constraint, assembly),
            ConstraintType::Perpendicular => self.evaluate_perpendicular(constraint, assembly),
            ConstraintType::Tangent => self.evaluate_tangent(constraint, assembly),
            ConstraintType::Concentric => self.evaluate_concentric(constraint, assembly),
            ConstraintType::Distance => self.evaluate_distance(constraint, assembly),
            ConstraintType::Angle => self.evaluate_angle(constraint, assembly),
            ConstraintType::Fix => self.evaluate_fix(constraint, assembly),
            ConstraintType::Symmetric => self.evaluate_symmetric(constraint, assembly),
        }
    }

    fn evaluate_coincident(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let pos1 = self.get_geometry_position(&constraint.first_geometry, assembly);
        let pos2 = self.get_geometry_position(&constraint.second_geometry, assembly);

        match (pos1, pos2) {
            (Some(p1), Some(p2)) => {
                let dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt();
                if dist < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(dist, format!("Points are {} units apart", dist))
                }
            }
            _ => ConstraintResult::conflict("Could not determine geometry positions".to_string()),
        }
    }

    fn evaluate_parallel(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let dir1 = self.get_geometry_direction(&constraint.first_geometry, assembly);
        let dir2 = self.get_geometry_direction(&constraint.second_geometry, assembly);

        match (dir1, dir2) {
            (Some(d1), Some(d2)) => {
                let cross = d1.cross(&d2);
                let cross_mag = (cross.x.powi(2) + cross.y.powi(2) + cross.z.powi(2)).sqrt();
                if cross_mag < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(cross_mag, "Directions are not parallel".to_string())
                }
            }
            _ => ConstraintResult::conflict("Could not determine geometry directions".to_string()),
        }
    }

    fn evaluate_perpendicular(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let dir1 = self.get_geometry_direction(&constraint.first_geometry, assembly);
        let dir2 = self.get_geometry_direction(&constraint.second_geometry, assembly);

        match (dir1, dir2) {
            (Some(d1), Some(d2)) => {
                let dot = d1.dot(&d2);
                if dot.abs() < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(dot.abs(), "Directions are not perpendicular".to_string())
                }
            }
            _ => ConstraintResult::conflict("Could not determine geometry directions".to_string()),
        }
    }

    fn evaluate_tangent(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let pos1 = self.get_geometry_position(&constraint.first_geometry, assembly);
        let dir1 = self.get_geometry_direction(&constraint.first_geometry, assembly);
        let pos2 = self.get_geometry_position(&constraint.second_geometry, assembly);
        let dir2 = self.get_geometry_direction(&constraint.second_geometry, assembly);

        match (pos1, dir1, pos2, dir2) {
            (Some(p1), Some(d1), Some(p2), Some(d2)) => {
                let diff = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
                let dot = diff.dot(&d1);
                let perp_dist = ((diff.x.powi(2) + diff.y.powi(2) + diff.z.powi(2)).sqrt() - dot.abs()).abs();
                let dot2 = d1.dot(&d2);
                if perp_dist < self.tolerance && dot2.abs() < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(perp_dist, "Not tangent".to_string())
                }
            }
            _ => ConstraintResult::conflict("Could not evaluate tangency".to_string()),
        }
    }

    fn evaluate_concentric(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let pos1 = self.get_geometry_position(&constraint.first_geometry, assembly);
        let pos2 = self.get_geometry_position(&constraint.second_geometry, assembly);

        match (pos1, pos2) {
            (Some(p1), Some(p2)) => {
                let dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt();
                if dist < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(dist, format!("Centers are {} units apart", dist))
                }
            }
            _ => ConstraintResult::conflict("Could not determine center positions".to_string()),
        }
    }

    fn evaluate_distance(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let pos1 = self.get_geometry_position(&constraint.first_geometry, assembly);
        let pos2 = self.get_geometry_position(&constraint.second_geometry, assembly);

        let target_distance = constraint.value.unwrap_or(0.0);

        match (pos1, pos2) {
            (Some(p1), Some(p2)) => {
                let actual_dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt();
                let error = (actual_dist - target_distance).abs();
                if error < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(error, format!("Distance error: {}", error))
                }
            }
            _ => ConstraintResult::conflict("Could not determine positions".to_string()),
        }
    }

    fn evaluate_angle(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let dir1 = self.get_geometry_direction(&constraint.first_geometry, assembly);
        let dir2 = self.get_geometry_direction(&constraint.second_geometry, assembly);

        let target_angle = constraint.value.unwrap_or(0.0).to_radians();

        match (dir1, dir2) {
            (Some(d1), Some(d2)) => {
                let dot = d1.dot(&d2).clamp(-1.0, 1.0);
                let actual_angle = dot.acos();
                let error = (actual_angle - target_angle).abs();
                if error < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(error, format!("Angle error: {} radians", error))
                }
            }
            _ => ConstraintResult::conflict("Could not determine directions".to_string()),
        }
    }

    fn evaluate_fix(&self, _constraint: &Constraint, _assembly: &Assembly) -> ConstraintResult {
        ConstraintResult::satisfied()
    }

    fn evaluate_symmetric(&self, constraint: &Constraint, assembly: &Assembly) -> ConstraintResult {
        let pos1 = self.get_geometry_position(&constraint.first_geometry, assembly);
        let pos2 = self.get_geometry_position(&constraint.second_geometry, assembly);

        match (pos1, pos2) {
            (Some(p1), Some(p2)) => {
                let _mid = Point::new(
                    (p1.x + p2.x) / 2.0,
                    (p1.y + p2.y) / 2.0,
                    (p1.z + p2.z) / 2.0,
                );
                let _diff = Vector::new(
                    p2.x - p1.x,
                    p2.y - p1.y,
                    p2.z - p1.z,
                );
                let dist = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt();
                if dist < self.tolerance {
                    ConstraintResult::satisfied()
                } else {
                    ConstraintResult::violated(dist / 2.0, "Not symmetric".to_string())
                }
            }
            _ => ConstraintResult::conflict("Could not determine positions".to_string()),
        }
    }

    fn get_geometry_position(&self, geom: &GeometryReference, assembly: &Assembly) -> Option<Point> {
        match geom {
            GeometryReference::Vertex(component_id, _vertex_idx) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(&Point::origin()))
            }
            GeometryReference::Edge(component_id, _edge_idx) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(&Point::origin()))
            }
            GeometryReference::Face(component_id, _face_idx) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(&Point::origin()))
            }
            GeometryReference::Axis(component_id, _axis) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(&Point::origin()))
            }
            GeometryReference::Plane(component_id, point, _normal) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(point))
            }
            GeometryReference::Origin(component_id) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms(&Point::origin()))
            }
        }
    }

    fn get_geometry_direction(&self, geom: &GeometryReference, assembly: &Assembly) -> Option<Vector> {
        match geom {
            GeometryReference::Edge(component_id, _edge_idx) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms_vec(&Vector::new(1.0, 0.0, 0.0)))
            }
            GeometryReference::Face(component_id, _face_idx) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms_vec(&Vector::new(0.0, 0.0, 1.0)))
            }
            GeometryReference::Axis(component_id, axis) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms_vec(axis))
            }
            GeometryReference::Plane(component_id, _point, normal) => {
                let component = assembly.get_component(*component_id)?;
                let transform = component.world_transform(assembly);
                Some(transform.transforms_vec(normal))
            }
            _ => Some(Vector::new(1.0, 0.0, 0.0)),
        }
    }

    pub fn get_constraints_for_component(&self, component_id: ComponentId) -> Vec<&Constraint> {
        self.constraints
            .values()
            .filter(|c| {
                c.first_geometry.component_id() == component_id
                    || c.second_geometry.component_id() == component_id
            })
            .collect()
    }

    pub fn get_dependent_components(&self, component_id: ComponentId) -> Vec<ComponentId> {
        let mut dependents = Vec::new();
        for constraint in self.constraints.values() {
            if constraint.first_geometry.component_id() == component_id {
                dependents.push(constraint.second_geometry.component_id());
            } else if constraint.second_geometry.component_id() == component_id {
                dependents.push(constraint.first_geometry.component_id());
            }
        }
        dependents.sort();
        dependents.dedup();
        dependents
    }

    pub fn check_circular_dependencies(&self) -> Vec<Vec<ComponentId>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        let components: std::collections::HashSet<ComponentId> = self
            .constraints
            .values()
            .flat_map(|c| {
                vec![
                    c.first_geometry.component_id(),
                    c.second_geometry.component_id(),
                ]
            })
            .collect();

        for &component in &components {
            let mut path = Vec::new();
            self.find_cycles(component, &mut visited, &mut rec_stack, &mut path, &mut cycles);
        }

        cycles
    }

    fn find_cycles(
        &self,
        component: ComponentId,
        visited: &mut std::collections::HashSet<ComponentId>,
        rec_stack: &mut std::collections::HashSet<ComponentId>,
        path: &mut Vec<ComponentId>,
        cycles: &mut Vec<Vec<ComponentId>>,
    ) {
        if rec_stack.contains(&component) {
            let cycle_start = path.iter().position(|&c| c == component).unwrap_or(0);
            let cycle: Vec<ComponentId> = path[cycle_start..].to_vec();
            if cycle.len() > 1 {
                cycles.push(cycle);
            }
            return;
        }

        if visited.contains(&component) {
            return;
        }

        visited.insert(component);
        rec_stack.insert(component);
        path.push(component);

        for dependent in self.get_dependent_components(component) {
            self.find_cycles(dependent, visited, rec_stack, path, cycles);
        }

        rec_stack.remove(&component);
        path.pop();
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstraintBuilder {
    constraint_type: ConstraintType,
    first_geometry: Option<GeometryReference>,
    second_geometry: Option<GeometryReference>,
    value: Option<StandardReal>,
    offset: Vector,
    name: String,
}

impl ConstraintBuilder {
    pub fn new(constraint_type: ConstraintType) -> Self {
        Self {
            constraint_type,
            first_geometry: None,
            second_geometry: None,
            value: None,
            offset: Vector::zero(),
            name: String::new(),
        }
    }

    pub fn first_geometry(mut self, geom: GeometryReference) -> Self {
        self.first_geometry = Some(geom);
        self
    }

    pub fn second_geometry(mut self, geom: GeometryReference) -> Self {
        self.second_geometry = Some(geom);
        self
    }

    pub fn value(mut self, value: StandardReal) -> Self {
        self.value = Some(value);
        self
    }

    pub fn offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn build(self) -> Result<Constraint, String> {
        let first = self.first_geometry.ok_or("First geometry is required")?;
        let second = self.second_geometry.ok_or("Second geometry is required")?;

        let mut constraint = Constraint::new(self.constraint_type, first, second);

        if let Some(value) = self.value {
            constraint = constraint.with_value(value);
        }

        if self.offset.x != 0.0 || self.offset.y != 0.0 || self.offset.z != 0.0 {
            constraint = constraint.with_offset(self.offset);
        }

        if !self.name.is_empty() {
            constraint = constraint.with_name(self.name);
        }

        Ok(constraint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_creation() {
        let component_id = ComponentId::new();
        let geom1 = GeometryReference::Origin(component_id);
        let geom2 = GeometryReference::Origin(ComponentId::new());

        let constraint = Constraint::new(ConstraintType::Coincident, geom1, geom2);
        assert_eq!(constraint.constraint_type(), ConstraintType::Coincident);
        assert!(constraint.is_enabled());
    }

    #[test]
    fn test_constraint_builder() {
        let component_id = ComponentId::new();
        let constraint = ConstraintBuilder::new(ConstraintType::Distance)
            .first_geometry(GeometryReference::Origin(component_id))
            .second_geometry(GeometryReference::Origin(ComponentId::new()))
            .value(10.0)
            .name("Distance10".to_string())
            .build()
            .unwrap();

        assert_eq!(constraint.constraint_type(), ConstraintType::Distance);
        assert_eq!(constraint.value(), Some(10.0));
        assert_eq!(constraint.name(), "Distance10");
    }

    #[test]
    fn test_constraint_solver() {
        let mut solver = ConstraintSolver::new();

        let component_id = ComponentId::new();
        let constraint = Constraint::new(
            ConstraintType::Coincident,
            GeometryReference::Origin(component_id),
            GeometryReference::Origin(component_id),
        );

        let id = solver.add_constraint(constraint);
        assert!(solver.get_constraint(id).is_some());
    }

    #[test]
    fn test_get_dependent_components() {
        let mut solver = ConstraintSolver::new();

        let comp1 = ComponentId::new();
        let comp2 = ComponentId::new();

        let constraint = Constraint::new(
            ConstraintType::Coincident,
            GeometryReference::Origin(comp1),
            GeometryReference::Origin(comp2),
        );

        solver.add_constraint(constraint);

        let dependents = solver.get_dependent_components(comp1);
        assert!(dependents.contains(&comp2));
    }
}
