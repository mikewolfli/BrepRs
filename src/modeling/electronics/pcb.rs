use crate::foundation::handle::Handle;
use crate::geometry::{Axis, Circle, Cylinder, Direction, Line, Point, Vector};
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShell, TopoDsSolid, TopoDsVertex, TopoDsWire};

/// PCB layer type
#[derive(Debug, Clone, PartialEq)]
pub enum PcbLayerType {
    Top,
    Bottom,
    Internal(usize),
    SolderMaskTop,
    SolderMaskBottom,
    SilkscreenTop,
    SilkscreenBottom,
    Dielectric,
}

/// PCB layer definition
#[derive(Debug, Clone)]
pub struct PcbLayer {
    pub layer_type: PcbLayerType,
    pub thickness: f64,
    pub material: String,
    pub position: f64, // Z-coordinate position
}

/// PCB pad shape
#[derive(Debug, Clone, PartialEq)]
pub enum PadShape {
    Round,
    Square,
    Rectangle,
    Oval,
    Custom,
}

/// PCB pad definition
#[derive(Debug, Clone)]
pub struct PcbPad {
    pub name: String,
    pub shape: PadShape,
    pub position: Point,
    pub size: (f64, f64), // width, height
    pub layer: PcbLayerType,
    pub drill_size: Option<f64>, // For through-hole pads
}

/// PCB via definition
#[derive(Debug, Clone)]
pub struct PcbVia {
    pub position: Point,
    pub drill_size: f64,
    pub pad_size: f64,
    pub layers: Vec<PcbLayerType>, // Layers the via connects
}

/// PCB trace segment
#[derive(Debug, Clone)]
pub struct PcbTraceSegment {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub layer: PcbLayerType,
}

/// PCB trace
#[derive(Debug, Clone)]
pub struct PcbTrace {
    pub name: String,
    pub segments: Vec<PcbTraceSegment>,
    pub layer: PcbLayerType,
}

/// PCB board definition
#[derive(Debug, Clone)]
pub struct PcbBoard {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub layers: Vec<PcbLayer>,
    pub pads: Vec<PcbPad>,
    pub vias: Vec<PcbVia>,
    pub traces: Vec<PcbTrace>,
    pub origin: Point,
}

impl PcbBoard {
    /// Create a new PCB board
    pub fn new(name: &str, width: f64, height: f64, origin: Point) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            layers: Vec::new(),
            pads: Vec::new(),
            vias: Vec::new(),
            traces: Vec::new(),
            origin,
        }
    }

    /// Add a layer to the PCB
    pub fn add_layer(&mut self, layer: PcbLayer) {
        self.layers.push(layer);
    }

    /// Add a pad to the PCB
    pub fn add_pad(&mut self, pad: PcbPad) {
        self.pads.push(pad);
    }

    /// Add a via to the PCB
    pub fn add_via(&mut self, via: PcbVia) {
        self.vias.push(via);
    }

    /// Add a trace to the PCB
    pub fn add_trace(&mut self, trace: PcbTrace) {
        self.traces.push(trace);
    }

    /// Generate the PCB as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // Create board layers
        for layer in &self.layers {
            let layer_shell = self.create_layer_shell(layer);
            solid.add_shell(layer_shell);
        }

        // Create pads
        for pad in &self.pads {
            let pad_shell = self.create_pad_shell(pad);
            solid.add_shell(pad_shell);
        }

        // Create vias
        for via in &self.vias {
            let via_shell = self.create_via_shell(via);
            solid.add_shell(via_shell);
        }

        // Create traces
        for trace in &self.traces {
            for segment in &trace.segments {
                let trace_shell = self.create_trace_segment_shell(segment);
                solid.add_shell(trace_shell);
            }
        }

        solid
    }

    /// Create a layer shell
    fn create_layer_shell(&self, layer: &PcbLayer) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Create layer rectangle
        let rectangle = Rectangle::new(
            self.origin + Vector::new(0.0, 0.0, layer.position),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            self.width,
            self.height,
        );

        let face = rectangle.to_face();
        shell.add_face(face);

        shell
    }

    /// Create a pad shell
    fn create_pad_shell(&self, pad: &PcbPad) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Find the layer position
        let layer_position = self.find_layer_position(&pad.layer).unwrap_or(0.0);

        match pad.shape {
            PadShape::Round => {
                let radius = pad.size.0 / 2.0;
                let circle = Circle::new(
                    Axis::new(
                        pad.position + Vector::new(0.0, 0.0, layer_position),
                        Vector::new(0.0, 0.0, 1.0),
                    ),
                    radius,
                );

                let face = circle.to_face();
                shell.add_face(face);
            }
            PadShape::Square => {
                let rectangle = Rectangle::new(
                    pad.position - Vector::new(pad.size.0 / 2.0, pad.size.1 / 2.0, 0.0)
                        + Vector::new(0.0, 0.0, layer_position),
                    Vector::new(1.0, 0.0, 0.0),
                    Vector::new(0.0, 1.0, 0.0),
                    pad.size.0,
                    pad.size.1,
                );

                let face = rectangle.to_face();
                shell.add_face(face);
            }
            PadShape::Rectangle => {
                let rectangle = Rectangle::new(
                    pad.position - Vector::new(pad.size.0 / 2.0, pad.size.1 / 2.0, 0.0)
                        + Vector::new(0.0, 0.0, layer_position),
                    Vector::new(1.0, 0.0, 0.0),
                    Vector::new(0.0, 1.0, 0.0),
                    pad.size.0,
                    pad.size.1,
                );

                let face = rectangle.to_face();
                shell.add_face(face);
            }
            _ => {
                // For other shapes, use a simple rectangle as placeholder
                let rectangle = Rectangle::new(
                    pad.position - Vector::new(pad.size.0 / 2.0, pad.size.1 / 2.0, 0.0)
                        + Vector::new(0.0, 0.0, layer_position),
                    Vector::new(1.0, 0.0, 0.0),
                    Vector::new(0.0, 1.0, 0.0),
                    pad.size.0,
                    pad.size.1,
                );

                let face = rectangle.to_face();
                shell.add_face(face);
            }
        }

        // Add drill hole if it's a through-hole pad
        if let Some(drill_size) = pad.drill_size {
            let drill_radius = drill_size / 2.0;
            let cylinder = Cylinder::new(
                Axis::new(
                    pad.position - Vector::new(0.0, 0.0, 1.0), // Extend below the board
                    Vector::new(0.0, 0.0, 1.0),
                ),
                drill_radius,
                2.0, // Height greater than board thickness
            );

            let face = cylinder.to_face();
            shell.add_face(face);
        }

        shell
    }

    /// Create a via shell
    fn create_via_shell(&self, via: &PcbVia) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Find the minimum and maximum layer positions
        let mut min_position = f64::MAX;
        let mut max_position = f64::MIN;

        for layer_type in &via.layers {
            if let Some(position) = self.find_layer_position(layer_type) {
                min_position = min_position.min(position);
                max_position = max_position.max(position);
            }
        }

        // Create via cylinder
        let height = max_position - min_position + 0.1; // Add some extra height
        let cylinder = Cylinder::new(
            Axis::new(
                via.position + Vector::new(0.0, 0.0, min_position - 0.05),
                Vector::new(0.0, 0.0, 1.0),
            ),
            via.pad_size / 2.0,
            height,
        );

        let face = cylinder.to_face();
        shell.add_face(face);

        // Create drill hole
        let drill_cylinder = Cylinder::new(
            Axis::new(
                via.position + Vector::new(0.0, 0.0, min_position - 0.05),
                Vector::new(0.0, 0.0, 1.0),
            ),
            via.drill_size / 2.0,
            height,
        );

        let drill_face = drill_cylinder.to_face();
        shell.add_face(drill_face);

        shell
    }

    /// Create a trace segment shell
    fn create_trace_segment_shell(&self, segment: &PcbTraceSegment) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Find the layer position
        let layer_position = self.find_layer_position(&segment.layer).unwrap_or(0.0);

        // Create trace segment
        let direction = (segment.end - segment.start).normalized();
        let normal = Vector::new(-direction.y, direction.x, 0.0); // Perpendicular to the trace

        // Create rectangle for the trace segment
        let start1 =
            segment.start + normal * segment.width / 2.0 + Vector::new(0.0, 0.0, layer_position);
        let start2 =
            segment.start - normal * segment.width / 2.0 + Vector::new(0.0, 0.0, layer_position);
        let end1 =
            segment.end + normal * segment.width / 2.0 + Vector::new(0.0, 0.0, layer_position);
        let end2 =
            segment.end - normal * segment.width / 2.0 + Vector::new(0.0, 0.0, layer_position);

        // Create wire from the four points
        let mut wire = TopoDsWire::new();

        let edge1 = TopoDsEdge::new(
            Handle::new(TopoDsVertex::new(start1)),
            Handle::new(TopoDsVertex::new(end1)),
        );
        let edge2 = TopoDsEdge::new(
            Handle::new(TopoDsVertex::new(end1)),
            Handle::new(TopoDsVertex::new(end2)),
        );
        let edge3 = TopoDsEdge::new(
            Handle::new(TopoDsVertex::new(end2)),
            Handle::new(TopoDsVertex::new(start2)),
        );
        let edge4 = TopoDsEdge::new(
            Handle::new(TopoDsVertex::new(start2)),
            Handle::new(TopoDsVertex::new(start1)),
        );

        wire.add_edge(edge1);
        wire.add_edge(edge2);
        wire.add_edge(edge3);
        wire.add_edge(edge4);

        let face = TopoDsFace::with_outer_wire(wire);
        shell.add_face(face);

        shell
    }

    /// Find the position of a layer
    fn find_layer_position(&self, layer_type: &PcbLayerType) -> Option<f64> {
        self.layers
            .iter()
            .find(|layer| &layer.layer_type == layer_type)
            .map(|layer| layer.position)
    }

    /// Calculate the total thickness of the PCB
    pub fn total_thickness(&self) -> f64 {
        if self.layers.is_empty() {
            return 0.0;
        }

        let min_position = self
            .layers
            .iter()
            .map(|layer| layer.position)
            .min()
            .unwrap_or(0.0);
        let max_position = self
            .layers
            .iter()
            .map(|layer| layer.position + layer.thickness)
            .max()
            .unwrap_or(0.0);

        max_position - min_position
    }

    /// Generate a simple 2-layer PCB
    pub fn two_layer_pcb(name: &str, width: f64, height: f64) -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let mut pcb = Self::new(name, width, height, origin);

        // Add top layer
        pcb.add_layer(PcbLayer {
            layer_type: PcbLayerType::Top,
            thickness: 0.035,
            material: "Copper".to_string(),
            position: 0.0,
        });

        // Add dielectric layer
        pcb.add_layer(PcbLayer {
            layer_type: PcbLayerType::Dielectric,
            thickness: 1.6,
            material: "FR4".to_string(),
            position: 0.035,
        });

        // Add bottom layer
        pcb.add_layer(PcbLayer {
            layer_type: PcbLayerType::Bottom,
            thickness: 0.035,
            material: "Copper".to_string(),
            position: 1.635,
        });

        pcb
    }
}

/// PCB component footprint
#[derive(Debug, Clone)]
pub struct PcbComponentFootprint {
    pub name: String,
    pub pads: Vec<PcbPad>,
    pub silkscreen: Vec<(Point, Point)>, // Lines for silkscreen
}

impl PcbComponentFootprint {
    /// Create a new component footprint
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pads: Vec::new(),
            silkscreen: Vec::new(),
        }
    }

    /// Add a pad to the footprint
    pub fn add_pad(&mut self, pad: PcbPad) {
        self.pads.push(pad);
    }

    /// Add a silkscreen line
    pub fn add_silkscreen_line(&mut self, start: Point, end: Point) {
        self.silkscreen.push((start, end));
    }
}
