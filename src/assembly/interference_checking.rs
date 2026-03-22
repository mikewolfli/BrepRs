use std::collections::HashMap;
use crate::geometry::{Point, Vector};
use crate::topology::topods_shape::TopoDsShape;
use crate::foundation::types::StandardReal;
use super::assembly_manager::{Assembly, ComponentId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterferenceType {
    Collision,
    Contact,
    Clearance,
    Containment,
    NoInterference,
}

#[derive(Debug, Clone)]
pub struct InterferenceResult {
    pub component1: ComponentId,
    pub component2: ComponentId,
    pub interference_type: InterferenceType,
    pub penetration_depth: StandardReal,
    pub contact_points: Vec<Point>,
    pub contact_normal: Option<Vector>,
    pub clearance: StandardReal,
}

impl InterferenceResult {
    pub fn new(component1: ComponentId, component2: ComponentId) -> Self {
        Self {
            component1,
            component2,
            interference_type: InterferenceType::NoInterference,
            penetration_depth: 0.0,
            contact_points: Vec::new(),
            contact_normal: None,
            clearance: 0.0,
        }
    }

    pub fn with_interference_type(mut self, interference_type: InterferenceType) -> Self {
        self.interference_type = interference_type;
        self
    }

    pub fn with_penetration_depth(mut self, depth: StandardReal) -> Self {
        self.penetration_depth = depth;
        self
    }

    pub fn with_contact_points(mut self, points: Vec<Point>) -> Self {
        self.contact_points = points;
        self
    }

    pub fn with_contact_normal(mut self, normal: Vector) -> Self {
        self.contact_normal = Some(normal);
        self
    }

    pub fn with_clearance(mut self, clearance: StandardReal) -> Self {
        self.clearance = clearance;
        self
    }
}

#[derive(Debug, Clone)]
pub struct InterferenceOptions {
    pub check_collision: bool,
    pub check_contact: bool,
    pub check_clearance: bool,
    pub clearance_tolerance: StandardReal,
    pub contact_tolerance: StandardReal,
    pub compute_contact_points: bool,
    pub max_contact_points: usize,
}

impl Default for InterferenceOptions {
    fn default() -> Self {
        Self {
            check_collision: true,
            check_contact: true,
            check_clearance: true,
            clearance_tolerance: 0.1,
            contact_tolerance: 1e-6,
            compute_contact_points: true,
            max_contact_points: 100,
        }
    }
}

pub struct InterferenceChecker {
    options: InterferenceOptions,
    results: Vec<InterferenceResult>,
}

impl InterferenceChecker {
    pub fn new() -> Self {
        Self {
            options: InterferenceOptions::default(),
            results: Vec::new(),
        }
    }

    pub fn with_options(options: InterferenceOptions) -> Self {
        Self {
            options,
            results: Vec::new(),
        }
    }

    pub fn options(&self) -> &InterferenceOptions {
        &self.options
    }

    pub fn set_options(&mut self, options: InterferenceOptions) {
        self.options = options;
    }

    pub fn check_assembly(&mut self, assembly: &Assembly) -> Vec<InterferenceResult> {
        self.results.clear();

        let components: Vec<(ComponentId, TopoDsShape)> = assembly
            .components()
            .iter()
            .filter_map(|(id, component)| {
                component.shape().cloned().map(|shape| (*id, shape))
            })
            .collect();

        for i in 0..components.len() {
            for j in (i + 1)..components.len() {
                let (id1, shape1) = &components[i];
                let (id2, shape2) = &components[j];

                if let Some(result) = self.check_pair(*id1, shape1, *id2, shape2) {
                    self.results.push(result);
                }
            }
        }

        self.results.clone()
    }

    pub fn check_pair(
        &self,
        component1: ComponentId,
        shape1: &TopoDsShape,
        component2: ComponentId,
        shape2: &TopoDsShape,
    ) -> Option<InterferenceResult> {
        let (min1, max1) = shape1.bounding_box();
        let (min2, max2) = shape2.bounding_box();

        if !self.bounding_boxes_overlap(&min1, &max1, &min2, &max2) {
            if self.options.check_clearance {
                let clearance = self.compute_clearance(&min1, &max1, &min2, &max2);
                if clearance < self.options.clearance_tolerance {
                    return Some(
                        InterferenceResult::new(component1, component2)
                            .with_interference_type(InterferenceType::Clearance)
                            .with_clearance(clearance),
                    );
                }
            }
            return None;
        }

        let mut result = InterferenceResult::new(component1, component2);

        if self.bounding_boxes_contact(&min1, &max1, &min2, &max2) {
            result.interference_type = InterferenceType::Contact;
            result.clearance = 0.0;

            if self.options.compute_contact_points {
                result.contact_points = self.compute_contact_points(&min1, &max1, &min2, &max2);
            }
        } else {
            let penetration = self.compute_penetration_depth(&min1, &max1, &min2, &max2);
            result.interference_type = InterferenceType::Collision;
            result.penetration_depth = penetration;
            result.clearance = -penetration;

            if self.options.compute_contact_points {
                result.contact_points = self.compute_contact_points(&min1, &max1, &min2, &max2);
            }

            result.contact_normal = self.compute_contact_normal(&min1, &max1, &min2, &max2);
        }

        Some(result)
    }

    fn bounding_boxes_overlap(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> bool {
        min1.x <= max2.x && max1.x >= min2.x
            && min1.y <= max2.y && max1.y >= min2.y
            && min1.z <= max2.z && max1.z >= min2.z
    }

    fn bounding_boxes_contact(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> bool {
        let tol = self.options.contact_tolerance;

        (min1.x - max2.x).abs() < tol
            || (max1.x - min2.x).abs() < tol
            || (min1.y - max2.y).abs() < tol
            || (max1.y - min2.y).abs() < tol
            || (min1.z - max2.z).abs() < tol
            || (max1.z - min2.z).abs() < tol
    }

    fn compute_clearance(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> StandardReal {
        let mut clearance = 0.0;

        if max1.x < min2.x {
            clearance = min2.x - max1.x;
        } else if max2.x < min1.x {
            clearance = min1.x - max2.x;
        }

        if max1.y < min2.y {
            clearance = clearance.max(min2.y - max1.y);
        } else if max2.y < min1.y {
            clearance = clearance.max(min1.y - max2.y);
        }

        if max1.z < min2.z {
            clearance = clearance.max(min2.z - max1.z);
        } else if max2.z < min1.z {
            clearance = clearance.max(min1.z - max2.z);
        }

        clearance
    }

    fn compute_penetration_depth(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> StandardReal {
        let overlap_x = (max1.x.min(max2.x) - min1.x.max(min2.x)).max(0.0);
        let overlap_y = (max1.y.min(max2.y) - min1.y.max(min2.y)).max(0.0);
        let overlap_z = (max1.z.min(max2.z) - min1.z.max(min2.z)).max(0.0);

        overlap_x.min(overlap_y).min(overlap_z)
    }

    fn compute_contact_points(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> Vec<Point> {
        let mut points = Vec::new();

        let overlap_min = Point::new(
            min1.x.max(min2.x),
            min1.y.max(min2.y),
            min1.z.max(min2.z),
        );
        let overlap_max = Point::new(
            max1.x.min(max2.x),
            max1.y.min(max2.y),
            max1.z.min(max2.z),
        );

        if overlap_min.x <= overlap_max.x
            && overlap_min.y <= overlap_max.y
            && overlap_min.z <= overlap_max.z
        {
            points.push(Point::new(
                (overlap_min.x + overlap_max.x) / 2.0,
                (overlap_min.y + overlap_max.y) / 2.0,
                (overlap_min.z + overlap_max.z) / 2.0,
            ));

            if self.options.max_contact_points > 1 {
                points.push(Point::new(overlap_min.x, overlap_min.y, overlap_min.z));
                points.push(Point::new(overlap_max.x, overlap_max.y, overlap_max.z));
                points.push(Point::new(overlap_min.x, overlap_max.y, overlap_min.z));
                points.push(Point::new(overlap_max.x, overlap_min.y, overlap_max.z));
            }
        }

        points.truncate(self.options.max_contact_points);
        points
    }

    fn compute_contact_normal(&self, min1: &Point, max1: &Point, min2: &Point, max2: &Point) -> Option<Vector> {
        let overlap_x = (max1.x.min(max2.x) - min1.x.max(min2.x)).max(0.0);
        let overlap_y = (max1.y.min(max2.y) - min1.y.max(min2.y)).max(0.0);
        let overlap_z = (max1.z.min(max2.z) - min1.z.max(min2.z)).max(0.0);

        if overlap_x <= overlap_y && overlap_x <= overlap_z {
            if min1.x < min2.x {
                Some(Vector::new(1.0, 0.0, 0.0))
            } else {
                Some(Vector::new(-1.0, 0.0, 0.0))
            }
        } else if overlap_y <= overlap_x && overlap_y <= overlap_z {
            if min1.y < min2.y {
                Some(Vector::new(0.0, 1.0, 0.0))
            } else {
                Some(Vector::new(0.0, -1.0, 0.0))
            }
        } else {
            if min1.z < min2.z {
                Some(Vector::new(0.0, 0.0, 1.0))
            } else {
                Some(Vector::new(0.0, 0.0, -1.0))
            }
        }
    }

    pub fn results(&self) -> &[InterferenceResult] {
        &self.results
    }

    pub fn has_collisions(&self) -> bool {
        self.results
            .iter()
            .any(|r| r.interference_type == InterferenceType::Collision)
    }

    pub fn has_contacts(&self) -> bool {
        self.results
            .iter()
            .any(|r| r.interference_type == InterferenceType::Contact)
    }

    pub fn get_collisions(&self) -> Vec<&InterferenceResult> {
        self.results
            .iter()
            .filter(|r| r.interference_type == InterferenceType::Collision)
            .collect()
    }

    pub fn get_contacts(&self) -> Vec<&InterferenceResult> {
        self.results
            .iter()
            .filter(|r| r.interference_type == InterferenceType::Contact)
            .collect()
    }

    pub fn get_clearances(&self) -> Vec<&InterferenceResult> {
        self.results
            .iter()
            .filter(|r| r.interference_type == InterferenceType::Clearance)
            .collect()
    }

    pub fn get_interferences_for_component(&self, component_id: ComponentId) -> Vec<&InterferenceResult> {
        self.results
            .iter()
            .filter(|r| r.component1 == component_id || r.component2 == component_id)
            .collect()
    }

    pub fn get_interference_between(
        &self,
        component1: ComponentId,
        component2: ComponentId,
    ) -> Option<&InterferenceResult> {
        self.results.iter().find(|r| {
            (r.component1 == component1 && r.component2 == component2)
                || (r.component1 == component2 && r.component2 == component1)
        })
    }

    pub fn generate_report(&self) -> InterferenceReport {
        let mut report = InterferenceReport::new();

        for result in &self.results {
            match result.interference_type {
                InterferenceType::Collision => {
                    report.collision_count += 1;
                    report.total_penetration += result.penetration_depth;
                }
                InterferenceType::Contact => {
                    report.contact_count += 1;
                }
                InterferenceType::Clearance => {
                    report.clearance_count += 1;
                    if result.clearance < report.min_clearance || report.min_clearance == 0.0 {
                        report.min_clearance = result.clearance;
                    }
                }
                InterferenceType::Containment => {
                    report.containment_count += 1;
                }
                InterferenceType::NoInterference => {}
            }
        }

        report.total_interferences = self.results.len();
        report
    }
}

impl Default for InterferenceChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterferenceReport {
    pub total_interferences: usize,
    pub collision_count: usize,
    pub contact_count: usize,
    pub clearance_count: usize,
    pub containment_count: usize,
    pub total_penetration: StandardReal,
    pub min_clearance: StandardReal,
}

impl InterferenceReport {
    pub fn new() -> Self {
        Self {
            total_interferences: 0,
            collision_count: 0,
            contact_count: 0,
            clearance_count: 0,
            containment_count: 0,
            total_penetration: 0.0,
            min_clearance: 0.0,
        }
    }

    pub fn has_issues(&self) -> bool {
        self.collision_count > 0
    }

    pub fn summary(&self) -> String {
        format!(
            "Interference Report:\n\
             - Total checks: {}\n\
             - Collisions: {}\n\
             - Contacts: {}\n\
             - Clearances: {}\n\
             - Containments: {}\n\
             - Total penetration: {:.4}\n\
             - Minimum clearance: {:.4}",
            self.total_interferences,
            self.collision_count,
            self.contact_count,
            self.clearance_count,
            self.containment_count,
            self.total_penetration,
            self.min_clearance
        )
    }
}

pub struct ClearanceAnalyzer {
    min_clearance: StandardReal,
    clearance_results: HashMap<(ComponentId, ComponentId), StandardReal>,
}

impl ClearanceAnalyzer {
    pub fn new() -> Self {
        Self {
            min_clearance: StandardReal::MAX,
            clearance_results: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, assembly: &Assembly) {
        self.clearance_results.clear();
        self.min_clearance = StandardReal::MAX;

        let components: Vec<(ComponentId, TopoDsShape)> = assembly
            .components()
            .iter()
            .filter_map(|(id, component)| {
                component.shape().cloned().map(|shape| (*id, shape))
            })
            .collect();

        for i in 0..components.len() {
            for j in (i + 1)..components.len() {
                let (id1, shape1) = &components[i];
                let (id2, shape2) = &components[j];

                let clearance = self.compute_clearance_between(shape1, shape2);

                if clearance < self.min_clearance {
                    self.min_clearance = clearance;
                }

                let key = if id1.0 < id2.0 { (*id1, *id2) } else { (*id2, *id1) };
                self.clearance_results.insert(key, clearance);
            }
        }
    }

    fn compute_clearance_between(&self, shape1: &TopoDsShape, shape2: &TopoDsShape) -> StandardReal {
        let (min1, max1) = shape1.bounding_box();
        let (min2, max2) = shape2.bounding_box();

        let mut clearance = 0.0;

        if max1.x < min2.x {
            clearance = min2.x - max1.x;
        } else if max2.x < min1.x {
            clearance = min1.x - max2.x;
        }

        if max1.y < min2.y {
            clearance = clearance.max(min2.y - max1.y);
        } else if max2.y < min1.y {
            clearance = clearance.max(min1.y - max2.y);
        }

        if max1.z < min2.z {
            clearance = clearance.max(min2.z - max1.z);
        } else if max2.z < min1.z {
            clearance = clearance.max(min1.z - max2.z);
        }

        clearance
    }

    pub fn min_clearance(&self) -> StandardReal {
        self.min_clearance
    }

    pub fn get_clearance(&self, component1: ComponentId, component2: ComponentId) -> Option<StandardReal> {
        let key = if component1.0 < component2.0 {
            (component1, component2)
        } else {
            (component2, component1)
        };
        self.clearance_results.get(&key).copied()
    }

    pub fn clearance_results(&self) -> &HashMap<(ComponentId, ComponentId), StandardReal> {
        &self.clearance_results
    }

    pub fn get_components_below_threshold(&self, threshold: StandardReal) -> Vec<(ComponentId, ComponentId, StandardReal)> {
        self.clearance_results
            .iter()
            .filter(|(_, &clearance)| clearance < threshold)
            .map(|(&(c1, c2), &clearance)| (c1, c2, clearance))
            .collect()
    }
}

impl Default for ClearanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_result_creation() {
        let comp1 = ComponentId::new();
        let comp2 = ComponentId::new();

        let result = InterferenceResult::new(comp1, comp2)
            .with_interference_type(InterferenceType::Collision)
            .with_penetration_depth(5.0);

        assert_eq!(result.interference_type, InterferenceType::Collision);
        assert_eq!(result.penetration_depth, 5.0);
    }

    #[test]
    fn test_interference_options_default() {
        let options = InterferenceOptions::default();
        assert!(options.check_collision);
        assert!(options.check_contact);
        assert_eq!(options.contact_tolerance, 1e-6);
    }

    #[test]
    fn test_interference_checker_creation() {
        let checker = InterferenceChecker::new();
        assert!(checker.results().is_empty());
    }

    #[test]
    fn test_interference_report() {
        let mut report = InterferenceReport::new();
        report.collision_count = 2;
        report.contact_count = 3;
        report.total_interferences = 5;

        assert!(report.has_issues());
        assert_eq!(report.collision_count, 2);
    }

    #[test]
    fn test_clearance_analyzer() {
        let analyzer = ClearanceAnalyzer::new();
        assert_eq!(analyzer.min_clearance(), StandardReal::MAX);
    }
}
