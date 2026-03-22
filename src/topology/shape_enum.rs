/// Enumeration of topological shape types
///
/// This enum defines all the types of topological shapes in the
/// boundary representation (BRep) model, ordered from simplest
/// to most complex.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ShapeType {
    /// A single point in 3D space
    Vertex,

    /// A curve bounded by two vertices
    Edge,

    /// An ordered set of connected edges
    Wire,

    /// A bounded portion of a surface
    Face,

    /// A set of connected faces
    Shell,

    /// A 3D region bounded by shells
    Solid,

    /// A collection of shapes of any type
    Compound,

    /// A collection of connected solids
    CompSolid,
}

impl ShapeType {
    /// Parse shape type from name
    pub fn from_name(name: &str) -> ShapeType {
        match name {
            "Vertex" => ShapeType::Vertex,
            "Edge" => ShapeType::Edge,
            "Wire" => ShapeType::Wire,
            "Face" => ShapeType::Face,
            "Shell" => ShapeType::Shell,
            "Solid" => ShapeType::Solid,
            "Compound" => ShapeType::Compound,
            "CompSolid" => ShapeType::CompSolid,
            _ => ShapeType::Vertex,
        }
    }

    /// Get the name of the shape type
    pub fn name(&self) -> &'static str {
        match self {
            ShapeType::Vertex => "Vertex",
            ShapeType::Edge => "Edge",
            ShapeType::Wire => "Wire",
            ShapeType::Face => "Face",
            ShapeType::Shell => "Shell",
            ShapeType::Solid => "Solid",
            ShapeType::Compound => "Compound",
            ShapeType::CompSolid => "CompSolid",
        }
    }

    /// Check if this shape type is more complex than or equal to another
    pub fn is_more_complex_or_equal(&self, other: &ShapeType) -> bool {
        self.complexity_level() >= other.complexity_level()
    }

    /// Check if this shape type is less complex than another
    pub fn is_less_complex(&self, other: &ShapeType) -> bool {
        self.complexity_level() < other.complexity_level()
    }

    /// Get the complexity level of the shape type
    fn complexity_level(&self) -> i32 {
        match self {
            ShapeType::Vertex => 0,
            ShapeType::Edge => 1,
            ShapeType::Wire => 2,
            ShapeType::Face => 3,
            ShapeType::Shell => 4,
            ShapeType::Solid => 5,
            ShapeType::Compound => 6,
            ShapeType::CompSolid => 7,
        }
    }

    /// Check if this shape type can contain another shape type
    pub fn can_contain(&self, other: &ShapeType) -> bool {
        self.complexity_level() > other.complexity_level()
    }

    /// Get the dimension of the shape type (0D, 1D, 2D, or 3D)
    pub fn dimension(&self) -> i32 {
        match self {
            ShapeType::Vertex => 0,
            ShapeType::Edge | ShapeType::Wire => 1,
            ShapeType::Face | ShapeType::Shell => 2,
            ShapeType::Solid | ShapeType::Compound | ShapeType::CompSolid => 3,
        }
    }
}

impl Default for ShapeType {
    fn default() -> Self {
        ShapeType::Compound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_type_names() {
        assert_eq!(ShapeType::Vertex.name(), "Vertex");
        assert_eq!(ShapeType::Edge.name(), "Edge");
        assert_eq!(ShapeType::Wire.name(), "Wire");
        assert_eq!(ShapeType::Face.name(), "Face");
        assert_eq!(ShapeType::Shell.name(), "Shell");
        assert_eq!(ShapeType::Solid.name(), "Solid");
        assert_eq!(ShapeType::Compound.name(), "Compound");
        assert_eq!(ShapeType::CompSolid.name(), "CompSolid");
    }

    #[test]
    fn test_complexity_comparison() {
        assert!(ShapeType::Solid.is_more_complex_or_equal(&ShapeType::Face));
        assert!(ShapeType::Edge.is_less_complex(&ShapeType::Wire));
        assert!(!ShapeType::Vertex.is_more_complex_or_equal(&ShapeType::Solid));
    }

    #[test]
    fn test_can_contain() {
        assert!(ShapeType::Solid.can_contain(&ShapeType::Face));
        assert!(ShapeType::Face.can_contain(&ShapeType::Wire));
        assert!(ShapeType::Wire.can_contain(&ShapeType::Edge));
        assert!(ShapeType::Edge.can_contain(&ShapeType::Vertex));
        assert!(!ShapeType::Vertex.can_contain(&ShapeType::Edge));
    }

    #[test]
    fn test_dimension() {
        assert_eq!(ShapeType::Vertex.dimension(), 0);
        assert_eq!(ShapeType::Edge.dimension(), 1);
        assert_eq!(ShapeType::Wire.dimension(), 1);
        assert_eq!(ShapeType::Face.dimension(), 2);
        assert_eq!(ShapeType::Shell.dimension(), 2);
        assert_eq!(ShapeType::Solid.dimension(), 3);
        assert_eq!(ShapeType::Compound.dimension(), 3);
        assert_eq!(ShapeType::CompSolid.dimension(), 3);
    }

    #[test]
    fn test_default() {
        assert_eq!(ShapeType::default(), ShapeType::Compound);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ShapeType::Vertex, ShapeType::Vertex);
        assert_ne!(ShapeType::Edge, ShapeType::Wire);
    }

    #[test]
    fn test_clone() {
        let shape_type = ShapeType::Solid;
        let cloned = shape_type.clone();
        assert_eq!(shape_type, cloned);
    }
}
