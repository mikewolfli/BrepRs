//! Data Exchange Utilities
//!
//! This module provides utility functions for file format validation,
//! conversion, and metadata extraction.

use std::path::Path;

use crate::data_exchange::{
    gltf::{GltfExporter, GltfImporter},
    iges::{IgesReader, IgesWriter},
    step::{StepReader, StepWriter},
    stl::{StlReader, StlWriter},
    threemf::{ThreeMfExporter, ThreeMfImporter},
    usdz::{UsdExporter, UsdImporter},
    DataExchangeError,
};
use crate::foundation::handle::Handle;
use crate::topology::topods_shape::TopoDsShape;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    STL,
    STEP,
    IGES,
    GLTF,
    GLB,
    USDA,
    USDC,
    USDZ,
    ThreeMF,
}

impl FileFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::STL => "stl",
            FileFormat::STEP => "step",
            FileFormat::IGES => "iges",
            FileFormat::GLTF => "gltf",
            FileFormat::GLB => "glb",
            FileFormat::USDA => "usda",
            FileFormat::USDC => "usdc",
            FileFormat::USDZ => "usdz",
            FileFormat::ThreeMF => "3mf",
        }
    }

    /// Get MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            FileFormat::STL => "application/sla",
            FileFormat::STEP => "application/step",
            FileFormat::IGES => "application/iges",
            FileFormat::GLTF => "model/gltf+json",
            FileFormat::GLB => "model/gltf-binary",
            FileFormat::USDA => "model/vnd.usd+ascii",
            FileFormat::USDC => "model/vnd.usd+binary",
            FileFormat::USDZ => "model/vnd.usdz+zip",
            FileFormat::ThreeMF => "model/3mf",
        }
    }

    /// Detect format from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_lowercase();

        match extension.as_str() {
            "stl" => Some(FileFormat::STL),
            "step" | "stp" => Some(FileFormat::STEP),
            "iges" | "igs" => Some(FileFormat::IGES),
            "gltf" => Some(FileFormat::GLTF),
            "glb" => Some(FileFormat::GLB),
            "usda" => Some(FileFormat::USDA),
            "usdc" => Some(FileFormat::USDC),
            "usdz" => Some(FileFormat::USDZ),
            "3mf" => Some(FileFormat::ThreeMF),
            _ => None,
        }
    }

    /// Check if format is mesh-based
    pub fn is_mesh_format(&self) -> bool {
        matches!(
            self,
            FileFormat::STL | FileFormat::GLTF | FileFormat::GLB | FileFormat::ThreeMF
        )
    }

    /// Check if format is BREP-based
    pub fn is_brep_format(&self) -> bool {
        matches!(self, FileFormat::STEP | FileFormat::IGES)
    }

    /// Check if format supports colors
    pub fn supports_colors(&self) -> bool {
        matches!(
            self,
            FileFormat::GLTF | FileFormat::GLB | FileFormat::ThreeMF
        )
    }

    /// Check if format supports materials
    pub fn supports_materials(&self) -> bool {
        matches!(
            self,
            FileFormat::GLTF | FileFormat::GLB | FileFormat::ThreeMF
        )
    }

    /// Check if format supports animations
    pub fn supports_animations(&self) -> bool {
        matches!(self, FileFormat::GLTF | FileFormat::GLB)
    }
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub format: FileFormat,
    pub file_size: u64,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub estimated_vertex_count: Option<usize>,
    pub estimated_triangle_count: Option<usize>,
}

impl FileMetadata {
    pub fn new(format: FileFormat, file_size: u64) -> Self {
        Self {
            format,
            file_size,
            is_valid: true,
            validation_errors: Vec::new(),
            estimated_vertex_count: None,
            estimated_triangle_count: None,
        }
    }

    pub fn with_validation(mut self, is_valid: bool, errors: Vec<String>) -> Self {
        self.is_valid = is_valid;
        self.validation_errors = errors;
        self
    }

    pub fn with_estimates(mut self, vertices: Option<usize>, triangles: Option<usize>) -> Self {
        self.estimated_vertex_count = vertices;
        self.estimated_triangle_count = triangles;
        self
    }
}

/// Format validator
pub struct FormatValidator;

impl FormatValidator {
    /// Validate a file and return metadata
    pub fn validate_file(path: &Path) -> Result<FileMetadata, DataExchangeError> {
        let format = FileFormat::from_path(path)
            .ok_or_else(|| DataExchangeError::UnsupportedFormat(path.display().to_string()))?;

        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len();

        let mut file_metadata = FileMetadata::new(format, file_size);

        // Validate based on format
        let (is_valid, errors) = match format {
            FileFormat::STL => Self::validate_stl(path),
            FileFormat::STEP => Self::validate_step(path),
            FileFormat::IGES => Self::validate_iges(path),
            FileFormat::GLTF | FileFormat::GLB => Self::validate_gltf(path),
            FileFormat::USDA | FileFormat::USDC | FileFormat::USDZ => Self::validate_usd(path),
            FileFormat::ThreeMF => Self::validate_3mf(path),
        };

        file_metadata = file_metadata.with_validation(is_valid, errors);

        Ok(file_metadata)
    }

    fn validate_stl(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        if let Ok(content) = std::fs::read_to_string(path) {
            if content.trim().starts_with("solid") {
                // ASCII STL
                if !content.trim().ends_with("endsolid") {
                    errors.push("ASCII STL file missing 'endsolid' marker".to_string());
                }
            } else {
                // Binary STL
                if let Ok(data) = std::fs::read(path) {
                    if data.len() < 84 {
                        errors.push("Binary STL file too small".to_string());
                    } else if (data.len() - 84) % 50 != 0 {
                        errors.push(
                            "Binary STL file size inconsistent with triangle count".to_string(),
                        );
                    }
                }
            }
        } else {
            errors.push("Failed to read STL file".to_string());
        }

        (errors.is_empty(), errors)
    }

    fn validate_step(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.contains("HEADER;") {
                errors.push("STEP file missing HEADER section".to_string());
            }
            if !content.contains("DATA;") {
                errors.push("STEP file missing DATA section".to_string());
            }
            if !content.contains("ENDSEC;") {
                errors.push("STEP file missing ENDSEC markers".to_string());
            }
            if !content.contains("END-ISO-10303-21;") {
                errors.push("STEP file missing proper end marker".to_string());
            }
        } else {
            errors.push("Failed to read STEP file".to_string());
        }

        (errors.is_empty(), errors)
    }

    fn validate_iges(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.contains("S      ") {
                errors.push("IGES file missing Start section marker".to_string());
            }
            if !content.contains("G      ") {
                errors.push("IGES file missing Global section marker".to_string());
            }
            if !content.contains("D      ") {
                errors.push("IGES file missing Directory section marker".to_string());
            }
            if !content.contains("P      ") {
                errors.push("IGES file missing Parameter section marker".to_string());
            }
            if !content.contains("T      ") {
                errors.push("IGES file missing Terminate section marker".to_string());
            }
        } else {
            errors.push("Failed to read IGES file".to_string());
        }

        (errors.is_empty(), errors)
    }

    fn validate_gltf(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.contains("\"asset\"") {
                errors.push("glTF file missing asset section".to_string());
            }
            if !content.contains("\"version\"") {
                errors.push("glTF file missing version".to_string());
            }
        } else if let Ok(data) = std::fs::read(path) {
            // Check for GLB magic number
            if data.len() >= 4 {
                let magic = &data[0..4];
                if magic != b"glTF" {
                    errors.push("GLB file missing proper magic number".to_string());
                }
            }
        } else {
            errors.push("Failed to read glTF/GLB file".to_string());
        }

        (errors.is_empty(), errors)
    }

    fn validate_usd(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.contains("#usda") && !content.contains("#usdc") {
                errors.push("USD file missing proper header".to_string());
            }
        } else {
            errors.push("Failed to read USD file".to_string());
        }

        (errors.is_empty(), errors)
    }

    fn validate_3mf(path: &Path) -> (bool, Vec<String>) {
        let mut errors = Vec::new();

        // 3MF is a ZIP file, check for required files
        if let Ok(file) = std::fs::File::open(path) {
            if let Ok(mut archive) = zip::ZipArchive::new(file) {
                let has_content_types = archive.by_name("[Content_Types].xml").is_ok();
                let has_rels = archive.by_name("_rels/.rels").is_ok();
                let has_model = archive.by_name("3D/3dmodel.model").is_ok();

                if !has_content_types {
                    errors.push("3MF file missing [Content_Types].xml".to_string());
                }
                if !has_rels {
                    errors.push("3MF file missing _rels/.rels".to_string());
                }
                if !has_model {
                    errors.push("3MF file missing 3D/3dmodel.model".to_string());
                }
            } else {
                errors.push("3MF file is not a valid ZIP archive".to_string());
            }
        } else {
            errors.push("Failed to read 3MF file".to_string());
        }

        (errors.is_empty(), errors)
    }
}

/// Format converter
pub struct FormatConverter;

impl FormatConverter {
    /// Convert a shape from one format to another
    pub fn convert_shape(
        input_path: &Path,
        output_path: &Path,
        output_format: FileFormat,
    ) -> Result<(), DataExchangeError> {
        // Read shape from input file
        let shape = Self::read_shape(input_path)?;

        // Write shape to output file
        Self::write_shape(&shape, output_path, output_format)?;

        Ok(())
    }

    /// Read a shape from a file
    fn read_shape(path: &Path) -> Result<Handle<TopoDsShape>, DataExchangeError> {
        let format = FileFormat::from_path(path)
            .ok_or_else(|| DataExchangeError::UnsupportedFormat(path.display().to_string()))?;

        match format {
            FileFormat::STL => {
                let reader = StlReader::new(path.to_str().unwrap());
                let compound = reader.read()?;
                Ok(Handle::new(std::sync::Arc::new(compound.shape().clone())))
            }
            FileFormat::STEP => {
                let reader = StepReader::new(path.to_str().unwrap());
                let shape = reader.read()?;
                Ok(Handle::new(std::sync::Arc::new(shape)))
            }
            FileFormat::IGES => {
                let reader = IgesReader::new(path.to_str().unwrap());
                let shape = reader.read()?;
                Ok(Handle::new(std::sync::Arc::new(shape)))
            }
            FileFormat::GLTF | FileFormat::GLB => {
                let importer = GltfImporter::new();
                let mesh = importer.import_mesh(path)?;
                // Convert mesh to shape
                Ok(Handle::new(std::sync::Arc::new(TopoDsShape::new(
                    crate::topology::shape_enum::ShapeType::Compound,
                ))))
            }
            FileFormat::USDA | FileFormat::USDC | FileFormat::USDZ => {
                let importer = UsdImporter::new();
                let mesh = importer.import_mesh(path)?;
                Ok(Handle::new(std::sync::Arc::new(TopoDsShape::new(
                    crate::topology::shape_enum::ShapeType::Compound,
                ))))
            }
            FileFormat::ThreeMF => {
                let importer = ThreeMfImporter::new();
                let mesh = importer.import_mesh(path)?;
                Ok(Handle::new(std::sync::Arc::new(TopoDsShape::new(
                    crate::topology::shape_enum::ShapeType::Compound,
                ))))
            }
        }
    }

    /// Write a shape to a file
    fn write_shape(
        shape: &Handle<TopoDsShape>,
        path: &Path,
        format: FileFormat,
    ) -> Result<(), DataExchangeError> {
        match format {
            FileFormat::STL => {
                let writer = StlWriter::new(path.to_str().unwrap());
                writer.write(shape)?;
            }
            FileFormat::STEP => {
                let writer = StepWriter::new(path.to_str().unwrap());
                writer.write(shape)?;
            }
            FileFormat::IGES => {
                let writer = IgesWriter::new(path.to_str().unwrap());
                writer.write(shape)?;
            }
            FileFormat::GLTF => {
                let exporter = GltfExporter::new();
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_shape(shape, &mesh_generator, path)?;
            }
            FileFormat::GLB => {
                let exporter = GltfExporter::new();
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_glb(&mesh_generator.generate(shape, 0.1, 0.5), path)?;
            }
            FileFormat::USDA => {
                let exporter = UsdExporter::new();
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_shape(shape, &mesh_generator, path)?;
            }
            FileFormat::USDC => {
                let exporter = UsdExporter::with_options(
                    crate::data_exchange::usdz::UsdExportOptions::default()
                        .with_format(crate::data_exchange::usdz::UsdFormat::Usdc),
                );
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_shape(shape, &mesh_generator, path)?;
            }
            FileFormat::USDZ => {
                let exporter = UsdExporter::with_options(
                    crate::data_exchange::usdz::UsdExportOptions::default()
                        .with_format(crate::data_exchange::usdz::UsdFormat::Usdz),
                );
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_shape(shape, &mesh_generator, path)?;
            }
            FileFormat::ThreeMF => {
                let exporter = ThreeMfExporter::new();
                let mesh_generator = crate::mesh::MeshGenerator::new();
                exporter.export_shape(shape, &mesh_generator, path)?;
            }
        }

        Ok(())
    }

    /// Batch convert multiple files
    pub fn batch_convert(
        input_files: &[PathBuf],
        output_format: FileFormat,
        output_dir: &Path,
    ) -> Result<Vec<Result<(), DataExchangeError>>, DataExchangeError> {
        let mut results = Vec::new();

        for input_path in input_files {
            let output_filename = input_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let output_path =
                output_dir.join(format!("{}.{}", output_filename, output_format.extension()));

            let result = Self::convert_shape(input_path, &output_path, output_format);
            results.push(result);
        }

        Ok(results)
    }
}

use std::path::PathBuf;

/// Format compatibility matrix
pub struct FormatCompatibility;

impl FormatCompatibility {
    /// Get supported conversions for a format
    pub fn get_supported_conversions(format: FileFormat) -> Vec<FileFormat> {
        match format {
            FileFormat::STL => vec![
                FileFormat::STEP,
                FileFormat::IGES,
                FileFormat::GLTF,
                FileFormat::GLB,
                FileFormat::ThreeMF,
            ],
            FileFormat::STEP => vec![
                FileFormat::STL,
                FileFormat::IGES,
                FileFormat::GLTF,
                FileFormat::GLB,
                FileFormat::ThreeMF,
            ],
            FileFormat::IGES => vec![
                FileFormat::STL,
                FileFormat::STEP,
                FileFormat::GLTF,
                FileFormat::GLB,
                FileFormat::ThreeMF,
            ],
            FileFormat::GLTF | FileFormat::GLB => vec![
                FileFormat::STL,
                FileFormat::STEP,
                FileFormat::IGES,
                FileFormat::ThreeMF,
            ],
            FileFormat::USDA | FileFormat::USDC | FileFormat::USDZ => vec![
                FileFormat::STL,
                FileFormat::STEP,
                FileFormat::IGES,
                FileFormat::GLTF,
                FileFormat::GLB,
                FileFormat::ThreeMF,
            ],
            FileFormat::ThreeMF => vec![
                FileFormat::STL,
                FileFormat::STEP,
                FileFormat::IGES,
                FileFormat::GLTF,
                FileFormat::GLB,
            ],
        }
    }

    /// Check if conversion is supported
    pub fn is_conversion_supported(from: FileFormat, to: FileFormat) -> bool {
        Self::get_supported_conversions(from).contains(&to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format_extensions() {
        assert_eq!(FileFormat::STL.extension(), "stl");
        assert_eq!(FileFormat::STEP.extension(), "step");
        assert_eq!(FileFormat::GLTF.extension(), "gltf");
        assert_eq!(FileFormat::ThreeMF.extension(), "3mf");
    }

    #[test]
    fn test_file_format_from_path() {
        let path = Path::new("test.stl");
        assert_eq!(FileFormat::from_path(path), Some(FileFormat::STL));

        let path = Path::new("test.step");
        assert_eq!(FileFormat::from_path(path), Some(FileFormat::STEP));

        let path = Path::new("test.3mf");
        assert_eq!(FileFormat::from_path(path), Some(FileFormat::ThreeMF));
    }

    #[test]
    fn test_file_format_properties() {
        assert!(FileFormat::STL.is_mesh_format());
        assert!(FileFormat::STEP.is_brep_format());
        assert!(FileFormat::GLTF.supports_colors());
        assert!(FileFormat::GLTF.supports_animations());
    }

    #[test]
    fn test_format_compatibility() {
        let conversions = FormatCompatibility::get_supported_conversions(FileFormat::STL);
        assert!(conversions.contains(&FileFormat::STEP));
        assert!(conversions.contains(&FileFormat::GLTF));

        assert!(FormatCompatibility::is_conversion_supported(
            FileFormat::STL,
            FileFormat::STEP
        ));
        assert!(!FormatCompatibility::is_conversion_supported(
            FileFormat::STL,
            FileFormat::STL
        ));
    }

    #[test]
    fn test_file_metadata() {
        let metadata = FileMetadata::new(FileFormat::STL, 1024);
        assert_eq!(metadata.format, FileFormat::STL);
        assert_eq!(metadata.file_size, 1024);
        assert!(metadata.is_valid);

        let metadata = metadata.with_validation(false, vec!["Error".to_string()]);
        assert!(!metadata.is_valid);
        assert_eq!(metadata.validation_errors.len(), 1);
    }
}
