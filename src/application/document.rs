//! Document management for application
//! 
//! This module provides document management functionality for BrepRs applications,
//! including document creation, saving, loading, and modification.

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::application::data_framework::{DataContainer, ShapeData};
use crate::topology::TopoDsShape;

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
            let obj_guard = obj.read().unwrap();
            let obj_ref = obj_guard.as_ref();
            if let Some(shape_data) = obj_ref.as_any().downcast_ref::<ShapeData>() {
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
        let document_data = DocumentData {
            metadata: self.metadata.clone(),
            objects: Vec::new(), // TODO: Implement proper serialization
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
    fn save_step(&mut self, _path: &Path) -> Result<(), String> {
        // TODO: Implement STEP export
        Err("STEP export not implemented".to_string())
    }

    /// Save as IGES format
    fn save_iges(&mut self, _path: &Path) -> Result<(), String> {
        // TODO: Implement IGES export
        Err("IGES export not implemented".to_string())
    }

    /// Save as STL format
    fn save_stl(&mut self, _path: &Path) -> Result<(), String> {
        // TODO: Implement STL export
        Err("STL export not implemented".to_string())
    }

    /// Save as GLTF format
    fn save_gltf(&mut self, _path: &Path) -> Result<(), String> {
        // TODO: Implement GLTF export
        Err("GLTF export not implemented".to_string())
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
        
        let document = Self {
            metadata: document_data.metadata,
            data: DataContainer::new(),
            path: Some(path.to_str().unwrap().to_string()),
            modified: false,
        };
        
        // TODO: Restore objects from document_data.objects
        
        Ok(document)
    }

    /// Load from STEP format
    fn load_step(_path: &Path) -> Result<Self, String> {
        // TODO: Implement STEP import
        Err("STEP import not implemented".to_string())
    }

    /// Load from IGES format
    fn load_iges(_path: &Path) -> Result<Self, String> {
        // TODO: Implement IGES import
        Err("IGES import not implemented".to_string())
    }

    /// Load from STL format
    fn load_stl(_path: &Path) -> Result<Self, String> {
        // TODO: Implement STL import
        Err("STL import not implemented".to_string())
    }

    /// Load from GLTF format
    fn load_gltf(_path: &Path) -> Result<Self, String> {
        // TODO: Implement GLTF import
        Err("GLTF import not implemented".to_string())
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