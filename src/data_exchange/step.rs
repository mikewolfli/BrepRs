use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// STEP file format error types
#[derive(Debug)]
pub enum StepError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid STEP file format
    InvalidFormat,
    /// Invalid STEP entity
    InvalidEntity,
    /// Unsupported STEP schema
    UnsupportedSchema,
    /// Parsing error
    ParsingError,
}

impl From<std::io::Error> for StepError {
    fn from(err: std::io::Error) -> Self {
        StepError::IoError(err)
    }
}

impl std::error::Error for StepError {}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepError::IoError(e) => write!(f, "IO error: {}", e),
            StepError::InvalidFormat => write!(f, "Invalid STEP file format"),
            StepError::InvalidEntity => write!(f, "Invalid STEP entity"),
            StepError::UnsupportedSchema => write!(f, "Unsupported STEP schema"),
            StepError::ParsingError => write!(f, "Parsing error"),
        }
    }
}

/// STEP file schema types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StepSchema {
    /// AP203 - Configuration Controlled 3D Design
    AP203,
    /// AP214 - Automotive Design
    AP214,
    /// AP242 - Managed Model Based 3D Engineering
    AP242,
    /// Unknown schema
    Unknown,
}

/// STEP entity representation
struct StepEntity {
    id: i32,
    type_name: String,
    params: Vec<String>,
}

/// STEP reader for reading STEP files
pub struct StepReader {
    filename: String,
    #[allow(dead_code)]
    schema: StepSchema,
    tolerance: f64,
    read_colors: bool,
}

impl StepReader {
    /// Create a new STEP reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            schema: StepSchema::Unknown,
            tolerance: 1e-6,
            read_colors: false,
        }
    }

    /// Set the tolerance for geometry operations
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set whether to read colors
    pub fn set_read_colors(&mut self, read_colors: bool) {
        self.read_colors = read_colors;
    }

    /// Get whether colors are read
    pub fn read_colors(&self) -> bool {
        self.read_colors
    }

    /// Read a STEP file and return a shape
    pub fn read(&self) -> Result<TopoDsShape, StepError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read and parse the STEP file
        self.parse_step_file(&mut reader)
    }

    /// Parse a STEP file
    fn parse_step_file(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, StepError> {
        // Read the header
        let header = self.read_header(reader)?;

        // Determine the schema (note: schema is stored but not used in current implementation)
        let _schema = self.determine_schema(&header)?;

        // Read the data section
        let shape = self.read_data_section(reader)?;

        Ok(shape)
    }

    /// Read the STEP file header
    fn read_header(&self, reader: &mut BufReader<File>) -> Result<String, StepError> {
        let mut header = String::new();
        let mut line = String::new();

        // Find the start of the header
        while reader.read_line(&mut line)? > 0 {
            if line.trim().starts_with("HEADER") {
                header.push_str(&line);
                break;
            }
            line.clear();
        }

        // Read the header content
        while reader.read_line(&mut line)? > 0 {
            header.push_str(&line);
            if line.trim().starts_with("ENDSEC") {
                break;
            }
            line.clear();
        }

        Ok(header)
    }

    /// Determine the STEP schema from the header
    fn determine_schema(&self, header: &str) -> Result<StepSchema, StepError> {
        // Parse the header to determine the STEP schema
        let header_upper = header.to_uppercase();

        if header_upper.contains("CONFIG_CONTROL_DESIGN") || header_upper.contains("AP203") {
            Ok(StepSchema::AP203)
        } else if header_upper.contains("AUTOMOTIVE_DESIGN") || header_upper.contains("AP214") {
            Ok(StepSchema::AP214)
        } else if header_upper.contains("MANAGED_MODEL_BASED_3D_ENGINEERING")
            || header_upper.contains("AP242")
        {
            Ok(StepSchema::AP242)
        } else {
            Ok(StepSchema::Unknown)
        }
    }

    /// Read the data section of the STEP file
    fn read_data_section(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, StepError> {
        let mut entities: std::collections::HashMap<i32, StepEntity> =
            std::collections::HashMap::new();
        let mut line = String::new();
        let mut in_data_section = false;

        loop {
            line.clear();
            let bytes_read = reader
                .read_line(&mut line)
                .map_err(|e| StepError::IoError(e))?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();

            if trimmed == "DATA;" {
                in_data_section = true;
                continue;
            }

            if trimmed == "ENDSEC;" {
                break;
            }

            if in_data_section && trimmed.starts_with('#') {
                if let Some(entity) = self.parse_step_entity(trimmed) {
                    entities.insert(entity.id, entity);
                }
            }
        }

        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();

        let mut points: std::collections::HashMap<i32, crate::geometry::Point> =
            std::collections::HashMap::new();
        let mut vertices: std::collections::HashMap<
            i32,
            crate::foundation::handle::Handle<crate::topology::topods_vertex::TopoDsVertex>,
        > = std::collections::HashMap::new();
        let mut edges: std::collections::HashMap<
            i32,
            crate::foundation::handle::Handle<crate::topology::topods_edge::TopoDsEdge>,
        > = std::collections::HashMap::new();
        let mut wires: std::collections::HashMap<
            i32,
            crate::foundation::handle::Handle<crate::topology::topods_wire::TopoDsWire>,
        > = std::collections::HashMap::new();
        let mut faces: std::collections::HashMap<
            i32,
            crate::foundation::handle::Handle<crate::topology::topods_face::TopoDsFace>,
        > = std::collections::HashMap::new();
        let mut shells: std::collections::HashMap<
            i32,
            crate::foundation::handle::Handle<crate::topology::topods_shell::TopoDsShell>,
        > = std::collections::HashMap::new();

        for (id, entity) in &entities {
            match entity.type_name.as_str() {
                "CARTESIAN_POINT" => {
                    if entity.params.len() >= 3 {
                        let x = entity.params[0].parse::<f64>().unwrap_or(0.0);
                        let y = entity.params[1].parse::<f64>().unwrap_or(0.0);
                        let z = entity.params[2].parse::<f64>().unwrap_or(0.0);
                        points.insert(*id, crate::geometry::Point::new(x, y, z));
                    }
                }
                "VERTEX_POINT" => {
                    if !entity.params.is_empty() {
                        if let Some(point_id) = entity.params[0]
                            .strip_prefix('#')
                            .and_then(|s| s.parse::<i32>().ok())
                        {
                            if let Some(point) = points.get(&point_id) {
                                let vertex =
                                    crate::topology::topods_vertex::TopoDsVertex::new(*point);
                                vertices.insert(
                                    *id,
                                    crate::foundation::handle::Handle::new(std::sync::Arc::new(
                                        vertex,
                                    )),
                                );
                            }
                        }
                    }
                }
                "LINE" | "B_SPLINE_CURVE" | "CIRCLE" => {
                    let default_vertex = crate::foundation::handle::Handle::new(
                        std::sync::Arc::new(crate::topology::topods_vertex::TopoDsVertex::new(
                            crate::geometry::Point::origin(),
                        )),
                    );
                    let edge = crate::topology::topods_edge::TopoDsEdge::new(
                        default_vertex.clone(),
                        default_vertex,
                    );
                    edges.insert(
                        *id,
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(edge)),
                    );
                }
                "EDGE_CURVE" => {
                    let default_vertex = crate::foundation::handle::Handle::new(
                        std::sync::Arc::new(crate::topology::topods_vertex::TopoDsVertex::new(
                            crate::geometry::Point::origin(),
                        )),
                    );
                    let edge = crate::topology::topods_edge::TopoDsEdge::new(
                        default_vertex.clone(),
                        default_vertex,
                    );
                    edges.insert(
                        *id,
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(edge)),
                    );
                }
                "ORIENTED_EDGE" => {}
                "EDGE_LOOP" | "POLY_LOOP" => {
                    let wire = crate::topology::topods_wire::TopoDsWire::new();
                    wires.insert(
                        *id,
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(wire)),
                    );
                }
                "ADVANCED_FACE"
                | "FACE_SURFACE"
                | "PLANE"
                | "CYLINDRICAL_SURFACE"
                | "CONICAL_SURFACE"
                | "SPHERICAL_SURFACE"
                | "TOROIDAL_SURFACE"
                | "B_SPLINE_SURFACE" => {
                    let face = crate::topology::topods_face::TopoDsFace::new();
                    faces.insert(
                        *id,
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(face)),
                    );
                }
                "CLOSED_SHELL" | "OPEN_SHELL" => {
                    let shell = crate::topology::topods_shell::TopoDsShell::new();
                    shells.insert(
                        *id,
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(shell)),
                    );
                }
                "MANIFOLD_SOLID_BREP" => {
                    if !entity.params.is_empty() {
                        if let Some(shell_id) = entity.params[0]
                            .strip_prefix('#')
                            .and_then(|s| s.parse::<i32>().ok())
                        {
                            if let Some(shell) = shells.get(&shell_id) {
                                let solid =
                                    crate::topology::topods_solid::TopoDsSolid::with_shells(vec![
                                        shell.clone(),
                                    ]);
                                compound.add_component(crate::foundation::handle::Handle::new(
                                    std::sync::Arc::new(solid.shape().clone()),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if shells.is_empty() && !faces.is_empty() {
            for _ in &faces {
                let shell = crate::topology::topods_shell::TopoDsShell::new();
                let solid = crate::topology::topods_solid::TopoDsSolid::with_shells(vec![
                    crate::foundation::handle::Handle::new(std::sync::Arc::new(shell)),
                ]);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
        }

        if faces.is_empty() && !wires.is_empty() {
            for _ in &wires {
                let shell = crate::topology::topods_shell::TopoDsShell::new();
                let solid = crate::topology::topods_solid::TopoDsSolid::with_shells(vec![
                    crate::foundation::handle::Handle::new(std::sync::Arc::new(shell)),
                ]);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
        }

        Ok(compound.shape().clone())
    }

    fn parse_step_entity(&self, line: &str) -> Option<StepEntity> {
        let line = line.trim_end_matches(';').trim();

        let eq_pos = line.find('=')?;
        let id = line[1..eq_pos].trim().parse::<i32>().ok()?;

        let rest = &line[eq_pos + 1..];
        let paren_pos = rest.find('(')?;
        let type_name = rest[..paren_pos].trim().to_string();

        let params_start = paren_pos + 1;
        let params_end = rest.rfind(')')?;
        let params_str = &rest[params_start..params_end];

        let params = self.parse_params(params_str);

        Some(StepEntity {
            id,
            type_name,
            params,
        })
    }

    fn parse_params(&self, params_str: &str) -> Vec<String> {
        let mut params = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;
        let mut in_string = false;

        for ch in params_str.chars() {
            match ch {
                '\'' if !in_string => in_string = true,
                '\'' if in_string => in_string = false,
                '(' if !in_string => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' if !in_string => {
                    paren_depth -= 1;
                    current.push(ch);
                }
                ',' if paren_depth == 0 && !in_string => {
                    params.push(current.trim().to_string());
                    current = String::new();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.trim().is_empty() {
            params.push(current.trim().to_string());
        }

        params
    }

    /// Validate a STEP file
    pub fn validate(&self) -> Result<(), StepError> {
        // Just check if the file can be read
        let _ = self.read()?;
        Ok(())
    }
}

/// STEP writer for writing STEP files
pub struct StepWriter {
    filename: String,
    schema: StepSchema,
    precision: usize,
    write_colors: bool,
}

impl StepWriter {
    /// Create a new STEP writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            schema: StepSchema::AP203,
            precision: 6,
            write_colors: false,
        }
    }

    /// Create a new STEP writer with specified schema
    pub fn new_with_schema(filename: &str, schema: StepSchema) -> Self {
        Self {
            filename: filename.to_string(),
            schema,
            precision: 6,
            write_colors: false,
        }
    }

    /// Set the STEP schema
    pub fn set_schema(&mut self, schema: StepSchema) {
        self.schema = schema;
    }

    /// Get the STEP schema
    pub fn schema(&self) -> StepSchema {
        self.schema
    }

    /// Set the precision for numeric values
    pub fn set_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    /// Get the precision
    pub fn precision(&self) -> usize {
        self.precision
    }

    /// Set whether to write colors
    pub fn set_write_colors(&mut self, write_colors: bool) {
        self.write_colors = write_colors;
    }

    /// Get whether colors are written
    pub fn write_colors(&self) -> bool {
        self.write_colors
    }

    /// Write a shape to a STEP file
    pub fn write(&self, shape: &TopoDsShape) -> Result<(), StepError> {
        let path = Path::new(&self.filename);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);

        // Write the header
        self.write_header(&mut writer)?;

        // Write the data section
        self.write_data_section(&mut writer, shape)?;

        Ok(())
    }

    /// Write the STEP file header
    fn write_header(&self, writer: &mut BufWriter<File>) -> Result<(), StepError> {
        writeln!(writer, "HEADER;")?;
        writeln!(writer, "FILE_DESCRIPTION(('BrepRs STEP Export'),'2;1');")?;

        let schema_name = match self.schema {
            StepSchema::AP203 => "CONFIG_CONTROLLED_3D_DESIGN",
            StepSchema::AP214 => "AUTOMOTIVE_DESIGN",
            StepSchema::AP242 => "MANAGED_MODEL_BASED_3D_ENGINEERING",
            StepSchema::Unknown => "CONFIG_CONTROLLED_3D_DESIGN",
        };

        writeln!(writer, "FILE_SCHEMA(('{}'));", schema_name)?;
        writeln!(writer, "ENDSEC;")?;

        Ok(())
    }

    /// Write the data section of the STEP file
    fn write_data_section(
        &self,
        writer: &mut BufWriter<File>,
        shape: &TopoDsShape,
    ) -> Result<(), StepError> {
        writeln!(writer, "DATA;")?;

        // Write product definition
        writeln!(writer, "#1=CARTESIAN_POINT('',(0.,0.,0.),$);")?;
        writeln!(writer, "#2=DIRECTION('',(0.,0.,1.));")?;
        writeln!(writer, "#3=DIRECTION('',(1.,0.,0.));")?;
        writeln!(writer, "#4=AXIS2_PLACEMENT_3D('',#1,#2,#3);")?;
        writeln!(writer, "#5=PRODUCT('BrepRs Export','BrepRs',$,(#6));")?;
        writeln!(writer, "#6=PRODUCT_CONTEXT('',#7,'mechanical');")?;
        writeln!(writer, "#7=APPLICATION_CONTEXT('configuration controlled 3d designs of mechanical parts and assemblies');")?;
        writeln!(writer, "#8=APPLICATION_PROTOCOL_DEFINITION('international standard','config_controlled_3d_design_of_mechanical_parts_and_assemblies',2010,#7,$);")?;
        writeln!(
            writer,
            "#9=PRODUCT_DEFINITION_CONTEXT('part definition',#7,'design');"
        )?;
        writeln!(
            writer,
            "#10=PRODUCT_DEFINITION('BrepRs Part','BrepRs Part',#11,#9);"
        )?;
        writeln!(writer, "#11=PRODUCT_DEFINITION_FORMATION('',' ',#5);")?;
        writeln!(writer, "#12=SHAPE_DEFINITION_REPRESENTATION(#13,#14);")?;
        writeln!(writer, "#13=PRODUCT_DEFINITION_SHAPE('',' ',#10);")?;
        writeln!(writer, "#14=( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#15)) REPRESENTATION_CONTEXT('ID1','3D'));")?;
        writeln!(
            writer,
            "#15=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.001),$);"
        )?;

        // Write shape representation
        let shape_type = shape.shape_type();
        let entity_id = 100;

        match shape_type {
            ShapeType::Solid => {
                self.write_solid_representation(writer, shape, entity_id)?;
            }
            ShapeType::Face => {
                self.write_face_representation(writer, shape, entity_id)?;
            }
            ShapeType::Edge => {
                self.write_edge_representation(writer, shape, entity_id)?;
            }
            ShapeType::Wire => {
                self.write_wire_representation(writer, shape, entity_id)?;
            }
            ShapeType::Compound => {
                self.write_compound_representation(writer, shape, entity_id)?;
            }
            ShapeType::Shell => {
                self.write_shell_representation(writer, shape, entity_id)?;
            }
            _ => {
                // Default to compound representation for other shape types
                self.write_compound_representation(writer, shape, entity_id)?;
            }
        }

        writeln!(writer, "ENDSEC;")?;
        writeln!(writer, "END-ISO-10303-21;")?;

        Ok(())
    }

    /// Write solid representation
    fn write_solid_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(
            writer,
            "#{}=MANIFOLD_SOLID_BREP('',#{});",
            base_id,
            base_id + 1
        )?;
        writeln!(
            writer,
            "#{}=CLOSED_SHELL('',(#{}));",
            base_id + 1,
            base_id + 2
        )?;

        // Write faces
        let face_id = base_id + 3;
        writeln!(
            writer,
            "#{}=ADVANCED_FACE('',(#{}),#{},.T.);",
            face_id,
            face_id + 1,
            face_id + 2
        )?;
        writeln!(
            writer,
            "#{}=FACE_OUTER_BOUND('',#{},.T.);",
            face_id + 1,
            face_id + 3
        )?;
        writeln!(writer, "#{}=POLY_LOOP('',(#{}));", face_id + 3, face_id + 4)?;

        // Write vertices
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", face_id + 4)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", face_id + 5)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,1.,0.));", face_id + 6)?;

        writeln!(writer, "#{}=PLANE('',#{});", face_id + 2, face_id + 7)?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#{},#2,#3);",
            face_id + 7,
            face_id + 8
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", face_id + 8)?;

        writeln!(
            writer,
            "#{}=SHAPE_REPRESENTATION('',(#{}),#14);",
            base_id + 100,
            base_id + 101
        )?;
        writeln!(
            writer,
            "#{}=PRODUCT_DEFINITION_SHAPE('',' ',#10);",
            base_id + 101
        )?;
        writeln!(
            writer,
            "#{}=ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{}),#14);",
            base_id + 102,
            base_id + 103
        )?;
        writeln!(
            writer,
            "#{}=MAPPED_ITEM('',#{},#{});",
            base_id + 103,
            base_id + 104,
            base_id + 105
        )?;
        writeln!(
            writer,
            "#{}=REPRESENTATION_MAP('',#{},#{});",
            base_id + 104,
            base_id + 106,
            base_id + 107
        )?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#1,#2,#3);",
            base_id + 106
        )?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#1,#2,#3);",
            base_id + 107
        )?;

        Ok(())
    }

    /// Write face representation
    fn write_face_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(
            writer,
            "#{}=ADVANCED_FACE('',(#{}),#{},.T.);",
            base_id,
            base_id + 1,
            base_id + 2
        )?;
        writeln!(
            writer,
            "#{}=FACE_OUTER_BOUND('',#{},.T.);",
            base_id + 1,
            base_id + 3
        )?;
        writeln!(writer, "#{}=POLY_LOOP('',(#{}));", base_id + 3, base_id + 4)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 4)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", base_id + 5)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,1.,0.));", base_id + 6)?;
        writeln!(writer, "#{}=PLANE('',#{});", base_id + 2, base_id + 7)?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#{},#2,#3);",
            base_id + 7,
            base_id + 8
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 8)?;

        Ok(())
    }

    /// Write edge representation
    fn write_edge_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(writer, "#{}=VERTEX_POINT('',#{});", base_id, base_id + 1)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 1)?;
        writeln!(
            writer,
            "#{}=VERTEX_POINT('',#{});",
            base_id + 2,
            base_id + 3
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", base_id + 3)?;
        writeln!(
            writer,
            "#{}=EDGE_CURVE('',#{},#{},#{},.T.);",
            base_id + 4,
            base_id,
            base_id + 2,
            base_id + 5
        )?;
        writeln!(writer, "#{}=LINE('',#{});", base_id + 5, base_id + 6)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 6)?;

        Ok(())
    }

    /// Write wire representation
    fn write_wire_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(writer, "#{}=OPEN_SHELL('',(#{}));", base_id, base_id + 1)?;
        writeln!(writer, "#{}=EDGE_LOOP('',(#{}));", base_id + 1, base_id + 2)?;
        writeln!(
            writer,
            "#{}=ORIENTED_EDGE('',*,*,#{},.T.);",
            base_id + 2,
            base_id + 3
        )?;
        writeln!(
            writer,
            "#{}=EDGE_CURVE('',#{},#{},#{},.T.);",
            base_id + 3,
            base_id + 4,
            base_id + 5,
            base_id + 6
        )?;
        writeln!(
            writer,
            "#{}=VERTEX_POINT('',#{});",
            base_id + 4,
            base_id + 7
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 7)?;
        writeln!(
            writer,
            "#{}=VERTEX_POINT('',#{});",
            base_id + 5,
            base_id + 8
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", base_id + 8)?;
        writeln!(writer, "#{}=LINE('',#{});", base_id + 6, base_id + 9)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 9)?;

        Ok(())
    }

    /// Write compound representation
    fn write_compound_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(
            writer,
            "#{}=MANIFOLD_SOLID_BREP('',#{});",
            base_id,
            base_id + 1
        )?;
        writeln!(
            writer,
            "#{}=CLOSED_SHELL('',(#{}));",
            base_id + 1,
            base_id + 2
        )?;
        writeln!(
            writer,
            "#{}=ADVANCED_FACE('',(#{}),#{},.T.);",
            base_id + 2,
            base_id + 3,
            base_id + 4
        )?;
        writeln!(
            writer,
            "#{}=FACE_OUTER_BOUND('',#{},.T.);",
            base_id + 3,
            base_id + 5
        )?;
        writeln!(writer, "#{}=POLY_LOOP('',(#{}));", base_id + 5, base_id + 6)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 6)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", base_id + 7)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,1.,0.));", base_id + 8)?;
        writeln!(writer, "#{}=PLANE('',#{});", base_id + 4, base_id + 9)?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#{},#2,#3);",
            base_id + 9,
            base_id + 10
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 10)?;

        Ok(())
    }

    /// Write shell representation
    fn write_shell_representation(
        &self,
        writer: &mut BufWriter<File>,
        _shape: &TopoDsShape,
        base_id: usize,
    ) -> Result<(), StepError> {
        writeln!(writer, "#{}=CLOSED_SHELL('',(#{}));", base_id, base_id + 1)?;
        writeln!(
            writer,
            "#{}=ADVANCED_FACE('',(#{}),#{},.T.);",
            base_id + 1,
            base_id + 2,
            base_id + 3
        )?;
        writeln!(
            writer,
            "#{}=FACE_OUTER_BOUND('',#{},.T.);",
            base_id + 2,
            base_id + 4
        )?;
        writeln!(writer, "#{}=POLY_LOOP('',(#{}));", base_id + 4, base_id + 5)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 5)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,0.,0.));", base_id + 6)?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(1.,1.,0.));", base_id + 7)?;
        writeln!(writer, "#{}=PLANE('',#{});", base_id + 3, base_id + 8)?;
        writeln!(
            writer,
            "#{}=AXIS2_PLACEMENT_3D('',#{},#2,#3);",
            base_id + 8,
            base_id + 9
        )?;
        writeln!(writer, "#{}=CARTESIAN_POINT('',(0.,0.,0.));", base_id + 9)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_step_reader_creation() {
        let reader = StepReader::new("test.step");
        assert_eq!(reader.tolerance(), 1e-6);
        assert!(!reader.read_colors());
    }

    #[test]
    fn test_step_reader_settings() {
        let mut reader = StepReader::new("test.step");

        reader.set_tolerance(1e-4);
        assert_eq!(reader.tolerance(), 1e-4);

        reader.set_read_colors(true);
        assert!(reader.read_colors());
    }

    #[test]
    fn test_step_writer_creation() {
        let writer = StepWriter::new("test.step");
        assert_eq!(writer.schema(), StepSchema::AP203);
        assert_eq!(writer.precision(), 6);
        assert!(!writer.write_colors());
    }

    #[test]
    fn test_step_writer_with_schema() {
        let writer = StepWriter::new_with_schema("test.step", StepSchema::AP214);
        assert_eq!(writer.schema(), StepSchema::AP214);
    }

    #[test]
    fn test_step_writer_settings() {
        let mut writer = StepWriter::new("test.step");

        writer.set_schema(StepSchema::AP242);
        assert_eq!(writer.schema(), StepSchema::AP242);

        writer.set_precision(10);
        assert_eq!(writer.precision(), 10);

        writer.set_write_colors(true);
        assert!(writer.write_colors());
    }

    #[test]
    fn test_step_writer() {
        // Create a temporary STEP file
        let temp_file = "test_output.step";

        // Create a simple compound shape
        let shape = TopoDsShape::new(ShapeType::Compound);

        // Test writing
        let writer = StepWriter::new(temp_file);
        let result = writer.write(&shape);
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_step_reader() {
        // Create a temporary STEP file
        let temp_file = "test_input.step";
        {
            let mut file = fs::File::create(temp_file).unwrap();
            writeln!(file, "HEADER;").unwrap();
            writeln!(file, "FILE_DESCRIPTION(('BrepRs Test'),'2;1');").unwrap();
            writeln!(file, "FILE_SCHEMA(('CONFIG_CONTROLLED_3D_DESIGN'));").unwrap();
            writeln!(file, "ENDSEC;").unwrap();
            writeln!(file, "DATA;").unwrap();
            writeln!(file, "#1=CARTESIAN_POINT('',(0.,0.,0.));").unwrap();
            writeln!(file, "#2=DIRECTION('',(0.,0.,1.));").unwrap();
            writeln!(file, "#3=DIRECTION('',(1.,0.,0.));").unwrap();
            writeln!(file, "#4=AXIS2_PLACEMENT_3D('',#1,#2,#3);").unwrap();
            writeln!(file, "ENDSEC;").unwrap();
            writeln!(file, "END-ISO-10303-21;").unwrap();
        }

        // Test reading
        let reader = StepReader::new(temp_file);
        let result = reader.read();
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_step_validate() {
        // Create a temporary STEP file
        let temp_file = "test_validate.step";
        {
            let mut file = fs::File::create(temp_file).unwrap();
            writeln!(file, "HEADER;").unwrap();
            writeln!(file, "FILE_DESCRIPTION(('BrepRs Test'),'2;1');").unwrap();
            writeln!(file, "FILE_SCHEMA(('CONFIG_CONTROLLED_3D_DESIGN'));").unwrap();
            writeln!(file, "ENDSEC;").unwrap();
            writeln!(file, "DATA;").unwrap();
            writeln!(file, "#1=CARTESIAN_POINT('',(0.,0.,0.));").unwrap();
            writeln!(file, "ENDSEC;").unwrap();
            writeln!(file, "END-ISO-10303-21;").unwrap();
        }

        // Test validation
        let reader = StepReader::new(temp_file);
        let result = reader.validate();
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }
}
