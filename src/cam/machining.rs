use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::topology::topods_shape::TopoDsShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachiningOperation {
    Roughing,
    SemiFinishing,
    Finishing,
    Drilling,
    Tapping,
    Boring,
    Reaming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    EndMill,
    BallMill,
    CornerRadiusMill,
    FaceMill,
    Drill,
    Tap,
    Reamer,
    BoringBar,
    ChamferMill,
    ThreadMill,
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub number: i32,
    pub name: String,
    pub tool_type: ToolType,
    pub diameter: StandardReal,
    pub length: StandardReal,
    pub corner_radius: StandardReal,
    pub flutes: i32,
    pub max_rpm: StandardReal,
    pub max_feed: StandardReal,
    pub material: String,
}

impl Tool {
    pub fn new(number: i32, name: String, tool_type: ToolType, diameter: StandardReal) -> Self {
        Self {
            number,
            name,
            tool_type,
            diameter,
            length: diameter * 3.0,
            corner_radius: 0.0,
            flutes: 2,
            max_rpm: 10000.0,
            max_feed: 5000.0,
            material: "HSS".to_string(),
        }
    }

    pub fn with_length(mut self, length: StandardReal) -> Self {
        self.length = length;
        self
    }

    pub fn with_corner_radius(mut self, radius: StandardReal) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_flutes(mut self, flutes: i32) -> Self {
        self.flutes = flutes;
        self
    }

    pub fn with_max_rpm(mut self, rpm: StandardReal) -> Self {
        self.max_rpm = rpm;
        self
    }

    pub fn with_max_feed(mut self, feed: StandardReal) -> Self {
        self.max_feed = feed;
        self
    }

    pub fn with_material(mut self, material: String) -> Self {
        self.material = material;
        self
    }

    pub fn calculate_feed_rate(&self, chip_load: StandardReal, rpm: StandardReal) -> StandardReal {
        chip_load * self.flutes as StandardReal * rpm
    }

    pub fn calculate_rpm(&self, cutting_speed: StandardReal) -> StandardReal {
        (cutting_speed * 1000.0) / (std::f64::consts::PI * self.diameter)
    }
}

#[derive(Debug, Clone)]
pub struct Workpiece {
    pub name: String,
    pub material: String,
    pub stock_shape: TopoDsShape,
    pub target_shape: Option<TopoDsShape>,
    pub bounding_box: (Point, Point),
}

impl Workpiece {
    pub fn new(name: String, material: String, stock_shape: TopoDsShape) -> Self {
        let bounding_box = stock_shape.bounding_box();
        Self {
            name,
            material,
            stock_shape,
            target_shape: None,
            bounding_box,
        }
    }

    pub fn with_target_shape(mut self, target: TopoDsShape) -> Self {
        self.target_shape = Some(target);
        self
    }

    pub fn stock_volume(&self) -> StandardReal {
        let (min, max) = self.bounding_box;
        let dx = max.x - min.x;
        let dy = max.y - min.y;
        let dz = max.z - min.z;
        dx * dy * dz
    }
}

#[derive(Debug, Clone)]
pub struct MachiningSetup {
    pub name: String,
    pub workpiece: Workpiece,
    pub tools: Vec<Tool>,
    pub operations: Vec<MachiningOperationData>,
    pub coordinate_system: CoordinateSystem,
    pub stock_to_leave: StandardReal,
}

#[derive(Debug, Clone)]
pub struct CoordinateSystem {
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub z_axis: Vector,
}

impl CoordinateSystem {
    pub fn new(origin: Point, x_axis: Vector, y_axis: Vector) -> Self {
        let z_axis = x_axis.cross(&y_axis);
        Self {
            origin,
            x_axis,
            y_axis,
            z_axis,
        }
    }

    pub fn transform_point(&self, point: &Point) -> Point {
        let local = Point::new(
            point.x - self.origin.x,
            point.y - self.origin.y,
            point.z - self.origin.z,
        );

        Point::new(
            local.x * self.x_axis.x + local.y * self.y_axis.x + local.z * self.z_axis.x,
            local.x * self.x_axis.y + local.y * self.y_axis.y + local.z * self.z_axis.y,
            local.x * self.x_axis.z + local.y * self.y_axis.z + local.z * self.z_axis.z,
        )
    }
}

#[derive(Debug, Clone)]
pub struct MachiningOperationData {
    pub name: String,
    pub operation_type: MachiningOperation,
    pub tool: Tool,
    pub spindle_speed: StandardReal,
    pub feed_rate: StandardReal,
    pub plunge_feed_rate: StandardReal,
    pub stepdown: StandardReal,
    pub stepover: StandardReal,
    pub stock_to_leave: StandardReal,
    pub tolerance: StandardReal,
}

impl MachiningOperationData {
    pub fn new(name: String, operation_type: MachiningOperation, tool: Tool) -> Self {
        Self {
            name,
            operation_type,
            tool,
            spindle_speed: 3000.0,
            feed_rate: 1000.0,
            plunge_feed_rate: 500.0,
            stepdown: 1.0,
            stepover: 0.5,
            stock_to_leave: 0.0,
            tolerance: 0.01,
        }
    }

    pub fn with_spindle_speed(mut self, rpm: StandardReal) -> Self {
        self.spindle_speed = rpm;
        self
    }

    pub fn with_feed_rate(mut self, feed: StandardReal) -> Self {
        self.feed_rate = feed;
        self
    }

    pub fn with_plunge_feed_rate(mut self, feed: StandardReal) -> Self {
        self.plunge_feed_rate = feed;
        self
    }

    pub fn with_stepdown(mut self, stepdown: StandardReal) -> Self {
        self.stepdown = stepdown;
        self
    }

    pub fn with_stepover(mut self, stepover: StandardReal) -> Self {
        self.stepover = stepover;
        self
    }

    pub fn with_stock_to_leave(mut self, stock: StandardReal) -> Self {
        self.stock_to_leave = stock;
        self
    }

    pub fn with_tolerance(mut self, tolerance: StandardReal) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn machining_time_estimate(&self, cutting_length: StandardReal) -> StandardReal {
        cutting_length / self.feed_rate
    }
}

pub struct CuttingParameters {
    pub material: String,
    pub tool_material: String,
    pub cutting_speed: StandardReal,
    pub feed_per_tooth: StandardReal,
    pub max_depth_of_cut: StandardReal,
}

impl CuttingParameters {
    pub fn new(material: String, tool_material: String) -> Self {
        Self {
            material,
            tool_material,
            cutting_speed: 100.0,
            feed_per_tooth: 0.1,
            max_depth_of_cut: 1.0,
        }
    }

    pub fn for_steel() -> Self {
        Self {
            material: "Steel".to_string(),
            tool_material: "Carbide".to_string(),
            cutting_speed: 80.0,
            feed_per_tooth: 0.08,
            max_depth_of_cut: 1.0,
        }
    }

    pub fn for_aluminum() -> Self {
        Self {
            material: "Aluminum".to_string(),
            tool_material: "Carbide".to_string(),
            cutting_speed: 300.0,
            feed_per_tooth: 0.15,
            max_depth_of_cut: 2.0,
        }
    }

    pub fn for_titanium() -> Self {
        Self {
            material: "Titanium".to_string(),
            tool_material: "Carbide".to_string(),
            cutting_speed: 40.0,
            feed_per_tooth: 0.05,
            max_depth_of_cut: 0.5,
        }
    }
}

pub struct MachiningCalculator;

impl MachiningCalculator {
    pub fn calculate_rpm(diameter: StandardReal, cutting_speed: StandardReal) -> StandardReal {
        (cutting_speed * 1000.0) / (std::f64::consts::PI * diameter)
    }

    pub fn calculate_feed_rate(rpm: StandardReal, feed_per_tooth: StandardReal, num_teeth: i32) -> StandardReal {
        rpm * feed_per_tooth * num_teeth as StandardReal
    }

    pub fn calculate_metal_removal_rate(feed_rate: StandardReal, depth_of_cut: StandardReal, width_of_cut: StandardReal) -> StandardReal {
        feed_rate * depth_of_cut * width_of_cut
    }

    pub fn calculate_power_requirement(mrr: StandardReal, unit_power: StandardReal) -> StandardReal {
        mrr * unit_power / 60.0
    }

    pub fn calculate_machining_time(length: StandardReal, feed_rate: StandardReal) -> StandardReal {
        if feed_rate > 0.0 {
            length / feed_rate
        } else {
            0.0
        }
    }

    pub fn estimate_tool_life(cutting_speed: StandardReal, tool_life_exponent: StandardReal, reference_speed: StandardReal, reference_life: StandardReal) -> StandardReal {
        reference_life * (reference_speed / cutting_speed).powf(tool_life_exponent)
    }
}

pub struct ToolLibrary {
    tools: Vec<Tool>,
}

impl ToolLibrary {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
        }
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn get_tool(&self, number: i32) -> Option<&Tool> {
        self.tools.iter().find(|t| t.number == number)
    }

    pub fn get_tools_by_type(&self, tool_type: ToolType) -> Vec<&Tool> {
        self.tools.iter().filter(|t| t.tool_type == tool_type).collect()
    }

    pub fn remove_tool(&mut self, number: i32) -> Option<Tool> {
        if let Some(pos) = self.tools.iter().position(|t| t.number == number) {
            Some(self.tools.remove(pos))
        } else {
            None
        }
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }

    pub fn create_standard_library() -> Self {
        let mut library = Self::new();

        library.add_tool(Tool::new(1, "3mm End Mill".to_string(), ToolType::EndMill, 3.0)
            .with_flutes(4)
            .with_material("Carbide".to_string()));

        library.add_tool(Tool::new(2, "6mm End Mill".to_string(), ToolType::EndMill, 6.0)
            .with_flutes(4)
            .with_material("Carbide".to_string()));

        library.add_tool(Tool::new(3, "10mm End Mill".to_string(), ToolType::EndMill, 10.0)
            .with_flutes(4)
            .with_material("Carbide".to_string()));

        library.add_tool(Tool::new(4, "6mm Ball Mill".to_string(), ToolType::BallMill, 6.0)
            .with_flutes(2)
            .with_material("Carbide".to_string()));

        library.add_tool(Tool::new(5, "3mm Drill".to_string(), ToolType::Drill, 3.0)
            .with_flutes(2)
            .with_material("HSS".to_string()));

        library.add_tool(Tool::new(6, "5mm Drill".to_string(), ToolType::Drill, 5.0)
            .with_flutes(2)
            .with_material("HSS".to_string()));

        library.add_tool(Tool::new(7, "M6 Tap".to_string(), ToolType::Tap, 6.0)
            .with_flutes(3)
            .with_material("HSS".to_string()));

        library.add_tool(Tool::new(8, "50mm Face Mill".to_string(), ToolType::FaceMill, 50.0)
            .with_flutes(5)
            .with_material("Carbide".to_string()));

        library
    }
}

impl Default for ToolLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = Tool::new(1, "Test Mill".to_string(), ToolType::EndMill, 10.0)
            .with_flutes(4)
            .with_material("Carbide".to_string());

        assert_eq!(tool.number, 1);
        assert_eq!(tool.diameter, 10.0);
        assert_eq!(tool.flutes, 4);
    }

    #[test]
    fn test_tool_calculations() {
        let tool = Tool::new(1, "Test".to_string(), ToolType::EndMill, 10.0)
            .with_flutes(4);

        let rpm = tool.calculate_rpm(100.0);
        assert!(rpm > 0.0);

        let feed = tool.calculate_feed_rate(0.1, 3000.0);
        assert!(feed > 0.0);
    }

    #[test]
    fn test_cutting_parameters() {
        let params = CuttingParameters::for_aluminum();
        assert_eq!(params.material, "Aluminum");
        assert!(params.cutting_speed > 200.0);
    }

    #[test]
    fn test_machining_calculator() {
        let rpm = MachiningCalculator::calculate_rpm(10.0, 100.0);
        assert!(rpm > 0.0);

        let feed = MachiningCalculator::calculate_feed_rate(3000.0, 0.1, 4);
        assert_eq!(feed, 1200.0);
    }

    #[test]
    fn test_tool_library() {
        let mut library = ToolLibrary::new();
        let tool = Tool::new(1, "Test".to_string(), ToolType::EndMill, 10.0);
        library.add_tool(tool);

        assert_eq!(library.tools().len(), 1);
        assert!(library.get_tool(1).is_some());
    }

    #[test]
    fn test_standard_tool_library() {
        let library = ToolLibrary::create_standard_library();
        assert!(!library.tools().is_empty());

        let end_mills = library.get_tools_by_type(ToolType::EndMill);
        assert!(!end_mills.is_empty());
    }

    #[test]
    fn test_coordinate_system() {
        let cs = CoordinateSystem::new(
            Point::origin(),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let point = Point::new(1.0, 2.0, 3.0);
        let transformed = cs.transform_point(&point);
        assert_eq!(transformed.x, 1.0);
        assert_eq!(transformed.y, 2.0);
    }
}
