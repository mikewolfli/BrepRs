//! Text to 3D Module
//!
//! This module provides functionality for generating 3D models from various inputs,
//! including text descriptions, sketches, and images, with text processing, model generation,
//! and result optimization.

use std::collections::HashMap;

use crate::ai_ml::protocol::{AiProtocolError, AiResult};
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

/// Text to 3D Generation Settings
#[derive(Debug, Default, Clone)]
pub struct TextTo3DSettings {
    pub model_complexity: f64, // 0.0 to 1.0
    pub detail_level: f64,     // 0.0 to 1.0
    pub style: String,         // e.g., "realistic", "cartoon", "low_poly"
    pub size: f64,             // default size in units
    pub seed: Option<u64>,     // random seed for reproducibility
}

/// Text to 3D Generation Result
pub struct TextTo3DResult {
    pub mesh: Mesh3D,
    pub description: String,
    pub settings: TextTo3DSettings,
    pub generation_time: f64, // in seconds
    pub quality_score: f64,   // 0.0 to 1.0
}

/// Text to 3D Generator
pub struct TextTo3DGenerator {
    settings: TextTo3DSettings,
    // In a real implementation, this would include AI models and other dependencies
}

impl TextTo3DGenerator {
    pub fn new() -> Self {
        Self {
            settings: TextTo3DSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: TextTo3DSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Generate 3D model from text description
    pub fn generate(&self, description: &str) -> AiResult<TextTo3DResult> {
        // Start timing
        let start_time = std::time::Instant::now();

        // Process text description
        let processed_description = self.process_description(description)?;

        // Extract features from description
        let features = self.extract_features(&processed_description)?;

        // Generate base mesh based on features
        let mut mesh = self.generate_base_mesh(&features)?;

        // Add details based on description
        self.add_details(&mut mesh, &features)?;

        // Optimize the generated mesh
        let optimized_mesh = self.optimize_mesh(&mesh)?;

        // Calculate generation time
        let generation_time = start_time.elapsed().as_secs_f64();

        // Calculate quality score
        let quality_score = self.calculate_quality_score(&optimized_mesh, description);

        Ok(TextTo3DResult {
            mesh: optimized_mesh,
            description: description.to_string(),
            settings: self.settings.clone(),
            generation_time,
            quality_score,
        })
    }

    /// Generate 3D model from sketch
    pub fn generate_from_sketch(&self, sketch_path: &str) -> AiResult<TextTo3DResult> {
        // Start timing
        let start_time = std::time::Instant::now();

        // Process sketch
        let processed_sketch = self.process_sketch(sketch_path)?;

        // Extract features from sketch
        let features = self.extract_features_from_sketch(&processed_sketch)?;

        // Generate base mesh based on features
        let mut mesh = self.generate_base_mesh(&features)?;

        // Add details based on sketch
        self.add_details(&mut mesh, &features)?;

        // Optimize the generated mesh
        let optimized_mesh = self.optimize_mesh(&mesh)?;

        // Calculate generation time
        let generation_time = start_time.elapsed().as_secs_f64();

        // Calculate quality score
        let quality_score = self.calculate_quality_score(&optimized_mesh, "sketch");

        Ok(TextTo3DResult {
            mesh: optimized_mesh,
            description: format!("Generated from sketch: {}", sketch_path),
            settings: self.settings.clone(),
            generation_time,
            quality_score,
        })
    }

    /// Generate 3D model from image
    pub fn generate_from_image(&self, image_path: &str) -> AiResult<TextTo3DResult> {
        // Start timing
        let start_time = std::time::Instant::now();

        // Process image
        let processed_image = self.process_image(image_path)?;

        // Extract features from image
        let features = self.extract_features_from_image(&processed_image)?;

        // Generate base mesh based on features
        let mut mesh = self.generate_base_mesh(&features)?;

        // Add details based on image
        self.add_details(&mut mesh, &features)?;

        // Optimize the generated mesh
        let optimized_mesh = self.optimize_mesh(&mesh)?;

        // Calculate generation time
        let generation_time = start_time.elapsed().as_secs_f64();

        // Calculate quality score
        let quality_score = self.calculate_quality_score(&optimized_mesh, "image");

        Ok(TextTo3DResult {
            mesh: optimized_mesh,
            description: format!("Generated from image: {}", image_path),
            settings: self.settings.clone(),
            generation_time,
            quality_score,
        })
    }

    /// Process text description
    fn process_description(&self, description: &str) -> AiResult<String> {
        // In a real implementation, this would include NLP processing
        // For now, we'll just return a cleaned version of the description
        let processed = description
            .trim()
            .to_lowercase()
            .replace(&['.', ',', '!', '?'][..], "");

        if processed.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "Empty description".to_string(),
            ));
        }

        Ok(processed)
    }

    /// Extract features from description
    fn extract_features(&self, description: &str) -> AiResult<HashMap<String, String>> {
        let mut features = HashMap::new();

        // Simple keyword extraction
        if description.contains("cube") || description.contains("box") {
            features.insert("shape".to_string(), "cube".to_string());
        } else if description.contains("sphere") || description.contains("ball") {
            features.insert("shape".to_string(), "sphere".to_string());
        } else if description.contains("cylinder") || description.contains("tube") {
            features.insert("shape".to_string(), "cylinder".to_string());
        } else if description.contains("cone") {
            features.insert("shape".to_string(), "cone".to_string());
        } else if description.contains("pyramid") {
            features.insert("shape".to_string(), "pyramid".to_string());
        } else {
            // Default to cube if no shape is specified
            features.insert("shape".to_string(), "cube".to_string());
        }

        // Extract size information
        if description.contains("large") {
            features.insert("size".to_string(), "large".to_string());
        } else if description.contains("small") {
            features.insert("size".to_string(), "small".to_string());
        } else {
            features.insert("size".to_string(), "medium".to_string());
        }

        // Extract color information
        if description.contains("red") {
            features.insert("color".to_string(), "red".to_string());
        } else if description.contains("blue") {
            features.insert("color".to_string(), "blue".to_string());
        } else if description.contains("green") {
            features.insert("color".to_string(), "green".to_string());
        } else if description.contains("yellow") {
            features.insert("color".to_string(), "yellow".to_string());
        } else if description.contains("black") {
            features.insert("color".to_string(), "black".to_string());
        } else if description.contains("white") {
            features.insert("color".to_string(), "white".to_string());
        } else {
            features.insert("color".to_string(), "gray".to_string());
        }

        Ok(features)
    }

    /// Generate base mesh based on features
    fn generate_base_mesh(&self, features: &HashMap<String, String>) -> AiResult<Mesh3D> {
        let cube_str = "cube".to_string();
        let medium_str = "medium".to_string();
        let shape = features.get("shape").unwrap_or(&cube_str);
        let size = features.get("size").unwrap_or(&medium_str);

        // Calculate size based on description
        let scale = match size.as_str() {
            "small" => 0.5,
            "large" => 1.5,
            _ => 1.0, // medium
        } * self.settings.size;

        match shape.as_str() {
            "cube" => Ok(self.generate_cube(scale)),
            "sphere" => Ok(self.generate_sphere(scale)),
            "cylinder" => Ok(self.generate_cylinder(scale)),
            "cone" => Ok(self.generate_cone(scale)),
            "pyramid" => Ok(self.generate_pyramid(scale)),
            _ => Ok(self.generate_cube(scale)), // default
        }
    }

    /// Generate cube mesh
    fn generate_cube(&self, size: f64) -> Mesh3D {
        let half_size = size / 2.0;

        // Vertices
        let vertices = vec![
            // Front face
            MeshVertex {
                id: 0,
                point: Point::new(-half_size, -half_size, half_size),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 1,
                point: Point::new(half_size, -half_size, half_size),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 2,
                point: Point::new(half_size, half_size, half_size),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 3,
                point: Point::new(-half_size, half_size, half_size),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            // Back face
            MeshVertex {
                id: 4,
                point: Point::new(-half_size, -half_size, -half_size),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 5,
                point: Point::new(half_size, -half_size, -half_size),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 6,
                point: Point::new(half_size, half_size, -half_size),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 7,
                point: Point::new(-half_size, half_size, -half_size),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
        ];

        // Faces
        let faces = vec![
            // Front face
            MeshFace {
                id: 0,
                vertices: vec![0, 1, 2, 3],
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            // Back face
            MeshFace {
                id: 1,
                vertices: vec![4, 5, 6, 7],
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            // Right face
            MeshFace {
                id: 2,
                vertices: vec![1, 5, 6, 2],
                normal: Some([1.0, 0.0, 0.0]),
                ..Default::default()
            },
            // Left face
            MeshFace {
                id: 3,
                vertices: vec![4, 0, 3, 7],
                normal: Some([-1.0, 0.0, 0.0]),
                ..Default::default()
            },
            // Top face
            MeshFace {
                id: 4,
                vertices: vec![3, 2, 6, 7],
                normal: Some([0.0, 1.0, 0.0]),
                ..Default::default()
            },
            // Bottom face
            MeshFace {
                id: 5,
                vertices: vec![4, 5, 1, 0],
                normal: Some([0.0, -1.0, 0.0]),
                ..Default::default()
            },
        ];

        let mut mesh = Mesh3D::new();
        mesh.vertices = vertices;
        mesh.faces = faces;
        mesh
    }

    /// Generate sphere mesh
    fn generate_sphere(&self, radius: f64) -> Mesh3D {
        let segments = 20;
        let rings = 10;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Generate vertices
        for i in 0..=rings {
            let v = i as f64 / rings as f64;
            let theta = v * std::f64::consts::PI;

            for j in 0..segments {
                let u = j as f64 / segments as f64;
                let phi = u * 2.0 * std::f64::consts::PI;

                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.cos();
                let z = radius * theta.sin() * phi.sin();

                let mut norm = Vector::new(x, y, z);
                norm.normalize();
                let normal = Some([norm.x, norm.y, norm.z]);
                vertices.push(MeshVertex {
                    id: vertices.len(),
                    point: Point::new(x, y, z),
                    normal,
                    ..Default::default()
                });
            }
        }

        // Generate faces
        for i in 0..rings {
            for j in 0..segments {
                let v0 = i * segments + j;
                let v1 = i * segments + (j + 1) % segments;
                let v2 = (i + 1) * segments + (j + 1) % segments;
                let v3 = (i + 1) * segments + j;

                // Calculate face normal
                let mut face_normal = Vector::new(0.0, 0.0, 0.0);
                if let Some(norm) = vertices[v0].normal {
                    face_normal.x += norm[0];
                    face_normal.y += norm[1];
                    face_normal.z += norm[2];
                }
                if let Some(norm) = vertices[v1].normal {
                    face_normal.x += norm[0];
                    face_normal.y += norm[1];
                    face_normal.z += norm[2];
                }
                if let Some(norm) = vertices[v2].normal {
                    face_normal.x += norm[0];
                    face_normal.y += norm[1];
                    face_normal.z += norm[2];
                }
                if let Some(norm) = vertices[v3].normal {
                    face_normal.x += norm[0];
                    face_normal.y += norm[1];
                    face_normal.z += norm[2];
                }
                face_normal.normalize();
                let face_normal = Some([face_normal.x, face_normal.y, face_normal.z]);

                faces.push(MeshFace {
                    id: faces.len(),
                    vertices: vec![v0, v1, v2, v3],
                    normal: face_normal,
                    ..Default::default()
                });
            }
        }

        let mut mesh = Mesh3D::new();
        mesh.vertices = vertices;
        mesh.faces = faces;
        mesh
    }

    /// Generate cylinder mesh
    fn generate_cylinder(&self, size: f64) -> Mesh3D {
        let radius = size / 2.0;
        let height = size;
        let segments = 20;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Generate top and bottom vertices
        vertices.push(MeshVertex {
            id: 0,
            point: Point::new(0.0, height / 2.0, 0.0),
            normal: Some([0.0, 1.0, 0.0]),
            ..Default::default()
        });
        vertices.push(MeshVertex {
            id: 1,
            point: Point::new(0.0, -height / 2.0, 0.0),
            normal: Some([0.0, -1.0, 0.0]),
            ..Default::default()
        });

        // Generate side vertices
        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            // Top vertex
            let mut norm = Vector::new(x, 0.0, z);
            norm.normalize();
            vertices.push(MeshVertex {
                id: vertices.len(),
                point: Point::new(x, height / 2.0, z),
                normal: Some([norm.x, norm.y, norm.z]),
                ..Default::default()
            });

            // Bottom vertex
            let mut norm = Vector::new(x, 0.0, z);
            norm.normalize();
            vertices.push(MeshVertex {
                id: vertices.len(),
                point: Point::new(x, -height / 2.0, z),
                normal: Some([norm.x, norm.y, norm.z]),
                ..Default::default()
            });
        }

        // Generate top face
        let mut top_face = Vec::new();
        top_face.push(0);
        for i in 0..segments {
            top_face.push(2 + i * 2);
        }
        faces.push(MeshFace {
            id: 0,
            vertices: top_face,
            normal: Some([0.0, 1.0, 0.0]),
            ..Default::default()
        });

        // Generate bottom face
        let mut bottom_face = Vec::new();
        bottom_face.push(1);
        for i in 0..segments {
            bottom_face.push(3 + i * 2);
        }
        faces.push(MeshFace {
            id: 1,
            vertices: bottom_face,
            normal: Some([0.0, -1.0, 0.0]),
            ..Default::default()
        });

        // Generate side faces
        for i in 0..segments {
            let v0 = 2 + i * 2;
            let v1 = 2 + ((i + 1) % segments) * 2;
            let v2 = 3 + ((i + 1) % segments) * 2;
            let v3 = 3 + i * 2;

            // Calculate face normal
            let mut face_normal = Vector::new(0.0, 0.0, 0.0);
            if let Some(norm) = vertices[v0].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            if let Some(norm) = vertices[v1].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            if let Some(norm) = vertices[v2].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            if let Some(norm) = vertices[v3].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            face_normal.normalize();
            let face_normal = Some([face_normal.x, face_normal.y, face_normal.z]);

            faces.push(MeshFace {
                id: faces.len(),
                vertices: vec![v0, v1, v2, v3],
                normal: face_normal,
                ..Default::default()
            });
        }

        let mut mesh = Mesh3D::new();
        mesh.vertices = vertices;
        mesh.faces = faces;
        mesh
    }

    /// Generate cone mesh
    fn generate_cone(&self, size: f64) -> Mesh3D {
        let radius = size / 2.0;
        let height = size;
        let segments = 20;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Generate apex vertex
        vertices.push(MeshVertex {
            id: 0,
            point: Point::new(0.0, height / 2.0, 0.0),
            normal: Some([0.0, 1.0, 0.0]),
            ..Default::default()
        });

        // Generate base center vertex
        vertices.push(MeshVertex {
            id: 1,
            point: Point::new(0.0, -height / 2.0, 0.0),
            normal: Some([0.0, -1.0, 0.0]),
            ..Default::default()
        });

        // Generate base vertices
        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            let mut norm = Vector::new(x, -height / 2.0, z);
            norm.normalize();
            vertices.push(MeshVertex {
                id: vertices.len(),
                point: Point::new(x, -height / 2.0, z),
                normal: Some([norm.x, norm.y, norm.z]),
                ..Default::default()
            });
        }

        // Generate base face
        let mut base_face = Vec::new();
        base_face.push(1);
        for i in 0..segments {
            base_face.push(2 + i);
        }
        faces.push(MeshFace {
            id: 0,
            vertices: base_face,
            normal: Some([0.0, -1.0, 0.0]),
            ..Default::default()
        });

        // Generate side faces
        for i in 0..segments {
            let v0 = 0;
            let v1 = 2 + i;
            let v2 = 2 + ((i + 1) % segments);

            // Calculate face normal
            let mut face_normal = Vector::new(0.0, 0.0, 0.0);
            if let Some(norm) = vertices[v0].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            if let Some(norm) = vertices[v1].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            if let Some(norm) = vertices[v2].normal {
                face_normal.x += norm[0];
                face_normal.y += norm[1];
                face_normal.z += norm[2];
            }
            face_normal.normalize();
            let face_normal = Some([face_normal.x, face_normal.y, face_normal.z]);

            faces.push(MeshFace {
                id: faces.len(),
                vertices: vec![v0, v1, v2],
                normal: face_normal,
                ..Default::default()
            });
        }

        let mut mesh = Mesh3D::new();
        mesh.vertices = vertices;
        mesh.faces = faces;
        mesh
    }

    /// Generate pyramid mesh
    fn generate_pyramid(&self, size: f64) -> Mesh3D {
        let half_size = size / 2.0;

        // Vertices
        let vertices = vec![
            // Apex
            MeshVertex {
                id: 0,
                point: Point::new(0.0, half_size, 0.0),
                normal: Some([0.0, 1.0, 0.0]),
                ..Default::default()
            },
            // Base vertices
            MeshVertex {
                id: 1,
                point: Point::new(-half_size, -half_size, -half_size),
                normal: Some([-0.7071, 0.0, -0.7071]),
                ..Default::default()
            },
            MeshVertex {
                id: 2,
                point: Point::new(half_size, -half_size, -half_size),
                normal: Some([0.7071, 0.0, -0.7071]),
                ..Default::default()
            },
            MeshVertex {
                id: 3,
                point: Point::new(half_size, -half_size, half_size),
                normal: Some([0.7071, 0.0, 0.7071]),
                ..Default::default()
            },
            MeshVertex {
                id: 4,
                point: Point::new(-half_size, -half_size, half_size),
                normal: Some([-0.7071, 0.0, 0.7071]),
                ..Default::default()
            },
        ];

        // Faces
        let faces = vec![
            // Base face
            MeshFace {
                id: 0,
                vertices: vec![1, 2, 3, 4],
                normal: Some([0.0, -1.0, 0.0]),
                ..Default::default()
            },
            // Side faces
            MeshFace {
                id: 1,
                vertices: vec![0, 1, 2],
                normal: Some([0.0, 0.7071, -0.7071]),
                ..Default::default()
            },
            MeshFace {
                id: 2,
                vertices: vec![0, 2, 3],
                normal: Some([0.7071, 0.7071, 0.0]),
                ..Default::default()
            },
            MeshFace {
                id: 3,
                vertices: vec![0, 3, 4],
                normal: Some([0.0, 0.7071, 0.7071]),
                ..Default::default()
            },
            MeshFace {
                id: 4,
                vertices: vec![0, 4, 1],
                normal: Some([-0.7071, 0.7071, 0.0]),
                ..Default::default()
            },
        ];

        let mut mesh = Mesh3D::new();
        mesh.vertices = vertices;
        mesh.faces = faces;
        mesh
    }

    /// Add details to the mesh based on features
    fn add_details(&self, _mesh: &mut Mesh3D, _features: &HashMap<String, String>) -> AiResult<()> {
        // In a real implementation, this would add more complex details
        // For now, we'll just return the mesh as is
        Ok(())
    }

    /// Optimize the generated mesh
    fn optimize_mesh(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        // In a real implementation, this would include mesh cleaning and optimization
        // For now, we'll just return a copy of the mesh
        Ok(mesh.clone())
    }

    /// Calculate quality score for the generated mesh
    fn calculate_quality_score(&self, mesh: &Mesh3D, _description: &str) -> f64 {
        // In a real implementation, this would include more sophisticated metrics
        // For now, we'll just return a score based on mesh complexity
        let vertex_count = mesh.vertices.len() as f64;
        let face_count = mesh.faces.len() as f64;

        let complexity_score = (vertex_count + face_count) / 1000.0;
        complexity_score.max(0.0).min(1.0)
    }

    /// Process sketch
    fn process_sketch(&self, sketch_path: &str) -> AiResult<String> {
        // In a real implementation, this would include sketch processing
        // For now, we'll just return the sketch path as a placeholder
        if sketch_path.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "Empty sketch path".to_string(),
            ));
        }

        Ok(sketch_path.to_string())
    }

    /// Extract features from sketch
    fn extract_features_from_sketch(&self, _sketch: &str) -> AiResult<HashMap<String, String>> {
        let mut features = HashMap::new();

        // In a real implementation, this would include sketch feature extraction
        // For now, we'll return default features
        features.insert("shape".to_string(), "cube".to_string());
        features.insert("size".to_string(), "medium".to_string());
        features.insert("color".to_string(), "gray".to_string());

        Ok(features)
    }

    /// Process image
    fn process_image(&self, image_path: &str) -> AiResult<String> {
        // In a real implementation, this would include image processing
        // For now, we'll just return the image path as a placeholder
        if image_path.is_empty() {
            return Err(AiProtocolError::InvalidData("Empty image path".to_string()));
        }

        Ok(image_path.to_string())
    }

    /// Extract features from image
    fn extract_features_from_image(&self, _image: &str) -> AiResult<HashMap<String, String>> {
        let mut features = HashMap::new();

        // In a real implementation, this would include image feature extraction
        // For now, we'll return default features
        features.insert("shape".to_string(), "cube".to_string());
        features.insert("size".to_string(), "medium".to_string());
        features.insert("color".to_string(), "gray".to_string());

        Ok(features)
    }
}

/// Extension methods for Mesh3D
pub trait TextTo3DExt {
    /// Generate mesh from text description
    fn from_text(description: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult>;
    /// Generate mesh from sketch
    fn from_sketch(sketch_path: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult>;
    /// Generate mesh from image
    fn from_image(image_path: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult>;
}

impl TextTo3DExt for Mesh3D {
    fn from_text(description: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult> {
        let generator = TextTo3DGenerator::new().with_settings((*settings).clone());
        generator.generate(description)
    }

    fn from_sketch(sketch_path: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult> {
        let generator = TextTo3DGenerator::new().with_settings((*settings).clone());
        generator.generate_from_sketch(sketch_path)
    }

    fn from_image(image_path: &str, settings: &TextTo3DSettings) -> AiResult<TextTo3DResult> {
        let generator = TextTo3DGenerator::new().with_settings((*settings).clone());
        generator.generate_from_image(image_path)
    }
}
