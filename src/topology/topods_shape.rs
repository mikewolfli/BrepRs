use crate::geometry::{Point, Transform};
use crate::topology::shape_enum::ShapeType;
use crate::topology::topods_location::TopoDsLocation;
use std::sync::atomic::{AtomicI32, Ordering};

/// Global counter for generating unique shape IDs
static SHAPE_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

/// Base class for all topological shapes
///
/// This is the abstract base class for all topological shapes in the
/// boundary representation (BRep) model. It provides the basic
/// functionality common to all shapes, including type identification,
/// location transformation, and shape hierarchy management.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopoDsShape {
    shape_type: ShapeType,
    location: Option<TopoDsLocation>,
    orientation: i32,
    mutable: bool,
    shape_id: i32,
}

impl TopoDsShape {
    /// Create a new shape with the specified type
    #[inline]
    pub fn new(shape_type: ShapeType) -> Self {
        Self {
            shape_type,
            location: None,
            orientation: 1,
            mutable: false,
            shape_id: SHAPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Get the shape type
    #[inline]
    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    /// Check if this shape is of the specified type
    #[inline]
    pub fn is_kind(&self, shape_type: ShapeType) -> bool {
        self.shape_type == shape_type
    }

    /// Check if this shape is a vertex
    #[inline]
    pub fn is_vertex(&self) -> bool {
        self.shape_type == ShapeType::Vertex
    }

    /// Check if this shape is an edge
    #[inline]
    pub fn is_edge(&self) -> bool {
        self.shape_type == ShapeType::Edge
    }

    /// Check if this shape is a wire
    #[inline]
    pub fn is_wire(&self) -> bool {
        self.shape_type == ShapeType::Wire
    }

    /// Check if this shape is a face
    #[inline]
    pub fn is_face(&self) -> bool {
        self.shape_type == ShapeType::Face
    }

    /// Check if this shape is a shell
    #[inline]
    pub fn is_shell(&self) -> bool {
        self.shape_type == ShapeType::Shell
    }

    /// Check if this shape is a solid
    #[inline]
    pub fn is_solid(&self) -> bool {
        self.shape_type == ShapeType::Solid
    }

    /// Check if this shape is a compound
    #[inline]
    pub fn is_compound(&self) -> bool {
        self.shape_type == ShapeType::Compound
    }

    /// Check if this shape is a compsolid
    #[inline]
    pub fn is_compsolid(&self) -> bool {
        self.shape_type == ShapeType::CompSolid
    }

    /// Get the location of the shape
    #[inline]
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.location.as_ref()
    }

    /// Set the location of the shape
    #[inline]
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.location = Some(location);
    }

    /// Get the orientation of the shape
    #[inline]
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Set the orientation of the shape
    #[inline]
    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }

    /// Check if the shape is mutable
    #[inline]
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    /// Set the mutability of the shape
    #[inline]
    pub fn set_mutable(&mut self, mutable: bool) {
        self.mutable = mutable
    }

    /// Get the unique identifier of the shape
    #[inline]
    pub fn shape_id(&self) -> i32 {
        self.shape_id
    }

    /// Set the unique identifier of the shape
    #[inline]
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape_id = id;
    }

    /// Check if this shape is more complex than another shape
    #[inline]
    pub fn is_more_complex(&self, other: &TopoDsShape) -> bool {
        self.shape_type.is_more_complex_or_equal(&other.shape_type)
    }

    /// Check if this shape is less complex than another shape
    #[inline]
    pub fn is_less_complex(&self, other: &TopoDsShape) -> bool {
        self.shape_type.is_less_complex(&other.shape_type)
    }

    /// Get the name of the shape type
    #[inline]
    pub fn type_name(&self) -> &'static str {
        self.shape_type.name()
    }

    /// Apply a transformation to the shape
    #[inline]
    pub fn transform(&mut self, transformation: &Transform) {
        if let Some(loc) = self.location.as_mut() {
            loc.transform(transformation);
        }
    }

    /// Get the transformed location of the shape
    #[inline]
    pub fn transformed_location(&self) -> Option<Point> {
        self.location.as_ref().map(|loc| {
            let transform = loc.to_transform();
            transform.transforms(&Point::origin())
        })
    }

    /// Create a copy of this shape with a new orientation
    #[inline]
    pub fn reversed(&self) -> Self {
        let mut result = self.clone();
        result.set_orientation(-result.orientation());
        result
    }

    /// Check if this shape is reversed
    #[inline]
    pub fn is_reversed(&self) -> bool {
        self.orientation() < 0
    }

    /// Clear the location of the shape
    #[inline]
    pub fn clear_location(&mut self) {
        self.location = None;
    }

    /// Check if the shape has a location
    #[inline]
    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }
}

impl Default for TopoDsShape {
    fn default() -> Self {
        Self::new(ShapeType::Compound)
    }
}

impl PartialEq for TopoDsShape {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id == other.shape_id
    }
}

impl Eq for TopoDsShape {}

impl TopoDsShape {
    /// Compute the bounding box of the shape
    /// 
    /// Returns (min_point, max_point) representing the bounding box
    pub fn bounding_box(&self) -> (Point, Point) {
        // Default implementation returns origin points
        // Subclasses should override this with proper implementation
        (Point::origin(), Point::origin())
    }

    /// Get faces of the shape
    /// 
    /// Default implementation returns empty vector
    /// Subclasses should override this with proper implementation
    pub fn faces(&self) -> Vec<crate::topology::topods_face::TopoDsFace> {
        Vec::new()
    }

    /// Try to cast to face reference
    /// 
    /// Returns None if this shape is not a face
    pub fn as_face(&self) -> Option<&crate::topology::topods_face::TopoDsFace> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        assert!(shape.is_vertex());
        assert!(!shape.is_edge());
        assert_eq!(shape.type_name(), "Vertex");
    }

    #[test]
    fn test_shape_type_checks() {
        let vertex = TopoDsShape::new(ShapeType::Vertex);
        let edge = TopoDsShape::new(ShapeType::Edge);
        let face = TopoDsShape::new(ShapeType::Face);
        let solid = TopoDsShape::new(ShapeType::Solid);

        assert!(vertex.is_vertex());
        assert!(edge.is_edge());
        assert!(face.is_face());
        assert!(solid.is_solid());
    }

    #[test]
    fn test_shape_complexity() {
        let vertex = TopoDsShape::new(ShapeType::Vertex);
        let edge = TopoDsShape::new(ShapeType::Edge);
        let face = TopoDsShape::new(ShapeType::Face);

        assert!(face.is_more_complex(&vertex));
        assert!(vertex.is_less_complex(&face));
        assert!(!vertex.is_more_complex(&edge));
    }

    #[test]
    fn test_shape_orientation() {
        let mut shape = TopoDsShape::new(ShapeType::Edge);
        assert_eq!(shape.orientation(), 1);

        shape.set_orientation(-1);
        assert_eq!(shape.orientation(), -1);
    }

    #[test]
    fn test_shape_mutable() {
        let mut shape = TopoDsShape::new(ShapeType::Face);
        assert!(!shape.is_mutable());

        shape.set_mutable(true);
        assert!(shape.is_mutable());
    }

    #[test]
    fn test_shape_id() {
        let mut shape = TopoDsShape::new(ShapeType::Solid);
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = shape.shape_id();
        assert!(initial_id > 0);

        shape.set_shape_id(42);
        assert_eq!(shape.shape_id(), 42);
    }

    #[test]
    fn test_shape_clone() {
        let mut shape1 = TopoDsShape::new(ShapeType::Edge);
        shape1.set_shape_id(10);
        shape1.set_orientation(-1);

        let shape2 = shape1.clone();
        assert_eq!(shape2.shape_id(), 10);
        assert_eq!(shape2.orientation(), -1);
        assert_eq!(shape1, shape2);
    }
}
