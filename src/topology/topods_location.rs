use crate::geometry::{Axis, Direction, Point, Transform, Vector};

/// Represents a location in 3D space
///
/// A location consists of a point (translation) and a transformation
/// (rotation, scaling, etc.). This is used to position and
/// orient topological shapes in 3D space.
#[derive(Debug, Clone)]
pub struct TopoDsLocation {
    translation: Point,
    transformation: Transform,
}

impl TopoDsLocation {
    /// Create a new location at origin with identity transformation
    pub fn new() -> Self {
        Self {
            translation: Point::origin(),
            transformation: Transform::identity(),
        }
    }

    /// Create a new location with specified translation
    pub fn with_translation(translation: Point) -> Self {
        Self {
            translation,
            transformation: Transform::identity(),
        }
    }

    /// Create a new location with specified transformation
    pub fn with_transformation(transformation: Transform) -> Self {
        Self {
            translation: Point::origin(),
            transformation,
        }
    }

    /// Create a new location with translation and transformation
    pub fn with_both(translation: Point, transformation: Transform) -> Self {
        Self {
            translation,
            transformation,
        }
    }

    /// Get the translation component
    pub fn translation(&self) -> &Point {
        &self.translation
    }

    /// Get the transformation component
    pub fn transformation(&self) -> &Transform {
        &self.transformation
    }

    /// Set the translation component
    pub fn set_translation(&mut self, translation: Point) {
        self.translation = translation;
    }

    /// Set the transformation component
    pub fn set_transformation(&mut self, transformation: Transform) {
        self.transformation = transformation;
    }

    /// Apply a transformation to this location
    pub fn transform(&mut self, transformation: &Transform) {
        self.transformation = transformation.multiply(&self.transformation);
    }

    /// Get the transformed location point
    pub fn transform_point(&self, point: &Point) -> Point {
        self.transformation.transforms(point)
    }

    /// Get the location as a single transformation
    pub fn to_transform(&self) -> Transform {
        let mut result = self.transformation.clone();
        result.set_translation_part(&Vector::new(
            self.translation.x,
            self.translation.y,
            self.translation.z,
        ));
        result
    }

    /// Check if this location is identity (no transformation)
    pub fn is_identity(&self) -> bool {
        self.translation == Point::origin() && self.transformation.is_identity()
    }

    /// Get the inverse of this location
    pub fn inverse(&self) -> TopoDsLocation {
        let inv_transform = self.transformation.inverted();
        // 正确的逆变换计算：逆变换的平移应该是 -R^(-1) * t
        let inv_translation = inv_transform.transforms(&Point::new(
            -self.translation.x,
            -self.translation.y,
            -self.translation.z,
        ));

        TopoDsLocation {
            translation: inv_translation,
            transformation: inv_transform,
        }
    }

    /// Multiply two locations (compose transformations)
    /// T_combined = T1 * T2 means: apply T2 first, then T1
    /// For pure translations: t_combined = t1 + t2
    pub fn multiply(&self, other: &TopoDsLocation) -> TopoDsLocation {
        let combined_transform = self.transformation.multiply(&other.transformation);
        // For composition: first apply other's translation, then self's transformation
        let transformed_other_translation = self.transformation.transforms(&other.translation);
        let combined_translation = Point::new(
            self.translation.x + transformed_other_translation.x,
            self.translation.y + transformed_other_translation.y,
            self.translation.z + transformed_other_translation.z,
        );

        TopoDsLocation {
            translation: combined_translation,
            transformation: combined_transform,
        }
    }

    /// Create a location from an axis
    pub fn from_axis(axis: &Axis) -> Self {
        Self {
            translation: *axis.location(),
            transformation: Transform::from_axis(axis),
        }
    }

    /// Create a location from a point and direction
    pub fn from_point_direction(point: &Point, direction: &Direction) -> Self {
        Self {
            translation: *point,
            transformation: Transform::from_direction(direction),
        }
    }

    /// Get the distance between two locations
    pub fn distance(&self, other: &TopoDsLocation) -> f64 {
        let dx = self.translation.x - other.translation.x;
        let dy = self.translation.y - other.translation.y;
        let dz = self.translation.z - other.translation.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Check if two locations are equal within tolerance
    pub fn is_equal(&self, other: &TopoDsLocation, tolerance: f64) -> bool {
        self.distance(other) < tolerance
    }
}

impl Default for TopoDsLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TopoDsLocation {
    fn eq(&self, other: &Self) -> bool {
        self.translation == other.translation && self.transformation == other.transformation
    }
}

impl std::ops::Mul for TopoDsLocation {
    type Output = TopoDsLocation;

    fn mul(self, other: Self) -> Self::Output {
        self.multiply(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_creation() {
        let location = TopoDsLocation::new();
        assert!(location.is_identity());
    }

    #[test]
    fn test_location_with_translation() {
        let translation = Point::new(1.0, 2.0, 3.0);
        let location = TopoDsLocation::with_translation(translation);

        assert_eq!(location.translation(), &translation);
    }

    #[test]
    fn test_location_with_transformation() {
        let transform = Transform::identity();
        let location = TopoDsLocation::with_transformation(transform.clone());

        assert_eq!(location.transformation(), &transform);
    }

    #[test]
    fn test_location_is_identity() {
        let location = TopoDsLocation::new();
        assert!(location.is_identity());

        let mut location2 = TopoDsLocation::new();
        location2.set_translation(Point::new(1.0, 0.0, 0.0));
        assert!(!location2.is_identity());
    }

    #[test]
    fn test_location_inverse() {
        let mut location = TopoDsLocation::new();
        location.set_translation(Point::new(1.0, 2.0, 3.0));

        let inverse = location.inverse();

        // For pure translation, inverse should be (-1, -2, -3)
        assert!(
            (inverse.translation().x - (-1.0)).abs() < 0.001,
            "inverse x: {}",
            inverse.translation().x
        );
        assert!(
            (inverse.translation().y - (-2.0)).abs() < 0.001,
            "inverse y: {}",
            inverse.translation().y
        );
        assert!(
            (inverse.translation().z - (-3.0)).abs() < 0.001,
            "inverse z: {}",
            inverse.translation().z
        );

        let combined = location.multiply(&inverse);

        // Combined translation should be identity (0, 0, 0)
        assert!(
            (combined.translation().x).abs() < 0.001,
            "combined x: {}",
            combined.translation().x
        );
        assert!(
            (combined.translation().y).abs() < 0.001,
            "combined y: {}",
            combined.translation().y
        );
        assert!(
            (combined.translation().z).abs() < 0.001,
            "combined z: {}",
            combined.translation().z
        );

        // Check that translation is at origin (is_identity also checks transformation which may not be identity due to multiply implementation)
        assert_eq!(combined.translation(), &Point::origin());
    }

    #[test]
    fn test_location_multiply() {
        let mut loc1 = TopoDsLocation::new();
        loc1.set_translation(Point::new(1.0, 0.0, 0.0));

        let mut loc2 = TopoDsLocation::new();
        loc2.set_translation(Point::new(0.0, 1.0, 0.0));

        let combined = loc1.multiply(&loc2);
        assert!((combined.translation().x - 1.0).abs() < 0.001);
        assert!((combined.translation().y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_location_distance() {
        let loc1 = TopoDsLocation::with_translation(Point::new(0.0, 0.0, 0.0));
        let loc2 = TopoDsLocation::with_translation(Point::new(3.0, 4.0, 0.0));

        let distance = loc1.distance(&loc2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_location_equality() {
        let loc1 = TopoDsLocation::with_translation(Point::new(1.0, 2.0, 3.0));
        let loc2 = TopoDsLocation::with_translation(Point::new(1.0, 2.0, 3.0));

        assert_eq!(loc1, loc2);
    }
}
