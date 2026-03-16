use crate::foundation::StandardReal;
use crate::geometry::{
    axis::Axis, bounding_box::BoundingBox, direction::Direction, plane::Plane, Point, Vector,
};
use crate::topology::{TopoDsShape, TopoDsWire};

/// Section plane for cutting models
#[derive(Debug, Clone)]
pub struct SectionPlane {
    /// Plane definition
    pub plane: Plane,
    /// Section direction (normal vector direction)
    pub direction: Vector,
}

/// Section type
#[derive(Debug, Clone, PartialEq)]
pub enum SectionType {
    /// Cross-section (perpendicular to a direction)
    CrossSection,
    /// Longitudinal section (parallel to a direction)
    LongitudinalSection,
    /// Oblique section (at an angle)
    ObliqueSection,
    /// Tangential section (tangent to a surface)
    TangentialSection,
}

/// Section result
#[derive(Debug, Clone)]
pub struct SectionResult {
    /// Section curves (wires)
    pub section_curves: Vec<TopoDsWire>,
    /// Section area
    pub section_area: StandardReal,
    /// Bounding box of the section
    pub bounding_box: BoundingBox,
    /// Section type
    pub section_type: SectionType,
}

/// Slicing parameters
#[derive(Debug, Clone)]
pub struct SlicingParameters {
    /// Start position
    pub start_position: StandardReal,
    /// End position
    pub end_position: StandardReal,
    /// Slice spacing
    pub slice_spacing: StandardReal,
    /// Slice direction
    pub direction: Vector,
}

/// Slicing result
#[derive(Debug, Clone)]
pub struct SlicingResult {
    /// Slice results
    pub slices: Vec<SectionResult>,
    /// Number of slices
    pub slice_count: usize,
    /// Total thickness
    pub total_thickness: StandardReal,
}

impl SectionPlane {
    /// Create a new section plane
    pub fn new(plane: Plane) -> Self {
        Self {
            plane: plane,
            direction: Vector::new(plane.normal().x, plane.normal().y, plane.normal().z),
        }
    }

    /// Create a cross-section plane perpendicular to a direction
    pub fn cross_section(direction: Vector, point: Point) -> Self {
        let dir = crate::geometry::Direction::from_vector(&direction);
        let plane = Plane::new(point, dir, crate::geometry::Direction::x_axis());
        Self::new(plane)
    }

    /// Create a longitudinal section plane parallel to a direction
    pub fn longitudinal_section(direction: Vector, point: Point, normal: Vector) -> Self {
        // Ensure normal is perpendicular to direction
        let corrected_normal = direction.cross(&normal).normalized();
        let dir = crate::geometry::Direction::from_vector(&corrected_normal);
        let plane = Plane::new(point, dir, crate::geometry::Direction::x_axis());
        Self::new(plane)
    }

    /// Create an oblique section plane at an angle
    pub fn oblique_section(direction: Vector, point: Point, angle: StandardReal) -> Self {
        // Rotate the direction by the angle to create an oblique plane
        let axis = Axis::new(Point::origin(), Direction::new(0.0, 1.0, 0.0));
        let rotated_direction = direction.rotate(&axis, angle);
        let dir = Direction::from_vector(&rotated_direction);
        let plane = Plane::new(point, dir, Direction::x_axis());
        Self::new(plane)
    }
}

impl SectionResult {
    /// Create a new section result
    pub fn new(
        section_curves: Vec<TopoDsWire>,
        section_area: StandardReal,
        bounding_box: BoundingBox,
        section_type: SectionType,
    ) -> Self {
        Self {
            section_curves,
            section_area,
            bounding_box,
            section_type,
        }
    }
}

impl SlicingParameters {
    /// Create new slicing parameters
    pub fn new(
        start_position: StandardReal,
        end_position: StandardReal,
        slice_spacing: StandardReal,
        direction: Vector,
    ) -> Self {
        Self {
            start_position,
            end_position,
            slice_spacing,
            direction: direction.normalized(),
        }
    }

    /// Calculate the number of slices
    pub fn calculate_slice_count(&self) -> usize {
        let total_thickness = (self.end_position - self.start_position).abs();
        ((total_thickness / self.slice_spacing).floor() as usize) + 1
    }
}

impl SlicingResult {
    /// Create a new slicing result
    pub fn new(slices: Vec<SectionResult>, total_thickness: StandardReal) -> Self {
        Self {
            slices,
            slice_count: slices.len(),
            total_thickness,
        }
    }
}

/// Trait for objects that can be sectioned
pub trait Sectionable {
    /// Generate a section using a plane
    fn section(&self, plane: &SectionPlane) -> SectionResult;

    /// Generate multiple slices
    fn slice(&self, params: &SlicingParameters) -> SlicingResult;

    /// Generate a cross-section perpendicular to a direction
    fn cross_section(&self, direction: Vector, position: StandardReal) -> SectionResult;

    /// Generate a longitudinal section parallel to a direction
    fn longitudinal_section(&self, direction: Vector, position: StandardReal) -> SectionResult;
}
// TODO: Implement correct perpendicular logic or remove if not needed.
// let normal = direction.perpendicular();
/// Implement sectioning for TopoDsShape
impl Sectionable for TopoDsShape {
    /// Generate a section using a plane
    fn section(&self, plane: &SectionPlane) -> SectionResult {
        // TODO: Implement actual sectioning logic
        // For now, return a placeholder
        let section_curves = Vec::new();
        let section_area = 0.0;
        let bounding_box = BoundingBox::new(Point::origin(), Point::origin());
        let section_type = SectionType::CrossSection;

        SectionResult::new(section_curves, section_area, bounding_box, section_type)
    }

    fn slice(&self, params: &SlicingParameters) -> SlicingResult {
        let slice_count = params.calculate_slice_count();
        let mut slices = Vec::with_capacity(slice_count);

        for i in 0..slice_count {
            let position = params.start_position + i as StandardReal * params.slice_spacing;
            let point = Point::origin() + params.direction * position;
            let plane = SectionPlane::cross_section(params.direction, point);
            let section = self.section(&plane);
            slices.push(section);
        }

        let total_thickness = (params.end_position - params.start_position).abs();
        SlicingResult::new(slices, total_thickness)
    }

    fn cross_section(&self, direction: Vector, position: StandardReal) -> SectionResult {
        let point = Point::origin() + direction * position;
        let plane = SectionPlane::cross_section(direction, point);
        self.section(&plane)
    }

    fn longitudinal_section(&self, direction: Vector, position: StandardReal) -> SectionResult {
        let point = Point::origin() + direction.perpendicular() * position;
        let normal = direction.perpendicular();
        let plane = SectionPlane::longitudinal_section(direction, point, normal);
        self.section(&plane)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{plane::Plane, Point, Vector};

    #[test]
    fn test_section_plane_creation() {
        let point = Point::new(0.0, 0.0, 0.0);
        let normal = Vector::new(0.0, 0.0, 1.0);
        let plane = Plane::new(point, normal);
        let section_plane = SectionPlane::new(plane);

        assert_eq!(section_plane.direction, normal);
    }

    #[test]
    fn test_cross_section_plane() {
        let direction = Vector::new(0.0, 0.0, 1.0);
        let point = Point::new(0.0, 0.0, 1.0);
        let section_plane = SectionPlane::cross_section(direction, point);

        assert_eq!(section_plane.direction, direction);
    }

    #[test]
    fn test_slicing_parameters() {
        let params = SlicingParameters::new(0.0, 1.0, 0.2, Vector::new(0.0, 0.0, 1.0));

        assert_eq!(params.calculate_slice_count(), 6); // 0.0, 0.2, 0.4, 0.6, 0.8, 1.0
    }

    #[test]
    fn test_section_result() {
        let section_curves = Vec::new();
        let section_area = 1.0;
        let bounding_box = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let section_type = SectionType::CrossSection;

        let section_result =
            SectionResult::new(section_curves, section_area, bounding_box, section_type);

        assert_eq!(section_result.section_area, 1.0);
        assert_eq!(section_result.section_type, SectionType::CrossSection);
    }
}
