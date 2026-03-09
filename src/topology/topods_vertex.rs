use crate::geometry::Point;
use crate::topology::{topods_location::TopoDsLocation, topods_shape::TopoDsShape};

/// Represents a vertex in the topological structure
///
/// A vertex is the simplest topological element, representing a point
/// in 3D space. It can be shared by multiple edges and faces.
#[derive(Debug)]
pub struct TopoDsVertex {
    shape: TopoDsShape,
    point: Point,
    tolerance: f64,
}

impl TopoDsVertex {
    /// Create a new vertex at specified point
    pub fn new(point: Point) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Vertex),
            point,
            tolerance: 0.001,
        }
    }

    /// Create a new vertex with specified tolerance
    pub fn with_tolerance(point: Point, tolerance: f64) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Vertex),
            point,
            tolerance,
        }
    }

    /// Get the geometric point of the vertex
    pub fn point(&self) -> &Point {
        &self.point
    }

    /// Get the tolerance of the vertex
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the vertex
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Set the geometric point of the vertex
    pub fn set_point(&mut self, point: Point) {
        self.point = point;
    }

    /// Get the location of the vertex
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the vertex
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Get the orientation of the vertex
    pub fn orientation(&self) -> i32 {
        self.shape.orientation()
    }

    /// Set the orientation of the vertex
    pub fn set_orientation(&mut self, orientation: i32) {
        self.shape.set_orientation(orientation);
    }

    /// Check if this vertex is equal to another within tolerance
    pub fn is_equal(&self, other: &TopoDsVertex) -> bool {
        let dx = self.point.x - other.point.x;
        let dy = self.point.y - other.point.y;
        let dz = self.point.z - other.point.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        distance < self.tolerance.max(other.tolerance)
    }

    /// Get the unique identifier of the vertex
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the vertex
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this vertex is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the vertex
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Get the distance to another vertex
    pub fn distance(&self, other: &TopoDsVertex) -> f64 {
        let dx = self.point.x - other.point.x;
        let dy = self.point.y - other.point.y;
        let dz = self.point.z - other.point.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get the squared distance to another vertex
    pub fn square_distance(&self, other: &TopoDsVertex) -> f64 {
        let dx = self.point.x - other.point.x;
        let dy = self.point.y - other.point.y;
        let dz = self.point.z - other.point.z;
        dx * dx + dy * dy + dz * dz
    }

    /// Check if this vertex is at the origin
    pub fn is_origin(&self) -> bool {
        self.point == Point::origin()
    }

    /// Get the x coordinate
    pub fn x(&self) -> f64 {
        self.point.x
    }

    /// Get the y coordinate
    pub fn y(&self) -> f64 {
        self.point.y
    }

    /// Get the z coordinate
    pub fn z(&self) -> f64 {
        self.point.z
    }

    /// Set the x coordinate
    pub fn set_x(&mut self, x: f64) {
        self.point.x = x;
    }

    /// Set the y coordinate
    pub fn set_y(&mut self, y: f64) {
        self.point.y = y;
    }

    /// Set the z coordinate
    pub fn set_z(&mut self, z: f64) {
        self.point.z = z;
    }
}

impl Default for TopoDsVertex {
    fn default() -> Self {
        Self::new(Point::origin())
    }
}

impl Clone for TopoDsVertex {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            point: self.point,
            tolerance: self.tolerance,
        }
    }
}

impl PartialEq for TopoDsVertex {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = TopoDsVertex::new(point);

        assert_eq!(vertex.point(), &point);
        assert!(vertex.shape().is_vertex());
        assert!(!vertex.shape().is_edge());
    }

    #[test]
    fn test_vertex_with_tolerance() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = TopoDsVertex::with_tolerance(point, 0.01);

        assert_eq!(vertex.tolerance(), 0.01);
    }

    #[test]
    fn test_vertex_equality() {
        let vertex1 = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));
        let vertex2 = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));
        // 差值为0.002，大于tolerance(0.001)，所以应该不相等
        let vertex3 = TopoDsVertex::new(Point::new(1.002, 2.0, 3.0));

        assert_eq!(vertex1, vertex2);
        // vertex1和vertex3的距离是0.002，大于tolerance，所以不相等
        assert_ne!(vertex1, vertex3);
    }

    #[test]
    fn test_vertex_distance() {
        let vertex1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let vertex2 = TopoDsVertex::new(Point::new(3.0, 4.0, 0.0));

        let distance = vertex1.distance(&vertex2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_vertex_square_distance() {
        let vertex1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let vertex2 = TopoDsVertex::new(Point::new(3.0, 4.0, 0.0));

        let distance = vertex1.square_distance(&vertex2);
        assert!((distance - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_vertex_coordinates() {
        let mut vertex = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));

        assert_eq!(vertex.x(), 1.0);
        assert_eq!(vertex.y(), 2.0);
        assert_eq!(vertex.z(), 3.0);

        vertex.set_x(4.0);
        vertex.set_y(5.0);
        vertex.set_z(6.0);

        assert_eq!(vertex.x(), 4.0);
        assert_eq!(vertex.y(), 5.0);
        assert_eq!(vertex.z(), 6.0);
    }

    #[test]
    fn test_vertex_is_origin() {
        let origin_vertex = TopoDsVertex::new(Point::origin());
        assert!(origin_vertex.is_origin());

        let other_vertex = TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
        assert!(!other_vertex.is_origin());
    }

    #[test]
    fn test_vertex_shape_id() {
        let mut vertex = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = vertex.shape_id();
        assert!(initial_id > 0);

        vertex.set_shape_id(42);
        assert_eq!(vertex.shape_id(), 42);
    }

    #[test]
    fn test_vertex_mutable() {
        let mut vertex = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));
        assert!(!vertex.is_mutable());

        vertex.set_mutable(true);
        assert!(vertex.is_mutable());
    }

    #[test]
    fn test_vertex_clone() {
        let mut vertex1 = TopoDsVertex::new(Point::new(1.0, 2.0, 3.0));
        vertex1.set_shape_id(10);

        let vertex2 = vertex1.clone();
        assert_eq!(vertex2.shape_id(), 10);
        assert_eq!(vertex1, vertex2);
    }
}
