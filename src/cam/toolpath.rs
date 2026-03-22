//! Toolpath Generation Module
//!
//! This module provides functionality for generating and managing CNC toolpaths.
//! It includes types for representing toolpath points, segments, and complete toolpaths.

use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::topology::topods_shape::TopoDsShape;

/// Type of toolpath operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolpathType {
    /// Contour following operation
    Contour,
    /// Pocket milling operation
    Pocket,
    /// Drilling operation
    Drill,
    /// Face milling operation
    Face,
    /// Engraving operation
    Engrave,
    /// Adaptive clearing operation
    Adaptive,
    /// Spiral toolpath pattern
    Spiral,
    /// Zigzag (back-and-forth) pattern
    Zigzag,
    /// Offset contour pattern
    Offset,
    /// Parallel passes pattern
    Parallel,
}

/// Direction of cut for machining operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutDirection {
    /// Climb milling (down-cut)
    Climb,
    /// Conventional milling (up-cut)
    Conventional,
    /// Both directions allowed
    Both,
}

/// A single point in a toolpath
#[derive(Debug, Clone)]
pub struct ToolpathPoint {
    /// 3D position of the tool
    pub position: Point,
    /// Feed rate at this point (mm/min)
    pub feed_rate: Option<StandardReal>,
    /// Spindle speed at this point (RPM)
    pub spindle_speed: Option<StandardReal>,
    /// Tool orientation vector (for 5-axis machining)
    pub tool_orientation: Option<Vector>,
    /// Whether this is a rapid (non-cutting) move
    pub is_rapid: bool,
}

impl ToolpathPoint {
    /// Create a new toolpath point at the given position
    pub fn new(position: Point) -> Self {
        Self {
            position,
            feed_rate: None,
            spindle_speed: None,
            tool_orientation: None,
            is_rapid: false,
        }
    }

    /// Set the feed rate for this point
    pub fn with_feed_rate(mut self, feed_rate: StandardReal) -> Self {
        self.feed_rate = Some(feed_rate);
        self
    }

    /// Set the spindle speed for this point
    pub fn with_spindle_speed(mut self, spindle_speed: StandardReal) -> Self {
        self.spindle_speed = Some(spindle_speed);
        self
    }

    /// Set the tool orientation for this point
    pub fn with_orientation(mut self, orientation: Vector) -> Self {
        self.tool_orientation = Some(orientation);
        self
    }

    /// Mark this point as a rapid move
    pub fn rapid(mut self) -> Self {
        self.is_rapid = true;
        self
    }
}

/// A segment connecting two toolpath points
#[derive(Debug, Clone)]
pub struct ToolpathSegment {
    /// Starting point of the segment
    pub start: ToolpathPoint,
    /// Ending point of the segment
    pub end: ToolpathPoint,
    /// Type of motion for this segment
    pub segment_type: SegmentType,
}

/// Type of motion segment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    /// Linear (straight line) motion
    Linear,
    /// Clockwise arc motion
    ArcCW,
    /// Counter-clockwise arc motion
    ArcCCW,
    /// Helical motion
    Helix,
}

/// Complete toolpath containing all points and segments
#[derive(Debug, Clone)]
pub struct Toolpath {
    /// Name identifier for this toolpath
    pub name: String,
    /// Type of toolpath operation
    pub toolpath_type: ToolpathType,
    /// List of toolpath points
    pub points: Vec<ToolpathPoint>,
    /// List of toolpath segments
    pub segments: Vec<ToolpathSegment>,
    /// Stock material to leave (for roughing)
    pub stock_to_leave: StandardReal,
    /// Stepover distance between passes
    pub stepover: StandardReal,
    /// Stepdown distance between levels
    pub stepdown: StandardReal,
    /// Safe height for rapid moves
    pub safe_height: StandardReal,
    /// Retract height for tool changes
    pub retract_height: StandardReal,
    /// Default feed rate (mm/min)
    pub feed_rate: StandardReal,
    /// Plunge feed rate (mm/min)
    pub plunge_feed_rate: StandardReal,
    /// Default spindle speed (RPM)
    pub spindle_speed: StandardReal,
}

impl Toolpath {
    /// Create a new toolpath with the given name and type
    pub fn new(name: String, toolpath_type: ToolpathType) -> Self {
        Self {
            name,
            toolpath_type,
            points: Vec::new(),
            segments: Vec::new(),
            stock_to_leave: 0.0,
            stepover: 0.0,
            stepdown: 0.0,
            safe_height: 10.0,
            retract_height: 5.0,
            feed_rate: 1000.0,
            plunge_feed_rate: 500.0,
            spindle_speed: 3000.0,
        }
    }

    pub fn add_point(&mut self, point: ToolpathPoint) {
        self.points.push(point);
    }

    pub fn add_segment(&mut self, segment: ToolpathSegment) {
        self.segments.push(segment);
    }

    pub fn with_stock_to_leave(mut self, stock: StandardReal) -> Self {
        self.stock_to_leave = stock;
        self
    }

    pub fn with_stepover(mut self, stepover: StandardReal) -> Self {
        self.stepover = stepover;
        self
    }

    pub fn with_stepdown(mut self, stepdown: StandardReal) -> Self {
        self.stepdown = stepdown;
        self
    }

    pub fn with_safe_height(mut self, height: StandardReal) -> Self {
        self.safe_height = height;
        self
    }

    pub fn with_retract_height(mut self, height: StandardReal) -> Self {
        self.retract_height = height;
        self
    }

    pub fn with_feed_rate(mut self, feed_rate: StandardReal) -> Self {
        self.feed_rate = feed_rate;
        self
    }

    pub fn with_plunge_feed_rate(mut self, feed_rate: StandardReal) -> Self {
        self.plunge_feed_rate = feed_rate;
        self
    }

    pub fn with_spindle_speed(mut self, speed: StandardReal) -> Self {
        self.spindle_speed = speed;
        self
    }

    pub fn length(&self) -> StandardReal {
        let mut length = 0.0;
        for i in 1..self.points.len() {
            let p1 = &self.points[i - 1].position;
            let p2 = &self.points[i].position;
            length += ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt();
        }
        length
    }

    pub fn machining_time(&self) -> StandardReal {
        let mut time = 0.0;
        for i in 1..self.points.len() {
            let p1 = &self.points[i - 1];
            let p2 = &self.points[i];
            let dist = ((p2.position.x - p1.position.x).powi(2)
                + (p2.position.y - p1.position.y).powi(2)
                + (p2.position.z - p1.position.z).powi(2)).sqrt();
            let feed = if p2.is_rapid {
                10000.0
            } else {
                p2.feed_rate.unwrap_or(self.feed_rate)
            };
            if feed > 0.0 {
                time += dist / feed;
            }
        }
        time
    }

    pub fn optimize(&mut self) {
        self.remove_duplicate_points();
        self.simplify_collinear_points();
    }

    fn remove_duplicate_points(&mut self) {
        let tolerance = 1e-6;
        let points = &self.points;
        let mut to_remove = Vec::new();
        for (i, p) in points.iter().enumerate() {
            for (j, other) in points.iter().enumerate() {
                if i != j && (
                    (p.position.x - other.position.x).abs() < tolerance &&
                    (p.position.y - other.position.y).abs() < tolerance &&
                    (p.position.z - other.position.z).abs() < tolerance
                ) {
                    to_remove.push(i);
                    break;
                }
            }
        }
        let mut i = 0;
        self.points.retain(|_| {
            let keep = !to_remove.contains(&i);
            i += 1;
            keep
        });
    }

    fn simplify_collinear_points(&mut self) {
        if self.points.len() < 3 {
            return;
        }

        let mut simplified = vec![self.points[0].clone()];
        for i in 1..self.points.len() - 1 {
            let p1 = &self.points[i - 1].position;
            let p2 = &self.points[i].position;
            let p3 = &self.points[i + 1].position;

            let v1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
            let v2 = Vector::new(p3.x - p2.x, p3.y - p2.y, p3.z - p2.z);

            let cross = v1.cross(&v2);
            let cross_mag = (cross.x.powi(2) + cross.y.powi(2) + cross.z.powi(2)).sqrt();

            if cross_mag > 1e-6 {
                simplified.push(self.points[i].clone());
            }
        }
        simplified.push(self.points.last().unwrap().clone());
        self.points = simplified;
    }
}

pub struct ToolpathGenerator {
    tolerance: StandardReal,
}

impl ToolpathGenerator {
    pub fn new() -> Self {
        Self {
            tolerance: 0.01,
        }
    }

    pub fn with_tolerance(mut self, tolerance: StandardReal) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn generate_contour_toolpath(&self, shape: &TopoDsShape, tool_diameter: StandardReal, depth: StandardReal) -> Toolpath {
        let mut toolpath = Toolpath::new("Contour".to_string(), ToolpathType::Contour);
        let (min, max) = shape.bounding_box();

        let z_levels = self.calculate_z_levels(min.z, max.z, depth);

        for z in z_levels {
            let corners = [
                Point::new(min.x - tool_diameter, min.y - tool_diameter, z),
                Point::new(max.x + tool_diameter, min.y - tool_diameter, z),
                Point::new(max.x + tool_diameter, max.y + tool_diameter, z),
                Point::new(min.x - tool_diameter, max.y + tool_diameter, z),
            ];

            toolpath.add_point(ToolpathPoint::new(corners[0]).rapid());
            for corner in &corners[1..] {
                toolpath.add_point(ToolpathPoint::new(*corner));
            }
            toolpath.add_point(ToolpathPoint::new(corners[0]));
        }

        toolpath
    }

    pub fn generate_pocket_toolpath(&self, shape: &TopoDsShape, tool_diameter: StandardReal, stepover: StandardReal, depth: StandardReal) -> Toolpath {
        let mut toolpath = Toolpath::new("Pocket".to_string(), ToolpathType::Pocket);
        let (min, max) = shape.bounding_box();

        let z_levels = self.calculate_z_levels(min.z, max.z, depth);
        let offset = tool_diameter / 2.0;

        for z in z_levels {
            let y_start = min.y + offset;
            let y_end = max.y - offset;
            let x_start = min.x + offset;
            let x_end = max.x - offset;

            let mut y = y_start;
            let mut direction = 1.0;

            toolpath.add_point(ToolpathPoint::new(Point::new(x_start, y, toolpath.safe_height)).rapid());
            toolpath.add_point(ToolpathPoint::new(Point::new(x_start, y, z)).rapid());

            while y <= y_end {
                let x2 = if direction > 0.0 { x_end } else { x_start };

                toolpath.add_point(ToolpathPoint::new(Point::new(x2, y, z)));

                y += stepover;
                if y <= y_end {
                    toolpath.add_point(ToolpathPoint::new(Point::new(x2, y.min(y_end), z)));
                }

                direction *= -1.0;
            }

            toolpath.add_point(ToolpathPoint::new(Point::new(x_start, y_end, toolpath.retract_height)).rapid());
        }

        toolpath.with_stepover(stepover).with_stepdown(depth)
    }

    pub fn generate_drill_toolpath(&self, positions: &[Point], depth: StandardReal, retract_height: StandardReal) -> Toolpath {
        let mut toolpath = Toolpath::new("Drill".to_string(), ToolpathType::Drill);

        for pos in positions {
            toolpath.add_point(ToolpathPoint::new(Point::new(pos.x, pos.y, retract_height)).rapid());
            toolpath.add_point(ToolpathPoint::new(Point::new(pos.x, pos.y, pos.z - depth)));
            toolpath.add_point(ToolpathPoint::new(Point::new(pos.x, pos.y, retract_height)).rapid());
        }

        toolpath.with_retract_height(retract_height)
    }

    pub fn generate_face_toolpath(&self, shape: &TopoDsShape, tool_diameter: StandardReal, stepover: StandardReal) -> Toolpath {
        self.generate_pocket_toolpath(shape, tool_diameter, stepover, 0.1)
    }

    pub fn generate_adaptive_toolpath(&self, shape: &TopoDsShape, tool_diameter: StandardReal, optimal_load: StandardReal) -> Toolpath {
        let mut toolpath = Toolpath::new("Adaptive".to_string(), ToolpathType::Adaptive);
        let (min, max) = shape.bounding_box();

        let offset = tool_diameter / 2.0;
        let mut current_offset = offset;

        while current_offset < (max.x - min.x) / 2.0 && current_offset < (max.y - min.y) / 2.0 {
            let corners = [
                Point::new(min.x + current_offset, min.y + current_offset, max.z),
                Point::new(max.x - current_offset, min.y + current_offset, max.z),
                Point::new(max.x - current_offset, max.y - current_offset, max.z),
                Point::new(min.x + current_offset, max.y - current_offset, max.z),
            ];

            toolpath.add_point(ToolpathPoint::new(corners[0]).rapid());
            for corner in &corners[1..] {
                toolpath.add_point(ToolpathPoint::new(*corner));
            }
            toolpath.add_point(ToolpathPoint::new(corners[0]));

            current_offset += optimal_load;
        }

        toolpath
    }

    fn calculate_z_levels(&self, z_min: StandardReal, z_max: StandardReal, stepdown: StandardReal) -> Vec<StandardReal> {
        let mut levels = Vec::new();
        let mut z = z_max;

        while z > z_min + self.tolerance {
            levels.push(z);
            z -= stepdown;
        }
        levels.push(z_min);

        levels
    }
}

impl Default for ToolpathGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ToolpathValidator {
    max_feed_rate: StandardReal,
    max_spindle_speed: StandardReal,
    #[allow(dead_code)]
    min_tool_diameter: StandardReal,
}

impl ToolpathValidator {
    pub fn new() -> Self {
        Self {
            max_feed_rate: 10000.0,
            max_spindle_speed: 30000.0,
            min_tool_diameter: 0.1,
        }
    }

    pub fn validate(&self, toolpath: &Toolpath) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if toolpath.feed_rate > self.max_feed_rate {
            errors.push(format!("Feed rate {} exceeds maximum {}", toolpath.feed_rate, self.max_feed_rate));
        }

        if toolpath.spindle_speed > self.max_spindle_speed {
            errors.push(format!("Spindle speed {} exceeds maximum {}", toolpath.spindle_speed, self.max_spindle_speed));
        }

        if toolpath.points.is_empty() {
            errors.push("Toolpath has no points".to_string());
        }

        for (i, point) in toolpath.points.iter().enumerate() {
            if let Some(feed) = point.feed_rate {
                if feed > self.max_feed_rate {
                    errors.push(format!("Point {} feed rate {} exceeds maximum {}", i, feed, self.max_feed_rate));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for ToolpathValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolpath_point_creation() {
        let point = ToolpathPoint::new(Point::new(1.0, 2.0, 3.0))
            .with_feed_rate(1000.0)
            .rapid();

        assert_eq!(point.position.x, 1.0);
        assert_eq!(point.feed_rate, Some(1000.0));
        assert!(point.is_rapid);
    }

    #[test]
    fn test_toolpath_creation() {
        let toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour)
            .with_feed_rate(1000.0)
            .with_spindle_speed(3000.0);

        assert_eq!(toolpath.name, "Test");
        assert_eq!(toolpath.toolpath_type, ToolpathType::Contour);
        assert_eq!(toolpath.feed_rate, 1000.0);
    }

    #[test]
    fn test_toolpath_length() {
        let mut toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour);
        toolpath.add_point(ToolpathPoint::new(Point::new(0.0, 0.0, 0.0)));
        toolpath.add_point(ToolpathPoint::new(Point::new(3.0, 4.0, 0.0)));

        assert_eq!(toolpath.length(), 5.0);
    }

    #[test]
    fn test_toolpath_generator() {
        let generator = ToolpathGenerator::new();
        assert_eq!(generator.tolerance, 0.01);
    }

    #[test]
    fn test_toolpath_validator() {
        let validator = ToolpathValidator::new();
        let toolpath = Toolpath::new("Test".to_string(), ToolpathType::Contour)
            .with_feed_rate(500.0)
            .with_spindle_speed(2000.0);

        assert!(validator.validate(&toolpath).is_ok());
    }
}
