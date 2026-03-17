use crate::foundation::handle::Handle;
use crate::foundation::StandardReal;
use crate::geometry::{axis::Axis, cylinder::Cylinder, Point, Vector};
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
use std::sync::Arc;

/// Impeller type for bioreactor stirrer
#[derive(Debug, Clone, PartialEq)]
pub enum ImpellerType {
    /// Rushton turbine impeller
    RushtonTurbine,
    /// Pitched blade impeller
    PitchedBlade(StandardReal), // pitch angle in radians
    /// Marine propeller
    MarinePropeller(StandardReal), // pitch angle in radians
    /// Anchor impeller
    Anchor,
    /// Propeller impeller
    Propeller(StandardReal), // pitch angle in radians
}

/// Stirrer shaft geometry
#[derive(Debug, Clone)]
pub struct StirrerShaft {
    /// Shaft diameter
    pub diameter: StandardReal,
    /// Shaft length
    pub length: StandardReal,
    /// Shaft origin (bottom end)
    pub origin: Point,
    /// Shaft axis
    pub axis: Axis,
    /// Number of impellers
    pub impeller_count: usize,
    /// Impeller positions (distances from origin)
    pub impeller_positions: Vec<StandardReal>,
    /// Impellers
    pub impellers: Vec<Impeller>,
}

/// Impeller geometry
#[derive(Debug, Clone)]
pub struct Impeller {
    /// Impeller type
    pub impeller_type: ImpellerType,
    /// Impeller diameter
    pub diameter: StandardReal,
    /// Number of blades
    pub blade_count: usize,
    /// Blade width
    pub blade_width: StandardReal,
    /// Blade height
    pub blade_height: StandardReal,
    /// Blade thickness
    pub blade_thickness: StandardReal,
    /// Hub diameter
    pub hub_diameter: StandardReal,
}

impl StirrerShaft {
    /// Create a new stirrer shaft with impellers
    pub fn new(
        diameter: StandardReal,
        length: StandardReal,
        origin: Point,
        axis: Axis,
        impellers: Vec<(StandardReal, Impeller)>, // (position, impeller)
    ) -> Self {
        let mut impeller_positions = Vec::new();
        let mut impeller_list = Vec::new();

        for (position, impeller) in impellers {
            impeller_positions.push(position);
            impeller_list.push(impeller);
        }

        Self {
            diameter,
            length,
            origin,
            axis,
            impeller_count: impeller_list.len(),
            impeller_positions,
            impellers: impeller_list,
        }
    }

    /// Create a simple stirrer with one Rushton turbine impeller
    pub fn with_single_rushton(
        shaft_diameter: StandardReal,
        shaft_length: StandardReal,
        impeller_diameter: StandardReal,
        impeller_position: StandardReal,
        origin: Point,
        axis: Axis,
    ) -> Self {
        let impeller = Impeller {
            impeller_type: ImpellerType::RushtonTurbine,
            diameter: impeller_diameter,
            blade_count: 6,
            blade_width: impeller_diameter * 0.2,
            blade_height: impeller_diameter * 0.1,
            blade_thickness: shaft_diameter * 0.2,
            hub_diameter: shaft_diameter * 1.5,
        };

        Self::new(
            shaft_diameter,
            shaft_length,
            origin,
            axis,
            vec![(impeller_position, impeller)],
        )
    }

    /// Generate the stirrer as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add shaft
        let shaft_shell = self.create_shaft_shell();
        solid.add_shell(Handle::new(Arc::new(shaft_shell)));

        // Add impellers
        for (i, impeller) in self.impellers.iter().enumerate() {
            let position = self.impeller_positions[i];
            let impeller_shell = self.create_impeller_shell(impeller, position);
            solid.add_shell(Handle::new(Arc::new(impeller_shell)));
        }

        solid
    }

    /// Create the shaft shell
    fn create_shaft_shell(&self) -> TopoDsShell {
        let cylinder = Cylinder::from_axis(&self.axis, self.diameter / 2.0);

        let mut shell = TopoDsShell::new();
        let face = TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
        )));
        shell.add_face(Handle::new(Arc::new(face)));

        shell
    }

    /// Create an impeller shell at a specific position
    fn create_impeller_shell(&self, impeller: &Impeller, position: StandardReal) -> TopoDsShell {
        let impeller_origin = *self.axis.location() + self.axis.direction().to_vector() * position;
        let impeller_axis = self.axis.clone();

        let mut shell = TopoDsShell::new();

        // Add hub
        let hub_cylinder = Cylinder::from_axis(&impeller_axis, impeller.hub_diameter / 2.0);
        let hub_face = TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(hub_cylinder),
        )));
        shell.add_face(Handle::new(Arc::new(hub_face)));

        // Add blades based on impeller type
        match &impeller.impeller_type {
            ImpellerType::RushtonTurbine => {
                self.add_rushton_blades(&mut shell, impeller, impeller_origin, impeller_axis);
            }
            ImpellerType::PitchedBlade(pitch_angle) => {
                self.add_pitched_blades(
                    &mut shell,
                    impeller,
                    impeller_origin,
                    impeller_axis,
                    *pitch_angle,
                );
            }
            ImpellerType::MarinePropeller(pitch_angle) => {
                self.add_marine_propeller_blades(
                    &mut shell,
                    impeller,
                    impeller_origin,
                    impeller_axis,
                    *pitch_angle,
                );
            }
            ImpellerType::Anchor => {
                self.add_anchor_blades(&mut shell, impeller, impeller_origin, impeller_axis);
            }
            ImpellerType::Propeller(pitch_angle) => {
                self.add_propeller_blades(
                    &mut shell,
                    impeller,
                    impeller_origin,
                    impeller_axis,
                    *pitch_angle,
                );
            }
        }

        shell
    }

    /// Add Rushton turbine blades
    fn add_rushton_blades(
        &self,
        shell: &mut TopoDsShell,
        impeller: &Impeller,
        origin: Point,
        _axis: Axis,
    ) {
        let blade_angle_step = 2.0 * std::f64::consts::PI / impeller.blade_count as StandardReal;

        for i in 0..impeller.blade_count {
            let angle = i as StandardReal * blade_angle_step;
            let blade_origin = origin
                + Vector::new(angle.cos(), angle.sin(), 0.0)
                    * (impeller.diameter / 2.0 - impeller.blade_width / 2.0);

            // Create blade as a rectangular prism
            let blade_length = impeller.diameter / 2.0 - impeller.blade_width / 2.0;
            let blade_height = impeller.blade_height;
            let blade_thickness = impeller.blade_thickness;

            // Create blade vertices
            let v1 = blade_origin;
            let v2 = blade_origin + Vector::new(angle.cos(), angle.sin(), 0.0) * blade_length;
            let v3 = v2 + Vector::new(0.0, 0.0, blade_height);
            let v4 = v1 + Vector::new(0.0, 0.0, blade_height);
            let v5 = v1 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v6 = v2 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v7 = v3 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v8 = v4 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;

            // Create blade faces
            let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
            let faces = vec![
                vec![0, 1, 2, 3], // Front face
                vec![4, 5, 6, 7], // Back face
                vec![0, 1, 5, 4], // Bottom face
                vec![2, 3, 7, 6], // Top face
                vec![0, 3, 7, 4], // Left face
                vec![1, 2, 6, 5], // Right face
            ];

            // Create blade solid and add to shell
            let blade_solid = TopoDsSolid::from_mesh(vertices, faces);
            for face in blade_solid.faces() {
                shell.add_face(face);
            }
        }
    }

    /// Add pitched blade impeller blades
    fn add_pitched_blades(
        &self,
        shell: &mut TopoDsShell,
        impeller: &Impeller,
        origin: Point,
        _axis: Axis,
        pitch_angle: StandardReal,
    ) {
        let blade_angle_step = 2.0 * std::f64::consts::PI / impeller.blade_count as StandardReal;

        for i in 0..impeller.blade_count {
            let angle = i as StandardReal * blade_angle_step;
            let blade_origin = origin
                + Vector::new(angle.cos(), angle.sin(), 0.0)
                    * (impeller.diameter / 2.0 - impeller.blade_width / 2.0);

            // Create pitched blade
            let blade_length = impeller.diameter / 2.0 - impeller.blade_width / 2.0;
            let blade_height = impeller.blade_height;
            let blade_thickness = impeller.blade_thickness;

            // Create blade vertices with pitch angle
            let v1 = blade_origin;
            let v2 = blade_origin + Vector::new(angle.cos(), angle.sin(), 0.0) * blade_length;
            let v3 = v2
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v4 = v1
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v5 = v1 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v6 = v2 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v7 = v3 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v8 = v4 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;

            // Create blade faces
            let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
            let faces = vec![
                vec![0, 1, 2, 3], // Front face
                vec![4, 5, 6, 7], // Back face
                vec![0, 1, 5, 4], // Bottom face
                vec![2, 3, 7, 6], // Top face
                vec![0, 3, 7, 4], // Left face
                vec![1, 2, 6, 5], // Right face
            ];

            // Create blade solid and add to shell
            let blade_solid = TopoDsSolid::from_mesh(vertices, faces);
            for face in blade_solid.faces() {
                shell.add_face(face);
            }
        }
    }

    /// Add marine propeller blades
    fn add_marine_propeller_blades(
        &self,
        shell: &mut TopoDsShell,
        impeller: &Impeller,
        origin: Point,
        _axis: Axis,
        pitch_angle: StandardReal,
    ) {
        let blade_angle_step = 2.0 * std::f64::consts::PI / impeller.blade_count as StandardReal;

        for i in 0..impeller.blade_count {
            let angle = i as StandardReal * blade_angle_step;
            let blade_origin = origin
                + Vector::new(angle.cos(), angle.sin(), 0.0)
                    * (impeller.diameter / 2.0 - impeller.blade_width / 2.0);

            // Create marine propeller blade with curved surface
            let blade_length = impeller.diameter / 2.0 - impeller.blade_width / 2.0;
            let blade_height = impeller.blade_height;
            let blade_thickness = impeller.blade_thickness;

            // Create blade vertices with curvature
            let v1 = blade_origin;
            let v2 = blade_origin + Vector::new(angle.cos(), angle.sin(), 0.0) * blade_length;
            let v3 = v2
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v4 = v1
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v5 = v1 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v6 = v2 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v7 = v3 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v8 = v4 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;

            // Create blade faces
            let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
            let faces = vec![
                vec![0, 1, 2, 3], // Front face
                vec![4, 5, 6, 7], // Back face
                vec![0, 1, 5, 4], // Bottom face
                vec![2, 3, 7, 6], // Top face
                vec![0, 3, 7, 4], // Left face
                vec![1, 2, 6, 5], // Right face
            ];

            // Create blade solid and add to shell
            let blade_solid = TopoDsSolid::from_mesh(vertices, faces);
            for face in blade_solid.faces() {
                shell.add_face(face);
            }
        }
    }

    /// Add anchor impeller blades
    fn add_anchor_blades(
        &self,
        shell: &mut TopoDsShell,
        impeller: &Impeller,
        origin: Point,
        _axis: Axis,
    ) {
        let blade_height = impeller.blade_height;
        let blade_thickness = impeller.blade_thickness;
        let radius = impeller.diameter / 2.0;

        // Create anchor blade vertices (rectangular shape following the vessel wall)
        let v1 = origin + Vector::new(radius, 0.0, -blade_height / 2.0);
        let v2 = origin + Vector::new(radius - blade_thickness, 0.0, -blade_height / 2.0);
        let v3 = origin + Vector::new(radius - blade_thickness, 0.0, blade_height / 2.0);
        let v4 = origin + Vector::new(radius, 0.0, blade_height / 2.0);

        let v5 = origin + Vector::new(0.0, radius, -blade_height / 2.0);
        let v6 = origin + Vector::new(0.0, radius - blade_thickness, -blade_height / 2.0);
        let v7 = origin + Vector::new(0.0, radius - blade_thickness, blade_height / 2.0);
        let v8 = origin + Vector::new(0.0, radius, blade_height / 2.0);

        let v9 = origin + Vector::new(-radius, 0.0, -blade_height / 2.0);
        let v10 = origin + Vector::new(-(radius - blade_thickness), 0.0, -blade_height / 2.0);
        let v11 = origin + Vector::new(-(radius - blade_thickness), 0.0, blade_height / 2.0);
        let v12 = origin + Vector::new(-radius, 0.0, blade_height / 2.0);

        let v13 = origin + Vector::new(0.0, -radius, -blade_height / 2.0);
        let v14 = origin + Vector::new(0.0, -(radius - blade_thickness), -blade_height / 2.0);
        let v15 = origin + Vector::new(0.0, -(radius - blade_thickness), blade_height / 2.0);
        let v16 = origin + Vector::new(0.0, -radius, blade_height / 2.0);

        // Create anchor blade faces
        let vertices = vec![
            v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16,
        ];

        // Add faces for each side of the anchor
        let faces = vec![
            // Front faces
            vec![0, 1, 2, 3],     // Right side
            vec![4, 5, 6, 7],     // Front side
            vec![8, 9, 10, 11],   // Left side
            vec![12, 13, 14, 15], // Back side
            // Connecting faces (top and bottom)
            vec![3, 7, 11, 15], // Top ring
            vec![0, 4, 8, 12],  // Bottom ring
        ];

        // Create anchor solid and add to shell
        let anchor_solid = TopoDsSolid::from_mesh(vertices, faces);
        for face in anchor_solid.faces() {
            shell.add_face(face);
        }
    }

    /// Add propeller impeller blades
    fn add_propeller_blades(
        &self,
        shell: &mut TopoDsShell,
        impeller: &Impeller,
        origin: Point,
        _axis: Axis,
        pitch_angle: StandardReal,
    ) {
        let blade_angle_step = 2.0 * std::f64::consts::PI / impeller.blade_count as StandardReal;

        for i in 0..impeller.blade_count {
            let angle = i as StandardReal * blade_angle_step;
            let blade_origin = origin
                + Vector::new(angle.cos(), angle.sin(), 0.0)
                    * (impeller.diameter / 2.0 - impeller.blade_width / 2.0);

            // Create propeller blade with helical shape
            let blade_length = impeller.diameter / 2.0 - impeller.blade_width / 2.0;
            let blade_height = impeller.blade_height;
            let blade_thickness = impeller.blade_thickness;

            // Create blade vertices with helical twist
            let v1 = blade_origin;
            let v2 = blade_origin + Vector::new(angle.cos(), angle.sin(), 0.0) * blade_length;
            let v3 = v2
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v4 = v1
                + Vector::new(
                    angle.cos() * pitch_angle.sin(),
                    angle.sin() * pitch_angle.sin(),
                    pitch_angle.cos(),
                ) * blade_height;
            let v5 = v1 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v6 = v2 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v7 = v3 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;
            let v8 = v4 + Vector::new(-angle.sin(), angle.cos(), 0.0) * blade_thickness / 2.0;

            // Create blade faces
            let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
            let faces = vec![
                vec![0, 1, 2, 3], // Front face
                vec![4, 5, 6, 7], // Back face
                vec![0, 1, 5, 4], // Bottom face
                vec![2, 3, 7, 6], // Top face
                vec![0, 3, 7, 4], // Left face
                vec![1, 2, 6, 5], // Right face
            ];

            // Create blade solid and add to shell
            let blade_solid = TopoDsSolid::from_mesh(vertices, faces);
            for face in blade_solid.faces() {
                shell.add_face(face);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{axis::Axis, Point, Vector};

    #[test]
    fn test_stirrer_creation() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(
            origin,
            crate::geometry::Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)),
        );

        let stirrer = StirrerShaft::with_single_rushton(0.1, 2.0, 0.5, 1.0, origin, axis);

        assert_eq!(stirrer.diameter, 0.1);
        assert_eq!(stirrer.length, 2.0);
        assert_eq!(stirrer.impeller_count, 1);
        assert!(matches!(
            stirrer.impellers[0].impeller_type,
            ImpellerType::RushtonTurbine
        ));
    }

    #[test]
    fn test_stirrer_with_multiple_impellers() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let axis = Axis::new(
            origin,
            crate::geometry::Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)),
        );

        let impeller1 = Impeller {
            impeller_type: ImpellerType::RushtonTurbine,
            diameter: 0.5,
            blade_count: 6,
            blade_width: 0.1,
            blade_height: 0.05,
            blade_thickness: 0.02,
            hub_diameter: 0.15,
        };

        let impeller2 = Impeller {
            impeller_type: ImpellerType::PitchedBlade(std::f64::consts::PI / 4.0),
            diameter: 0.4,
            blade_count: 4,
            blade_width: 0.08,
            blade_height: 0.04,
            blade_thickness: 0.02,
            hub_diameter: 0.15,
        };

        let stirrer = StirrerShaft::new(
            0.1,
            2.0,
            origin,
            axis,
            vec![(0.8, impeller1), (1.6, impeller2)],
        );

        assert_eq!(stirrer.impeller_count, 2);
        assert_eq!(stirrer.impeller_positions.len(), 2);
        assert!(matches!(
            stirrer.impellers[0].impeller_type,
            ImpellerType::RushtonTurbine
        ));
        assert!(matches!(
            stirrer.impellers[1].impeller_type,
            ImpellerType::PitchedBlade(_)
        ));
    }
}
