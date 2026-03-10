//! Document management for application
//! 
//! This module provides document management functionality for BrepRs applications,
//! including document creation, saving, loading, and modification.

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::application::data_framework::{DataContainer, ShapeData};
// ShapeType is already imported via pub use in topology/mod.rs
use crate::topology::TopoDsShape;
// Remove duplicate import
use crate::topology::shape_enum;
use crate::application::data_framework::DataObject;

/// Document format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    /// BrepRs native format
    Native,
    /// STEP format
    Step,
    /// IGES format
    Iges,
    /// STL format
    Stl,
    /// GLTF format
    Gltf,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document name
    pub name: String,
    /// Document version
    pub version: String,
    /// Creation date
    pub creation_date: String,
    /// Last modified date
    pub last_modified: String,
    /// Author
    pub author: String,
    /// Description
    pub description: String,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            version: "1.0".to_string(),
            creation_date: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
            author: "".to_string(),
            description: "".to_string(),
        }
    }
}

/// Document structure
pub struct Document {
    /// Document metadata
    metadata: DocumentMetadata,
    /// Data container
    data: DataContainer,
    /// File path
    path: Option<String>,
    /// Modified flag
    modified: bool,
}

impl Document {
    /// Create a new document
    pub fn new(name: &str) -> Self {
        let mut metadata = DocumentMetadata::default();
        metadata.name = name.to_string();
        
        Self {
            metadata,
            data: DataContainer::new(),
            path: None,
            modified: false,
        }
    }

    /// Get document metadata
    pub fn metadata(&self) -> &DocumentMetadata {
        &self.metadata
    }

    /// Get mutable metadata
    pub fn metadata_mut(&mut self) -> &mut DocumentMetadata {
        self.modified = true;
        &mut self.metadata
    }

    /// Get data container
    pub fn data(&self) -> &DataContainer {
        &self.data
    }

    /// Get mutable data container
    pub fn data_mut(&mut self) -> &mut DataContainer {
        self.modified = true;
        &mut self.data
    }

    /// Add a shape to the document
    pub fn add_shape(&mut self, name: &str, shape: TopoDsShape) -> String {
        self.modified = true;
        let id = format!("shape_{}", uuid::Uuid::new_v4());
        let shape_data = ShapeData::new(id.clone(), name.to_string(), shape);
        self.data.add_object(Box::new(shape_data));
        id
    }

    /// Get a shape by ID
    pub fn get_shape(&self, id: &str) -> Option<TopoDsShape> {
        self.data.get_object(id).and_then(|obj| {
            let obj = obj.read().unwrap();
            if let Some(shape_data) = obj.as_any().downcast_ref::<ShapeData>() {
                Some(shape_data.shape().clone())
            } else {
                None
            }
        })
    }

    /// Save the document
    pub fn save(&mut self, path: &Path, format: DocumentFormat) -> Result<(), String> {
        match format {
            DocumentFormat::Native => self.save_native(path),
            DocumentFormat::Step => self.save_step(path),
            DocumentFormat::Iges => self.save_iges(path),
            DocumentFormat::Stl => self.save_stl(path),
            DocumentFormat::Gltf => self.save_gltf(path),
        }
    }

    /// Save as native format
    fn save_native(&mut self, path: &Path) -> Result<(), String> {
        let objects = self.data.get_all_objects()
            .iter()
            .filter_map(|obj| {
                let obj = obj.read().unwrap();
                obj.as_any().downcast_ref::<ShapeData>()
                    .map(|sd| serde_json::to_value(sd).unwrap_or(serde_json::Value::Null))
            })
            .collect();
        let document_data = DocumentData {
            metadata: self.metadata.clone(),
            objects,
        };
        
        let json = serde_json::to_string_pretty(&document_data).map_err(|e| e.to_string())?;
        
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        
        self.path = Some(path.to_str().unwrap().to_string());
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
        self.modified = false;
        
        Ok(())
    }

    /// Save as STEP format
    fn save_step(&mut self, path: &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        writeln!(file, "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('BrepRs STEP Export'), '1');\nENDSEC;\nDATA;")
            .map_err(|e| e.to_string())?;
        for obj in self.data.get_all_objects() {
            let obj = obj.read().map_err(|e| e.to_string())?;
            if let Some(shape_data) = obj.as_any().downcast_ref::<ShapeData>() {
                writeln!(file, "#{} = SHAPE('{}', '{}');", shape_data.id(), shape_data.name(), shape_data.shape().shape_type().name())
                    .map_err(|e| e.to_string())?;
            }
        }
        writeln!(file, "ENDSEC;\nEND-ISO-10303-21;").map_err(|e| e.to_string())?;
        self.path = Some(path.to_str().unwrap().to_string());
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
        self.modified = false;
        Ok(())
    }

    /// Save as IGES format
    fn save_iges(&mut self, path: &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        writeln!(file, "IGES Export by BrepRs").map_err(|e| e.to_string())?;
        for obj in self.data.get_all_objects() {
            let obj = obj.read().map_err(|e| e.to_string())?;
            if let Some(shape_data) = obj.as_any().downcast_ref::<ShapeData>() {
                writeln!(file, "{}:{}", shape_data.name(), shape_data.shape().shape_type().name()).map_err(|e| e.to_string())?;
            }
        }
        self.path = Some(path.to_str().unwrap().to_string());
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
        self.modified = false;
        Ok(())
    }

    /// Save as STL format
    fn save_stl(&mut self, path: &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        writeln!(file, "solid BrepRs").map_err(|e| e.to_string())?;
        for obj in self.data.get_all_objects() {
            let obj = obj.read().map_err(|e| e.to_string())?;
            if let Some(shape_data) = obj.as_any().downcast_ref::<ShapeData>() {
                writeln!(file, "// shape: {} type: {}", shape_data.name(), shape_data.shape().shape_type().name()).map_err(|e| e.to_string())?;
                // STL三角面片生成略，需补充具体几何数据导出
            }
        }
        writeln!(file, "endsolid BrepRs").map_err(|e| e.to_string())?;
        self.path = Some(path.to_str().unwrap().to_string());
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
        self.modified = false;
        Ok(())
    }

    /// Save as GLTF format
    fn save_gltf(&mut self, path: &Path) -> Result<(), String> {
        let mut shapes = Vec::new();
        for obj in self.data.get_all_objects() {
            let obj = obj.read().map_err(|e| e.to_string())?;
            if let Some(shape_data) = obj.as_any().downcast_ref::<ShapeData>() {
                shapes.push(serde_json::json!({
                    "id": shape_data.id(),
                    "name": shape_data.name(),
                    "type": shape_data.shape().shape_type().name(),
                }));
            }
        }
        let gltf = serde_json::json!({
            "asset": { "version": "2.0", "generator": "BrepRs" },
            "shapes": shapes
        });
        let json = serde_json::to_string_pretty(&gltf).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        self.path = Some(path.to_str().unwrap().to_string());
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
        self.modified = false;
        Ok(())
    }

    /// Load a document
    pub fn load(path: &Path, format: DocumentFormat) -> Result<Self, String> {
        match format {
            DocumentFormat::Native => Self::load_native(path),
            DocumentFormat::Step => Self::load_step(path),
            DocumentFormat::Iges => Self::load_iges(path),
            DocumentFormat::Stl => Self::load_stl(path),
            DocumentFormat::Gltf => Self::load_gltf(path),
        }
    }

    /// Load from native format
    fn load_native(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut json = String::new();
        file.read_to_string(&mut json).map_err(|e| e.to_string())?;
        let document_data: DocumentData = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        let mut data = DataContainer::new();
        for obj_val in document_data.objects {
            if let Ok(shape_data) = serde_json::from_value::<ShapeData>(obj_val) {
                data.add_object(Box::new(shape_data));
            }
        }
        let document = Self {
            metadata: document_data.metadata,
            data,
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        };
        Ok(document)
    }

    /// Load from STEP format
    fn load_step(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        let mut data = DataContainer::new();
        for line in content.lines() {
            if line.contains("SHAPE(") {
                let parts: Vec<&str> = line.split(['(', ',', ')']).collect();
                if parts.len() >= 4 {
                    let id = parts[1].trim_start_matches('#').to_string();
                    let name = parts[2].trim_matches('"').to_string();
                    let type_name = parts[3].trim_matches('"').to_string();
                    let shape_type = shape_enum::ShapeType::from_name(&type_name);
                    let shape = TopoDsShape::new(shape_type);
                    let shape_data = ShapeData::new(id, name, shape);
                    data.add_object(Box::new(shape_data));
                }
            }
        }
        Ok(Self {
            metadata: DocumentMetadata::default(),
            data,
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        })
    }

    /// Load from IGES format
    fn load_iges(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        let mut data = DataContainer::new();
        for line in content.lines() {
            if line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let type_name = parts[1].to_string();
                    let shape_type = shape_enum::ShapeType::from_name(&type_name);
                    let shape = TopoDsShape::new(shape_type);
                    let id = format!("{}-iges", name);
                    let shape_data = ShapeData::new(id, name, shape);
                    data.add_object(Box::new(shape_data));
                }
            }
        }
        Ok(Self {
            metadata: DocumentMetadata::default(),
            data,
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        })
    }

    /// Load from STL format
    fn load_stl(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        let mut data = DataContainer::new();
        for line in content.lines() {
            if line.contains("// shape:") {
                let parts: Vec<&str> = line.split([':', ' ']).collect();
                if parts.len() >= 6 {
                    let name = parts[2].to_string();
                    let type_name = parts[5].to_string();
                    let shape_type = shape_enum::ShapeType::from_name(&type_name);
                    let shape = TopoDsShape::new(shape_type);
                    let id = format!("{}-stl", name);
                    let shape_data = ShapeData::new(id, name, shape);
                    data.add_object(Box::new(shape_data));
                }
            }
        }
        Ok(Self {
            metadata: DocumentMetadata::default(),
            data,
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        })
    }

    /// Load from GLTF format
    fn load_gltf(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut json = String::new();
        file.read_to_string(&mut json).map_err(|e| e.to_string())?;
        let gltf: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        let mut data = DataContainer::new();
        if let Some(shapes) = gltf.get("shapes").and_then(|s| s.as_array()) {
            for shape_val in shapes {
                let id = shape_val.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let name = shape_val.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let type_name = shape_val.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let shape_type = shape_enum::ShapeType::from_name(&type_name);
                let shape = TopoDsShape::new(shape_type);
                let shape_data = ShapeData::new(id, name, shape);
                data.add_object(Box::new(shape_data));
            }
        }
        Ok(Self {
            metadata: DocumentMetadata::default(),
            data,
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        })
    }

    /// Check if document is modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get document path
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Clear the document
    pub fn clear(&mut self) {
        self.data.clear();
        self.metadata = DocumentMetadata::default();
        self.path = None;
        self.modified = false;
    }
}

/// Document data for serialization
#[derive(Debug, Serialize, Deserialize)]
struct DocumentData {
    metadata: DocumentMetadata,
    objects: Vec<serde_json::Value>,
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::TopoDsShape;
    use crate::topology::ShapeType;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Test Document");
        assert_eq!(doc.metadata().name, "Test Document");
        assert!(!doc.is_modified());
    }

    #[test]
    fn test_add_shape() {
        let mut doc = Document::new("Test Document");
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let id = doc.add_shape("Test Vertex", shape);
        assert!(!id.is_empty());
        assert!(doc.is_modified());
    }

    #[test]
    fn test_metadata_modification() {
        let mut doc = Document::new("Test Document");
        doc.metadata_mut().name = "Modified Document".to_string();
        assert_eq!(doc.metadata().name, "Modified Document");
        assert!(doc.is_modified());
    }

    #[test]
    fn test_clear_document() {
        let mut doc = Document::new("Test Document");
        let shape = TopoDsShape::new(ShapeType::Vertex);
        doc.add_shape("Test Vertex", shape);
        assert_eq!(doc.data().count(), 1);
        doc.clear();
        assert_eq!(doc.data().count(), 0);
        assert!(!doc.is_modified());
    }
}