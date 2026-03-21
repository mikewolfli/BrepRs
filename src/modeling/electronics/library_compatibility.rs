use crate::geometry::Point;
use crate::modeling::electronics::{PadShape, PcbComponentFootprint, PcbLayerType, PcbPad};
use crate::topology::TopoDsSolid;

/// Library type
#[derive(Debug, Clone, PartialEq)]
pub enum LibraryType {
    ElectricalComponent,
    LogicGate,
    ChipDevice,
    Other,
}

/// Library component definition
#[derive(Debug, Clone)]
pub struct LibraryComponent {
    pub name: String,
    pub library_type: LibraryType,
    pub footprint: PcbComponentFootprint,
    pub body_geometry: Option<TopoDsSolid>,
    pub parameters: std::collections::HashMap<String, String>,
    pub manufacturer: String,
    pub part_number: String,
}

impl LibraryComponent {
    /// Create a new library component
    pub fn new(
        name: &str,
        library_type: LibraryType,
        manufacturer: &str,
        part_number: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            library_type,
            footprint: PcbComponentFootprint::new(name),
            body_geometry: None,
            parameters: std::collections::HashMap::new(),
            manufacturer: manufacturer.to_string(),
            part_number: part_number.to_string(),
        }
    }

    /// Set the component's body geometry
    pub fn set_body_geometry(&mut self, geometry: TopoDsSolid) {
        self.body_geometry = Some(geometry);
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.to_string(), value.to_string());
    }

    /// Generate the component as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        // If body geometry is available, return it
        if let Some(body) = &self.body_geometry {
            body.clone()
        } else {
            // Create a default bounding box solid based on footprint dimensions
            let mut solid = TopoDsSolid::new();
            
            // Calculate bounding box from footprint pads
            if !self.footprint.pads.is_empty() {
                let mut min_x = f64::MAX;
                let mut max_x = f64::MIN;
                let mut min_y = f64::MAX;
                let mut max_y = f64::MIN;
                
                for pad in &self.footprint.pads {
                    let (width, height) = pad.size;
                    min_x = min_x.min(pad.position.x - width / 2.0);
                    max_x = max_x.max(pad.position.x + width / 2.0);
                    min_y = min_y.min(pad.position.y - height / 2.0);
                    max_y = max_y.max(pad.position.y + height / 2.0);
                }
                
                // Add small margin and height
                let margin = 0.001;
                let height = 0.002;
                
                // Create a simple box solid using primitives
                let origin = Point::new(
                    (min_x + max_x) / 2.0,
                    (min_y + max_y) / 2.0,
                    height / 2.0,
                );
                let box_solid = crate::modeling::primitives::make_box(
                    max_x - min_x + margin * 2.0,
                    max_y - min_y + margin * 2.0,
                    height,
                    Some(origin),
                );
                
                solid = box_solid;
            }
            
            solid
        }
    }
}

/// Electrical component library
#[derive(Debug, Clone)]
pub struct ElectricalComponentLibrary {
    pub name: String,
    pub components: std::collections::HashMap<String, LibraryComponent>,
}

impl ElectricalComponentLibrary {
    /// Create a new electrical component library
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: std::collections::HashMap::new(),
        }
    }

    /// Add a component to the library
    pub fn add_component(&mut self, component: LibraryComponent) {
        self.components.insert(component.name.clone(), component);
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&LibraryComponent> {
        self.components.get(name)
    }

    /// Generate a standard resistor component
    pub fn standard_resistor(&self, resistance: f64, power: f64) -> LibraryComponent {
        let name = format!("Resistor_{:.2}Ohm_{:.2}W", resistance, power);
        let mut component =
            LibraryComponent::new(&name, LibraryType::ElectricalComponent, "Generic", &name);

        // Add parameters
        component.add_parameter("Resistance", &resistance.to_string());
        component.add_parameter("PowerRating", &power.to_string());
        component.add_parameter("Type", "Through-Hole");

        // Create footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: Point::new(0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);

        // Create body geometry
        let body = crate::modeling::primitives::make_cylinder(
            0.002,
            0.002,
            Some(Point::new(0.0, 0.0, 0.001)),
        );

        component.set_body_geometry(body);

        component
    }

    /// Generate a standard capacitor component
    pub fn standard_capacitor(&self, capacitance: f64, voltage: f64) -> LibraryComponent {
        let name = format!("Capacitor_{:.2}uF_{:.2}V", capacitance * 1e6, voltage);
        let mut component =
            LibraryComponent::new(&name, LibraryType::ElectricalComponent, "Generic", &name);

        // Add parameters
        component.add_parameter("Capacitance", &capacitance.to_string());
        component.add_parameter("VoltageRating", &voltage.to_string());
        component.add_parameter("Type", "Through-Hole");

        // Create footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: Point::new(0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);

        // Create body geometry
        let body = crate::modeling::primitives::make_cylinder(
            0.0015,
            0.003,
            Some(Point::new(0.0, 0.0, 0.0015)),
        );

        component.set_body_geometry(body);

        component
    }
}

/// Logic gate library
#[derive(Debug, Clone)]
pub struct LogicGateLibrary {
    pub name: String,
    pub components: std::collections::HashMap<String, LibraryComponent>,
}

impl LogicGateLibrary {
    /// Create a new logic gate library
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: std::collections::HashMap::new(),
        }
    }

    /// Add a component to the library
    pub fn add_component(&mut self, component: LibraryComponent) {
        self.components.insert(component.name.clone(), component);
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&LibraryComponent> {
        self.components.get(name)
    }

    /// Generate a standard AND gate
    pub fn standard_and_gate(&self) -> LibraryComponent {
        let name = "AND_Gate";
        let mut component = LibraryComponent::new(name, LibraryType::LogicGate, "Generic", name);

        // Add parameters
        component.add_parameter("LogicFamily", "TTL");
        component.add_parameter("NumberOfInputs", "2");
        component.add_parameter("Package", "DIP");

        // Create footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.003, 0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.003, -0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad3 = PcbPad {
            name: "3".to_string(),
            shape: PadShape::Round,
            position: Point::new(0.003, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);
        component.footprint.add_pad(pad3);

        // Create body geometry
        let body = crate::modeling::primitives::make_box(
            0.006,
            0.006,
            0.002,
            Some(Point::new(0.0, 0.0, 0.001)),
        );

        component.set_body_geometry(body);

        component
    }

    /// Generate a standard OR gate
    pub fn standard_or_gate(&self) -> LibraryComponent {
        let name = "OR_Gate";
        let mut component = LibraryComponent::new(name, LibraryType::LogicGate, "Generic", name);

        // Add parameters
        component.add_parameter("LogicFamily", "TTL");
        component.add_parameter("NumberOfInputs", "2");
        component.add_parameter("Package", "DIP");

        // Create footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.003, 0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: Point::new(-0.003, -0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        let pad3 = PcbPad {
            name: "3".to_string(),
            shape: PadShape::Round,
            position: Point::new(0.003, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: Some(0.001),
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);
        component.footprint.add_pad(pad3);

        // Create body geometry
        let body = crate::modeling::primitives::make_box(
            0.006,
            0.006,
            0.002,
            Some(Point::new(0.0, 0.0, 0.001)),
        );

        component.set_body_geometry(body);

        component
    }
}

/// Chip device library
#[derive(Debug, Clone)]
pub struct ChipDeviceLibrary {
    pub name: String,
    pub components: std::collections::HashMap<String, LibraryComponent>,
}

impl ChipDeviceLibrary {
    /// Create a new chip device library
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: std::collections::HashMap::new(),
        }
    }

    /// Add a component to the library
    pub fn add_component(&mut self, component: LibraryComponent) {
        self.components.insert(component.name.clone(), component);
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&LibraryComponent> {
        self.components.get(name)
    }

    /// Generate a standard microcontroller
    pub fn standard_microcontroller(&self) -> LibraryComponent {
        let name = "Microcontroller";
        let mut component = LibraryComponent::new(name, LibraryType::ChipDevice, "Generic", name);

        // Add parameters
        component.add_parameter("Architecture", "ARM");
        component.add_parameter("ClockSpeed", "100MHz");
        component.add_parameter("Package", "QFP");
        component.add_parameter("PinCount", "48");

        // Create footprint
        // Add 48 pins in a QFP package
        let pin_pitch = 0.0005;
        let package_size = 0.01;

        // Top row
        for i in 0..12 {
            let pad = PcbPad {
                name: (i + 1).to_string(),
                shape: PadShape::Rectangle,
                position: Point::new(
                    -package_size / 2.0 + i as f64 * pin_pitch,
                    package_size / 2.0,
                    0.0,
                ),
                size: (0.0004, 0.0002),
                layer: PcbLayerType::Top,
                drill_size: None,
            };
            component.footprint.add_pad(pad);
        }

        // Right row
        for i in 0..12 {
            let pad = PcbPad {
                name: (i + 13).to_string(),
                shape: PadShape::Rectangle,
                position: Point::new(
                    package_size / 2.0,
                    package_size / 2.0 - i as f64 * pin_pitch,
                    0.0,
                ),
                size: (0.0002, 0.0004),
                layer: PcbLayerType::Top,
                drill_size: None,
            };
            component.footprint.add_pad(pad);
        }

        // Bottom row
        for i in 0..12 {
            let pad = PcbPad {
                name: (i + 25).to_string(),
                shape: PadShape::Rectangle,
                position: Point::new(
                    package_size / 2.0 - i as f64 * pin_pitch,
                    -package_size / 2.0,
                    0.0,
                ),
                size: (0.0004, 0.0002),
                layer: PcbLayerType::Top,
                drill_size: None,
            };
            component.footprint.add_pad(pad);
        }

        // Left row
        for i in 0..12 {
            let pad = PcbPad {
                name: (i + 37).to_string(),
                shape: PadShape::Rectangle,
                position: Point::new(
                    -package_size / 2.0,
                    -package_size / 2.0 + i as f64 * pin_pitch,
                    0.0,
                ),
                size: (0.0002, 0.0004),
                layer: PcbLayerType::Top,
                drill_size: None,
            };
            component.footprint.add_pad(pad);
        }

        // Create body geometry
        let body = crate::modeling::primitives::make_box(
            package_size,
            package_size,
            0.002,
            Some(Point::new(0.0, 0.0, 0.001)),
        );

        component.set_body_geometry(body);

        component
    }

    /// Generate a standard memory chip
    pub fn standard_memory_chip(&self) -> LibraryComponent {
        let name = "Memory_Chip";
        let mut component = LibraryComponent::new(name, LibraryType::ChipDevice, "Generic", name);

        // Add parameters
        component.add_parameter("MemoryType", "RAM");
        component.add_parameter("Capacity", "1GB");
        component.add_parameter("Package", "BGA");
        component.add_parameter("BallCount", "144");

        // Create footprint
        // Add 144 balls in a BGA package
        let ball_pitch = 0.001;
        let package_size = 0.012;
        let rows = 12;
        let cols = 12;

        for row in 0..rows {
            for col in 0..cols {
                let pad = PcbPad {
                    name: (row * cols + col + 1).to_string(),
                    shape: PadShape::Round,
                    position: Point::new(
                        -package_size / 2.0 + col as f64 * ball_pitch,
                        -package_size / 2.0 + row as f64 * ball_pitch,
                        0.0,
                    ),
                    size: (0.0005, 0.0005),
                    layer: PcbLayerType::Top,
                    drill_size: None,
                };
                component.footprint.add_pad(pad);
            }
        }

        // Create body geometry
        let body = crate::modeling::primitives::make_box(
            package_size,
            package_size,
            0.002,
            Some(Point::new(0.0, 0.0, 0.001)),
        );

        component.set_body_geometry(body);

        component
    }
}

/// Library manager
#[derive(Debug, Clone)]
pub struct LibraryManager {
    pub electrical_libraries: std::collections::HashMap<String, ElectricalComponentLibrary>,
    pub logic_libraries: std::collections::HashMap<String, LogicGateLibrary>,
    pub chip_libraries: std::collections::HashMap<String, ChipDeviceLibrary>,
}

impl LibraryManager {
    /// Create a new library manager
    pub fn new() -> Self {
        Self {
            electrical_libraries: std::collections::HashMap::new(),
            logic_libraries: std::collections::HashMap::new(),
            chip_libraries: std::collections::HashMap::new(),
        }
    }

    /// Add an electrical component library
    pub fn add_electrical_library(&mut self, library: ElectricalComponentLibrary) {
        self.electrical_libraries
            .insert(library.name.clone(), library);
    }

    /// Add a logic gate library
    pub fn add_logic_library(&mut self, library: LogicGateLibrary) {
        self.logic_libraries.insert(library.name.clone(), library);
    }

    /// Add a chip device library
    pub fn add_chip_library(&mut self, library: ChipDeviceLibrary) {
        self.chip_libraries.insert(library.name.clone(), library);
    }

    /// Get an electrical component
    pub fn get_electrical_component(
        &self,
        library_name: &str,
        component_name: &str,
    ) -> Option<&LibraryComponent> {
        self.electrical_libraries
            .get(library_name)?
            .get_component(component_name)
    }

    /// Get a logic gate component
    pub fn get_logic_component(
        &self,
        library_name: &str,
        component_name: &str,
    ) -> Option<&LibraryComponent> {
        self.logic_libraries
            .get(library_name)?
            .get_component(component_name)
    }

    /// Get a chip device component
    pub fn get_chip_component(
        &self,
        library_name: &str,
        component_name: &str,
    ) -> Option<&LibraryComponent> {
        self.chip_libraries
            .get(library_name)?
            .get_component(component_name)
    }

    /// Create a default library manager with standard components
    pub fn default() -> Self {
        let mut manager = Self::new();

        // Create electrical component library
        let mut electrical_lib = ElectricalComponentLibrary::new("Standard_Electrical");
        electrical_lib.add_component(electrical_lib.standard_resistor(100.0, 0.25));
        electrical_lib.add_component(electrical_lib.standard_capacitor(1e-6, 25.0));
        manager.add_electrical_library(electrical_lib);

        // Create logic gate library
        let mut logic_lib = LogicGateLibrary::new("Standard_Logic");
        logic_lib.add_component(logic_lib.standard_and_gate());
        logic_lib.add_component(logic_lib.standard_or_gate());
        manager.add_logic_library(logic_lib);

        // Create chip device library
        let mut chip_lib = ChipDeviceLibrary::new("Standard_Chips");
        chip_lib.add_component(chip_lib.standard_microcontroller());
        chip_lib.add_component(chip_lib.standard_memory_chip());
        manager.add_chip_library(chip_lib);

        manager
    }
}
