use std::collections::HashMap;
use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector, Plane};
use crate::topology::topods_shape::TopoDsShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SketchEntityId(pub u64);

impl SketchEntityId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for SketchEntityId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SketchEntityType {
    Point,
    Line,
    Circle,
    Arc,
    Ellipse,
    Spline,
    Polygon,
}

#[derive(Debug, Clone)]
pub enum SketchEntity {
    Point(SketchPoint),
    Line(SketchLine),
    Circle(SketchCircle),
    Arc(SketchArc),
    Ellipse(SketchEllipse),
    Spline(SketchSpline),
    Polygon(SketchPolygon),
}

impl SketchEntity {
    pub fn id(&self) -> SketchEntityId {
        match self {
            SketchEntity::Point(e) => e.id,
            SketchEntity::Line(e) => e.id,
            SketchEntity::Circle(e) => e.id,
            SketchEntity::Arc(e) => e.id,
            SketchEntity::Ellipse(e) => e.id,
            SketchEntity::Spline(e) => e.id,
            SketchEntity::Polygon(e) => e.id,
        }
    }

    pub fn entity_type(&self) -> SketchEntityType {
        match self {
            SketchEntity::Point(_) => SketchEntityType::Point,
            SketchEntity::Line(_) => SketchEntityType::Line,
            SketchEntity::Circle(_) => SketchEntityType::Circle,
            SketchEntity::Arc(_) => SketchEntityType::Arc,
            SketchEntity::Ellipse(_) => SketchEntityType::Ellipse,
            SketchEntity::Spline(_) => SketchEntityType::Spline,
            SketchEntity::Polygon(_) => SketchEntityType::Polygon,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SketchPoint {
    pub id: SketchEntityId,
    pub position: Point,
    pub fixed: bool,
    pub construction: bool,
}

impl SketchPoint {
    pub fn new(position: Point) -> Self {
        Self {
            id: SketchEntityId::new(),
            position,
            fixed: false,
            construction: false,
        }
    }

    pub fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchLine {
    pub id: SketchEntityId,
    pub start_point: SketchEntityId,
    pub end_point: SketchEntityId,
    pub construction: bool,
}

impl SketchLine {
    pub fn new(start_point: SketchEntityId, end_point: SketchEntityId) -> Self {
        Self {
            id: SketchEntityId::new(),
            start_point,
            end_point,
            construction: false,
        }
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchCircle {
    pub id: SketchEntityId,
    pub center: SketchEntityId,
    pub radius: StandardReal,
    pub construction: bool,
}

impl SketchCircle {
    pub fn new(center: SketchEntityId, radius: StandardReal) -> Self {
        Self {
            id: SketchEntityId::new(),
            center,
            radius,
            construction: false,
        }
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchArc {
    pub id: SketchEntityId,
    pub center: SketchEntityId,
    pub start_point: SketchEntityId,
    pub end_point: SketchEntityId,
    pub radius: StandardReal,
    pub construction: bool,
}

impl SketchArc {
    pub fn new(center: SketchEntityId, start_point: SketchEntityId, end_point: SketchEntityId, radius: StandardReal) -> Self {
        Self {
            id: SketchEntityId::new(),
            center,
            start_point,
            end_point,
            radius,
            construction: false,
        }
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchEllipse {
    pub id: SketchEntityId,
    pub center: SketchEntityId,
    pub major_axis: StandardReal,
    pub minor_axis: StandardReal,
    pub angle: StandardReal,
    pub construction: bool,
}

impl SketchEllipse {
    pub fn new(center: SketchEntityId, major_axis: StandardReal, minor_axis: StandardReal) -> Self {
        Self {
            id: SketchEntityId::new(),
            center,
            major_axis,
            minor_axis,
            angle: 0.0,
            construction: false,
        }
    }

    pub fn with_angle(mut self, angle: StandardReal) -> Self {
        self.angle = angle;
        self
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchSpline {
    pub id: SketchEntityId,
    pub control_points: Vec<SketchEntityId>,
    pub degree: i32,
    pub closed: bool,
    pub construction: bool,
}

impl SketchSpline {
    pub fn new(control_points: Vec<SketchEntityId>, degree: i32) -> Self {
        Self {
            id: SketchEntityId::new(),
            control_points,
            degree,
            closed: false,
            construction: false,
        }
    }

    pub fn closed(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchPolygon {
    pub id: SketchEntityId,
    pub center: SketchEntityId,
    pub num_sides: usize,
    pub radius: StandardReal,
    pub angle: StandardReal,
    pub construction: bool,
}

impl SketchPolygon {
    pub fn new(center: SketchEntityId, num_sides: usize, radius: StandardReal) -> Self {
        Self {
            id: SketchEntityId::new(),
            center,
            num_sides,
            radius,
            angle: 0.0,
            construction: false,
        }
    }

    pub fn with_angle(mut self, angle: StandardReal) -> Self {
        self.angle = angle;
        self
    }

    pub fn construction(mut self) -> Self {
        self.construction = true;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    Coincident,
    Horizontal,
    Vertical,
    Parallel,
    Perpendicular,
    Tangent,
    Equal,
    Symmetric,
    Distance,
    Angle,
    Radius,
    Diameter,
    Fix,
    Concentric,
}

#[derive(Debug, Clone)]
pub struct SketchConstraint {
    pub id: SketchEntityId,
    pub constraint_type: ConstraintType,
    pub entities: Vec<SketchEntityId>,
    pub value: Option<StandardReal>,
    pub enabled: bool,
}

impl SketchConstraint {
    pub fn new(constraint_type: ConstraintType, entities: Vec<SketchEntityId>) -> Self {
        Self {
            id: SketchEntityId::new(),
            constraint_type,
            entities,
            value: None,
            enabled: true,
        }
    }

    pub fn with_value(mut self, value: StandardReal) -> Self {
        self.value = Some(value);
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SketchDimension {
    pub id: SketchEntityId,
    pub dimension_type: ConstraintType,
    pub entities: Vec<SketchEntityId>,
    pub value: StandardReal,
    pub position: Point,
    pub text: String,
}

impl SketchDimension {
    pub fn new(dimension_type: ConstraintType, entities: Vec<SketchEntityId>, value: StandardReal) -> Self {
        Self {
            id: SketchEntityId::new(),
            dimension_type,
            entities,
            value,
            position: Point::origin(),
            text: format!("{:.2}", value),
        }
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

pub struct Sketch {
    name: String,
    plane: Plane,
    entities: HashMap<SketchEntityId, SketchEntity>,
    constraints: Vec<SketchConstraint>,
    dimensions: Vec<SketchDimension>,
    _origin: Point,
    _x_axis: Vector,
    _y_axis: Vector,
    solved: bool,
}

impl Sketch {
    pub fn new(name: String, plane: Plane) -> Self {
        let origin = plane.location().clone();
        let x_dir = plane.x_direction();
        let y_dir = plane.y_direction();

        Self {
            name,
            plane,
            entities: HashMap::new(),
            constraints: Vec::new(),
            dimensions: Vec::new(),
            _origin: origin,
            _x_axis: Vector::new(x_dir.x, x_dir.y, x_dir.z),
            _y_axis: Vector::new(y_dir.x, y_dir.y, y_dir.z),
            solved: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn plane(&self) -> &Plane {
        &self.plane
    }

    pub fn add_point(&mut self, position: Point) -> SketchEntityId {
        let point = SketchPoint::new(position);
        let id = point.id;
        self.entities.insert(id, SketchEntity::Point(point));
        self.solved = false;
        id
    }

    pub fn add_line(&mut self, start: SketchEntityId, end: SketchEntityId) -> Result<SketchEntityId, String> {
        if !self.entities.contains_key(&start) || !self.entities.contains_key(&end) {
            return Err("Start or end point not found".to_string());
        }

        let line = SketchLine::new(start, end);
        let id = line.id;
        self.entities.insert(id, SketchEntity::Line(line));
        self.solved = false;
        Ok(id)
    }

    pub fn add_circle(&mut self, center: SketchEntityId, radius: StandardReal) -> Result<SketchEntityId, String> {
        if !self.entities.contains_key(&center) {
            return Err("Center point not found".to_string());
        }

        let circle = SketchCircle::new(center, radius);
        let id = circle.id;
        self.entities.insert(id, SketchEntity::Circle(circle));
        self.solved = false;
        Ok(id)
    }

    pub fn add_arc(&mut self, center: SketchEntityId, start: SketchEntityId, end: SketchEntityId, radius: StandardReal) -> Result<SketchEntityId, String> {
        if !self.entities.contains_key(&center) || !self.entities.contains_key(&start) || !self.entities.contains_key(&end) {
            return Err("Center, start, or end point not found".to_string());
        }

        let arc = SketchArc::new(center, start, end, radius);
        let id = arc.id;
        self.entities.insert(id, SketchEntity::Arc(arc));
        self.solved = false;
        Ok(id)
    }

    pub fn add_constraint(&mut self, constraint: SketchConstraint) -> Result<(), String> {
        for entity_id in &constraint.entities {
            if !self.entities.contains_key(entity_id) {
                return Err(format!("Entity {:?} not found", entity_id));
            }
        }

        self.constraints.push(constraint);
        self.solved = false;
        Ok(())
    }

    pub fn add_dimension(&mut self, dimension: SketchDimension) -> Result<(), String> {
        for entity_id in &dimension.entities {
            if !self.entities.contains_key(entity_id) {
                return Err(format!("Entity {:?} not found", entity_id));
            }
        }

        self.dimensions.push(dimension);
        self.solved = false;
        Ok(())
    }

    pub fn get_entity(&self, id: SketchEntityId) -> Option<&SketchEntity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: SketchEntityId) -> Option<&mut SketchEntity> {
        self.entities.get_mut(&id)
    }

    pub fn entities(&self) -> &HashMap<SketchEntityId, SketchEntity> {
        &self.entities
    }

    pub fn constraints(&self) -> &[SketchConstraint] {
        &self.constraints
    }

    pub fn dimensions(&self) -> &[SketchDimension] {
        &self.dimensions
    }

    pub fn solve(&mut self) -> Result<bool, String> {
        let mut changed = true;
        let mut iterations = 0;
        let max_iterations = 100;

        let constraints_to_apply: Vec<SketchConstraint> = self.constraints.iter().filter(|c| c.enabled).cloned().collect();

        while changed && iterations < max_iterations {
            changed = false;
            iterations += 1;

            for constraint in &constraints_to_apply {
                match self.apply_constraint(constraint) {
                    Ok(c) => changed = changed || c,
                    Err(e) => return Err(format!("Constraint solving failed: {}", e)),
                }
            }
        }

        self.solved = iterations < max_iterations;
        Ok(self.solved)
    }

    fn apply_constraint(&mut self, constraint: &SketchConstraint) -> Result<bool, String> {
        match constraint.constraint_type {
            ConstraintType::Horizontal => {
                if let Some(entity) = self.entities.get(&constraint.entities[0]).cloned() {
                    if let SketchEntity::Line(line) = entity {
                        if let (Some(SketchEntity::Point(start)), Some(SketchEntity::Point(end))) = (
                            self.entities.get(&line.start_point).cloned(),
                            self.entities.get(&line.end_point).cloned(),
                        ) {
                            let mut start_mut = start;
                            let mut end_mut = end;
                            let y_avg = (start_mut.position.y + end_mut.position.y) / 2.0;
                            start_mut.position.y = y_avg;
                            end_mut.position.y = y_avg;
                            self.entities.insert(line.start_point, SketchEntity::Point(start_mut));
                            self.entities.insert(line.end_point, SketchEntity::Point(end_mut));
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            ConstraintType::Vertical => {
                if let Some(entity) = self.entities.get(&constraint.entities[0]).cloned() {
                    if let SketchEntity::Line(line) = entity {
                        if let (Some(SketchEntity::Point(start)), Some(SketchEntity::Point(end))) = (
                            self.entities.get(&line.start_point).cloned(),
                            self.entities.get(&line.end_point).cloned(),
                        ) {
                            let mut start_mut = start;
                            let mut end_mut = end;
                            let x_avg = (start_mut.position.x + end_mut.position.x) / 2.0;
                            start_mut.position.x = x_avg;
                            end_mut.position.x = x_avg;
                            self.entities.insert(line.start_point, SketchEntity::Point(start_mut));
                            self.entities.insert(line.end_point, SketchEntity::Point(end_mut));
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            ConstraintType::Coincident => Ok(false),
            ConstraintType::Parallel => Ok(false),
            ConstraintType::Perpendicular => Ok(false),
            ConstraintType::Tangent => Ok(false),
            ConstraintType::Equal => Ok(false),
            ConstraintType::Symmetric => Ok(false),
            ConstraintType::Distance => Ok(false),
            ConstraintType::Angle => Ok(false),
            ConstraintType::Radius => Ok(false),
            ConstraintType::Diameter => Ok(false),
            ConstraintType::Fix => Ok(false),
            ConstraintType::Concentric => Ok(false),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.solved
    }

    pub fn to_shape(&self) -> Result<TopoDsShape, String> {
        Err("Sketch to shape conversion not implemented".to_string())
    }

    pub fn extrude(&self, _distance: StandardReal) -> Result<TopoDsShape, String> {
        Err("Extrude operation not implemented".to_string())
    }

    pub fn revolve(&self, _axis: SketchEntityId, _angle: StandardReal) -> Result<TopoDsShape, String> {
        Err("Revolve operation not implemented".to_string())
    }

    pub fn get_bounding_box(&self) -> Option<(Point, Point)> {
        let mut min_point: Option<Point> = None;
        let mut max_point: Option<Point> = None;

        for entity in self.entities.values() {
            if let SketchEntity::Point(point) = entity {
                min_point = Some(match min_point {
                    None => point.position,
                    Some(min) => Point::new(
                        min.x.min(point.position.x),
                        min.y.min(point.position.y),
                        min.z.min(point.position.z),
                    ),
                });

                max_point = Some(match max_point {
                    None => point.position,
                    Some(max) => Point::new(
                        max.x.max(point.position.x),
                        max.y.max(point.position.y),
                        max.z.max(point.position.z),
                    ),
                });
            }
        }

        match (min_point, max_point) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}

pub struct SketchBuilder {
    name: String,
    plane: Plane,
    entities: Vec<SketchEntity>,
    constraints: Vec<SketchConstraint>,
    dimensions: Vec<SketchDimension>,
}

impl SketchBuilder {
    pub fn new(name: String, plane: Plane) -> Self {
        Self {
            name,
            plane,
            entities: Vec::new(),
            constraints: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn add_entity(mut self, entity: SketchEntity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn add_constraint(mut self, constraint: SketchConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    pub fn add_dimension(mut self, dimension: SketchDimension) -> Self {
        self.dimensions.push(dimension);
        self
    }

    pub fn build(self) -> Sketch {
        let mut sketch = Sketch::new(self.name, self.plane);

        for entity in self.entities {
            sketch.entities.insert(entity.id(), entity);
        }

        sketch.constraints = self.constraints;
        sketch.dimensions = self.dimensions;

        sketch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_plane() -> Plane {
        Plane::from_point_normal(Point::origin(), crate::geometry::Direction::new(0.0, 0.0, 1.0))
    }

    #[test]
    fn test_sketch_creation() {
        let plane = create_test_plane();
        let sketch = Sketch::new("TestSketch".to_string(), plane);
        assert_eq!(sketch.name(), "TestSketch");
        assert!(sketch.entities().is_empty());
    }

    #[test]
    fn test_add_point() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        let id = sketch.add_point(Point::new(1.0, 2.0, 0.0));
        assert_eq!(sketch.entities().len(), 1);
        assert!(sketch.get_entity(id).is_some());
    }

    #[test]
    fn test_add_line() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        let p1 = sketch.add_point(Point::new(0.0, 0.0, 0.0));
        let p2 = sketch.add_point(Point::new(1.0, 1.0, 0.0));
        let line_id = sketch.add_line(p1, p2).unwrap();
        assert!(sketch.get_entity(line_id).is_some());
    }

    #[test]
    fn test_add_circle() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        let center = sketch.add_point(Point::new(0.0, 0.0, 0.0));
        let circle_id = sketch.add_circle(center, 5.0).unwrap();
        assert!(sketch.get_entity(circle_id).is_some());
    }

    #[test]
    fn test_add_constraint() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        let p1 = sketch.add_point(Point::new(0.0, 0.0, 0.0));
        let p2 = sketch.add_point(Point::new(1.0, 1.0, 0.0));
        let line_id = sketch.add_line(p1, p2).unwrap();

        let constraint = SketchConstraint::new(ConstraintType::Horizontal, vec![line_id]);
        sketch.add_constraint(constraint).unwrap();
        assert_eq!(sketch.constraints().len(), 1);
    }

    #[test]
    fn test_add_dimension() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        let p1 = sketch.add_point(Point::new(0.0, 0.0, 0.0));
        let p2 = sketch.add_point(Point::new(1.0, 0.0, 0.0));
        let line_id = sketch.add_line(p1, p2).unwrap();

        let dimension = SketchDimension::new(ConstraintType::Distance, vec![line_id], 1.0);
        sketch.add_dimension(dimension).unwrap();
        assert_eq!(sketch.dimensions().len(), 1);
    }

    #[test]
    fn test_get_bounding_box() {
        let plane = create_test_plane();
        let mut sketch = Sketch::new("TestSketch".to_string(), plane);
        sketch.add_point(Point::new(0.0, 0.0, 0.0));
        sketch.add_point(Point::new(1.0, 2.0, 0.0));
        sketch.add_point(Point::new(-1.0, -2.0, 0.0));

        let (min, max) = sketch.get_bounding_box().unwrap();
        assert_eq!(min.x, -1.0);
        assert_eq!(max.x, 1.0);
    }
}
