use crate::foundation::handle::Handle;
use crate::foundation::StandardReal;

use crate::geometry::{axis::Axis, cone::Cone, cylinder::Cylinder, Point, Vector};
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
use std::sync::Arc;

/// Elbow geometry (pipe bend)
#[derive(Debug, Clone)]
pub struct Elbow {
    /// Pipe diameter
    pub diameter: StandardReal,
    /// Bend radius
    pub bend_radius: StandardReal,
    /// Bend angle (radians)
    pub bend_angle: StandardReal,
    /// Wall thickness
    pub wall_thickness: StandardReal,
    /// Origin (center of bend)
    pub origin: Point,
    /// Axis (bend axis)
    pub axis: Axis,
}

/// Reducer geometry (conical pipe transition)
#[derive(Debug, Clone)]
pub struct Reducer {
    /// Inlet diameter
    pub inlet_diameter: StandardReal,
    /// Outlet diameter
    pub outlet_diameter: StandardReal,
    /// Length
    pub length: StandardReal,
    /// Wall thickness
    pub wall_thickness: StandardReal,
    /// Origin (inlet center)
    pub origin: Point,
    /// Axis
    pub axis: Axis,
}

/// Flange geometry (pipe connection)
#[derive(Debug, Clone)]
pub struct Flange {
    /// Pipe diameter
    pub pipe_diameter: StandardReal,
    /// Flange diameter
    pub flange_diameter: StandardReal,
    /// Flange thickness
    pub flange_thickness: StandardReal,
    /// Number of bolt holes
    pub bolt_hole_count: usize,
    /// Bolt hole diameter
    pub bolt_hole_diameter: StandardReal,
    /// Bolt circle diameter
    pub bolt_circle_diameter: StandardReal,
    /// Origin (flange center)
    pub origin: Point,
    /// Axis
    pub axis: Axis,
}

/// Joint geometry (pipe connection)
#[derive(Debug, Clone)]
pub struct Joint {
    /// Joint type
    pub joint_type: JointType,
    /// Pipe diameter
    pub pipe_diameter: StandardReal,
    /// Joint length
    pub length: StandardReal,
    /// Wall thickness
    pub wall_thickness: StandardReal,
    /// Origin (joint center)
    pub origin: Point,
    /// Axis
    pub axis: Axis,
}

/// Joint type
#[derive(Debug, Clone, PartialEq)]
pub enum JointType {
    /// Socket weld joint
    SocketWeld,
    /// Threaded joint
    Threaded,
    /// Butt weld joint
    ButtWeld,
    /// Flanged joint
    Flanged,
}

impl Elbow {
    /// Create a new elbow
    pub fn new(
        diameter: StandardReal,
        bend_radius: StandardReal,
        bend_angle: StandardReal,
        wall_thickness: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            diameter,
            bend_radius,
            bend_angle,
            wall_thickness,
            origin,
            axis,
        }
    }

    /// Generate the elbow as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // TODO: Implement proper elbow geometry
        // For now, create a simple cylinder as placeholder
        let cylinder = Cylinder::from_axis(
            &self.axis,
            self.diameter / 2.0,
        );

        let mut shell = TopoDsShell::new();
        let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
        )))));
        shell.add_face(face);

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl Reducer {
    /// Create a new reducer
    pub fn new(
        inlet_diameter: StandardReal,
        outlet_diameter: StandardReal,
        length: StandardReal,
        wall_thickness: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            inlet_diameter,
            outlet_diameter,
            length,
            wall_thickness,
            origin,
            axis,
        }
    }

    /// Generate the reducer as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create outer cone
        let outer_cone = Cone::from_axis(
            &self.axis,
            self.inlet_diameter / 2.0,
            self.outlet_diameter / 2.0,
        );

        // Create inner cone (hollow)
        let inner_cone = Cone::from_axis(
            &self.axis,
            (self.inlet_diameter - 2.0 * self.wall_thickness) / 2.0,
            (self.outlet_diameter - 2.0 * self.wall_thickness) / 2.0,
        );

        let mut shell = TopoDsShell::new();
        let outer_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cone(outer_cone),
        )))));

        let inner_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cone(inner_cone),
        )))));

        shell.add_face(outer_face);
        shell.add_face(inner_face);

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl Flange {
    /// Create a new flange
    pub fn new(
        pipe_diameter: StandardReal,
        flange_diameter: StandardReal,
        flange_thickness: StandardReal,
        bolt_hole_count: usize,
        bolt_hole_diameter: StandardReal,
        bolt_circle_diameter: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            pipe_diameter,
            flange_diameter,
            flange_thickness,
            bolt_hole_count,
            bolt_hole_diameter,
            bolt_circle_diameter,
            origin,
            axis,
        }
    }

    /// Generate the flange as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create flange body
        let flange_cylinder =
            Cylinder::from_axis(&self.axis, self.flange_diameter / 2.0);

        // Create pipe hole
        let pipe_hole_cylinder =
            Cylinder::from_axis(&self.axis, self.pipe_diameter / 2.0);

        let mut shell = TopoDsShell::new();
        let flange_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(flange_cylinder),
        )))));
        let pipe_hole_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(pipe_hole_cylinder),
        )))));

        shell.add_face(flange_face);
        shell.add_face(pipe_hole_face);

        // Add bolt holes
        let hole_angle_step = 2.0 * std::f64::consts::PI / self.bolt_hole_count as StandardReal;

        for i in 0..self.bolt_hole_count {
            let angle = i as StandardReal * hole_angle_step;
            let hole_origin = self.origin
                + Vector::new(
                    angle.cos() * self.bolt_circle_diameter / 2.0,
                    angle.sin() * self.bolt_circle_diameter / 2.0,
                    0.0,
                );

            let hole_cylinder = Cylinder::from_axis(
                &Axis::new(hole_origin, *self.axis.direction()),
                self.bolt_hole_diameter / 2.0,
            );

            let hole_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                crate::geometry::surface_enum::SurfaceEnum::Cylinder(hole_cylinder),
            )))));
            shell.add_face(hole_face);
        }

        solid.add_shell(Handle::new(Arc::new(shell)));
        solid
    }
}

impl Joint {
    /// Create a new joint
    pub fn new(
        joint_type: JointType,
        pipe_diameter: StandardReal,
        length: StandardReal,
        wall_thickness: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        Self {
            joint_type,
            pipe_diameter,
            length,
            wall_thickness,
            origin,
            axis,
        }
    }

    /// Generate the joint as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        match self.joint_type {
            JointType::SocketWeld => {
                // Socket weld joint has a socket for the pipe
                let socket_cylinder = Cylinder::from_axis(
                    &self.axis,
                    (self.pipe_diameter + 0.01) / 2.0, // Slightly larger than pipe
                );

                let pipe_cylinder = Cylinder::from_axis(&self.axis, self.pipe_diameter / 2.0);

                let mut shell = TopoDsShell::new();
                let socket_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(socket_cylinder),
                )))));
                let pipe_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(pipe_cylinder),
                )))));

                shell.add_face(socket_face);
                shell.add_face(pipe_face);

                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            JointType::Threaded => {
                // Threaded joint has external threads
                let joint_cylinder = Cylinder::from_axis(
                    &self.axis,
                    (self.pipe_diameter + 0.02) / 2.0, // Slightly larger for threads
                );

                let mut shell = TopoDsShell::new();
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(joint_cylinder),
                )))));
                shell.add_face(face);

                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            JointType::ButtWeld => {
                // Butt weld joint is a simple cylinder
                let joint_cylinder =
                    Cylinder::from_axis(&self.axis, self.pipe_diameter / 2.0);

                let mut shell = TopoDsShell::new();
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(joint_cylinder),
                )))));
                shell.add_face(face);

                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            JointType::Flanged => {
                // Flanged joint uses a flange
                let flange = Flange::new(
                    self.pipe_diameter,
                    self.pipe_diameter * 2.0,
                    self.length / 2.0,
                    4,
                    0.01,
                    self.pipe_diameter * 1.5,
                    self.origin,
                    self.axis,
                );

                let flange_solid = flange.to_solid();
                solid = flange_solid;
            }
        }

        solid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{axis::Axis, Direction, Point, Vector};

    #[test]
    fn test_elbow_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let elbow = Elbow::new(0.1, 0.2, std::f64::consts::PI / 2.0, 0.01, origin, axis);

        assert_eq!(elbow.diameter, 0.1);
        assert_eq!(elbow.bend_radius, 0.2);
        assert_eq!(elbow.bend_angle, std::f64::consts::PI / 2.0);
        assert_eq!(elbow.wall_thickness, 0.01);
    }

    #[test]
    fn test_reducer_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let reducer = Reducer::new(0.2, 0.1, 0.3, 0.01, origin, axis);

        assert_eq!(reducer.inlet_diameter, 0.2);
        assert_eq!(reducer.outlet_diameter, 0.1);
        assert_eq!(reducer.length, 0.3);
        assert_eq!(reducer.wall_thickness, 0.01);
    }

    #[test]
    fn test_flange_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let flange = Flange::new(0.1, 0.2, 0.05, 4, 0.01, 0.15, origin, axis);

        assert_eq!(flange.pipe_diameter, 0.1);
        assert_eq!(flange.flange_diameter, 0.2);
        assert_eq!(flange.flange_thickness, 0.05);
        assert_eq!(flange.bolt_hole_count, 4);
        assert_eq!(flange.bolt_hole_diameter, 0.01);
        assert_eq!(flange.bolt_circle_diameter, 0.15);
    }

    #[test]
    fn test_joint_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(origin, Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)));

        let joint = Joint::new(JointType::SocketWeld, 0.1, 0.1, 0.01, origin, axis);

        assert!(matches!(joint.joint_type, JointType::SocketWeld));
        assert_eq!(joint.pipe_diameter, 0.1);
        assert_eq!(joint.length, 0.1);
        assert_eq!(joint.wall_thickness, 0.01);
    }
}
