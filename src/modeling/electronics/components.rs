use crate::geometry::{Cylinder, Direction, Point, Sphere, Vector};
use crate::modeling::electronics::pcb::{PadShape, PcbComponentFootprint, PcbLayerType, PcbPad};
use crate::topology::TopoDsSolid;

/// Component type
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Transistor,
    IC,
    Connector,
    Sensor,
    Switch,
    Relay,
    Fuse,
    Other,
}

/// Connector type
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectorType {
    Header,
    Socket,
    USB,
    HDMI,
    Ethernet,
    Power,
    Audio,
    RF,
    Other,
}

/// Sensor type
#[derive(Debug, Clone, PartialEq)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Light,
    Motion,
    Proximity,
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Other,
}

/// Board-level component
#[derive(Debug, Clone)]
pub struct BoardComponent {
    pub name: String,
    pub component_type: ComponentType,
    pub footprint: PcbComponentFootprint,
    pub body_geometry: Option<TopoDsSolid>,
    pub position: Point,
    pub rotation: (f64, f64, f64), // Roll, pitch, yaw in radians
    pub height: f64,
    pub color: (f32, f32, f32), // RGB
}

impl BoardComponent {
    /// Create a new board component
    pub fn new(name: &str, component_type: ComponentType, position: Point) -> Self {
        Self {
            name: name.to_string(),
            component_type,
            footprint: PcbComponentFootprint::new(name),
            body_geometry: None,
            position,
            rotation: (0.0, 0.0, 0.0),
            height: 0.0,
            color: (0.8, 0.8, 0.8), // Default gray color
        }
    }

    /// Set the component's body geometry
    pub fn set_body_geometry(&mut self, geometry: TopoDsSolid) {
        self.body_geometry = Some(geometry);
    }

    /// Generate the component as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add body geometry if available
        if let Some(body) = &self.body_geometry {
            // Implement shell addition
            // Add all shells from the body geometry to the solid
            for shell in body.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }

    /// Create a resistor component
    pub fn resistor(name: &str, position: Point, _resistance: f64, _power: f64) -> Self {
        let mut component = Self::new(name, ComponentType::Resistor, position);

        // Create resistor footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(-0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);

        // Create resistor body
        let body = crate::modeling::primitives::make_cylinder(
            0.001,
            0.004,
            Some(position + Vector::new(0.0, 0.0, 0.002)),
        );

        component.set_body_geometry(body);
        component.height = 0.004;
        component.color = (0.6, 0.6, 0.6); // Resistor color

        component
    }

    /// Create a capacitor component
    pub fn capacitor(name: &str, position: Point, _capacitance: f64, _voltage: f64) -> Self {
        let mut component = Self::new(name, ComponentType::Capacitor, position);

        // Create capacitor footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(-0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(0.002, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        component.footprint.add_pad(pad1);
        component.footprint.add_pad(pad2);

        // Create capacitor body
        let body = crate::modeling::primitives::make_cylinder(
            0.0015,
            0.003,
            Some(position + Vector::new(0.0, 0.0, 0.0015)),
        );

        component.set_body_geometry(body);
        component.height = 0.003;
        component.color = (0.0, 0.6, 0.0); // Capacitor color

        component
    }
}

/// Connector component
#[derive(Debug, Clone)]
pub struct Connector {
    pub name: String,
    pub connector_type: ConnectorType,
    pub footprint: PcbComponentFootprint,
    pub body_geometry: Option<TopoDsSolid>,
    pub position: Point,
    pub rotation: (f64, f64, f64), // Roll, pitch, yaw in radians
    pub height: f64,
    pub pin_count: usize,
    pub pin_pitch: f64,
    pub color: (f32, f32, f32), // RGB
}

impl Connector {
    /// Create a new connector
    pub fn new(name: &str, connector_type: ConnectorType, position: Point) -> Self {
        Self {
            name: name.to_string(),
            connector_type,
            footprint: PcbComponentFootprint::new(name),
            body_geometry: None,
            position,
            rotation: (0.0, 0.0, 0.0),
            height: 0.0,
            pin_count: 0,
            pin_pitch: 0.0,
            color: (0.5, 0.5, 0.5), // Default gray color
        }
    }

    /// Set the connector's body geometry
    pub fn set_body_geometry(&mut self, geometry: TopoDsSolid) {
        self.body_geometry = Some(geometry);
    }

    /// Generate the connector as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add body geometry if available
        if let Some(body) = &self.body_geometry {
            // Implement shell addition
            // Add all shells from the body geometry to the solid
            for shell in body.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }

    /// Create a header connector
    pub fn header(name: &str, position: Point, pin_count: usize, pin_pitch: f64) -> Self {
        let mut connector = Self::new(name, ConnectorType::Header, position);
        connector.pin_count = pin_count;
        connector.pin_pitch = pin_pitch;

        // Create header footprint
        for i in 0..pin_count {
            let pad = PcbPad {
                name: (i + 1).to_string(),
                shape: PadShape::Round,
                position: position + Vector::new(0.0, i as f64 * pin_pitch, 0.0),
                size: (0.002, 0.002),
                layer: PcbLayerType::Top,
                drill_size: Some(0.001),
            };
            connector.footprint.add_pad(pad);
        }

        // Create header body
        let body = crate::modeling::primitives::make_box(
            0.004,
            pin_count as f64 * pin_pitch + pin_pitch,
            0.002,
            Some(position + Vector::new(0.0, (pin_count - 1) as f64 * pin_pitch / 2.0, 0.001)),
        );

        connector.set_body_geometry(body);
        connector.height = 0.002;
        connector.color = (0.4, 0.4, 0.4); // Header color

        connector
    }

    /// Create a USB connector
    pub fn usb(name: &str, position: Point) -> Self {
        let mut connector = Self::new(name, ConnectorType::USB, position);

        // Create USB footprint
        let pad1 = PcbPad {
            name: "1".to_string(),
            shape: PadShape::Rectangle,
            position: position + Vector::new(-0.005, 0.003, 0.0),
            size: (0.002, 0.006),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad2 = PcbPad {
            name: "2".to_string(),
            shape: PadShape::Rectangle,
            position: position + Vector::new(-0.002, 0.003, 0.0),
            size: (0.002, 0.006),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad3 = PcbPad {
            name: "3".to_string(),
            shape: PadShape::Rectangle,
            position: position + Vector::new(0.001, 0.003, 0.0),
            size: (0.002, 0.006),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad4 = PcbPad {
            name: "4".to_string(),
            shape: PadShape::Rectangle,
            position: position + Vector::new(0.004, 0.003, 0.0),
            size: (0.002, 0.006),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        connector.footprint.add_pad(pad1);
        connector.footprint.add_pad(pad2);
        connector.footprint.add_pad(pad3);
        connector.footprint.add_pad(pad4);

        // Create USB body
        let body = crate::modeling::primitives::make_box(
            0.015,
            0.01,
            0.01,
            Some(position + Vector::new(0.0, 0.0, 0.005)),
        );

        connector.set_body_geometry(body);
        connector.height = 0.01;
        connector.color = (0.8, 0.8, 0.8); // USB color

        connector
    }
}

/// Sensor component
#[derive(Debug, Clone)]
pub struct Sensor {
    pub name: String,
    pub sensor_type: SensorType,
    pub footprint: PcbComponentFootprint,
    pub body_geometry: Option<TopoDsSolid>,
    pub position: Point,
    pub rotation: (f64, f64, f64), // Roll, pitch, yaw in radians
    pub height: f64,
    pub color: (f32, f32, f32), // RGB
}

impl Sensor {
    /// Create a new sensor
    pub fn new(name: &str, sensor_type: SensorType, position: Point) -> Self {
        Self {
            name: name.to_string(),
            sensor_type,
            footprint: PcbComponentFootprint::new(name),
            body_geometry: None,
            position,
            rotation: (0.0, 0.0, 0.0),
            height: 0.0,
            color: (0.2, 0.6, 0.8), // Default sensor color
        }
    }

    /// Set the sensor's body geometry
    pub fn set_body_geometry(&mut self, geometry: TopoDsSolid) {
        self.body_geometry = Some(geometry);
    }

    /// Generate the sensor as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add body geometry if available
        if let Some(body) = &self.body_geometry {
            // Implement shell addition
            // Add all shells from the body geometry to the solid
            for shell in body.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }

    /// Create a temperature sensor
    pub fn temperature(name: &str, position: Point) -> Self {
        let mut sensor = Self::new(name, SensorType::Temperature, position);

        // Create temperature sensor footprint
        let pad1 = PcbPad {
            name: "VCC".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(-0.003, 0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad2 = PcbPad {
            name: "GND".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(0.003, 0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad3 = PcbPad {
            name: "OUT".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(0.0, -0.003, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        sensor.footprint.add_pad(pad1);
        sensor.footprint.add_pad(pad2);
        sensor.footprint.add_pad(pad3);

        // Create temperature sensor body
        let body = crate::modeling::primitives::make_cylinder(
            0.004,
            0.004,
            Some(position + Vector::new(0.0, 0.0, 0.002)),
        );

        sensor.set_body_geometry(body);
        sensor.height = 0.004;
        sensor.color = (0.8, 0.4, 0.0); // Temperature sensor color

        sensor
    }

    /// Create a light sensor
    pub fn light(name: &str, position: Point) -> Self {
        let mut sensor = Self::new(name, SensorType::Light, position);

        // Create light sensor footprint
        let pad1 = PcbPad {
            name: "VCC".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(-0.003, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        let pad2 = PcbPad {
            name: "GND".to_string(),
            shape: PadShape::Round,
            position: position + Vector::new(0.003, 0.0, 0.0),
            size: (0.002, 0.002),
            layer: PcbLayerType::Top,
            drill_size: None,
        };

        sensor.footprint.add_pad(pad1);
        sensor.footprint.add_pad(pad2);

        // Create light sensor body
        let body = crate::modeling::primitives::make_sphere(
            0.003,
            Some(position + Vector::new(0.0, 0.0, 0.003)),
        );

        sensor.set_body_geometry(body);
        sensor.height = 0.006;
        sensor.color = (0.9, 0.9, 0.0); // Light sensor color

        sensor
    }
}

/// Board assembly containing multiple components
#[derive(Debug, Clone)]
pub struct BoardAssembly {
    pub name: String,
    pub components: Vec<BoardComponent>,
    pub connectors: Vec<Connector>,
    pub sensors: Vec<Sensor>,
}

impl BoardAssembly {
    /// Create a new board assembly
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: Vec::new(),
            connectors: Vec::new(),
            sensors: Vec::new(),
        }
    }

    /// Add a component to the assembly
    pub fn add_component(&mut self, component: BoardComponent) {
        self.components.push(component);
    }

    /// Add a connector to the assembly
    pub fn add_connector(&mut self, connector: Connector) {
        self.connectors.push(connector);
    }

    /// Add a sensor to the assembly
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Generate the assembly as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Add all components
        for component in &self.components {
            let component_solid = component.to_solid();
            // Implement shell addition
            for shell in component_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        // Add all connectors
        for connector in &self.connectors {
            let connector_solid = connector.to_solid();
            // Implement shell addition
            for shell in connector_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        // Add all sensors
        for sensor in &self.sensors {
            let sensor_solid = sensor.to_solid();
            // Implement shell addition
            for shell in sensor_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
    }
}
