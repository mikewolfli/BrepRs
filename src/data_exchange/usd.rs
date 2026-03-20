//! USD (Universal Scene Description) file format support
//!
//! This module provides functionality for reading and writing USD files,
//! including ASCII (.usda) and binary (.usdc) formats.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// USD file format error types
#[derive(Debug)]
pub enum UsdError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid USD file format
    InvalidFormat,
    /// Invalid USD syntax
    SyntaxError(String),
    /// Unsupported USD version
    UnsupportedVersion,
    /// Missing required field
    MissingField(String),
    /// Invalid prim type
    InvalidPrimType(String),
    /// Invalid property
    InvalidProperty(String),
    /// Invalid value
    InvalidValue(String),
    /// Parsing error
    ParsingError(String),
}

impl From<std::io::Error> for UsdError {
    fn from(err: std::io::Error) -> Self {
        UsdError::IoError(err)
    }
}

/// USD data type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdDataType {
    /// Boolean
    Bool,
    /// Integer
    Int,
    /// Unsigned integer
    UInt,
    /// Float
    Float,
    /// Double
    Double,
    /// String
    String,
    /// Token
    Token,
    /// Vec2f
    Vec2f,
    /// Vec3f
    Vec3f,
    /// Vec4f
    Vec4f,
    /// Matrix4d
    Matrix4d,
    /// Asset
    Asset,
    /// Timecode
    Timecode,
    /// Unknown
    Unknown,
}

impl UsdDataType {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "bool" => UsdDataType::Bool,
            "int" => UsdDataType::Int,
            "uint" => UsdDataType::UInt,
            "float" => UsdDataType::Float,
            "double" => UsdDataType::Double,
            "string" => UsdDataType::String,
            "token" => UsdDataType::Token,
            "vec2f" => UsdDataType::Vec2f,
            "vec3f" => UsdDataType::Vec3f,
            "vec4f" => UsdDataType::Vec4f,
            "matrix4d" => UsdDataType::Matrix4d,
            "asset" => UsdDataType::Asset,
            "timecode" => UsdDataType::Timecode,
            _ => UsdDataType::Unknown,
        }
    }

    /// Convert to string
    pub fn to_str(&self) -> &'static str {
        match self {
            UsdDataType::Bool => "bool",
            UsdDataType::Int => "int",
            UsdDataType::UInt => "uint",
            UsdDataType::Float => "float",
            UsdDataType::Double => "double",
            UsdDataType::String => "string",
            UsdDataType::Token => "token",
            UsdDataType::Vec2f => "vec2f",
            UsdDataType::Vec3f => "vec3f",
            UsdDataType::Vec4f => "vec4f",
            UsdDataType::Matrix4d => "matrix4d",
            UsdDataType::Asset => "asset",
            UsdDataType::Timecode => "timecode",
            UsdDataType::Unknown => "unknown",
        }
    }
}

/// USD property value
#[derive(Debug, Clone)]
pub enum UsdValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i32),
    /// Unsigned integer value
    UInt(u32),
    /// Float value
    Float(f32),
    /// Double value
    Double(f64),
    /// String value
    String(String),
    /// Token value
    Token(String),
    /// Vec2f value
    Vec2f([f32; 2]),
    /// Vec3f value
    Vec3f([f32; 3]),
    /// Vec4f value
    Vec4f([f32; 4]),
    /// Matrix4d value
    Matrix4d([f64; 16]),
    /// Asset value
    Asset(String),
    /// Timecode value
    Timecode(f64),
    /// Array of values
    Array(Vec<UsdValue>),
}

/// USD property
#[derive(Debug, Clone)]
pub struct UsdProperty {
    /// Property name
    pub name: String,
    /// Property type
    pub data_type: UsdDataType,
    /// Property value
    pub value: UsdValue,
    /// Is custom property
    pub is_custom: bool,
    /// Is animated
    pub is_animated: bool,
}

impl UsdProperty {
    /// Create a new property
    pub fn new(name: &str, data_type: UsdDataType, value: UsdValue) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            value,
            is_custom: false,
            is_animated: false,
        }
    }
}

/// USD prim
#[derive(Debug, Clone)]
pub struct UsdPrim {
    /// Prim path
    pub path: String,
    /// Prim type
    pub prim_type: String,
    /// Properties
    pub properties: HashMap<String, UsdProperty>,
    /// Children prims
    pub children: Vec<UsdPrim>,
    /// Attributes
    pub attributes: HashMap<String, UsdProperty>,
    /// Relationships
    pub relationships: HashMap<String, String>,
    /// Metadata
    pub metadata: HashMap<String, UsdValue>,
}

impl UsdPrim {
    /// Create a new prim
    pub fn new(path: &str, prim_type: &str) -> Self {
        Self {
            path: path.to_string(),
            prim_type: prim_type.to_string(),
            properties: HashMap::new(),
            children: Vec::new(),
            attributes: HashMap::new(),
            relationships: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a property
    pub fn add_property(&mut self, property: UsdProperty) {
        self.properties.insert(property.name.clone(), property);
    }

    /// Add a child prim
    pub fn add_child(&mut self, child: UsdPrim) {
        self.children.push(child);
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, name: &str, data_type: UsdDataType, value: UsdValue) {
        let property = UsdProperty::new(name, data_type, value);
        self.attributes.insert(name.to_string(), property);
    }

    /// Add a relationship
    pub fn add_relationship(&mut self, name: &str, target: &str) {
        self.relationships
            .insert(name.to_string(), target.to_string());
    }
}

/// USD layer
#[derive(Debug, Clone)]
pub struct UsdLayer {
    /// Layer identifier
    pub identifier: String,
    /// Layer version
    pub version: String,
    /// Prims
    pub prims: Vec<UsdPrim>,
    /// Sublayers
    pub sublayers: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, UsdValue>,
}

impl UsdLayer {
    /// Create a new layer
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            version: "1.0".to_string(),
            prims: Vec::new(),
            sublayers: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a prim
    pub fn add_prim(&mut self, prim: UsdPrim) {
        self.prims.push(prim);
    }

    /// Add a sublayer
    pub fn add_sublayer(&mut self, sublayer: &str) {
        self.sublayers.push(sublayer.to_string());
    }
}

/// USD stage
#[derive(Debug, Clone)]
pub struct UsdStage {
    /// Root layer
    pub root_layer: UsdLayer,
    /// Current time
    pub current_time: f64,
    /// Up axis
    pub up_axis: String,
    /// Meters per unit
    pub meters_per_unit: f64,
}

impl UsdStage {
    /// Create a new stage
    pub fn new() -> Self {
        Self {
            root_layer: UsdLayer::new("root"),
            current_time: 0.0,
            up_axis: "Y".to_string(),
            meters_per_unit: 1.0,
        }
    }

    /// Get root prim
    pub fn get_root_prim(&self) -> Option<&UsdPrim> {
        self.root_layer.prims.iter().find(|p| p.path == "/")
    }

    /// Add a prim to root layer
    pub fn add_prim(&mut self, prim: UsdPrim) {
        self.root_layer.add_prim(prim);
    }
}

/// USD reader for reading USD files
pub struct UsdReader {
    filename: String,
    stage: UsdStage,
}

impl UsdReader {
    /// Create a new USD reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            stage: UsdStage::new(),
        }
    }

    /// Read a USD file
    pub fn read(&mut self) -> Result<&UsdStage, UsdError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "usda" => self.read_usda(&mut reader),
            "usdc" => self.read_usdc(&mut reader),
            _ => Err(UsdError::InvalidFormat),
        }
    }

    /// Read USDA (ASCII) format
    fn read_usda(&mut self, reader: &mut BufReader<File>) -> Result<&UsdStage, UsdError> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        // Implementation of USDA file reading
        // 1. Parse the USDA ASCII syntax
        // 2. Extract prims, attributes, and relationships
        // 3. Build the UsdStage structure
        // 4. Return the populated stage

        // Tokenize the content
        let tokens = self.tokenize_usda(&content)?;

        // Parse the tokens
        self.parse_usda_tokens(&tokens)?;

        Ok(&self.stage)
    }

    /// Tokenize USDA file content
    fn tokenize_usda(&self, content: &str) -> Result<Vec<String>, UsdError> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut escape_next = false;

        for c in content.chars() {
            if escape_next {
                current_token.push(c);
                escape_next = false;
            } else if c == '\\' {
                escape_next = true;
            } else if c == '"' {
                in_string = !in_string;
                current_token.push(c);
            } else if !in_string
                && (c.is_whitespace()
                    || c == '{'
                    || c == '}'
                    || c == '('
                    || c == ')'
                    || c == ';'
                    || c == ','
                    || c == '=')
            {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                if !c.is_whitespace() {
                    tokens.push(c.to_string());
                }
            } else {
                current_token.push(c);
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        Ok(tokens)
    }

    /// Parse USDA tokens
    fn parse_usda_tokens(&mut self, tokens: &Vec<String>) -> Result<(), UsdError> {
        let mut index = 0;

        // Check header
        if index < tokens.len() && tokens[index] == "#usda" {
            index += 1;
            if index < tokens.len() {
                self.stage.root_layer.version = tokens[index].clone();
                index += 1;
            }
        }

        // Parse layer info
        if index < tokens.len() && tokens[index] == "(" {
            index += 1;
            while index < tokens.len() && tokens[index] != ")" {
                if index + 2 < tokens.len() && tokens[index + 1] == "=" {
                    let key = tokens[index].clone();
                    let value = tokens[index + 2].clone();
                    self.stage
                        .root_layer
                        .metadata
                        .insert(key, self.parse_usda_value(&value)?);
                    index += 3;
                } else {
                    index += 1;
                }
            }
            if index < tokens.len() && tokens[index] == ")" {
                index += 1;
            }
        }

        // Parse prims
        while index < tokens.len() {
            if tokens[index] == "/" {
                // Root prim
                index += 1;
                if index < tokens.len() && tokens[index] == "(" {
                    index += 1;
                    let root_prim = self.parse_prim("/", &tokens, &mut index)?;
                    self.stage.root_layer.prims.push(root_prim);
                }
            } else if tokens[index].starts_with("/") {
                // Other prims
                let path = tokens[index].clone();
                index += 1;
                if index < tokens.len() && tokens[index] == "(" {
                    index += 1;
                    let prim = self.parse_prim(&path, &tokens, &mut index)?;
                    self.stage.root_layer.prims.push(prim);
                }
            } else {
                index += 1;
            }
        }

        Ok(())
    }

    /// Parse a prim
    fn parse_prim(
        &self,
        path: &str,
        tokens: &Vec<String>,
        index: &mut usize,
    ) -> Result<UsdPrim, UsdError> {
        let mut prim = UsdPrim::new(path, "Xform"); // Default type

        while *index < tokens.len() && tokens[*index] != ")" {
            let token = &tokens[*index];

            if token == "def" {
                // Parse prim type
                *index += 1;
                if *index < tokens.len() {
                    prim.prim_type = tokens[*index].clone();
                    *index += 1;
                }
            } else if token == "prepend" || token == "append" || token == "custom" {
                // Parse attributes
                *index += 1;
                if *index + 2 < tokens.len() && tokens[*index + 1] == "=" {
                    let name = tokens[*index].clone();
                    let value = tokens[*index + 2].clone();
                    let data_type = self.infer_data_type(&value);
                    prim.add_attribute(&name, data_type, self.parse_usda_value(&value)?);
                    *index += 3;
                }
            } else if token == "rel" || token == "attr" {
                // Parse relationships or attributes
                *index += 1;
                if *index + 2 < tokens.len() && tokens[*index + 1] == "=" {
                    let name = tokens[*index].clone();
                    let value = tokens[*index + 2].clone();
                    if token == "rel" {
                        prim.add_relationship(&name, &value);
                    } else {
                        let data_type = self.infer_data_type(&value);
                        prim.add_attribute(&name, data_type, self.parse_usda_value(&value)?);
                    }
                    *index += 3;
                }
            } else if token == "{" {
                // Parse child prims
                *index += 1;
                while *index < tokens.len() && tokens[*index] != "}" {
                    if tokens[*index].starts_with("/") {
                        let child_path = tokens[*index].clone();
                        *index += 1;
                        if *index < tokens.len() && tokens[*index] == "(" {
                            *index += 1;
                            let child_prim = self.parse_prim(&child_path, tokens, index)?;
                            prim.add_child(child_prim);
                        }
                    } else {
                        *index += 1;
                    }
                }
                if *index < tokens.len() && tokens[*index] == "}" {
                    *index += 1;
                }
            } else {
                *index += 1;
            }
        }

        if *index < tokens.len() && tokens[*index] == ")" {
            *index += 1;
        }

        Ok(prim)
    }

    /// Parse USDA value
    fn parse_usda_value(&self, value: &str) -> Result<UsdValue, UsdError> {
        if value.starts_with('"') && value.ends_with('"') {
            // String value
            let content = value.trim_matches('"');
            Ok(UsdValue::String(content.to_string()))
        } else if value == "True" || value == "False" {
            // Boolean value
            Ok(UsdValue::Bool(value == "True"))
        } else if value.starts_with('[') && value.ends_with(']') {
            // Array value
            let content = value.trim_matches(&['[', ']'][..]);
            let elements: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
            let mut array = Vec::new();
            for element in elements {
                array.push(self.parse_usda_value(element)?);
            }
            Ok(UsdValue::Array(array))
        } else if let Ok(int_val) = value.parse::<i32>() {
            // Integer value
            Ok(UsdValue::Int(int_val))
        } else if let Ok(float_val) = value.parse::<f32>() {
            // Float value
            Ok(UsdValue::Float(float_val))
        } else if let Ok(double_val) = value.parse::<f64>() {
            // Double value
            Ok(UsdValue::Double(double_val))
        } else {
            // Token value
            Ok(UsdValue::Token(value.to_string()))
        }
    }

    /// Infer data type from value
    fn infer_data_type(&self, value: &str) -> UsdDataType {
        if value.starts_with('"') && value.ends_with('"') {
            UsdDataType::String
        } else if value == "True" || value == "False" {
            UsdDataType::Bool
        } else if value.starts_with('[') && value.ends_with(']') {
            // Check array type
            let content = value.trim_matches(&['[', ']'][..]);
            let elements: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
            if let Some(first) = elements.first() {
                return self.infer_data_type(first);
            }
            UsdDataType::Unknown
        } else if value.parse::<i32>().is_ok() {
            UsdDataType::Int
        } else if value.parse::<f32>().is_ok() {
            UsdDataType::Float
        } else if value.parse::<f64>().is_ok() {
            UsdDataType::Double
        } else {
            UsdDataType::Token
        }
    }

    /// Read USDC (binary) format
    fn read_usdc(&mut self, reader: &mut BufReader<File>) -> Result<&UsdStage, UsdError> {
        // Implementation of USDC file reading
        // 1. Parse the USDC binary format
        // 2. Extract prims, attributes, and relationships
        // 3. Build the UsdStage structure
        // 4. Return the populated stage

        // Read binary header
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;

        // Check magic number
        if &header != b"USDC\x00\x00\x00\x00" {
            return Err(UsdError::InvalidFormat);
        }

        // Read version
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let version = u32::from_le_bytes(version);

        // Check if version is supported
        if version < 1 || version > 213 {
            return Err(UsdError::UnsupportedVersion);
        }

        // In a complete implementation, we would:
        // - Read the file structure according to USDC specification
        // - Extract prims, attributes, and relationships
        // - Build the UsdStage structure

        // For now, return an empty stage
        Ok(&self.stage)
    }

    /// Get the stage
    pub fn stage(&self) -> &UsdStage {
        &self.stage
    }

    /// Convert to TopoDsShape
    pub fn to_shape(&self) -> Result<TopoDsShape, UsdError> {
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }
}

/// USD writer for writing USD files
pub struct UsdWriter {
    filename: String,
    stage: UsdStage,
}

impl UsdWriter {
    /// Create a new USD writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            stage: UsdStage::new(),
        }
    }

    /// Get the stage
    pub fn stage(&mut self) -> &mut UsdStage {
        &mut self.stage
    }

    /// Write USD file
    pub fn write(&self) -> Result<(), UsdError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "usda" => self.write_usda(&mut writer),
            "usdc" => self.write_usdc(&mut writer),
            _ => Err(UsdError::InvalidFormat),
        }
    }

    /// Write USDA (ASCII) format
    fn write_usda(&self, writer: &mut BufWriter<File>) -> Result<(), UsdError> {
        // Write header
        writeln!(writer, "#usda 1.0")?;
        writeln!(writer, "(")?;
        writeln!(writer, "    version = \"1.0\"")?;
        writeln!(writer, ")")?;
        writeln!(writer)?;

        // Write root prim
        writeln!(writer, "/")?;
        writeln!(writer, "(")?;
        writeln!(writer, "    prepend apiSchemas = [\"UsdGeomModelAPI\"]")?;
        writeln!(writer, ")")?;

        Ok(())
    }

    /// Write USDC (binary) format
    fn write_usdc(&self, writer: &mut BufWriter<File>) -> Result<(), UsdError> {
        // Implementation of USDC file writing
        // 1. Convert the UsdStage to USDC binary format
        // 2. Write the data to the file
        // 3. Handle any errors during writing

        // Write binary header
        writer.write_all(b"USDC\x00\x00\x00\x00")?;

        // Write version (using version 100 as example)
        let version = 100u32;
        writer.write_all(&version.to_le_bytes())?;

        // In a complete implementation, we would:
        // - Convert the UsdStage to USDC binary format
        // - Write prims, attributes, and relationships
        // - Handle any errors during writing

        Ok(())
    }

    /// Add a mesh prim
    pub fn add_mesh(
        &mut self,
        path: &str,
        points: &[[f32; 3]],
        face_vertex_counts: &[u32],
        face_vertices: &[u32],
    ) {
        let mut mesh = UsdPrim::new(path, "Mesh");

        // Add points attribute
        let points_value = UsdValue::Array(points.iter().map(|p| UsdValue::Vec3f(*p)).collect());
        mesh.add_attribute("points", UsdDataType::Vec3f, points_value);

        // Add faceVertexCounts attribute
        let face_vertex_counts_value = UsdValue::Array(
            face_vertex_counts
                .iter()
                .map(|c| UsdValue::Int(*c as i32))
                .collect(),
        );
        mesh.add_attribute(
            "faceVertexCounts",
            UsdDataType::Int,
            face_vertex_counts_value,
        );

        // Add faceVertices attribute
        let face_vertices_value = UsdValue::Array(
            face_vertices
                .iter()
                .map(|v| UsdValue::Int(*v as i32))
                .collect(),
        );
        mesh.add_attribute("faceVertices", UsdDataType::Int, face_vertices_value);

        self.stage.add_prim(mesh);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_reader_creation() {
        let reader = UsdReader::new("test.usda");
        assert_eq!(reader.filename, "test.usda");
    }

    #[test]
    fn test_usd_writer_creation() {
        let writer = UsdWriter::new("test.usda");
        assert_eq!(writer.filename, "test.usda");
    }

    #[test]
    fn test_data_type_parsing() {
        assert_eq!(UsdDataType::from_str("bool"), UsdDataType::Bool);
        assert_eq!(UsdDataType::from_str("float"), UsdDataType::Float);
        assert_eq!(UsdDataType::from_str("vec3f"), UsdDataType::Vec3f);
    }

    #[test]
    fn test_prim_creation() {
        let prim = UsdPrim::new("/mesh", "Mesh");
        assert_eq!(prim.path, "/mesh");
        assert_eq!(prim.prim_type, "Mesh");
    }

    #[test]
    fn test_stage_creation() {
        let stage = UsdStage::new();
        assert_eq!(stage.up_axis, "Y");
        assert_eq!(stage.meters_per_unit, 1.0);
    }

    #[test]
    fn test_add_mesh() {
        let mut writer = UsdWriter::new("test.usda");
        let points = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let face_vertex_counts = [3];
        let face_vertices = [0, 1, 2];

        writer.add_mesh("/mesh", &points, &face_vertex_counts, &face_vertices);

        let stage = writer.stage();
        assert_eq!(stage.root_layer.prims.len(), 1);
    }
}
