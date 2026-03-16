use crate::foundation::handle::Handle;
use crate::foundation::StandardReal;

use crate::geometry::{axis::Axis, cylinder::Cylinder, sphere::Sphere, Point, Vector, Direction};
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
use std::sync::Arc;

/// Draft tube geometry (used to direct flow in bioreactors)
#[derive(Debug, Clone)]
pub struct DraftTube {
    /// Draft tube diameter
    pub diameter: StandardReal,
    /// Draft tube height
    pub height: StandardReal,
    /// Bottom clearance (distance from vessel bottom)
    pub bottom_clearance: StandardReal,
    /// Wall thickness
    pub wall_thickness: StandardReal,
    /// Origin (bottom center)
    pub origin: Point,
    /// Axis
    pub axis: Axis,
}

/// Baffle geometry (used to break up flow patterns in bioreactors)
#[derive(Debug, Clone)]
pub struct Baffle {
    /// Baffle width
    pub width: StandardReal,
    /// Baffle height
    pub height: StandardReal,
    /// Baffle thickness
    pub thickness: StandardReal,
    /// Distance from vessel wall
    pub distance_from_wall: StandardReal,
    /// Angle position (radians from reference direction)
    pub angle_position: StandardReal,
    /// Origin (bottom center)
    pub origin: Point,
    /// Axis (vertical axis)
    pub axis: Axis,
}

/// Spray arm geometry (used for liquid distribution)
#[derive(Debug, Clone)]
pub struct SprayArm {
    /// Arm length
    pub length: StandardReal,
    /// Arm diameter
    pub diameter: StandardReal,
    /// Number of nozzles
    pub nozzle_count: usize,
    /// Nozzle diameter
    pub nozzle_diameter: StandardReal,
    /// Nozzle spacing
    pub nozzle_spacing: StandardReal,
    /// Rotation speed (rpm)
    pub rotation_speed: StandardReal,
    /// Origin (center of rotation)
    pub origin: Point,
    /// Axis (rotation axis)
    pub axis: Axis,
}

/// Aeration nozzle geometry (used for gas sparging)
#[derive(Debug, Clone)]
pub struct AerationNozzle {
    /// Nozzle type
    pub nozzle_type: NozzleType,
    /// Nozzle diameter
    pub diameter: StandardReal,
    /// Number of holes
    pub hole_count: usize,
    /// Hole diameter
    pub hole_diameter: StandardReal,
    /// Origin (nozzle center)
    pub origin: Point,
    /// Axis (nozzle direction)
    pub axis: Axis,
}

/// Nozzle type for aeration
#[derive(Debug, Clone, PartialEq)]
pub enum NozzleType {
    /// Simple orifice
    Orifice,
    /// Frit (porous material)
    Frit,
    /// Sparger (multiple holes)
    Sparger,
    /// Lateral (side-facing holes)
    Lateral,
}

impl DraftTube {
    /// Create a new draft tube
    pub fn new(
        diameter: StandardReal,
        height: StandardReal,
        bottom_clearance: StandardReal,
        wall_thickness: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            diameter,
            height,
            bottom_clearance,
            wall_thickness,
            origin,
            axis,
        }
    }

    /// Generate the draft tube as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create outer cylinder
        let outer_cylinder = Cylinder::new(
            self.origin + self.axis.direction().to_vector() * self.bottom_clearance,
            *self.axis.direction(),
            self.diameter / 2.0,
        );

        // Create inner cylinder (hollow)
        let inner_cylinder = Cylinder::new(
            self.origin + self.axis.direction().to_vector() * self.bottom_clearance,
            *self.axis.direction(),
            (self.diameter - 2.0 * self.wall_thickness) / 2.0,
        );

        // Create shell from cylinders
        let mut shell = TopoDsShell::new();
        let outer_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(outer_cylinder),
        )))));
        let inner_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(inner_cylinder),
        )))));

        shell.add_face(outer_face);
        shell.add_face(inner_face);

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl Baffle {
    /// Create a new baffle
    pub fn new(
        width: StandardReal,
        height: StandardReal,
        thickness: StandardReal,
        distance_from_wall: StandardReal,
        angle_position: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            width,
            height,
            thickness,
            distance_from_wall,
            angle_position,
            origin,
            axis,
        }
    }

    /// Generate the baffle as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Calculate baffle position
        let baffle_origin = self.origin
            + Vector::new(self.angle_position.cos(), self.angle_position.sin(), 0.0)
                * self.distance_from_wall;

        // Create baffle as a rectangular prism
        // TODO: Implement proper baffle geometry
        let baffle_cylinder = Cylinder::new(
            baffle_origin,
            Direction::new(
                -self.angle_position.sin(),
                self.angle_position.cos(),
                0.0,
            ),
            self.thickness / 2.0,
        );

        let mut shell = TopoDsShell::new();
        let face = TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(baffle_cylinder),
        )));
        shell.add_face(Handle::new(Arc::new(face)));

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl SprayArm {
    /// Create a new spray arm
    pub fn new(
        length: StandardReal,
        diameter: StandardReal,
        nozzle_count: usize,
        nozzle_diameter: StandardReal,
        nozzle_spacing: StandardReal,
        rotation_speed: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            length,
            diameter,
            nozzle_count,
            nozzle_diameter,
            nozzle_spacing,
            rotation_speed,
            origin,
            axis,
        }
    }

    /// Generate the spray arm as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create arm
        let arm_cylinder = Cylinder::new(
            self.origin,
            Direction::new(1.0, 0.0, 0.0), // Horizontal arm
            self.diameter / 2.0,
        );

        let mut shell = TopoDsShell::new();
        let arm_face = TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(arm_cylinder),
        )));
        shell.add_face(Handle::new(Arc::new(arm_face)));

        // Add nozzles
        for i in 0..self.nozzle_count {
            let nozzle_position =
                self.origin + Vector::new((i + 1) as StandardReal * self.nozzle_spacing, 0.0, 0.0);

            let nozzle_cylinder = Cylinder::new(
                nozzle_position,
                Direction::new(0.0, 0.0, -1.0), // Downward facing
                self.nozzle_diameter / 2.0,
            );

            let nozzle_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                crate::geometry::surface_enum::SurfaceEnum::Cylinder(nozzle_cylinder),
            )));
            shell.add_face(Handle::new(Arc::new(nozzle_face)));
        }

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl AerationNozzle {
    /// Create a new aeration nozzle
    pub fn new(
        nozzle_type: NozzleType,
        diameter: StandardReal,
        hole_count: usize,
        hole_diameter: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            nozzle_type,
            diameter,
            hole_count,
            hole_diameter,
            origin,
            axis,
        }
    }

    /// Generate the aeration nozzle as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create nozzle body
        let nozzle_cylinder = Cylinder::new(
            *self.axis.location(),
            *self.axis.direction(),
            self.diameter / 2.0,
        );

        let mut shell = TopoDsShell::new();
        let nozzle_face = TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(nozzle_cylinder),
        )));
        shell.add_face(Handle::new(Arc::new(nozzle_face)));

        // Add holes based on nozzle type
        match self.nozzle_type {
            NozzleType::Orifice => {
                // Single central hole
                let hole_cylinder = Cylinder::new(
                    self.origin,
                    *self.axis.direction(),
                    self.hole_diameter / 2.0,
                );
                let hole_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(hole_cylinder),
                )));
                shell.add_face(Handle::new(Arc::new(hole_face)));
            }
            NozzleType::Sparger => {
                // Multiple holes around the nozzle
                let hole_angle_step = 2.0 * std::f64::consts::PI / self.hole_count as StandardReal;

                for i in 0..self.hole_count {
                    let angle = i as StandardReal * hole_angle_step;
                    let hole_axis = Axis::new(
                        self.origin,
                        crate::geometry::direction::Direction::new(
                            angle.cos() * self.axis.direction().x
                                - angle.sin() * self.axis.direction().y,
                            angle.sin() * self.axis.direction().x
                                + angle.cos() * self.axis.direction().y,
                            self.axis.direction().z,
                        ),
                    );

                    let hole_cylinder = Cylinder::new(
                        *hole_axis.location(),
                        *hole_axis.direction(),
                        self.hole_diameter / 2.0,
                    );

                    let hole_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                        crate::geometry::surface_enum::SurfaceEnum::Cylinder(hole_cylinder),
                    )));
                    shell.add_face(Handle::new(Arc::new(hole_face)));
                }
            }
            NozzleType::Lateral => {
                // Side-facing holes
                let hole_angle_step = 2.0 * std::f64::consts::PI / self.hole_count as StandardReal;

                for i in 0..self.hole_count {
                    let angle = i as StandardReal * hole_angle_step;
                    let hole_axis = Axis::new(
                        self.origin
                            + Vector::new(
                                self.axis.direction().x,
                                self.axis.direction().y,
                                self.axis.direction().z,
                            ) * self.diameter,
                        crate::geometry::direction::Direction::new(angle.cos(), angle.sin(), 0.0),
                    );

                    let hole_cylinder = Cylinder::new(
                        *hole_axis.location(),
                        *hole_axis.direction(),
                        self.hole_diameter / 2.0,
                    );

                    let hole_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                        crate::geometry::surface_enum::SurfaceEnum::Cylinder(hole_cylinder),
                    )));
                    shell.add_face(Handle::new(Arc::new(hole_face)));
                }
            }
            NozzleType::Frit => {
                // Porous material representation (simplified as solid)
                let frit_sphere = Sphere::new(
                    self.origin + self.axis.direction().to_vector() * self.diameter,
                    self.diameter / 2.0,
                );

                let frit_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(frit_sphere),
                )));
                shell.add_face(Handle::new(Arc::new(frit_face)));
            }
        }

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{axis::Axis, Direction, Point, Vector};

    #[test]
    fn test_draft_tube_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let draft_tube = DraftTube::new(1.0, 2.0, 0.2, 0.05, origin, axis);

        assert_eq!(draft_tube.diameter, 1.0);
        assert_eq!(draft_tube.height, 2.0);
        assert_eq!(draft_tube.bottom_clearance, 0.2);
        assert_eq!(draft_tube.wall_thickness, 0.05);
    }

    #[test]
    fn test_baffle_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let baffle = Baffle::new(0.1, 2.0, 0.02, 0.05, 0.0, origin, axis);

        assert_eq!(baffle.width, 0.1);
        assert_eq!(baffle.height, 2.0);
        assert_eq!(baffle.thickness, 0.02);
        assert_eq!(baffle.distance_from_wall, 0.05);
        assert_eq!(baffle.angle_position, 0.0);
    }

    #[test]
    fn test_spray_arm_creation() {
        let origin = Point::new(0.0, 0.0, 2.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let spray_arm = SprayArm::new(1.0, 0.05, 5, 0.01, 0.2, 10.0, origin, axis);

        assert_eq!(spray_arm.length, 1.0);
        assert_eq!(spray_arm.diameter, 0.05);
        assert_eq!(spray_arm.nozzle_count, 5);
        assert_eq!(spray_arm.nozzle_diameter, 0.01);
        assert_eq!(spray_arm.nozzle_spacing, 0.2);
        assert_eq!(spray_arm.rotation_speed, 10.0);
    }

    #[test]
    fn test_aeration_nozzle_creation() {
        let origin = Point::new(0.0, 0.0, 0.1);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let nozzle = AerationNozzle::new(NozzleType::Sparger, 0.05, 8, 0.005, origin, axis);

        assert!(matches!(nozzle.nozzle_type, NozzleType::Sparger));
        assert_eq!(nozzle.diameter, 0.05);
        assert_eq!(nozzle.hole_count, 8);
        assert_eq!(nozzle.hole_diameter, 0.005);
    }
}
