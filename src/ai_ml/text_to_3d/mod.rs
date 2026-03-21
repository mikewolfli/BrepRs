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
    // AI models and other dependencies
    model_path: Option<String>,
    nlp_model_path: Option<String>,
    sketch_model_path: Option<String>,
    image_model_path: Option<String>,
}

impl TextTo3DGenerator {
    pub fn new() -> Self {
        Self {
            settings: TextTo3DSettings::default(),
            model_path: None,
            nlp_model_path: None,
            sketch_model_path: None,
            image_model_path: None,
        }
    }

    pub fn with_settings(mut self, settings: TextTo3DSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn with_model_path(mut self, path: &str) -> Self {
        self.model_path = Some(path.to_string());
        self
    }

    pub fn with_nlp_model_path(mut self, path: &str) -> Self {
        self.nlp_model_path = Some(path.to_string());
        self
    }

    pub fn with_sketch_model_path(mut self, path: &str) -> Self {
        self.sketch_model_path = Some(path.to_string());
        self
    }

    pub fn with_image_model_path(mut self, path: &str) -> Self {
        self.image_model_path = Some(path.to_string());
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
        // Real implementation: use NLP library for processing
        #[cfg(feature = "nlp")] // Example: using nlp crate
        {
            use nlp::preprocessing::clean_text;
            use nlp::keyphrase::extract_key_phrases;
            let processed = clean_text(description);
            if processed.is_empty() {
                return Err(AiProtocolError::InvalidData("Empty description".to_string()));
            }
            let key_phrases = extract_key_phrases(&processed);
            println!("Extracted key phrases: {:?}", key_phrases);
            Ok(processed)
        }
        #[cfg(not(feature = "nlp"))]
        {
            let processed = description
                .trim()
                .to_lowercase()
                .replace(&['.', ',', '!', '?'][..], "")
                .replace("  ", " ");
            if processed.is_empty() {
                return Err(AiProtocolError::InvalidData("Empty description".to_string()));
            }
            let key_phrases = self.extract_key_phrases(&processed);
            println!("Extracted key phrases: {:?}", key_phrases);
            Ok(processed)
        }
    }

    /// Extract key phrases from description
    fn extract_key_phrases(&self, description: &str) -> Vec<String> {
        // Simple key phrase extraction
        let mut key_phrases = Vec::new();
        let _words: Vec<&str> = description.split_whitespace().collect();

        // Look for shape-related phrases
        let shape_keywords = ["cube", "box", "sphere", "ball", "cylinder", "tube", "cone", "pyramid"];
        for keyword in &shape_keywords {
            if description.contains(keyword) {
                key_phrases.push(keyword.to_string());
            }
        }

        // Look for size-related phrases
        let size_keywords = ["small", "medium", "large", "huge", "tiny"];
        for keyword in &size_keywords {
            if description.contains(keyword) {
                key_phrases.push(keyword.to_string());
            }
        }

        // Look for color-related phrases
        let color_keywords = ["red", "blue", "green", "yellow", "black", "white", "gray", "brown", "purple", "pink"];
        for keyword in &color_keywords {
            if description.contains(keyword) {
                key_phrases.push(keyword.to_string());
            }
        }

        key_phrases
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
    fn add_details(&self, mesh: &mut Mesh3D, features: &HashMap<String, String>) -> AiResult<()> {
        // Add details based on features
        if let Some(shape) = features.get("shape") {
            match shape.as_str() {
                "sphere" => {
                    // Add subtle bumps to sphere
                    self.add_bumps(mesh, 0.1, 10);
                }
                "cylinder" => {
                    // Add grooves to cylinder
                    self.add_grooves(mesh, 0.05, 5);
                }
                "cube" => {
                    // Add bevels to cube edges
                    self.add_bevels(mesh, 0.1);
                }
                "cone" => {
                    // Add ridges to cone
                    self.add_ridges(mesh, 0.05, 8);
                }
                "pyramid" => {
                    // Add texture to pyramid faces
                    self.add_texture(mesh, 0.1, 0.1);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Add bumps to a mesh
    fn add_bumps(&self, mesh: &mut Mesh3D, amplitude: f64, _count: usize) {
        for vertex in &mut mesh.vertices {
            let _distance = vertex.point.distance(&crate::geometry::Point::origin());
            let noise = (vertex.id as f64 * 0.1).sin() * amplitude;
            let mut direction = Vector::new(vertex.point.x, vertex.point.y, vertex.point.z);
            if direction.magnitude() > 0.0 {
                direction.normalize();
                vertex.point += direction * noise;
            }
        }
    }

    /// Add grooves to a mesh
    fn add_grooves(&self, mesh: &mut Mesh3D, depth: f64, count: usize) {
        for vertex in &mut mesh.vertices {
            let y = vertex.point.y;
            let groove = (y * count as f64).sin() * depth;
            let mut direction = Vector::new(vertex.point.x, vertex.point.y, vertex.point.z);
            direction.y = 0.0;
            if direction.magnitude() > 0.0 {
                direction.normalize();
                vertex.point += direction * groove;
            }
        }
    }

    /// Add bevels to a mesh
    fn add_bevels(&self, mesh: &mut Mesh3D, radius: f64) {
        // Simplified bevel implementation
        for vertex in &mut mesh.vertices {
            let abs_x = vertex.point.x.abs();
            let abs_y = vertex.point.y.abs();
            let abs_z = vertex.point.z.abs();
            let max_coord = abs_x.max(abs_y).max(abs_z);

            if (abs_x - max_coord).abs() < radius {
                vertex.point.x = if vertex.point.x > 0.0 {
                    max_coord - radius
                } else {
                    -max_coord + radius
                };
            }
            if (abs_y - max_coord).abs() < radius {
                vertex.point.y = if vertex.point.y > 0.0 {
                    max_coord - radius
                } else {
                    -max_coord + radius
                };
            }
            if (abs_z - max_coord).abs() < radius {
                vertex.point.z = if vertex.point.z > 0.0 {
                    max_coord - radius
                } else {
                    -max_coord + radius
                };
            }
        }
    }

    /// Add ridges to a mesh
    fn add_ridges(&self, mesh: &mut Mesh3D, height: f64, count: usize) {
        for vertex in &mut mesh.vertices {
            let distance =
                vertex
                    .point
                    .distance(&crate::geometry::Point::new(0.0, -vertex.point.y, 0.0));
            let ridge = (distance * count as f64).sin() * height;
            let mut direction = Vector::new(vertex.point.x, vertex.point.y, vertex.point.z);
            direction.normalize();
            vertex.point += direction * ridge;
        }
    }

    /// Add texture to a mesh
    fn add_texture(&self, mesh: &mut Mesh3D, scale_x: f64, scale_y: f64) {
        for vertex in &mut mesh.vertices {
            let noise = (vertex.point.x * scale_x).sin() * (vertex.point.z * scale_y).cos() * 0.05;
            vertex.point.y += noise;
        }
    }

    /// Optimize the generated mesh
    fn optimize_mesh(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        let mut optimized_mesh = mesh.clone();

        // Clean up duplicate vertices
        self.remove_duplicate_vertices(&mut optimized_mesh);

        // Fix degenerate faces
        self.remove_degenerate_faces(&mut optimized_mesh);

        // Recalculate normals
        self.recalculate_normals(&mut optimized_mesh);

        Ok(optimized_mesh)
    }

    /// Remove duplicate vertices from the mesh
    fn remove_duplicate_vertices(&self, mesh: &mut Mesh3D) {
        let mut unique_vertices = Vec::new();
        let mut vertex_map = std::collections::HashMap::new();

        let tolerance = 1000.0;
        for (_index, vertex) in mesh.vertices.iter().enumerate() {
            let key = (
                (vertex.point.x * tolerance).round() as i64,
                (vertex.point.y * tolerance).round() as i64,
                (vertex.point.z * tolerance).round() as i64,
            );
            if !vertex_map.contains_key(&key) {
                vertex_map.insert(key, unique_vertices.len());
                let mut new_vertex = vertex.clone();
                new_vertex.id = unique_vertices.len();
                unique_vertices.push(new_vertex);
            }
        }

        // Update face vertex indices
        for face in &mut mesh.faces {
            for i in 0..face.vertices.len() {
                let old_index = face.vertices[i];
                let old_vertex = &mesh.vertices[old_index];
                let key = (
                    (old_vertex.point.x * tolerance).round() as i64,
                    (old_vertex.point.y * tolerance).round() as i64,
                    (old_vertex.point.z * tolerance).round() as i64,
                );
                if let Some(&new_index) = vertex_map.get(&key) {
                    face.vertices[i] = new_index;
                }
            }
        }

        mesh.vertices = unique_vertices;
    }

    /// Remove degenerate faces from the mesh
    fn remove_degenerate_faces(&self, mesh: &mut Mesh3D) {
        let mut valid_faces = Vec::new();

        for face in &mesh.faces {
            // Check if the face has at least 3 unique vertices
            let mut unique_vertices = std::collections::HashSet::new();
            for &vertex_id in &face.vertices {
                unique_vertices.insert(vertex_id);
            }

            if unique_vertices.len() >= 3 {
                // Check if the face area is not too small
                if self.calculate_face_area(mesh, face) > 1e-6 {
                    let mut new_face = face.clone();
                    new_face.id = valid_faces.len();
                    valid_faces.push(new_face);
                }
            }
        }

        mesh.faces = valid_faces;
    }

    /// Calculate the area of a face
    fn calculate_face_area(&self, mesh: &Mesh3D, face: &MeshFace) -> f64 {
        if face.vertices.len() < 3 {
            return 0.0;
        }

        // Calculate face area using the shoelace formula for 3D
        let mut area = 0.0;
        let v0 = &mesh.vertices[face.vertices[0]];

        for i in 1..face.vertices.len() - 1 {
            let v1 = &mesh.vertices[face.vertices[i]];
            let v2 = &mesh.vertices[face.vertices[i + 1]];

            let vec1 = v1.point - v0.point;
            let vec2 = v2.point - v0.point;
            let cross = vec1.cross(&vec2);
            area += cross.magnitude() / 2.0;
        }

        area
    }

    /// Recalculate normals for the mesh
    fn recalculate_normals(&self, mesh: &mut Mesh3D) {
        // Reset normals
        for vertex in &mut mesh.vertices {
            vertex.normal = None;
        }

        // Calculate face normals and accumulate vertex normals
        for face in &mut mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]];
                let v1 = &mesh.vertices[face.vertices[1]];
                let v2 = &mesh.vertices[face.vertices[2]];

                let vec1 = v1.point - v0.point;
                let vec2 = v2.point - v0.point;
                let mut normal = vec1.cross(&vec2);

                if normal.magnitude() > 1e-6 {
                    normal.normalize();
                    let face_normal = Some([normal.x, normal.y, normal.z]);
                    face.normal = face_normal;

                    // Accumulate normal to vertices
                    for &vertex_id in &face.vertices {
                        let vertex = &mut mesh.vertices[vertex_id];
                        if let Some(vertex_normal) = vertex.normal {
                            let mut new_normal = crate::geometry::Vector::new(
                                vertex_normal[0] + normal.x,
                                vertex_normal[1] + normal.y,
                                vertex_normal[2] + normal.z,
                            );
                            if new_normal.magnitude() > 1e-6 {
                                new_normal.normalize();
                                vertex.normal = Some([new_normal.x, new_normal.y, new_normal.z]);
                            }
                        } else {
                            vertex.normal = face_normal;
                        }
                    }
                }
            }
        }
    }

    /// Calculate quality score for the generated mesh
    fn calculate_quality_score(&self, mesh: &Mesh3D, _description: &str) -> f64 {
        // Currently calculates quality score based on complexity, face quality, and normal quality
        // Future implementation will include more sophisticated metrics
        let vertex_count = mesh.vertices.len() as f64;
        let face_count = mesh.faces.len() as f64;
        
        // Complexity score
        let complexity_score = (vertex_count + face_count) / 1000.0;
        let normalized_complexity = complexity_score.max(0.0).min(1.0);
        
        // Face quality score (based on face areas)
        let mut face_quality = 0.0;
        let mut valid_faces = 0;
        for face in &mesh.faces {
            let area = self.calculate_face_area(mesh, face);
            if area > 1e-6 {
                face_quality += 1.0;
                valid_faces += 1;
            }
        }
        let normalized_face_quality = if valid_faces > 0 {
            face_quality / valid_faces as f64
        } else {
            0.0
        };
        
        // Vertex normal quality (based on presence of normals)
        let mut normal_quality = 0.0;
        for vertex in &mesh.vertices {
            if vertex.normal.is_some() {
                normal_quality += 1.0;
            }
        }
        let normalized_normal_quality = if mesh.vertices.len() > 0 {
            normal_quality / mesh.vertices.len() as f64
        } else {
            0.0
        };
        
        // Combine scores with weights
        let total_score = 0.4 * normalized_complexity + 0.4 * normalized_face_quality + 0.2 * normalized_normal_quality;
        total_score.max(0.0).min(1.0)
    }

    /// Process sketch
    fn process_sketch(&self, sketch_path: &str) -> AiResult<String> {
        if sketch_path.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "Empty sketch path".to_string(),
            ));
        }

        // Check if the file exists
        if !std::path::Path::new(sketch_path).exists() {
            return Err(AiProtocolError::InvalidData(format!(
                "Sketch file not found: {}",
                sketch_path
            )));
        }

        // Currently validates sketch file existence
        // Future implementation will include sketch processing and feature extraction
        Ok(sketch_path.to_string())
    }

    /// Extract features from sketch
    fn extract_features_from_sketch(&self, _sketch: &str) -> AiResult<HashMap<String, String>> {
        #[cfg(feature = "sketch2mesh")]
        {
            // TODO: integrate with sketch2mesh library for real feature extraction
            // let features = sketch2mesh::extract_features(_sketch);
            // Ok(features)
            let mut features = HashMap::new();
            features.insert("shape".to_string(), "cube".to_string());
            features.insert("size".to_string(), "medium".to_string());
            features.insert("color".to_string(), "gray".to_string());
            Ok(features)
        }
        #[cfg(not(feature = "sketch2mesh"))]
        {
            let mut features = HashMap::new();
            features.insert("shape".to_string(), "cube".to_string());
            features.insert("size".to_string(), "medium".to_string());
            features.insert("color".to_string(), "gray".to_string());
            Ok(features)
        }
    }

    /// Process image
    fn process_image(&self, image_path: &str) -> AiResult<String> {
        if image_path.is_empty() {
            return Err(AiProtocolError::InvalidData("Empty image path".to_string()));
        }

        // Check if the file exists
        if !std::path::Path::new(image_path).exists() {
            return Err(AiProtocolError::InvalidData(format!(
                "Image file not found: {}",
                image_path
            )));
        }

        // Currently validates image file existence
        // Future implementation will include image processing and feature extraction
        Ok(image_path.to_string())
    }

    /// Extract features from image
    fn extract_features_from_image(&self, _image: &str) -> AiResult<HashMap<String, String>> {
        #[cfg(feature = "imageproc")]
        {
            // TODO: integrate with imageproc or similar library for real feature extraction
            // let features = imageproc::extract_features(_image);
            // Ok(features)
            let mut features = HashMap::new();
            features.insert("shape".to_string(), "cube".to_string());
            features.insert("size".to_string(), "medium".to_string());
            features.insert("color".to_string(), "gray".to_string());
            Ok(features)
        }
        #[cfg(not(feature = "imageproc"))]
        {
            let mut features = HashMap::new();
            features.insert("shape".to_string(), "cube".to_string());
            features.insert("size".to_string(), "medium".to_string());
            features.insert("color".to_string(), "gray".to_string());
            Ok(features)
        }
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
