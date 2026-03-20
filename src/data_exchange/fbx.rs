//! FBX (Filmbox) file format support
//!
//! This module provides functionality for reading and writing FBX files,
//! including binary and ASCII formats.

use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// FBX file format error types
#[derive(Debug)]
pub enum FbxError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid FBX file format
    InvalidFormat,
    /// Unsupported FBX version
    UnsupportedVersion(u32),
    /// Missing required node
    MissingNode(String),
    /// Invalid node data
    InvalidNodeData(String),
    /// Parsing error
    ParsingError(String),
    /// Unsupported feature
    UnsupportedFeature(String),
}

impl From<std::io::Error> for FbxError {
    fn from(err: std::io::Error) -> Self {
        FbxError::IoError(err)
    }
}

/// FBX node property type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FbxPropertyType {
    /// Boolean
    Bool = 'C' as isize,
    /// Integer 16-bit
    Short = 'Y' as isize,
    /// Integer 32-bit
    Int = 'I' as isize,
    /// Float 32-bit
    Float = 'F' as isize,
    /// Double 64-bit
    Double = 'D' as isize,
    /// Integer 64-bit
    LongLong = 'L' as isize,
    /// String
    String = 'S' as isize,
    /// Binary data
    Binary = 'R' as isize,
    /// Array of booleans
    BoolArray = 'b' as isize,
    /// Array of integers 32-bit
    IntArray = 'i' as isize,
    /// Array of floats 32-bit
    FloatArray = 'f' as isize,
    /// Array of doubles 64-bit
    DoubleArray = 'd' as isize,
    /// Array of integers 64-bit
    LongLongArray = 'l' as isize,
    /// Array of strings
    StringArray = 's' as isize,
}

/// FBX property value
#[derive(Debug, Clone)]
pub enum FbxProperty {
    /// Boolean value
    Bool(bool),
    /// Short value
    Short(i16),
    /// Integer value
    Int(i32),
    /// Float value
    Float(f32),
    /// Double value
    Double(f64),
    /// Long long value
    LongLong(i64),
    /// String value
    String(String),
    /// Binary data
    Binary(Vec<u8>),
    /// Array of booleans
    BoolArray(Vec<bool>),
    /// Array of integers
    IntArray(Vec<i32>),
    /// Array of floats
    FloatArray(Vec<f32>),
    /// Array of doubles
    DoubleArray(Vec<f64>),
    /// Array of long longs
    LongLongArray(Vec<i64>),
    /// Array of strings
    StringArray(Vec<String>),
}

/// FBX node
#[derive(Debug, Clone)]
pub struct FbxNode {
    /// Node name
    pub name: String,
    /// Properties
    pub properties: Vec<FbxProperty>,
    /// Children nodes
    pub children: Vec<FbxNode>,
}

impl FbxNode {
    /// Create a new node
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Add a property
    pub fn add_property(&mut self, property: FbxProperty) {
        self.properties.push(property);
    }

    /// Add a child node
    pub fn add_child(&mut self, child: FbxNode) {
        self.children.push(child);
    }

    /// Find child by name
    pub fn find_child(&self, name: &str) -> Option<&FbxNode> {
        self.children.iter().find(|child| child.name == name)
    }

    /// Find child by name (mutable)
    pub fn find_child_mut(&mut self, name: &str) -> Option<&mut FbxNode> {
        self.children.iter_mut().find(|child| child.name == name)
    }
}

/// FBX header
#[derive(Debug, Clone)]
pub struct FbxHeader {
    /// Magic number
    pub magic: [u8; 21],
    /// Version
    pub version: u32,
}

/// FBX document
#[derive(Debug, Clone)]
pub struct FbxDocument {
    /// Header
    pub header: FbxHeader,
    /// Root node
    pub root: FbxNode,
}

impl FbxDocument {
    /// Create a new document
    pub fn new() -> Self {
        Self {
            header: FbxHeader {
                magic: *b"Kaydara FBX Binary   ",
                version: 7500,
            },
            root: FbxNode::new("FBX"),
        }
    }
}

/// FBX reader for reading FBX files
pub struct FbxReader {
    filename: String,
    document: FbxDocument,
}

impl FbxReader {
    /// Create a new FBX reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: FbxDocument::new(),
        }
    }

    /// Read an FBX file
    pub fn read(&mut self) -> Result<&FbxDocument, FbxError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Check if it's binary or ASCII format
        let mut magic = [0u8; 11];
        reader.read_exact(&mut magic)?;

        if &magic == b"Kaydara FBX" {
            // Binary format
            self.read_binary(&mut reader)
        } else {
            // ASCII format
            self.read_ascii(&mut reader)
        }
    }

    /// Read binary FBX format
    fn read_binary(&mut self, reader: &mut BufReader<File>) -> Result<&FbxDocument, FbxError> {
        // 1. Parse the binary FBX format header
        let mut magic = [0u8; 21];
        reader.read_exact(&mut magic)?;

        // Check magic number
        if &magic != b"Kaydara FBX Binary   " {
            return Err(FbxError::InvalidFormat);
        }

        // Read version
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let version = u32::from_le_bytes(version);

        // Check if version is supported
        if version < 7100 || version > 7700 {
            return Err(FbxError::UnsupportedVersion(version));
        }

        // Update header
        self.document.header.magic = magic;
        self.document.header.version = version;

        // 2. Read the node hierarchy
        self.read_binary_node(reader, &mut self.document.root)?;

        Ok(&self.document)
    }

    /// Read binary node
    fn read_binary_node(
        &self,
        reader: &mut BufReader<File>,
        parent: &mut FbxNode,
    ) -> Result<(), FbxError> {
        // Read node header
        let mut node_header = [0u8; 13];
        reader.read_exact(&mut node_header)?;

        let end_offset = u64::from_le_bytes(node_header[0..8].try_into().unwrap());
        let num_properties = u32::from_le_bytes(node_header[8..12].try_into().unwrap());
        let property_list_len = u8::from_le_bytes(node_header[12..13].try_into().unwrap());

        // Read node name
        let mut name_buffer = vec![0u8; property_list_len as usize];
        reader.read_exact(&mut name_buffer)?;
        let name = String::from_utf8_lossy(&name_buffer).to_string();

        let mut node = FbxNode::new(&name);

        // Read properties
        for _ in 0..num_properties {
            let property = self.read_binary_property(reader)?;
            node.add_property(property);
        }

        // Read children
        let current_pos = reader.stream_position()?;
        if current_pos < end_offset {
            while reader.stream_position()? < end_offset {
                self.read_binary_node(reader, &mut node)?;
            }
        }

        parent.add_child(node);
        Ok(())
    }

    /// Read binary property
    fn read_binary_property(&self, reader: &mut BufReader<File>) -> Result<FbxProperty, FbxError> {
        let mut type_code = [0u8; 1];
        reader.read_exact(&mut type_code)?;

        match type_code[0] as char {
            'C' => {
                // Boolean
                let mut value = [0u8; 1];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Bool(value[0] != 0))
            }
            'Y' => {
                // Short
                let mut value = [0u8; 2];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Short(i16::from_le_bytes(value)))
            }
            'I' => {
                // Int
                let mut value = [0u8; 4];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Int(i32::from_le_bytes(value)))
            }
            'F' => {
                // Float
                let mut value = [0u8; 4];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Float(f32::from_le_bytes(value)))
            }
            'D' => {
                // Double
                let mut value = [0u8; 8];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Double(f64::from_le_bytes(value)))
            }
            'L' => {
                // Long long
                let mut value = [0u8; 8];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::LongLong(i64::from_le_bytes(value)))
            }
            'S' => {
                // String
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                let mut value = vec![0u8; length as usize];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::String(
                    String::from_utf8_lossy(&value).to_string(),
                ))
            }
            'R' => {
                // Binary
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                let mut value = vec![0u8; length as usize];
                reader.read_exact(&mut value)?;
                Ok(FbxProperty::Binary(value))
            }
            'b' => {
                // Bool array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut value = [0u8; 1];
                    reader.read_exact(&mut value)?;
                    values.push(value[0] != 0);
                }
                Ok(FbxProperty::BoolArray(values))
            }
            'i' => {
                // Int array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut value = [0u8; 4];
                    reader.read_exact(&mut value)?;
                    values.push(i32::from_le_bytes(value));
                }
                Ok(FbxProperty::IntArray(values))
            }
            'f' => {
                // Float array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut value = [0u8; 4];
                    reader.read_exact(&mut value)?;
                    values.push(f32::from_le_bytes(value));
                }
                Ok(FbxProperty::FloatArray(values))
            }
            'd' => {
                // Double array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut value = [0u8; 8];
                    reader.read_exact(&mut value)?;
                    values.push(f64::from_le_bytes(value));
                }
                Ok(FbxProperty::DoubleArray(values))
            }
            'l' => {
                // Long long array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut value = [0u8; 8];
                    reader.read_exact(&mut value)?;
                    values.push(i64::from_le_bytes(value));
                }
                Ok(FbxProperty::LongLongArray(values))
            }
            's' => {
                // String array
                let mut length = [0u8; 4];
                reader.read_exact(&mut length)?;
                let length = u32::from_le_bytes(length);

                // Skip encoding
                let mut encoding = [0u8; 4];
                reader.read_exact(&mut encoding)?;

                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut str_length = [0u8; 4];
                    reader.read_exact(&mut str_length)?;
                    let str_length = u32::from_le_bytes(str_length);

                    let mut value = vec![0u8; str_length as usize];
                    reader.read_exact(&mut value)?;
                    values.push(String::from_utf8_lossy(&value).to_string());
                }
                Ok(FbxProperty::StringArray(values))
            }
            _ => Err(FbxError::InvalidNodeData(format!(
                "Unknown property type: {}",
                type_code[0] as char
            ))),
        }
    }

    /// Read ASCII FBX format
    fn read_ascii(&mut self, reader: &mut BufReader<File>) -> Result<&FbxDocument, FbxError> {
        // 1. Parse the ASCII FBX format header
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let lines: Vec<&str> = content.lines().collect();
        let mut index = 0;

        // Skip comments and empty lines
        while index < lines.len()
            && (lines[index].trim().is_empty() || lines[index].trim().starts_with(';'))
        {
            index += 1;
        }

        // Check header
        if index >= lines.len() || !lines[index].contains("Kaydara FBX ASCII") {
            return Err(FbxError::InvalidFormat);
        }

        // Read version
        index += 1;
        if index >= lines.len() {
            return Err(FbxError::InvalidFormat);
        }

        let version_line = lines[index].trim();
        if !version_line.starts_with("Version:") {
            return Err(FbxError::InvalidFormat);
        }

        let version_str = version_line.split(":").nth(1).unwrap_or("").trim();
        let version = version_str
            .parse::<u32>()
            .map_err(|_| FbxError::InvalidFormat)?;

        // Check if version is supported
        if version < 7100 || version > 7700 {
            return Err(FbxError::UnsupportedVersion(version));
        }

        // Update header
        self.document.header.version = version;

        // 2. Read the node hierarchy
        index += 1;
        self.read_ascii_nodes(&lines, &mut index, &mut self.document.root)?;

        Ok(&self.document)
    }

    /// Read ASCII nodes
    fn read_ascii_nodes(
        &self,
        lines: &Vec<&str>,
        index: &mut usize,
        parent: &mut FbxNode,
    ) -> Result<(), FbxError> {
        while *index < lines.len() {
            let line = lines[*index].trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with(';') {
                *index += 1;
                continue;
            }

            // Check for node start
            if line.ends_with('{') {
                let node_name = line.trim_end_matches('{').trim();
                let mut node = FbxNode::new(node_name);

                // Read properties
                *index += 1;
                while *index < lines.len() {
                    let prop_line = lines[*index].trim();

                    if prop_line.is_empty() || prop_line.starts_with(';') {
                        *index += 1;
                        continue;
                    }

                    if prop_line == "}" {
                        // End of node
                        *index += 1;
                        parent.add_child(node);
                        return Ok(());
                    } else if prop_line.ends_with('{') {
                        // Child node
                        self.read_ascii_nodes(lines, index, &mut node)?;
                    } else {
                        // Property
                        let property = self.parse_ascii_property(prop_line)?;
                        node.add_property(property);
                        *index += 1;
                    }
                }
            } else if line == "}" {
                // End of parent node
                *index += 1;
                return Ok(());
            } else {
                // Property
                let property = self.parse_ascii_property(line)?;
                parent.add_property(property);
                *index += 1;
            }
        }

        Ok(())
    }

    /// Parse ASCII property
    fn parse_ascii_property(&self, line: &str) -> Result<FbxProperty, FbxError> {
        let trimmed = line.trim();

        // Check for different property types
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            // String
            let value = trimmed.trim_matches('"');
            Ok(FbxProperty::String(value.to_string()))
        } else if trimmed == "Y" {
            // Boolean true
            Ok(FbxProperty::Bool(true))
        } else if trimmed == "N" {
            // Boolean false
            Ok(FbxProperty::Bool(false))
        } else if trimmed.starts_with('b')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // Bool array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<bool> = content.split(',').map(|s| s.trim() == "Y").collect();
            Ok(FbxProperty::BoolArray(values))
        } else if trimmed.starts_with('i')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // Int array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<i32> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();
            Ok(FbxProperty::IntArray(values))
        } else if trimmed.starts_with('f')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // Float array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<f32> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();
            Ok(FbxProperty::FloatArray(values))
        } else if trimmed.starts_with('d')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // Double array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<f64> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();
            Ok(FbxProperty::DoubleArray(values))
        } else if trimmed.starts_with('l')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // Long long array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<i64> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();
            Ok(FbxProperty::LongLongArray(values))
        } else if trimmed.starts_with('s')
            && trimmed[1..].starts_with('{')
            && trimmed.ends_with('}')
        {
            // String array
            let content = trimmed[2..trimmed.len() - 1].trim();
            let values: Vec<String> = content
                .split('"')
                .filter(|s| !s.trim().is_empty() && s != ",")
                .map(|s| s.to_string())
                .collect();
            Ok(FbxProperty::StringArray(values))
        } else if let Ok(int_val) = trimmed.parse::<i32>() {
            // Int
            Ok(FbxProperty::Int(int_val))
        } else if let Ok(float_val) = trimmed.parse::<f32>() {
            // Float
            Ok(FbxProperty::Float(float_val))
        } else if let Ok(double_val) = trimmed.parse::<f64>() {
            // Double
            Ok(FbxProperty::Double(double_val))
        } else if let Ok(long_val) = trimmed.parse::<i64>() {
            // Long long
            Ok(FbxProperty::LongLong(long_val))
        } else {
            // Default to string
            Ok(FbxProperty::String(trimmed.to_string()))
        }
    }

    /// Get the document
    pub fn document(&self) -> &FbxDocument {
        &self.document
    }

    /// Convert to TopoDsShape
    pub fn to_shape(&self) -> Result<TopoDsShape, FbxError> {
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }
}

/// FBX writer for writing FBX files
pub struct FbxWriter {
    filename: String,
    document: FbxDocument,
}

impl FbxWriter {
    /// Create a new FBX writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: FbxDocument::new(),
        }
    }

    /// Get the document
    pub fn document(&mut self) -> &mut FbxDocument {
        &mut self.document
    }

    /// Write FBX file
    pub fn write(&self) -> Result<(), FbxError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write binary format
        self.write_binary(&mut writer)
    }

    /// Write binary FBX format
    fn write_binary(&self, writer: &mut BufWriter<File>) -> Result<(), FbxError> {
        // 1. Write the binary FBX format header
        writer.write_all(&self.document.header.magic)?;
        writer.write_all(&self.document.header.version.to_le_bytes())?;

        // 2. Write the node hierarchy
        self.write_binary_node(writer, &self.document.root)?;

        Ok(())
    }

    /// Write binary node
    fn write_binary_node(
        &self,
        writer: &mut BufWriter<File>,
        node: &FbxNode,
    ) -> Result<(), FbxError> {
        // Calculate node size
        let start_pos = writer.stream_position()?;

        // Write placeholder for node header
        let mut node_header = [0u8; 13];
        writer.write_all(&node_header)?;

        // Write node name
        let name_bytes = node.name.as_bytes();
        writer.write_all(name_bytes)?;

        // Write properties
        for property in &node.properties {
            self.write_binary_property(writer, property)?;
        }

        // Write children
        for child in &node.children {
            self.write_binary_node(writer, child)?;
        }

        // Calculate end offset
        let end_pos = writer.stream_position()?;

        // Update node header
        let end_offset = end_pos.to_le_bytes();
        let num_properties = (node.properties.len() as u32).to_le_bytes();
        let property_list_len = (name_bytes.len() as u8).to_le_bytes();

        node_header[0..8].copy_from_slice(&end_offset);
        node_header[8..12].copy_from_slice(&num_properties);
        node_header[12..13].copy_from_slice(&property_list_len);

        // Seek back to write node header
        writer.seek(std::io::SeekFrom::Start(start_pos))?;
        writer.write_all(&node_header)?;

        // Seek back to end
        writer.seek(std::io::SeekFrom::Start(end_pos))?;

        Ok(())
    }

    /// Write binary property
    fn write_binary_property(
        &self,
        writer: &mut BufWriter<File>,
        property: &FbxProperty,
    ) -> Result<(), FbxError> {
        match property {
            FbxProperty::Bool(value) => {
                writer.write_all(&['C' as u8])?;
                writer.write_all(&[*value as u8])?;
            }
            FbxProperty::Short(value) => {
                writer.write_all(&['Y' as u8])?;
                writer.write_all(&value.to_le_bytes())?;
            }
            FbxProperty::Int(value) => {
                writer.write_all(&['I' as u8])?;
                writer.write_all(&value.to_le_bytes())?;
            }
            FbxProperty::Float(value) => {
                writer.write_all(&['F' as u8])?;
                writer.write_all(&value.to_le_bytes())?;
            }
            FbxProperty::Double(value) => {
                writer.write_all(&['D' as u8])?;
                writer.write_all(&value.to_le_bytes())?;
            }
            FbxProperty::LongLong(value) => {
                writer.write_all(&['L' as u8])?;
                writer.write_all(&value.to_le_bytes())?;
            }
            FbxProperty::String(value) => {
                writer.write_all(&['S' as u8])?;
                let length = (value.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                writer.write_all(value.as_bytes())?;
            }
            FbxProperty::Binary(value) => {
                writer.write_all(&['R' as u8])?;
                let length = (value.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                writer.write_all(value)?;
            }
            FbxProperty::BoolArray(values) => {
                writer.write_all(&['b' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    writer.write_all(&[*value as u8])?;
                }
            }
            FbxProperty::IntArray(values) => {
                writer.write_all(&['i' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    writer.write_all(&value.to_le_bytes())?;
                }
            }
            FbxProperty::FloatArray(values) => {
                writer.write_all(&['f' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    writer.write_all(&value.to_le_bytes())?;
                }
            }
            FbxProperty::DoubleArray(values) => {
                writer.write_all(&['d' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    writer.write_all(&value.to_le_bytes())?;
                }
            }
            FbxProperty::LongLongArray(values) => {
                writer.write_all(&['l' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    writer.write_all(&value.to_le_bytes())?;
                }
            }
            FbxProperty::StringArray(values) => {
                writer.write_all(&['s' as u8])?;
                let length = (values.len() as u32).to_le_bytes();
                writer.write_all(&length)?;
                // Write encoding (0 for default)
                writer.write_all(&[0u8; 4])?;
                for value in values {
                    let str_length = (value.len() as u32).to_le_bytes();
                    writer.write_all(&str_length)?;
                    writer.write_all(value.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    /// Write ASCII FBX format
    #[allow(dead_code)]
    fn write_ascii(&self, writer: &mut BufWriter<File>) -> Result<(), FbxError> {
        // 1. Write the ASCII FBX format header
        writeln!(writer, "Kaydara FBX ASCII {}", self.document.header.version)?;
        writeln!(writer, "{}", "\x00")?; // Null terminator
        writeln!(writer, "Version: {}", self.document.header.version)?;
        writeln!(writer)?;

        // 2. Write the node hierarchy
        self.write_ascii_node(writer, &self.document.root, 0)?;

        Ok(())
    }

    /// Write ASCII node
    fn write_ascii_node(
        &self,
        writer: &mut BufWriter<File>,
        node: &FbxNode,
        indent: usize,
    ) -> Result<(), FbxError> {
        let indent_str = "    ".repeat(indent);

        // Write node start
        writeln!(writer, "{}{} {{", indent_str, node.name)?;

        // Write properties
        for property in &node.properties {
            let prop_str = self.format_ascii_property(property)?;
            writeln!(writer, "{}{}", indent_str, prop_str)?;
        }

        // Write children
        for child in &node.children {
            self.write_ascii_node(writer, child, indent + 1)?;
        }

        // Write node end
        writeln!(writer, "{}}}", indent_str)?;

        Ok(())
    }

    /// Format ASCII property
    fn format_ascii_property(&self, property: &FbxProperty) -> Result<String, FbxError> {
        match property {
            FbxProperty::Bool(value) => Ok(if *value {
                "Y".to_string()
            } else {
                "N".to_string()
            }),
            FbxProperty::Short(value) => Ok(value.to_string()),
            FbxProperty::Int(value) => Ok(value.to_string()),
            FbxProperty::Float(value) => Ok(value.to_string()),
            FbxProperty::Double(value) => Ok(value.to_string()),
            FbxProperty::LongLong(value) => Ok(value.to_string()),
            FbxProperty::String(value) => Ok(format!("\"{}\"", value)),
            FbxProperty::Binary(_value) => {
                // Binary data is not supported in ASCII format
                Ok("\"".to_string())
            }
            FbxProperty::BoolArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| if *v { "Y" } else { "N" })
                    .collect::<Vec<&str>>()
                    .join(",");
                Ok(format!("b{{{}}}", values_str))
            }
            FbxProperty::IntArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                Ok(format!("i{{{}}}", values_str))
            }
            FbxProperty::FloatArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                Ok(format!("f{{{}}}", values_str))
            }
            FbxProperty::DoubleArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                Ok(format!("d{{{}}}", values_str))
            }
            FbxProperty::LongLongArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                Ok(format!("l{{{}}}", values_str))
            }
            FbxProperty::StringArray(values) => {
                let values_str = values
                    .iter()
                    .map(|v| format!("\"{}\"", v))
                    .collect::<Vec<String>>()
                    .join(",");
                Ok(format!("s{{{}}}", values_str))
            }
        }
    }

    /// Add a mesh node
    pub fn add_mesh(&mut self, name: &str, vertices: &[[f32; 3]], indices: &[u32]) {
        let mut mesh_node = FbxNode::new("Geometry");
        mesh_node.add_property(FbxProperty::String(name.to_string()));

        // Add vertices
        let mut vertices_node = FbxNode::new("Vertices");
        let mut vertex_data = Vec::new();
        for v in vertices {
            vertex_data.extend_from_slice(v);
        }
        vertices_node.add_property(FbxProperty::FloatArray(vertex_data));
        mesh_node.add_child(vertices_node);

        // Add indices
        let mut indices_node = FbxNode::new("PolygonVertexIndex");
        let indices_data: Vec<i32> = indices.iter().map(|&i| i as i32).collect();
        indices_node.add_property(FbxProperty::IntArray(indices_data));
        mesh_node.add_child(indices_node);

        self.document.root.add_child(mesh_node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fbx_reader_creation() {
        let reader = FbxReader::new("test.fbx");
        assert_eq!(reader.filename, "test.fbx");
    }

    #[test]
    fn test_fbx_writer_creation() {
        let writer = FbxWriter::new("test.fbx");
        assert_eq!(writer.filename, "test.fbx");
    }

    #[test]
    fn test_node_creation() {
        let node = FbxNode::new("Geometry");
        assert_eq!(node.name, "Geometry");
    }

    #[test]
    fn test_add_property() {
        let mut node = FbxNode::new("Test");
        node.add_property(FbxProperty::String("value".to_string()));
        assert_eq!(node.properties.len(), 1);
    }

    #[test]
    fn test_find_child() {
        let mut parent = FbxNode::new("Parent");
        let child = FbxNode::new("Child");
        parent.add_child(child);

        assert!(parent.find_child("Child").is_some());
        assert!(parent.find_child("NonExistent").is_none());
    }

    #[test]
    fn test_add_mesh() {
        let mut writer = FbxWriter::new("test.fbx");
        let vertices = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let indices = [0, 1, 2];

        writer.add_mesh("Mesh", &vertices, &indices);

        let geometry_node = writer.document.root.find_child("Geometry");
        assert!(geometry_node.is_some());
    }
}
