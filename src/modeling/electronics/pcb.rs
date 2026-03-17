use crate::foundation::handle::Handle;
use crate::geometry::{Axis, Cylinder, Direction, Point, Vector};
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShell, TopoDsSolid, TopoDsVertex, TopoDsWire};
use std::sync::Arc;

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
            solid.add_shell(Handle::new(Arc::new(layer_shell)));
        }

        // Create pads
        for pad in &self.pads {
            let pad_shell = self.create_pad_shell(pad);
            solid.add_shell(Handle::new(Arc::new(pad_shell)));
        }

        // Create vias
        for via in &self.vias {
            let via_shell = self.create_via_shell(via);
            solid.add_shell(Handle::new(Arc::new(via_shell)));
        }

        // Create traces
        for trace in &self.traces {
            for segment in &trace.segments {
                let trace_shell = self.create_trace_segment_shell(segment);
                solid.add_shell(Handle::new(Arc::new(trace_shell)));
            }
        }

        solid
    }

    /// Create a layer shell
    fn create_layer_shell(&self, layer: &PcbLayer) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Create layer rectangle manually
        let z = layer.position;
        let origin = self.origin;
        let w = self.width;
        let h = self.height;

        // Create four corners
        let p1 = Point::new(origin.x, origin.y, z);
        let p2 = Point::new(origin.x + w, origin.y, z);
        let p3 = Point::new(origin.x + w, origin.y + h, z);
        let p4 = Point::new(origin.x, origin.y + h, z);

        // Create vertices
        let v1 = Arc::new(TopoDsVertex::new(p1));
        let v2 = Arc::new(TopoDsVertex::new(p2));
        let v3 = Arc::new(TopoDsVertex::new(p3));
        let v4 = Arc::new(TopoDsVertex::new(p4));

        // Create edges
        let e1 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v1.clone()), Handle::new(v2.clone()))));
        let e2 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v2.clone()), Handle::new(v3.clone()))));
        let e3 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v3.clone()), Handle::new(v4.clone()))));
        let e4 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v4.clone()), Handle::new(v1.clone()))));

        // Create wire
        let mut wire = TopoDsWire::new();
        wire.add_edge(e1);
        wire.add_edge(e2);
        wire.add_edge(e3);
        wire.add_edge(e4);

        // Create face
        let face = Handle::new(Arc::new(TopoDsFace::with_outer_wire(wire)));
        shell.add_face(face);

        shell
    }

    /// Create a pad shell
    fn create_pad_shell(&self, pad: &PcbPad) -> TopoDsShell {
        let mut shell = TopoDsShell::new();

        // Find the layer position
        let layer_position = self.find_layer_position(&pad.layer).unwrap_or(0.0);
        let z = layer_position;

        match pad.shape {
            PadShape::Round => {
                // Approximate round pad with an octagon
                let radius = pad.size.0 / 2.0;
                let center_x = pad.position.x;
                let center_y = pad.position.y;
                let n_sides = 8;
                
                let mut vertices: Vec<Arc<TopoDsVertex>> = Vec::with_capacity(n_sides);
                for i in 0..n_sides {
                    let angle = 2.0 * std::f64::consts::PI * i as f64 / n_sides as f64;
                    let x = center_x + radius * angle.cos();
                    let y = center_y + radius * angle.sin();
                    vertices.push(Arc::new(TopoDsVertex::new(Point::new(x, y, z))));
                }
                
                let mut wire = TopoDsWire::new();
                for i in 0..n_sides {
                    let j = (i + 1) % n_sides;
                    let edge = Handle::new(Arc::new(TopoDsEdge::new(
                        Handle::new(vertices[i].clone()),
                        Handle::new(vertices[j].clone()),
                    )));
                    wire.add_edge(edge);
                }
                
                let face = Handle::new(Arc::new(TopoDsFace::with_outer_wire(wire)));
                shell.add_face(face);
            }
            PadShape::Square | PadShape::Rectangle | _ => {
                // Create rectangle manually
                let half_w = pad.size.0 / 2.0;
                let half_h = pad.size.1 / 2.0;
                let cx = pad.position.x;
                let cy = pad.position.y;

                let p1 = Point::new(cx - half_w, cy - half_h, z);
                let p2 = Point::new(cx + half_w, cy - half_h, z);
                let p3 = Point::new(cx + half_w, cy + half_h, z);
                let p4 = Point::new(cx - half_w, cy + half_h, z);

                let v1 = Arc::new(TopoDsVertex::new(p1));
                let v2 = Arc::new(TopoDsVertex::new(p2));
                let v3 = Arc::new(TopoDsVertex::new(p3));
                let v4 = Arc::new(TopoDsVertex::new(p4));

                let e1 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v1.clone()), Handle::new(v2.clone()))));
                let e2 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v2.clone()), Handle::new(v3.clone()))));
                let e3 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v3.clone()), Handle::new(v4.clone()))));
                let e4 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v4.clone()), Handle::new(v1.clone()))));

                let mut wire = TopoDsWire::new();
                wire.add_edge(e1);
                wire.add_edge(e2);
                wire.add_edge(e3);
                wire.add_edge(e4);

                let face = Handle::new(Arc::new(TopoDsFace::with_outer_wire(wire)));
                shell.add_face(face);
            }
        }

        // Add drill hole if it's a through-hole pad
        if let Some(drill_size) = pad.drill_size {
            let drill_radius = drill_size / 2.0;
            let axis = Axis::new(
                Point::new(pad.position.x, pad.position.y, -1.0),
                Direction::new(0.0, 0.0, 1.0),
            );
            let cylinder = Cylinder::from_axis(&axis, drill_radius);
            let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
            )))));
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
        let axis = Axis::new(
            Point::new(via.position.x, via.position.y, min_position - 0.05),
            Direction::new(0.0, 0.0, 1.0),
        );
        let cylinder = Cylinder::from_axis(&axis, via.pad_size / 2.0);
        let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
        )))));
        shell.add_face(face);

        // Create drill hole
        let drill_cylinder = Cylinder::from_axis(&axis, via.drill_size / 2.0);
        let drill_face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
            crate::geometry::surface_enum::SurfaceEnum::Cylinder(drill_cylinder),
        )))));
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
        let start1 = Point::new(
            segment.start.x + normal.x * segment.width / 2.0,
            segment.start.y + normal.y * segment.width / 2.0,
            layer_position,
        );
        let start2 = Point::new(
            segment.start.x - normal.x * segment.width / 2.0,
            segment.start.y - normal.y * segment.width / 2.0,
            layer_position,
        );
        let end1 = Point::new(
            segment.end.x + normal.x * segment.width / 2.0,
            segment.end.y + normal.y * segment.width / 2.0,
            layer_position,
        );
        let end2 = Point::new(
            segment.end.x - normal.x * segment.width / 2.0,
            segment.end.y - normal.y * segment.width / 2.0,
            layer_position,
        );

        // Create wire from the four points
        let mut wire = TopoDsWire::new();

        let v1 = Arc::new(TopoDsVertex::new(start1));
        let v2 = Arc::new(TopoDsVertex::new(end1));
        let v3 = Arc::new(TopoDsVertex::new(end2));
        let v4 = Arc::new(TopoDsVertex::new(start2));

        let edge1 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v1.clone()), Handle::new(v2.clone()))));
        let edge2 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v2.clone()), Handle::new(v3.clone()))));
        let edge3 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v3.clone()), Handle::new(v4.clone()))));
        let edge4 = Handle::new(Arc::new(TopoDsEdge::new(Handle::new(v4.clone()), Handle::new(v1.clone()))));

        wire.add_edge(edge1);
        wire.add_edge(edge2);
        wire.add_edge(edge3);
        wire.add_edge(edge4);

        let face = Handle::new(Arc::new(TopoDsFace::with_outer_wire(wire)));
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
            .reduce(|a, b| a.min(b))
            .unwrap_or(0.0);
        let max_position = self
            .layers
            .iter()
            .map(|layer| layer.position + layer.thickness)
            .reduce(|a, b| a.max(b))
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
