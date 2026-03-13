use crate::foundation::types::StandardReal;
use crate::geometry::{Axis, Direction, Point, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cone {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
    angle: StandardReal,
    radius: StandardReal,
}

impl Cone {
    pub fn new(
        location: Point,
        direction: Direction,
        angle: StandardReal,
        radius: StandardReal,
    ) -> Self {
        let x_dir = if direction.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            direction.cross(&Direction::z_axis()).normalized()
        };
        let y_dir = direction.cross(&x_dir).normalized();

        Self {
            location,
            direction,
            x_direction: x_dir,
            y_direction: y_dir,
            angle,
            radius,
        }
    }

    pub fn from_axis(
        axis: &Axis,
        angle: StandardReal,
        radius: StandardReal,
    ) -> Self {
        let main_dir = axis.direction();
        let x_dir = if main_dir.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            main_dir.cross(&Direction::z_axis()).normalized()
        };
        let y_dir = main_dir.cross(&x_dir).normalized();

        Self {
            location: *axis.location(),
            direction: *main_dir,
            x_direction: x_dir,
            y_direction: y_dir,
            angle,
            radius,
        }
    }

    pub fn from_point_axis_angle_radius(
        location: Point,
        direction: Direction,
        angle: StandardReal,
        radius: StandardReal,
    ) -> Self {
        Self::new(location, direction, angle, radius)
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn x_direction(&self) -> &Direction {
        &self.x_direction
    }

    pub fn y_direction(&self) -> &Direction {
        &self.y_direction
    }

    pub fn angle(&self) -> StandardReal {
        self.angle
    }

    pub fn radius(&self) -> StandardReal {
        self.radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.update_x_y_directions();
    }

    pub fn set_angle(&mut self, angle: StandardReal) {
        self.angle = angle;
    }

    pub fn set_radius(&mut self, radius: StandardReal) {
        self.radius = radius;
    }

    fn update_x_y_directions(&mut self) {
        self.x_direction = if self.direction.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            self.direction.cross(&Direction::z_axis()).normalized()
        };
        self.y_direction = self.direction.cross(&self.x_direction).normalized();
    }

    pub fn axis(&self) -> Axis {
        Axis::new(self.location, self.direction)
    }

    pub fn apex(&self) -> Point {
        let height = self.radius / self.angle.tan();
        let dir_vec = crate::geometry::Vector::new(
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        Point::new(
            self.location.x - height * dir_vec.x,
            self.location.y - height * dir_vec.y,
            self.location.z - height * dir_vec.z,
        )
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        let distance = self.distance(point);
        distance <= tolerance
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let dir_vec = crate::geometry::Vector::new(
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );

        let projection = vec.dot(&dir_vec);
        let projected_point = Point::new(
            self.location.x + projection * dir_vec.x,
            self.location.y + projection * dir_vec.y,
            self.location.z + projection * dir_vec.z,
        );

        let center_to_point = crate::geometry::Vector::from_point(&self.location, &projected_point);
        let distance_from_axis = center_to_point.magnitude();

        let expected_radius = self.radius + projection * self.angle.tan();
        (distance_from_axis - expected_radius).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn area(&self) -> StandardReal {
        let slant_height = self.radius / self.angle.sin();
        std::f64::consts::PI * self.radius * slant_height
    }

    pub fn volume(&self) -> StandardReal {
        let height = self.radius / self.angle.tan();
        (1.0 / 3.0) * std::f64::consts::PI * self.radius * self.radius * height
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Cone {
        Cone {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
            angle: self.angle,
            radius: self.radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Cone {
        Cone {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
            angle: self.angle,
            radius: self.radius,
        }
    }

    pub fn scale(&mut self, point: &Point, scale: StandardReal) {
        self.location.scale(point, scale);
        self.radius *= scale.abs();
    }

    pub fn scaled(&self, point: &Point, scale: StandardReal) -> Cone {
        Cone {
            location: self.location.scaled(point, scale),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            angle: self.angle,
            radius: self.radius * scale.abs(),
        }
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.location.transform(transform);
        self.direction.transform(transform);
        self.x_direction.transform(transform);
        self.y_direction.transform(transform);
    }

    pub fn transformed(&self, transform: &Transform) -> Cone {
        Cone {
            location: self.location.transformed(transform),
            direction: self.direction.transformed(transform),
            x_direction: self.x_direction.transformed(transform),
            y_direction: self.y_direction.transformed(transform),
            angle: self.angle,
            radius: self.radius,
        }
    }

    pub fn translate(&mut self, vector: &crate::geometry::Vector) {
        self.location.translate(vector);
    }

    pub fn translated(&self, vector: &crate::geometry::Vector) -> Cone {
        Cone {
            location: self.location.translated(vector),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            angle: self.angle,
            radius: self.radius,
        }
    }

    pub fn translate_by_coords(&mut self, dx: StandardReal, dy: StandardReal, dz: StandardReal) {
        self.location.translate_by_coords(dx, dy, dz);
    }

    pub fn translated_by_coords(&self, dx: StandardReal, dy: StandardReal, dz: StandardReal) -> Cone {
        Cone {
            location: self.location.translated_by_coords(dx, dy, dz),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            angle: self.angle,
            radius: self.radius,
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(
            Point::origin(),
            Direction::z_axis(),
            std::f64::consts::PI / 6.0,
            1.0,
        )
    }
}

impl crate::topology::topods_face::Surface for Cone {
    fn value(&self, u: f64, v: f64) -> Point {
        let height = v * self.radius / self.angle.tan();
        let radius_at_height = self.radius - v * self.radius;
        
        let cos_u = u.cos();
        let sin_u = u.sin();
        
        let x_vec = crate::geometry::Vector::new(
            self.x_direction.x,
            self.x_direction.y,
            self.x_direction.z,
        );
        let y_vec = crate::geometry::Vector::new(
            self.y_direction.x,
            self.y_direction.y,
            self.y_direction.z,
        );
        let dir_vec = crate::geometry::Vector::new(
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        
        let point = self.location
            + dir_vec * height
            + x_vec * (radius_at_height * cos_u)
            + y_vec * (radius_at_height * sin_u);
        
        point
    }

    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector {
        let _height = v * self.radius / self.angle.tan();
        let radius_at_height = self.radius - v * self.radius;
        
        let cos_u = u.cos();
        let sin_u = u.sin();
        
        let x_vec = crate::geometry::Vector::new(
            self.x_direction.x,
            self.x_direction.y,
            self.x_direction.z,
        );
        let y_vec = crate::geometry::Vector::new(
            self.y_direction.x,
            self.y_direction.y,
            self.y_direction.z,
        );
        let dir_vec = crate::geometry::Vector::new(
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        
        // Calculate normal vector
        let tangent_u = x_vec * (-radius_at_height * sin_u) + y_vec * (radius_at_height * cos_u);
        let tangent_v = dir_vec * (self.radius / self.angle.tan()) - (x_vec * cos_u + y_vec * sin_u) * self.radius;
        
        let normal = tangent_u.cross(&tangent_v).normalized();
        normal
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        ((0.0, 2.0 * std::f64::consts::PI), (0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cone_creation() {
        let cone = Cone::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::z_axis(),
            std::f64::consts::PI / 6.0,
            1.0,
        );
        assert_eq!(cone.radius(), 1.0);
        assert_eq!(cone.angle(), std::f64::consts::PI / 6.0);
    }

    #[test]
    fn test_cone_apex() {
        let cone = Cone::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::z_axis(),
            std::f64::consts::PI / 4.0,
            1.0,
        );
        let apex = cone.apex();
        assert!(apex.z < 0.0); // Apex should be below the base
    }

    #[test]
    fn test_cone_volume() {
        let cone = Cone::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::z_axis(),
            std::f64::consts::PI / 4.0,
            1.0,
        );
        let volume = cone.volume();
        assert!(volume > 0.0);
    }
}
