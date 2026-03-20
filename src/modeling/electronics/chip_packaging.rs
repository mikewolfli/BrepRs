use crate::foundation::StandardReal;
use crate::geometry::{cylinder::Cylinder, sphere::Sphere, Point, Vector};
use crate::topology::TopoDsSolid;

/// Chip package type
#[derive(Debug, Clone, PartialEq)]
pub enum PackageType {
    /// Ball Grid Array
    BGA,
    /// Quad Flat Package
    QFP,
    /// Dual In-line Package
    DIP,
    /// Small Outline Integrated Circuit
    SOIC,
    /// Chip Scale Package
    CSP,
    /// Flip Chip
    FlipChip,
}

/// BGA solder ball geometry
#[derive(Debug, Clone)]
pub struct SolderBall {
    /// Ball diameter
    pub diameter: StandardReal,
    /// Ball position
    pub position: Point,
    /// Ball height
    pub height: StandardReal,
}

/// Pin geometry
#[derive(Debug, Clone)]
pub struct Pin {
    /// Pin type
    pub pin_type: PinType,
    /// Pin width
    pub width: StandardReal,
    /// Pin length
    pub length: StandardReal,
    /// Pin thickness
    pub thickness: StandardReal,
    /// Pin position
    pub position: Point,
    /// Pin orientation
    pub orientation: Vector,
}

/// Pin type
#[derive(Debug, Clone, PartialEq)]
pub enum PinType {
    /// Through-hole pin
    ThroughHole,
    /// Surface mount pin
    SurfaceMount,
    /// BGA ball
    BGABall,
    /// Lead frame pin
    LeadFrame,
}

/// Chip package geometry
#[derive(Debug, Clone)]
pub struct ChipPackage {
    /// Package type
    pub package_type: PackageType,
    /// Package dimensions (width, length, height)
    pub dimensions: (StandardReal, StandardReal, StandardReal),
    /// Die dimensions (width, length, height)
    pub die_dimensions: (StandardReal, StandardReal, StandardReal),
    /// Pins or balls
    pub pins: Vec<Pin>,
    /// Solder balls (for BGA)
    pub solder_balls: Vec<SolderBall>,
    /// Origin (package center)
    pub origin: Point,
}

impl SolderBall {
    /// Create a new solder ball
    pub fn new(diameter: StandardReal, position: Point, height: StandardReal) -> Self {
        Self {
            diameter,
            position,
            height,
        }
    }

    /// Generate the solder ball as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        use crate::geometry::surface_enum::SurfaceEnum;
        use crate::handle::Handle;
        use crate::topology::{TopoDsFace, TopoDsShell};
        use std::sync::Arc;

        let mut solid = TopoDsSolid::new();

        // Create ball
        let sphere = Sphere::new(self.position, self.diameter / 2.0);

        // Create surface and face
        let surface = SurfaceEnum::Sphere(sphere);
        let face = TopoDsFace::with_surface(Handle::new(Arc::new(surface)));

        // Create shell and add face
        let mut shell = TopoDsShell::new();
        shell.add_face(Handle::new(Arc::new(face)));

        // Add shell to solid
        solid.add_shell(Handle::new(Arc::new(shell)));

        solid
    }
}

impl Pin {
    /// Create a new pin
    pub fn new(
        pin_type: PinType,
        width: StandardReal,
        length: StandardReal,
        thickness: StandardReal,
        position: Point,
        orientation: Vector,
    ) -> Self {
        Self {
            pin_type,
            width,
            length,
            thickness,
            position,
            orientation: orientation.normalized(),
        }
    }

    /// Generate the pin as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        use crate::geometry::surface_enum::SurfaceEnum;
        use crate::handle::Handle;
        use crate::topology::{TopoDsFace, TopoDsShell};
        use std::sync::Arc;

        let mut solid = TopoDsSolid::new();

        match self.pin_type {
            PinType::ThroughHole => {
                // Through-hole pin is a cylinder
                let cylinder = Cylinder::new(
                    self.position,
                    crate::geometry::Direction::from_vector(&self.orientation),
                    self.width / 2.0,
                );

                // Create surface and face
                let surface = SurfaceEnum::Cylinder(cylinder);
                let face = TopoDsFace::with_surface(Handle::new(Arc::new(surface)));

                // Create shell and add face
                let mut shell = TopoDsShell::new();
                shell.add_face(Handle::new(Arc::new(face)));

                // Add shell to solid
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            PinType::SurfaceMount => {
                // Surface mount pin is a rectangular prism
                // For simplicity, use a cylinder as placeholder
                let cylinder = Cylinder::new(
                    self.position,
                    crate::geometry::Direction::from_vector(&self.orientation),
                    self.width / 2.0,
                );

                // Create surface and face
                let surface = SurfaceEnum::Cylinder(cylinder);
                let face = TopoDsFace::with_surface(Handle::new(Arc::new(surface)));

                // Create shell and add face
                let mut shell = TopoDsShell::new();
                shell.add_face(Handle::new(Arc::new(face)));

                // Add shell to solid
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            PinType::BGABall => {
                // BGA ball is a sphere
                let sphere = Sphere::new(self.position, self.width / 2.0);

                // Create surface and face
                let surface = SurfaceEnum::Sphere(sphere);
                let face = TopoDsFace::with_surface(Handle::new(Arc::new(surface)));

                // Create shell and add face
                let mut shell = TopoDsShell::new();
                shell.add_face(Handle::new(Arc::new(face)));

                // Add shell to solid
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            PinType::LeadFrame => {
                // Lead frame pin is a thin rectangular prism
                // For simplicity, use a cylinder as placeholder
                let cylinder = Cylinder::new(
                    self.position,
                    crate::geometry::Direction::from_vector(&self.orientation),
                    self.width / 2.0,
                );

                // Create surface and face
                let surface = SurfaceEnum::Cylinder(cylinder);
                let face = TopoDsFace::with_surface(Handle::new(Arc::new(surface)));

                // Create shell and add face
                let mut shell = TopoDsShell::new();
                shell.add_face(Handle::new(Arc::new(face)));

                // Add shell to solid
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
        }

        solid
    }
}

impl ChipPackage {
    /// Create a new chip package
    pub fn new(
        package_type: PackageType,
        dimensions: (StandardReal, StandardReal, StandardReal),
        die_dimensions: (StandardReal, StandardReal, StandardReal),
        pins: Vec<Pin>,
        solder_balls: Vec<SolderBall>,
        origin: Point,
    ) -> Self {
        Self {
            package_type,
            dimensions,
            die_dimensions,
            pins,
            solder_balls,
            origin,
        }
    }

    /// Create a BGA package
    pub fn bga_package(
        package_width: StandardReal,
        package_length: StandardReal,
        package_height: StandardReal,
        die_width: StandardReal,
        die_length: StandardReal,
        die_height: StandardReal,
        ball_grid: (usize, usize), // rows, columns
        ball_pitch: StandardReal,
        ball_diameter: StandardReal,
        origin: Point,
    ) -> Self {
        let mut solder_balls = Vec::new();

        // Generate BGA balls in a grid
        let (rows, cols) = ball_grid;
        let start_x = -((cols - 1) as StandardReal * ball_pitch) / 2.0;
        let start_y = -((rows - 1) as StandardReal * ball_pitch) / 2.0;

        for row in 0..rows {
            for col in 0..cols {
                let x = start_x + col as StandardReal * ball_pitch;
                let y = start_y + row as StandardReal * ball_pitch;
                let position = origin + Vector::new(x, y, -package_height / 2.0);

                let ball = SolderBall::new(ball_diameter, position, package_height * 0.8);
                solder_balls.push(ball);
            }
        }

        Self::new(
            PackageType::BGA,
            (package_width, package_length, package_height),
            (die_width, die_length, die_height),
            Vec::new(), // BGA uses solder balls instead of pins
            solder_balls,
            origin,
        )
    }

    /// Create a QFP package
    pub fn qfp_package(
        package_width: StandardReal,
        package_length: StandardReal,
        package_height: StandardReal,
        die_width: StandardReal,
        die_length: StandardReal,
        die_height: StandardReal,
        pins_per_side: usize,
        pin_pitch: StandardReal,
        pin_width: StandardReal,
        pin_length: StandardReal,
        origin: Point,
    ) -> Self {
        let mut pins = Vec::new();

        // Generate pins on all four sides

        let pin_thickness = 0.1;

        // Right side
        for i in 0..pins_per_side {
            let x = package_width / 2.0;
            let y = -((pins_per_side - 1) as StandardReal * pin_pitch) / 2.0
                + i as StandardReal * pin_pitch;
            let position = origin + Vector::new(x, y, 0.0);
            let orientation = Vector::new(1.0, 0.0, 0.0);

            let pin = Pin::new(
                PinType::SurfaceMount,
                pin_width,
                pin_length,
                pin_thickness,
                position,
                orientation,
            );
            pins.push(pin);
        }

        // Left side
        for i in 0..pins_per_side {
            let x = -package_width / 2.0;
            let y = -((pins_per_side - 1) as StandardReal * pin_pitch) / 2.0
                + i as StandardReal * pin_pitch;
            let position = origin + Vector::new(x, y, 0.0);
            let orientation = Vector::new(-1.0, 0.0, 0.0);

            let pin = Pin::new(
                PinType::SurfaceMount,
                pin_width,
                pin_length,
                pin_thickness,
                position,
                orientation,
            );
            pins.push(pin);
        }

        // Top side
        for i in 0..pins_per_side {
            let x = -((pins_per_side - 1) as StandardReal * pin_pitch) / 2.0
                + i as StandardReal * pin_pitch;
            let y = package_length / 2.0;
            let position = origin + Vector::new(x, y, 0.0);
            let orientation = Vector::new(0.0, 1.0, 0.0);

            let pin = Pin::new(
                PinType::SurfaceMount,
                pin_width,
                pin_length,
                pin_thickness,
                position,
                orientation,
            );
            pins.push(pin);
        }

        // Bottom side
        for i in 0..pins_per_side {
            let x = -((pins_per_side - 1) as StandardReal * pin_pitch) / 2.0
                + i as StandardReal * pin_pitch;
            let y = -package_length / 2.0;
            let position = origin + Vector::new(x, y, 0.0);
            let orientation = Vector::new(0.0, -1.0, 0.0);

            let pin = Pin::new(
                PinType::SurfaceMount,
                pin_width,
                pin_length,
                pin_thickness,
                position,
                orientation,
            );
            pins.push(pin);
        }

        Self::new(
            PackageType::QFP,
            (package_width, package_length, package_height),
            (die_width, die_length, die_height),
            pins,
            Vec::new(),
            origin,
        )
    }

    /// Generate the chip package as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create package body
        let (width, length, height) = self.dimensions;
        let package_origin = Point::new(self.origin.x, self.origin.y, self.origin.z);

        // Create a box as package body
        let body = crate::modeling::primitives::make_box(
            width,
            length,
            height,
            Some(package_origin),
        );

        // Add body shells to solid
        for shell in body.shells() {
            solid.add_shell(shell.clone());
        }

        // Add pins
        for pin in &self.pins {
            let pin_solid = pin.to_solid();
            for shell in pin_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        // Add solder balls
        for ball in &self.solder_balls {
            let ball_solid = ball.to_solid();
            for shell in ball_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }

    /// Calculate the package's bounding box
    pub fn bounding_box(&self) -> (Point, Point) {
        let (width, length, height) = self.dimensions;

        let min_point = Point::new(
            self.origin.x - width / 2.0,
            self.origin.y - length / 2.0,
            self.origin.z - height / 2.0,
        );
        let max_point = Point::new(
            self.origin.x + width / 2.0,
            self.origin.y + length / 2.0,
            self.origin.z + height / 2.0,
        );

        (min_point, max_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point, Vector};

    #[test]
    fn test_solder_ball_creation() {
        let position = Point::new(0.0, 0.0, 0.0);
        let ball = SolderBall::new(0.5, position, 0.3);

        assert_eq!(ball.diameter, 0.5);
        assert_eq!(ball.position, position);
        assert_eq!(ball.height, 0.3);
    }

    #[test]
    fn test_pin_creation() {
        let position = Point::new(0.0, 0.0, 0.0);
        let orientation = Vector::new(1.0, 0.0, 0.0);
        let pin = Pin::new(PinType::SurfaceMount, 0.2, 1.0, 0.1, position, orientation);

        assert!(matches!(pin.pin_type, PinType::SurfaceMount));
        assert_eq!(pin.width, 0.2);
        assert_eq!(pin.length, 1.0);
        assert_eq!(pin.thickness, 0.1);
        assert_eq!(pin.position, position);
        assert_eq!(pin.orientation, orientation);
    }

    #[test]
    fn test_bga_package_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let package =
            ChipPackage::bga_package(10.0, 10.0, 1.0, 8.0, 8.0, 0.5, (8, 8), 1.0, 0.5, origin);

        assert!(matches!(package.package_type, PackageType::BGA));
        assert_eq!(package.solder_balls.len(), 64); // 8x8 grid
    }

    #[test]
    fn test_qfp_package_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let package =
            ChipPackage::qfp_package(10.0, 10.0, 1.0, 8.0, 8.0, 0.5, 10, 0.5, 0.2, 1.0, origin);

        assert!(matches!(package.package_type, PackageType::QFP));
        assert_eq!(package.pins.len(), 40); // 10 pins per side x 4 sides
    }
}
