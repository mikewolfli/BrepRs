use crate::foundation::handle::Handle;
use crate::foundation::StandardReal;
use crate::geometry::{
    axis::Axis, bounding_box::BoundingBox, cone::Cone, cylinder::Cylinder, plane::Plane,
    sphere::Sphere, Point,
};
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
use std::sync::Arc;

/// Head type for bioreactor vessel
#[derive(Debug, Clone, PartialEq)]
pub enum HeadType {
    /// Flat head
    Flat,
    /// Dish head (torispherical)
    Dish(StandardReal), // radius
    /// Elliptical head
    Elliptical(StandardReal, StandardReal), // major and minor axes
    /// Hemispherical head
    Hemispherical,
    /// Conical head
    Conical(StandardReal), // half angle in radians
}

/// Bioreactor vessel geometry
#[derive(Debug, Clone)]
pub struct BioreactorVessel {
    /// Cylinder section parameters
    pub cylinder_radius: StandardReal,
    pub cylinder_height: StandardReal,

    /// Top head type and parameters
    pub top_head: HeadType,

    /// Bottom head type and parameters
    pub bottom_head: HeadType,

    /// Vessel origin (center of bottom head)
    pub origin: Point,

    /// Vessel axis (vertical axis)
    pub axis: Axis,
}

impl BioreactorVessel {
    /// Create a new bioreactor vessel
    pub fn new(
        cylinder_radius: StandardReal,
        cylinder_height: StandardReal,
        top_head: HeadType,
        bottom_head: HeadType,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            cylinder_radius,
            cylinder_height,
            top_head,
            bottom_head,
            origin,
            axis,
        }
    }

    /// Create a vessel with elliptical heads (common configuration)
    pub fn with_elliptical_heads(
        cylinder_radius: StandardReal,
        cylinder_height: StandardReal,
        head_ratio: StandardReal, // 2:1 for standard elliptical head
        origin: Point,
        axis: Axis,
    ) -> Self {
        let top_head = HeadType::Elliptical(cylinder_radius * 2.0, cylinder_radius * head_ratio);
        let bottom_head = HeadType::Elliptical(cylinder_radius * 2.0, cylinder_radius * head_ratio);

        Self::new(
            cylinder_radius,
            cylinder_height,
            top_head,
            bottom_head,
            origin,
            axis,
        )
    }

    /// Create a vessel with dish heads
    pub fn with_dish_heads(
        cylinder_radius: StandardReal,
        cylinder_height: StandardReal,
        dish_radius: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        let top_head = HeadType::Dish(dish_radius);
        let bottom_head = HeadType::Dish(dish_radius);

        Self::new(
            cylinder_radius,
            cylinder_height,
            top_head,
            bottom_head,
            origin,
            axis,
        )
    }

    /// Create a vessel with hemispherical heads
    pub fn with_hemispherical_heads(
        cylinder_radius: StandardReal,
        cylinder_height: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        let top_head = HeadType::Hemispherical;
        let bottom_head = HeadType::Hemispherical;

        Self::new(
            cylinder_radius,
            cylinder_height,
            top_head,
            bottom_head,
            origin,
            axis,
        )
    }

    /// Create a vessel with flat heads
    pub fn with_flat_heads(
        cylinder_radius: StandardReal,
        cylinder_height: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        let top_head = HeadType::Flat;
        let bottom_head = HeadType::Flat;

        Self::new(
            cylinder_radius,
            cylinder_height,
            top_head,
            bottom_head,
            origin,
            axis,
        )
    }

    /// Calculate the total height of the vessel
    pub fn total_height(&self) -> StandardReal {
        let top_head_height = self.head_height(&self.top_head);
        let bottom_head_height = self.head_height(&self.bottom_head);

        top_head_height + self.cylinder_height + bottom_head_height
    }

    /// Calculate the height of a specific head
    fn head_height(&self, head: &HeadType) -> StandardReal {
        match head {
            HeadType::Flat => 0.0,
            HeadType::Dish(radius) => {
                // Dish head height: sqrt(radius² - cylinder_radius²)
                (radius * radius - self.cylinder_radius * self.cylinder_radius).sqrt()
            }
            HeadType::Elliptical(_, minor) => {
                // Elliptical head height: minor axis
                *minor
            }
            HeadType::Hemispherical => {
                // Hemispherical head height: cylinder radius
                self.cylinder_radius
            }
            HeadType::Conical(half_angle) => {
                // Conical head height: cylinder_radius / tan(half_angle)
                self.cylinder_radius / half_angle.tan()
            }
        }
    }

    /// Generate the vessel as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        // Create cylinder shell
        let cylinder = self.create_cylinder_shell();

        // Create top head shell
        let top_head = self.create_head_shell(&self.top_head, true);

        // Create bottom head shell
        let bottom_head = self.create_head_shell(&self.bottom_head, false);

        // Combine all shells into a solid
        let mut solid = TopoDsSolid::new();
        solid.add_shell(Handle::new(std::sync::Arc::new(cylinder)));
        solid.add_shell(Handle::new(std::sync::Arc::new(top_head)));
        solid.add_shell(Handle::new(std::sync::Arc::new(bottom_head)));

        solid
    }

    /// Create the cylinder section shell
    fn create_cylinder_shell(&self) -> TopoDsShell {
        let cylinder = Cylinder::from_axis(&self.axis, self.cylinder_radius);

        // Create shell from cylinder
        let mut shell = TopoDsShell::new();

        // Add lateral face
        let lateral_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
        )))));
        shell.add_face(lateral_face);

        // Add top and bottom faces (will be replaced by heads)
        let top_plane = Plane::from_point_normal(
            *self.axis.location() + self.axis.direction().to_vector() * self.cylinder_height,
            *self.axis.direction(),
        );
        let bottom_direction = self.axis.direction().reversed();
        let bottom_plane = Plane::from_point_normal(*self.axis.location(), bottom_direction);

        let top_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Plane(top_plane),
        )))));
        let bottom_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Plane(bottom_plane),
        )))));

        shell.add_face(top_face);
        shell.add_face(bottom_face);

        shell
    }

    /// Create a head shell
    fn create_head_shell(&self, head: &HeadType, is_top: bool) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Calculate head position
        let head_height = self.head_height(head);
        let head_origin = if is_top {
            *self.axis.location()
                + self.axis.direction().to_vector() * (self.cylinder_height + head_height / 2.0)
        } else {
            *self.axis.location() + self.axis.direction().to_vector() * (-head_height / 2.0)
        };

        match head {
            HeadType::Flat => {
                // Flat head is just a plane
                let plane = Plane::from_point_normal(
                    if is_top {
                        *self.axis.location()
                            + self.axis.direction().to_vector() * self.cylinder_height
                    } else {
                        *self.axis.location()
                    },
                    if is_top {
                        *self.axis.direction()
                    } else {
                        self.axis.direction().reversed()
                    },
                );

                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Plane(plane),
                )))));
                shell.add_face(face);
            }
            HeadType::Dish(radius) => {
                // Dish head is a portion of a sphere
                let sphere = Sphere::new(head_origin, *radius);

                // Create a cutting plane to get the dish shape
                let _cutting_plane = Plane::from_point_normal(
                    if is_top {
                        *self.axis.location()
                            + self.axis.direction().to_vector() * self.cylinder_height
                    } else {
                        *self.axis.location()
                    },
                    if is_top {
                        self.axis.direction().reversed()
                    } else {
                        *self.axis.direction()
                    },
                );

                // Cut the sphere with the plane to get the dish head
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
                )))));
                // Simplification: directly add sphere, actual cutting logic needs to be completed later
                shell.add_face(face);
            }
            HeadType::Elliptical(_, _) => {
                // Elliptical head
                // Simplification: use sphere approximation, actual implementation should construct ellipsoid surface
                let ellipsoid = Sphere::new(head_origin, self.cylinder_radius); // Ellipsoid needs custom type
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(ellipsoid),
                )))));
                shell.add_face(face);
            }
            HeadType::Hemispherical => {
                // Hemispherical head is half of a sphere
                let _sphere = Sphere::new(head_origin, self.cylinder_radius);

                // Create a cutting plane to get the hemisphere
                let _cutting_plane = Plane::from_point_normal(
                    if is_top {
                        *self.axis.location()
                            + self.axis.direction().to_vector() * self.cylinder_height
                    } else {
                        *self.axis.location()
                    },
                    if is_top {
                        self.axis.direction().reversed()
                    } else {
                        *self.axis.direction()
                    },
                );

                // Create a sphere solid
                let _sphere_solid = crate::modeling::primitives::make_sphere(
                    self.cylinder_radius,
                    Some(head_origin),
                );

                // Create cutting plane for hemisphere
                let plane_origin = head_origin;
                let plane_normal = if is_top {
                    self.axis.direction().reversed()
                } else {
                    *self.axis.direction()
                };
                let _plane = Plane::new(plane_origin, plane_normal, plane_normal);

                let _plane_solid = crate::modeling::primitives::make_box(
                    4.0 * self.cylinder_radius, // Width
                    4.0 * self.cylinder_radius, // Depth
                    2.0 * self.cylinder_radius, // Height
                    Some(Point::new(
                        head_origin.x() - 0.0,
                        head_origin.y() - 0.0,
                        head_origin.z() - self.cylinder_radius,
                    )),
                );

                // Cut the sphere to get hemisphere
                let _boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();

                // For simplicity, just create a new shell
                shell = TopoDsShell::new();
            }
            HeadType::Conical(half_angle) => {
                // Conical head
                let direction = if is_top {
                    *self.axis.direction()
                } else {
                    self.axis.direction().reversed()
                };
                let cone = Cone::new(head_origin, direction, *half_angle, self.cylinder_radius);

                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cone(cone),
                )))));
                shell.add_face(face);
            }
        }

        shell
    }

    /// Calculate the vessel's bounding box
    pub fn bounding_box(&self) -> BoundingBox {
        let total_height = self.total_height();
        let max_radius = self.cylinder_radius;

        let min_point = Point::new(
            self.origin.x - max_radius,
            self.origin.y - max_radius,
            self.origin.z,
        );

        let max_point = Point::new(
            self.origin.x + max_radius,
            self.origin.y + max_radius,
            self.origin.z + total_height,
        );

        BoundingBox::new(min_point, max_point)
    }

    /// Calculate the vessel's volume
    pub fn volume(&self) -> StandardReal {
        // Cylinder volume
        let cylinder_volume = std::f64::consts::PI
            * self.cylinder_radius
            * self.cylinder_radius
            * self.cylinder_height;

        // Top head volume
        let top_head_volume = self.head_volume(&self.top_head);

        // Bottom head volume
        let bottom_head_volume = self.head_volume(&self.bottom_head);

        cylinder_volume + top_head_volume + bottom_head_volume
    }

    /// Calculate the volume of a specific head
    fn head_volume(&self, head: &HeadType) -> StandardReal {
        match head {
            HeadType::Flat => 0.0,
            HeadType::Dish(_radius) => {
                // Dish head volume approximation
                let h = self.head_height(head);
                std::f64::consts::PI
                    * h
                    * (3.0 * self.cylinder_radius * self.cylinder_radius + h * h)
                    / 6.0
            }
            HeadType::Elliptical(_major, _minor) => {
                // Elliptical head volume
                std::f64::consts::PI
                    * self.cylinder_radius
                    * self.cylinder_radius
                    * self.cylinder_radius
                    * 2.0
                    / 3.0
            }
            HeadType::Hemispherical => {
                // Hemispherical head volume
                (2.0 / 3.0)
                    * std::f64::consts::PI
                    * self.cylinder_radius
                    * self.cylinder_radius
                    * self.cylinder_radius
            }
            HeadType::Conical(_half_angle) => {
                // Conical head volume
                let h = self.head_height(head);
                (1.0 / 3.0) * std::f64::consts::PI * self.cylinder_radius * self.cylinder_radius * h
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{axis::Axis, Direction, Point, Vector};

    #[test]
    fn test_vessel_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let vessel = BioreactorVessel::with_elliptical_heads(
            1.0, 2.0, 2.0, // 2:1 elliptical head
            origin, axis,
        );

        assert_eq!(vessel.cylinder_radius, 1.0);
        assert_eq!(vessel.cylinder_height, 2.0);
        assert!(matches!(vessel.top_head, HeadType::Elliptical(_, _)));
        assert!(matches!(vessel.bottom_head, HeadType::Elliptical(_, _)));
    }

    #[test]
    fn test_vessel_total_height() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let vessel = BioreactorVessel::with_hemispherical_heads(1.0, 2.0, origin, axis);

        // Total height: 1 (bottom head) + 2 (cylinder) + 1 (top head) = 4
        assert!((vessel.total_height() - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_vessel_volume() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let vessel = BioreactorVessel::with_flat_heads(1.0, 2.0, origin, axis);

        // Cylinder volume: π * r² * h = π * 1 * 2 = 2π
        let expected_volume = 2.0 * std::f64::consts::PI;
        assert!((vessel.volume() - expected_volume).abs() < 1e-6);
    }
}
