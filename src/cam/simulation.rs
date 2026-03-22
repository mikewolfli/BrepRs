use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::topology::topods_shape::TopoDsShape;
use super::toolpath::Toolpath;
use super::machining::Tool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    ToolHolder,
    ToolShaft,
    ToolFlute,
    Fixture,
    Workpiece,
    None,
}

#[derive(Debug, Clone)]
pub struct Collision {
    pub collision_type: CollisionType,
    pub position: Point,
    pub depth: StandardReal,
    pub time: StandardReal,
}

impl Collision {
    pub fn new(collision_type: CollisionType, position: Point, depth: StandardReal, time: StandardReal) -> Self {
        Self {
            collision_type,
            position,
            depth,
            time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationStep {
    pub time: StandardReal,
    pub position: Point,
    pub tool_orientation: Vector,
    pub spindle_speed: StandardReal,
    pub feed_rate: StandardReal,
    pub material_removal_rate: StandardReal,
    pub chip_load: StandardReal,
    pub is_cutting: bool,
}

impl SimulationStep {
    pub fn new(time: StandardReal, position: Point) -> Self {
        Self {
            time,
            position,
            tool_orientation: Vector::new(0.0, 0.0, -1.0),
            spindle_speed: 0.0,
            feed_rate: 0.0,
            material_removal_rate: 0.0,
            chip_load: 0.0,
            is_cutting: false,
        }
    }

    pub fn with_orientation(mut self, orientation: Vector) -> Self {
        self.tool_orientation = orientation;
        self
    }

    pub fn with_spindle_speed(mut self, rpm: StandardReal) -> Self {
        self.spindle_speed = rpm;
        self
    }

    pub fn with_feed_rate(mut self, feed: StandardReal) -> Self {
        self.feed_rate = feed;
        self
    }

    pub fn with_material_removal_rate(mut self, mrr: StandardReal) -> Self {
        self.material_removal_rate = mrr;
        self
    }

    pub fn cutting(mut self) -> Self {
        self.is_cutting = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub steps: Vec<SimulationStep>,
    pub collisions: Vec<Collision>,
    pub total_time: StandardReal,
    pub total_distance: StandardReal,
    pub material_removed: StandardReal,
    pub min_spindle_speed: StandardReal,
    pub max_spindle_speed: StandardReal,
    pub min_feed_rate: StandardReal,
    pub max_feed_rate: StandardReal,
}

impl SimulationResult {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            collisions: Vec::new(),
            total_time: 0.0,
            total_distance: 0.0,
            material_removed: 0.0,
            min_spindle_speed: StandardReal::MAX,
            max_spindle_speed: 0.0,
            min_feed_rate: StandardReal::MAX,
            max_feed_rate: 0.0,
        }
    }

    pub fn add_step(&mut self, step: SimulationStep) {
        self.total_time = step.time;
        
        if step.spindle_speed > 0.0 {
            self.min_spindle_speed = self.min_spindle_speed.min(step.spindle_speed);
            self.max_spindle_speed = self.max_spindle_speed.max(step.spindle_speed);
        }
        
        if step.feed_rate > 0.0 {
            self.min_feed_rate = self.min_feed_rate.min(step.feed_rate);
            self.max_feed_rate = self.max_feed_rate.max(step.feed_rate);
        }
        
        self.steps.push(step);
    }

    pub fn add_collision(&mut self, collision: Collision) {
        self.collisions.push(collision);
    }

    pub fn has_collisions(&self) -> bool {
        !self.collisions.is_empty()
    }

    pub fn average_feed_rate(&self) -> StandardReal {
        if self.steps.is_empty() {
            return 0.0;
        }
        let sum: StandardReal = self.steps.iter().map(|s| s.feed_rate).sum();
        sum / self.steps.len() as StandardReal
    }

    pub fn average_spindle_speed(&self) -> StandardReal {
        if self.steps.is_empty() {
            return 0.0;
        }
        let sum: StandardReal = self.steps.iter().map(|s| s.spindle_speed).sum();
        sum / self.steps.len() as StandardReal
    }
}

impl Default for SimulationResult {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MachiningSimulator {
    time_step: StandardReal,
    collision_tolerance: StandardReal,
}

impl MachiningSimulator {
    pub fn new() -> Self {
        Self {
            time_step: 0.001,
            collision_tolerance: 0.01,
        }
    }

    pub fn with_time_step(mut self, time_step: StandardReal) -> Self {
        self.time_step = time_step;
        self
    }

    pub fn with_collision_tolerance(mut self, tolerance: StandardReal) -> Self {
        self.collision_tolerance = tolerance;
        self
    }

    pub fn simulate(&self, toolpath: &Toolpath, tool: &Tool, _workpiece: &TopoDsShape) -> SimulationResult {
        let mut result = SimulationResult::new();
        let mut current_time = 0.0;
        let mut prev_position: Option<Point> = None;

        for point in &toolpath.points {
            let step = SimulationStep::new(current_time, point.position)
                .with_spindle_speed(point.spindle_speed.unwrap_or(tool.calculate_rpm(100.0)))
                .with_feed_rate(point.feed_rate.unwrap_or(1000.0));

            let step = if let Some(orientation) = point.tool_orientation {
                step.with_orientation(orientation)
            } else {
                step
            };

            let step = if !point.is_rapid {
                step.cutting()
            } else {
                step
            };

            if let Some(prev) = prev_position {
                let dist = ((point.position.x - prev.x).powi(2)
                    + (point.position.y - prev.y).powi(2)
                    + (point.position.z - prev.z).powi(2)).sqrt();
                result.total_distance += dist;
            }

            result.add_step(step);
            prev_position = Some(point.position);

            let feed = point.feed_rate.unwrap_or(1000.0);
            if feed > 0.0 {
                current_time += self.time_step;
            }
        }

        result
    }

    pub fn check_collisions(&self, toolpath: &Toolpath, tool: &Tool, fixtures: &[TopoDsShape]) -> Vec<Collision> {
        let mut collisions = Vec::new();
        let mut current_time = 0.0;

        for point in &toolpath.points {
            for fixture in fixtures {
                if self.check_tool_fixture_collision(&point.position, tool, fixture) {
                    collisions.push(Collision::new(
                        CollisionType::Fixture,
                        point.position,
                        0.0,
                        current_time,
                    ));
                }
            }

            let feed = point.feed_rate.unwrap_or(1000.0);
            if feed > 0.0 && !point.is_rapid {
                current_time += self.time_step;
            }
        }

        collisions
    }

    fn check_tool_fixture_collision(&self, position: &Point, _tool: &Tool, fixture: &TopoDsShape) -> bool {
        let (min, max) = fixture.bounding_box();
        
        position.x >= min.x && position.x <= max.x
            && position.y >= min.y && position.y <= max.y
            && position.z >= min.z && position.z <= max.z
    }

    pub fn estimate_machining_time(&self, toolpath: &Toolpath) -> StandardReal {
        toolpath.machining_time()
    }

    pub fn calculate_material_removal(&self, initial_stock: &TopoDsShape, final_part: &TopoDsShape) -> StandardReal {
        let (stock_min, stock_max) = initial_stock.bounding_box();
        let stock_volume = (stock_max.x - stock_min.x) 
            * (stock_max.y - stock_min.y) 
            * (stock_max.z - stock_min.z);

        let (part_min, part_max) = final_part.bounding_box();
        let part_volume = (part_max.x - part_min.x)
            * (part_max.y - part_min.y)
            * (part_max.z - part_min.z);

        stock_volume - part_volume
    }
}

impl Default for MachiningSimulator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ToolpathOptimizer;

impl ToolpathOptimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn optimize_feed_rates(&self, toolpath: &mut Toolpath, tool: &Tool, max_mrr: StandardReal) {
        for point in &mut toolpath.points {
            if let Some(mrr) = point.tool_orientation.as_ref().map(|_| max_mrr) {
                let optimal_feed = self.calculate_optimal_feed(tool, mrr);
                point.feed_rate = Some(optimal_feed.min(point.feed_rate.unwrap_or(optimal_feed)));
            }
        }
    }

    fn calculate_optimal_feed(&self, tool: &Tool, target_mrr: StandardReal) -> StandardReal {
        let cross_section = std::f64::consts::PI * (tool.diameter / 2.0).powi(2);
        target_mrr / cross_section
    }

    pub fn optimize_spindle_speed(&self, toolpath: &mut Toolpath, tool: &Tool, material: &str) {
        let cutting_speed = match material {
            "Aluminum" => 300.0,
            "Steel" => 80.0,
            "Titanium" => 40.0,
            _ => 100.0,
        };

        let optimal_rpm = tool.calculate_rpm(cutting_speed);

        for point in &mut toolpath.points {
            point.spindle_speed = Some(optimal_rpm.min(tool.max_rpm));
        }
    }

    pub fn reduce_air_moves(&self, toolpath: &mut Toolpath) {
        if toolpath.points.len() < 2 {
            return;
        }

        let mut optimized = vec![toolpath.points[0].clone()];

        for i in 1..toolpath.points.len() {
            let prev = &optimized[optimized.len() - 1];
            let curr = &toolpath.points[i];

            if prev.is_rapid && curr.is_rapid {
                let dist = ((curr.position.x - prev.position.x).powi(2)
                    + (curr.position.y - prev.position.y).powi(2)
                    + (curr.position.z - prev.position.z).powi(2)).sqrt();

                if dist < 0.001 {
                    optimized.pop();
                }
            }

            optimized.push(curr.clone());
        }

        toolpath.points = optimized;
    }
}

impl Default for ToolpathOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VerificationReport {
    pub toolpath_name: String,
    pub simulation_result: SimulationResult,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub recommendations: Vec<String>,
}

impl VerificationReport {
    pub fn new(toolpath_name: String, simulation_result: SimulationResult) -> Self {
        Self {
            toolpath_name,
            simulation_result,
            warnings: Vec::new(),
            errors: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn generate_summary(&self) -> String {
        let mut summary = format!("Verification Report for: {}\n", self.toolpath_name);
        summary.push_str(&format!("Total Time: {:.2} minutes\n", self.simulation_result.total_time));
        summary.push_str(&format!("Total Distance: {:.2} mm\n", self.simulation_result.total_distance));
        summary.push_str(&format!("Collisions: {}\n", self.simulation_result.collisions.len()));
        summary.push_str(&format!("Warnings: {}\n", self.warnings.len()));
        summary.push_str(&format!("Errors: {}\n", self.errors.len()));
        summary.push_str(&format!("Recommendations: {}\n", self.recommendations.len()));
        summary
    }
}

pub struct ToolpathVerifier {
    max_spindle_speed: StandardReal,
    max_feed_rate: StandardReal,
    max_depth_of_cut: StandardReal,
}

impl ToolpathVerifier {
    pub fn new() -> Self {
        Self {
            max_spindle_speed: 30000.0,
            max_feed_rate: 10000.0,
            max_depth_of_cut: 10.0,
        }
    }

    pub fn verify(&self, toolpath: &Toolpath, tool: &Tool) -> VerificationReport {
        let _simulator = MachiningSimulator::new();
        let result = SimulationResult::new();
        let mut report = VerificationReport::new(toolpath.name.clone(), result);

        if toolpath.spindle_speed > self.max_spindle_speed {
            report.add_error(format!(
                "Spindle speed {} exceeds maximum {}",
                toolpath.spindle_speed, self.max_spindle_speed
            ));
        }

        if toolpath.spindle_speed > tool.max_rpm {
            report.add_warning(format!(
                "Spindle speed {} exceeds tool maximum {}",
                toolpath.spindle_speed, tool.max_rpm
            ));
        }

        if toolpath.feed_rate > self.max_feed_rate {
            report.add_error(format!(
                "Feed rate {} exceeds maximum {}",
                toolpath.feed_rate, self.max_feed_rate
            ));
        }

        if toolpath.feed_rate > tool.max_feed {
            report.add_warning(format!(
                "Feed rate {} exceeds tool maximum {}",
                toolpath.feed_rate, tool.max_feed
            ));
        }

        if toolpath.points.len() < 2 {
            report.add_error("Toolpath has insufficient points".to_string());
        }

        if toolpath.stepdown > self.max_depth_of_cut {
            report.add_warning(format!(
                "Stepdown {} may be excessive",
                toolpath.stepdown
            ));
        }

        if report.simulation_result.total_time > 60.0 {
            report.add_recommendation("Consider optimizing toolpath for shorter machining time".to_string());
        }

        report
    }
}

impl Default for ToolpathVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::toolpath::{Toolpath, ToolpathType, ToolpathPoint};
    use super::super::machining::{Tool, ToolType};

    #[test]
    fn test_simulation_step_creation() {
        let step = SimulationStep::new(0.0, Point::new(1.0, 2.0, 3.0))
            .with_feed_rate(1000.0)
            .cutting();

        assert_eq!(step.time, 0.0);
        assert_eq!(step.position.x, 1.0);
        assert!(step.is_cutting);
    }

    #[test]
    fn test_simulation_result() {
        let mut result = SimulationResult::new();
        let step = SimulationStep::new(0.0, Point::origin())
            .with_feed_rate(1000.0)
            .with_spindle_speed(3000.0);
        
        result.add_step(step);
        
        assert_eq!(result.steps.len(), 1);
        assert_eq!(result.max_feed_rate, 1000.0);
        assert_eq!(result.max_spindle_speed, 3000.0);
    }

    #[test]
    fn test_machining_simulator() {
        let simulator = MachiningSimulator::new();
        let tool = Tool::new(1, "Test".to_string(), ToolType::EndMill, 10.0);
        let mut toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour);
        
        toolpath.add_point(ToolpathPoint::new(Point::new(0.0, 0.0, 10.0)).rapid());
        toolpath.add_point(ToolpathPoint::new(Point::new(10.0, 0.0, 0.0)));
        toolpath.add_point(ToolpathPoint::new(Point::new(10.0, 10.0, 0.0)));

        let result = simulator.simulate(&toolpath, &tool, &TopoDsShape::new(crate::topology::ShapeType::Compound));
        
        assert!(!result.steps.is_empty());
    }

    #[test]
    fn test_verification_report() {
        let result = SimulationResult::new();
        let mut report = VerificationReport::new("Test".to_string(), result);
        
        report.add_warning("Test warning".to_string());
        report.add_recommendation("Test recommendation".to_string());
        
        assert!(report.is_valid());
        assert_eq!(report.warnings.len(), 1);
    }

    #[test]
    fn test_toolpath_verifier() {
        let verifier = ToolpathVerifier::new();
        let tool = Tool::new(1, "Test".to_string(), ToolType::EndMill, 10.0);
        let toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour)
            .with_spindle_speed(1000.0)
            .with_feed_rate(500.0);

        let report = verifier.verify(&toolpath, &tool);
        
        assert!(report.is_valid());
    }

    #[test]
    fn test_toolpath_optimizer() {
        let optimizer = ToolpathOptimizer::new();
        let tool = Tool::new(1, "Test".to_string(), ToolType::EndMill, 10.0);
        let mut toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour);
        
        toolpath.add_point(ToolpathPoint::new(Point::new(0.0, 0.0, 10.0)).rapid());
        toolpath.add_point(ToolpathPoint::new(Point::new(10.0, 0.0, 0.0)));

        optimizer.optimize_spindle_speed(&mut toolpath, &tool, "Aluminum");
        
        assert!(toolpath.points.iter().any(|p| p.spindle_speed.is_some()));
    }
}
