use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{TopoDsShell, TopoDsSolid};

/// Internal layer type
#[derive(Debug, Clone, PartialEq)]
pub enum InternalLayerType {
    Die,
    Substrate,
    Metal,
    Oxide,
    PolySilicon,
    Diffusion,
    Contact,
    Via,
    Other,
}

/// Internal layer definition
#[derive(Debug, Clone)]
pub struct InternalLayer {
    pub layer_type: InternalLayerType,
    pub thickness: f64,
    pub material: String,
    pub position: f64, // Z-coordinate position
    pub pattern: Option<InternalPattern>,
}

/// Internal pattern type
#[derive(Debug, Clone, PartialEq)]
pub enum InternalPatternType {
    Circuit,
    Contact,
    Via,
    Diffusion,
    PolySilicon,
    Metal,
    Other,
}

/// Internal pattern definition
#[derive(Debug, Clone)]
pub struct InternalPattern {
    pub pattern_type: InternalPatternType,
    pub polygons: Vec<Vec<Point>>, // 2D polygons in layer plane
    pub holes: Vec<Vec<Point>>,    // 2D holes in pattern
}

/// Electronic device internal structure
#[derive(Debug, Clone)]
pub struct ElectronicDeviceStructure {
    pub name: String,
    pub width: f64,
    pub length: f64,
    pub height: f64,
    pub layers: Vec<InternalLayer>,
    pub origin: Point,
}

impl ElectronicDeviceStructure {
    /// Create a new electronic device structure
    pub fn new(name: &str, width: f64, length: f64, height: f64, origin: Point) -> Self {
        Self {
            name: name.to_string(),
            width,
            length,
            height,
            layers: Vec::new(),
            origin,
        }
    }

    /// Add a layer to the device
    pub fn add_layer(&mut self, layer: InternalLayer) {
        self.layers.push(layer);
    }

    /// Generate a simple IC structure
    pub fn simple_ic(name: &str, width: f64, length: f64, height: f64) -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let mut device = Self::new(name, width, length, height, origin);
        // Add a default layer (e.g., Die)
        let layer = InternalLayer {
            layer_type: InternalLayerType::Die,
            thickness: height,
            material: "Silicon".to_string(),
            position: 0.0,
            pattern: None,
        };
        device.add_layer(layer);
        device
    }

    /// Generate the device structure as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();
        for layer in &self.layers {
            let shell = self.create_layer_shell(layer);
            solid.add_shell(Handle::new(std::sync::Arc::new(shell)));
        }
        solid
    }

    /// Create a shell for a layer
    fn create_layer_shell(&self, layer: &InternalLayer) -> TopoDsShell {
        let _ = layer; // suppress unused variable warning
        let center = self.origin
            + Vector::new(
                self.width / 2.0,
                self.length / 2.0,
                layer.position + layer.thickness / 2.0,
            );
        let box_solid = crate::modeling::primitives::make_box(
            self.width,
            self.length,
            layer.thickness,
            Some(center),
        );
        // Extract the first shell from the solid (if any)
        if let Some(shell_handle) = box_solid.shells().get(0) {
            return (**shell_handle).clone();
        }
        TopoDsShell::new()
    }
}

/// Chip internal structure
#[derive(Debug, Clone)]
pub struct ChipStructure {
    pub name: String,
    pub die: ElectronicDeviceStructure,
    pub bond_wires: Vec<BondWire>,
    pub package: Option<TopoDsSolid>,
    pub origin: Point,
}

/// Bond wire definition
#[derive(Debug, Clone)]
pub struct BondWire {
    pub start: Point,
    pub end: Point,
    pub diameter: f64,
    pub material: String,
}

impl ChipStructure {
    /// Create a new chip structure
    pub fn new(name: &str, die: ElectronicDeviceStructure, origin: Point) -> Self {
        Self {
            name: name.to_string(),
            die,
            bond_wires: Vec::new(),
            package: None,
            origin,
        }
    }

    /// Add a bond wire
    pub fn add_bond_wire(&mut self, bond_wire: BondWire) {
        self.bond_wires.push(bond_wire);
    }

    /// Set the package
    pub fn set_package(&mut self, package: TopoDsSolid) {
        self.package = Some(package);
    }

    /// Generate the chip structure as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add die
        let die_solid = self.die.to_solid();
        for shell in die_solid.shells() {
            solid.add_shell(shell.clone());
        }

        // Add bond wires
        for bond_wire in &self.bond_wires {
            let bond_wire_solid = self.create_bond_wire_solid(bond_wire);
            for shell in bond_wire_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        // Add package if available
        if let Some(package) = &self.package {
            for shell in package.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }

    /// Create a bond wire solid
    fn create_bond_wire_solid(&self, bond_wire: &BondWire) -> TopoDsSolid {
        // Compute axis and length
        let length = (bond_wire.end - bond_wire.start).magnitude();
        let direction_vec = (bond_wire.end - bond_wire.start).normalized();
        let direction = crate::geometry::Direction::from_vector(&direction_vec);
        let axis = crate::geometry::Axis::new(bond_wire.start, direction);
        // Use primitives::make_cylinder
        let solid = crate::modeling::primitives::make_cylinder(
            bond_wire.diameter / 2.0,
            length,
            Some(bond_wire.start),
            Some(axis),
        );
        solid
    }

    /// Generate a simple chip with bond wires
    pub fn simple_chip(name: &str, die_width: f64, die_length: f64, die_height: f64) -> Self {
        let die = ElectronicDeviceStructure::simple_ic("Die", die_width, die_length, die_height);

        let origin = Point::new(0.0, 0.0, 0.0);
        let mut chip = Self::new(name, die, origin);

        // Add bond wires
        let bond_wire1 = BondWire {
            start: Point::new(die_width * 0.1, die_length * 0.1, die_height),
            end: Point::new(die_width * 1.5, die_length * 0.1, die_height * 2.0),
            diameter: 0.001,
            material: "Gold".to_string(),
        };

        let bond_wire2 = BondWire {
            start: Point::new(die_width * 0.9, die_length * 0.1, die_height),
            end: Point::new(die_width * 1.5, die_length * 0.5, die_height * 2.0),
            diameter: 0.001,
            material: "Gold".to_string(),
        };

        let bond_wire3 = BondWire {
            start: Point::new(die_width * 0.1, die_length * 0.9, die_height),
            end: Point::new(die_width * 1.5, die_length * 0.9, die_height * 2.0),
            diameter: 0.001,
            material: "Gold".to_string(),
        };

        chip.add_bond_wire(bond_wire1);
        chip.add_bond_wire(bond_wire2);
        chip.add_bond_wire(bond_wire3);

        chip
    }
}

/// PCB internal layer structure
#[derive(Debug, Clone)]
pub struct PcbInternalStructure {
    pub name: String,
    pub width: f64,
    pub length: f64,
    pub layers: Vec<InternalLayer>,
    pub vias: Vec<PcbVia3D>,
    pub origin: Point,
}

/// 3D via definition
#[derive(Debug, Clone)]
pub struct PcbVia3D {
    pub position: Point,
    pub drill_size: f64,
    pub pad_size: f64,
    pub start_layer: usize,
    pub end_layer: usize,
}

impl PcbInternalStructure {
    /// Create a new PCB internal structure
    pub fn new(name: &str, width: f64, length: f64, origin: Point) -> Self {
        Self {
            name: name.to_string(),
            width,
            length,
            layers: Vec::new(),
            vias: Vec::new(),
            origin,
        }
    }

    /// Add a layer to the PCB structure
    pub fn add_layer(&mut self, layer: InternalLayer) {
        self.layers.push(layer);
    }

    /// Add a via to the PCB structure
    pub fn add_via(&mut self, via: PcbVia3D) {
        self.vias.push(via);
    }

    /// Generate the PCB internal structure as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create PCB layers
        for layer in &self.layers {
            let layer_shell = self.create_layer_shell(layer);
            solid.add_shell(Handle::new(std::sync::Arc::new(layer_shell)));
        }

        // Create vias
        for via in &self.vias {
            let via_shell = self.create_via_shell(via);
            solid.add_shell(Handle::new(std::sync::Arc::new(via_shell)));
        }

        solid
    }

    /// Create a layer shell
    fn create_layer_shell(&self, layer: &InternalLayer) -> TopoDsShell {
        // Use primitives::make_box to create a thin box as the layer shell
        let center = self.origin
            + Vector::new(
                self.width / 2.0,
                self.length / 2.0,
                layer.position + layer.thickness / 2.0,
            );
        let box_solid = crate::modeling::primitives::make_box(
            self.width,
            self.length,
            layer.thickness,
            Some(center),
        );
        // Extract the first shell from the solid (if any)
        if let Some(shell_handle) = box_solid.shells().get(0) {
            return (**shell_handle).clone();
        }
        TopoDsShell::new()
    }

    /// Create a via shell
    fn create_via_shell(&self, via: &PcbVia3D) -> TopoDsShell {
        if self.layers.len() <= via.start_layer || self.layers.len() <= via.end_layer {
            return TopoDsShell::new();
        }

        let start_layer = &self.layers[via.start_layer];
        let end_layer = &self.layers[via.end_layer];

        let start_position = start_layer.position;
        let end_position = end_layer.position + end_layer.thickness;
        let height = end_position - start_position;

        // Create via cylinder solid
        let center = via.position + Vector::new(0.0, 0.0, start_position + height / 2.0);
        let via_solid = crate::modeling::primitives::make_cylinder(
            via.pad_size / 2.0,
            height,
            Some(center),
            None,
        );
        // Extract the first shell from the solid (if any)
        if let Some(shell_handle) = via_solid.shells().get(0) {
            return (**shell_handle).clone();
        }
        TopoDsShell::new()
    }

    /// Generate a simple 4-layer PCB internal structure
    pub fn four_layer_pcb(name: &str, width: f64, length: f64) -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let mut pcb = Self::new(name, width, length, origin);

        // Add top copper layer
        pcb.add_layer(InternalLayer {
            layer_type: InternalLayerType::Metal,
            thickness: 0.035,
            material: "Copper".to_string(),
            position: 0.0,
            pattern: Some(InternalPattern {
                pattern_type: InternalPatternType::Metal,
                polygons: vec![vec![
                    Point::new(width * 0.1, length * 0.1, 0.0),
                    Point::new(width * 0.9, length * 0.1, 0.0),
                    Point::new(width * 0.9, length * 0.9, 0.0),
                    Point::new(width * 0.1, length * 0.9, 0.0),
                ]],
                holes: Vec::new(),
            }),
        });

        // Add dielectric layer 1
        pcb.add_layer(InternalLayer {
            layer_type: InternalLayerType::Other,
            thickness: 0.8,
            material: "FR4".to_string(),
            position: 0.035,
            pattern: None,
        });

        // Add internal copper layer 1
        pcb.add_layer(InternalLayer {
            layer_type: InternalLayerType::Metal,
            thickness: 0.035,
            material: "Copper".to_string(),
            position: 0.835,
            pattern: Some(InternalPattern {
                pattern_type: InternalPatternType::Metal,
                polygons: vec![vec![
                    Point::new(width * 0.2, length * 0.2, 0.0),
                    Point::new(width * 0.8, length * 0.2, 0.0),
                    Point::new(width * 0.8, length * 0.8, 0.0),
                    Point::new(width * 0.2, length * 0.8, 0.0),
                ]],
                holes: Vec::new(),
            }),
        });

        // Add dielectric layer 2
        pcb.add_layer(InternalLayer {
            layer_type: InternalLayerType::Other,
            thickness: 0.8,
            material: "FR4".to_string(),
            position: 0.87,
            pattern: None,
        });

        // Add bottom copper layer
        pcb.add_layer(InternalLayer {
            layer_type: InternalLayerType::Metal,
            thickness: 0.035,
            material: "Copper".to_string(),
            position: 1.67,
            pattern: Some(InternalPattern {
                pattern_type: InternalPatternType::Metal,
                polygons: vec![vec![
                    Point::new(width * 0.3, length * 0.3, 0.0),
                    Point::new(width * 0.7, length * 0.3, 0.0),
                    Point::new(width * 0.7, length * 0.7, 0.0),
                    Point::new(width * 0.3, length * 0.7, 0.0),
                ]],
                holes: Vec::new(),
            }),
        });

        // Add via
        let via = PcbVia3D {
            position: Point::new(width * 0.5, length * 0.5, 0.0),
            drill_size: 0.1,
            pad_size: 0.2,
            start_layer: 0,
            end_layer: 4,
        };

        pcb.add_via(via);

        pcb
    }
}
