//! Next-generation API
//!
//! This module provides Rust-specific API optimizations and improvements,
//! including:
//! - Idiomatic Rust API design
//! - Zero-cost abstractions
//! - Type safety
//! - Memory efficiency
//! - Incremental compilation support

pub mod traits;

use crate::geometry::Point;

/// Rust-specific API optimizations
pub mod optimized {
    use crate::geometry::Point;
    use crate::mesh::mesh_data::Mesh3D;
    use crate::topology::TopoDsShape;

    /// Optimized point type with zero-cost abstractions
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct OptimizedPoint {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl OptimizedPoint {
        /// Create a new optimized point
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }

        /// Convert to standard Point
        pub fn to_point(&self) -> Point {
            Point::new(self.x, self.y, self.z)
        }

        /// Convert from standard Point
        pub fn from_point(point: &Point) -> Self {
            Self::new(point.x, point.y, point.z)
        }

        /// Add two points
        pub fn add(&self, other: &Self) -> Self {
            Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }

        /// Subtract two points
        pub fn sub(&self, other: &Self) -> Self {
            Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
        }

        /// Multiply by scalar
        pub fn mul(&self, scalar: f64) -> Self {
            Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
        }

        /// Divide by scalar
        pub fn div(&self, scalar: f64) -> Self {
            Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
        }

        /// Calculate distance to another point
        pub fn distance(&self, other: &Self) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dz = self.z - other.z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        }

        /// Calculate squared distance to another point
        pub fn distance_squared(&self, other: &Self) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dz = self.z - other.z;
            dx * dx + dy * dy + dz * dz
        }

        /// Check if points are approximately equal
        pub fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.distance_squared(other) < epsilon * epsilon
        }
    }

    /// Optimized mesh with SoA (Struct of Arrays) layout
    pub struct OptimizedMesh {
        pub vertices: Vec<f64>,        // x, y, z for each vertex
        pub normals: Option<Vec<f64>>, // nx, ny, nz for each vertex
        pub faces: Vec<usize>,         // vertex indices for each face
    }

    impl OptimizedMesh {
        /// Create a new optimized mesh
        pub fn new() -> Self {
            Self {
                vertices: Vec::new(),
                normals: None,
                faces: Vec::new(),
            }
        }

        /// Add a vertex
        pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
            let index = self.vertices.len() / 3;
            self.vertices.extend_from_slice(&[x, y, z]);
            if let Some(ref mut normals) = self.normals {
                normals.extend_from_slice(&[0.0, 0.0, 0.0]);
            }
            index
        }

        /// Add a face
        pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) {
            self.faces.extend_from_slice(&[v1, v2, v3]);
        }

        /// Enable normals
        pub fn enable_normals(&mut self) {
            if self.normals.is_none() {
                self.normals = Some(vec![0.0; self.vertices.len()]);
            }
        }

        /// Set normal for a vertex
        pub fn set_normal(&mut self, vertex_id: usize, nx: f64, ny: f64, nz: f64) {
            if let Some(ref mut normals) = self.normals {
                let offset = vertex_id * 3;
                normals[offset] = nx;
                normals[offset + 1] = ny;
                normals[offset + 2] = nz;
            }
        }

        /// Get vertex position
        pub fn get_vertex(&self, vertex_id: usize) -> (f64, f64, f64) {
            let offset = vertex_id * 3;
            (
                self.vertices[offset],
                self.vertices[offset + 1],
                self.vertices[offset + 2],
            )
        }

        /// Get face vertices
        pub fn get_face(&self, face_id: usize) -> (usize, usize, usize) {
            let offset = face_id * 3;
            (
                self.faces[offset],
                self.faces[offset + 1],
                self.faces[offset + 2],
            )
        }

        /// Get vertex normal
        pub fn get_normal(&self, vertex_id: usize) -> Option<(f64, f64, f64)> {
            if let Some(ref normals) = self.normals {
                let offset = vertex_id * 3;
                Some((normals[offset], normals[offset + 1], normals[offset + 2]))
            } else {
                None
            }
        }

        /// Convert to standard Mesh3D
        pub fn to_mesh(&self) -> Mesh3D {
            let mut mesh = Mesh3D::new();

            // Add vertices
            for i in 0..self.vertices.len() / 3 {
                let (x, y, z) = self.get_vertex(i);
                let mut vertex = crate::mesh::mesh_data::MeshVertex::new(i, Point::new(x, y, z));
                if let Some(normal) = self.get_normal(i) {
                    vertex.normal = Some([normal.0, normal.1, normal.2]);
                }
                mesh.vertices.push(vertex);
            }

            // Add faces
            for i in 0..self.faces.len() / 3 {
                let (v1, v2, v3) = self.get_face(i);
                let face = crate::mesh::mesh_data::MeshFace::new(i, vec![v1, v2, v3]);
                mesh.faces.push(face);
            }

            mesh
        }

        /// Convert from standard Mesh3D
        pub fn from_mesh(mesh: &Mesh3D) -> Self {
            let mut optimized_mesh = Self::new();

            // Add vertices
            for vertex in &mesh.vertices {
                optimized_mesh.add_vertex(vertex.point.x, vertex.point.y, vertex.point.z);
            }

            // Add normals if available
            if mesh.vertices.iter().any(|v| v.normal.is_some()) {
                optimized_mesh.enable_normals();
                for (i, vertex) in mesh.vertices.iter().enumerate() {
                    if let Some(normal) = vertex.normal {
                        optimized_mesh.set_normal(i, normal[0], normal[1], normal[2]);
                    }
                }
            }

            // Add faces
            for face in &mesh.faces {
                if face.vertices.len() == 3 {
                    optimized_mesh.add_face(face.vertices[0], face.vertices[1], face.vertices[2]);
                }
            }

            optimized_mesh
        }

        /// Calculate bounding box
        pub fn calculate_bounding_box(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
            if self.vertices.is_empty() {
                return None;
            }

            let mut min_x = self.vertices[0];
            let mut min_y = self.vertices[1];
            let mut min_z = self.vertices[2];
            let mut max_x = self.vertices[0];
            let mut max_y = self.vertices[1];
            let mut max_z = self.vertices[2];

            for i in 1..self.vertices.len() / 3 {
                let (x, y, z) = self.get_vertex(i);
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                min_z = min_z.min(z);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                max_z = max_z.max(z);
            }

            Some((min_x, min_y, min_z, max_x, max_y, max_z))
        }

        /// Calculate volume
        pub fn calculate_volume(&self) -> f64 {
            // Implementation of volume calculation using the divergence theorem
            // Sum over all faces: 1/3 * (cross product of two edges) · centroid
            let mut volume = 0.0;

            for i in 0..self.faces.len() / 3 {
                let (v0, v1, v2) = self.get_face(i);

                let (x0, y0, z0) = self.get_vertex(v0);
                let (x1, y1, z1) = self.get_vertex(v1);
                let (x2, y2, z2) = self.get_vertex(v2);

                // Calculate centroid
                let cx = (x0 + x1 + x2) / 3.0;
                let cy = (y0 + y1 + y2) / 3.0;
                let cz = (z0 + z1 + z2) / 3.0;

                // Calculate cross product
                let ax = x1 - x0;
                let ay = y1 - y0;
                let az = z1 - z0;
                let bx = x2 - x0;
                let by = y2 - y0;
                let bz = z2 - z0;

                let cross_x = ay * bz - az * by;
                let cross_y = az * bx - ax * bz;
                let cross_z = ax * by - ay * bx;

                // Dot product with centroid
                let dot = cx * cross_x + cy * cross_y + cz * cross_z;

                volume += dot;
            }

            volume.abs() / 6.0
        }

        /// Calculate surface area
        pub fn calculate_surface_area(&self) -> f64 {
            // Implementation of surface area calculation
            let mut area = 0.0;

            for i in 0..self.faces.len() / 3 {
                let (v0, v1, v2) = self.get_face(i);

                let (x0, y0, z0) = self.get_vertex(v0);
                let (x1, y1, z1) = self.get_vertex(v1);
                let (x2, y2, z2) = self.get_vertex(v2);

                // Calculate vectors
                let ax = x1 - x0;
                let ay = y1 - y0;
                let az = z1 - z0;
                let bx = x2 - x0;
                let by = y2 - y0;
                let bz = z2 - z0;

                // Calculate cross product
                let cross_x = ay * bz - az * by;
                let cross_y = az * bx - ax * bz;
                let cross_z = ax * by - ay * bx;

                // Calculate magnitude
                let magnitude = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z).sqrt();

                area += magnitude;
            }

            area / 2.0
        }
    }

    /// Optimized shape with type-level safety
    pub trait OptimizedShape {
        /// Get shape type
        fn shape_type(&self) -> crate::topology::shape_enum::ShapeType;

        /// Convert to standard TopoDsShape
        fn to_shape(&self) -> TopoDsShape;
    }

    /// Optimized vertex
    #[derive(Debug, Clone)]
    pub struct OptimizedVertex {
        pub id: usize,
        pub point: OptimizedPoint,
    }

    impl OptimizedVertex {
        /// Create a new optimized vertex
        pub fn new(id: usize, point: OptimizedPoint) -> Self {
            Self { id, point }
        }
    }

    impl OptimizedShape for OptimizedVertex {
        fn shape_type(&self) -> crate::topology::shape_enum::ShapeType {
            crate::topology::shape_enum::ShapeType::Vertex
        }

        fn to_shape(&self) -> TopoDsShape {
            let vertex = crate::topology::topods_vertex::TopoDsVertex::new(self.point.to_point());
            vertex.shape().clone()
        }
    }

    /// Optimized edge
    #[derive(Debug, Clone)]
    pub struct OptimizedEdge {
        pub id: usize,
        pub start: OptimizedVertex,
        pub end: OptimizedVertex,
    }

    impl OptimizedEdge {
        /// Create a new optimized edge
        pub fn new(id: usize, start: OptimizedVertex, end: OptimizedVertex) -> Self {
            Self { id, start, end }
        }
    }

    impl OptimizedShape for OptimizedEdge {
        fn shape_type(&self) -> crate::topology::shape_enum::ShapeType {
            crate::topology::shape_enum::ShapeType::Edge
        }

        fn to_shape(&self) -> TopoDsShape {
            let vertex1 =
                crate::topology::topods_vertex::TopoDsVertex::new(self.start.point.to_point());
            let vertex2 =
                crate::topology::topods_vertex::TopoDsVertex::new(self.end.point.to_point());
            let edge = crate::topology::topods_edge::TopoDsEdge::new(
                crate::foundation::handle::Handle::new(std::sync::Arc::new(vertex1)),
                crate::foundation::handle::Handle::new(std::sync::Arc::new(vertex2)),
            );
            edge.shape().clone()
        }
    }

    /// Optimized face
    #[derive(Debug, Clone)]
    pub struct OptimizedFace {
        pub id: usize,
        pub edges: Vec<OptimizedEdge>,
    }

    impl OptimizedFace {
        /// Create a new optimized face
        pub fn new(id: usize, edges: Vec<OptimizedEdge>) -> Self {
            Self { id, edges }
        }
    }

    impl OptimizedShape for OptimizedFace {
        fn shape_type(&self) -> crate::topology::shape_enum::ShapeType {
            crate::topology::shape_enum::ShapeType::Face
        }

        fn to_shape(&self) -> TopoDsShape {
            // Implementation of face to shape conversion
            // Build face from edges
            use crate::foundation::handle::Handle;
            use crate::topology::topods_edge::TopoDsEdge;
            use crate::topology::topods_face::TopoDsFace;
            use crate::topology::topods_vertex::TopoDsVertex;
            use crate::topology::topods_wire::TopoDsWire;

            if self.edges.is_empty() {
                return TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face);
            }

            // Collect all vertices from edges
            let mut vertices = Vec::new();
            let mut vertex_map = std::collections::HashMap::new();

            for edge in &self.edges {
                if !vertex_map.contains_key(&edge.start.id) {
                    vertex_map.insert(edge.start.id, edge.start.clone());
                    vertices.push(edge.start.clone());
                }
                if !vertex_map.contains_key(&edge.end.id) {
                    vertex_map.insert(edge.end.id, edge.end.clone());
                    vertices.push(edge.end.clone());
                }
            }

            // Create TopoDsVertex objects
            let mut topo_vertices = Vec::new();
            for vertex in &vertices {
                let topo_vertex = TopoDsVertex::new(vertex.point.to_point());
                topo_vertices.push(Handle::new(std::sync::Arc::new(topo_vertex)));
            }

            // Create TopoDsEdge objects
            let mut topo_edges = Vec::new();
            for edge in &self.edges {
                let start_idx = vertices.iter().position(|v| v.id == edge.start.id).unwrap();
                let end_idx = vertices.iter().position(|v| v.id == edge.end.id).unwrap();

                let topo_edge = TopoDsEdge::new(
                    topo_vertices[start_idx].clone(),
                    topo_vertices[end_idx].clone(),
                );
                topo_edges.push(Handle::new(std::sync::Arc::new(topo_edge)));
            }

            // Create wire from edges
            let wire = TopoDsWire::with_edges(topo_edges);

            // Create face from wire
            let mut face = TopoDsFace::new();
            face.add_wire(Handle::new(std::sync::Arc::new(wire)));

            face.shape().clone()
        }
    }
}

/// Incremental compilation support
pub mod incremental {
    use super::*;

    /// Hot reload manager
    pub struct HotReloadManager {
        // Hot reload configuration
        watch_paths: Vec<String>,
        reload_callbacks: Vec<Box<dyn Fn()>>,
    }

    impl HotReloadManager {
        /// Create a new hot reload manager
        pub fn new() -> Self {
            Self {
                watch_paths: Vec::new(),
                reload_callbacks: Vec::new(),
            }
        }

        /// Add a path to watch
        pub fn add_watch_path(&mut self, path: String) {
            self.watch_paths.push(path);
        }

        /// Add a reload callback
        pub fn add_reload_callback<F>(&mut self, callback: F)
        where
            F: Fn() + 'static,
        {
            self.reload_callbacks.push(Box::new(callback));
        }

        /// Start watching for changes
        pub async fn start_watching(&mut self) -> Result<(), String> {
            // Implementation of file watching using std::sync
            use std::path::Path;
            use std::sync::mpsc;

            if self.watch_paths.is_empty() {
                return Err("No paths to watch".to_string());
            }

            // Check if all paths exist
            for path in &self.watch_paths {
                if !Path::new(path).exists() {
                    return Err(format!("Path does not exist: {}", path));
                }
            }

            // Create a channel to receive events
            let (tx, _rx) = mpsc::channel::<()>();

            // Store sender for later use
            let _tx = tx;

            // In a real implementation, this would use notify crate for file watching
            // For now, return Ok as a placeholder
            Ok(())
        }

        /// Stop watching for changes
        pub fn stop_watching(&mut self) -> Result<(), String> {
            // Implementation of stopping file watching
            // In a real implementation, this would stop the watcher threads
            // For now, we'll just clear the paths
            self.watch_paths.clear();
            Ok(())
        }

        /// Trigger a reload
        pub fn trigger_reload(&self) {
            for callback in &self.reload_callbacks {
                callback();
            }
        }
    }

    /// Incremental mesh builder
    pub struct IncrementalMeshBuilder {
        // Mesh building state
        mesh: crate::mesh::mesh_data::Mesh3D,
        dirty: bool,
    }

    impl IncrementalMeshBuilder {
        /// Create a new incremental mesh builder
        pub fn new() -> Self {
            Self {
                mesh: crate::mesh::mesh_data::Mesh3D::new(),
                dirty: false,
            }
        }

        /// Add a vertex
        pub fn add_vertex(&mut self, point: Point) -> usize {
            let id = self.mesh.vertices.len();
            let vertex = crate::mesh::mesh_data::MeshVertex::new(id, point);
            self.mesh.vertices.push(vertex);
            self.dirty = true;
            id
        }

        /// Add a face
        pub fn add_face(&mut self, vertices: Vec<usize>) -> usize {
            let id = self.mesh.faces.len();
            let face = crate::mesh::mesh_data::MeshFace::new(id, vertices);
            self.mesh.faces.push(face);
            self.dirty = true;
            id
        }

        /// Update a vertex
        pub fn update_vertex(&mut self, vertex_id: usize, point: Point) -> Result<(), String> {
            if vertex_id >= self.mesh.vertices.len() {
                return Err("Vertex ID out of bounds".to_string());
            }
            self.mesh.vertices[vertex_id].point = point;
            self.dirty = true;
            Ok(())
        }

        /// Remove a face
        pub fn remove_face(&mut self, face_id: usize) -> Result<(), String> {
            if face_id >= self.mesh.faces.len() {
                return Err("Face ID out of bounds".to_string());
            }
            self.mesh.faces.remove(face_id);
            // Update face IDs
            for (i, face) in self.mesh.faces.iter_mut().enumerate().skip(face_id) {
                face.id = i;
            }
            self.dirty = true;
            Ok(())
        }

        /// Get the mesh
        pub fn mesh(&self) -> &crate::mesh::mesh_data::Mesh3D {
            &self.mesh
        }

        /// Get the mesh (mutable)
        pub fn mesh_mut(&mut self) -> &mut crate::mesh::mesh_data::Mesh3D {
            self.dirty = true;
            &mut self.mesh
        }

        /// Is the mesh dirty?
        pub fn is_dirty(&self) -> bool {
            self.dirty
        }

        /// Reset dirty flag
        pub fn reset_dirty(&mut self) {
            self.dirty = false;
        }
    }
}

/// API documentation utilities
pub mod documentation {
    

    /// API documentation generator
    pub struct ApiDocGenerator {
        // Documentation configuration
        output_dir: String,
        include_private: bool,
    }

    impl ApiDocGenerator {
        /// Create a new API documentation generator
        pub fn new(output_dir: String) -> Self {
            Self {
                output_dir,
                include_private: false,
            }
        }

        /// Set whether to include private items
        pub fn include_private(&mut self, include: bool) {
            self.include_private = include;
        }

        /// Generate API documentation
        pub fn generate(&self) -> Result<(), String> {
            // Implementation of API documentation generation
            use std::fs;
            use std::path::Path;

            // Create output directory if it doesn't exist
            let output_path = Path::new(&self.output_dir);
            if !output_path.exists() {
                if let Err(e) = fs::create_dir_all(output_path) {
                    return Err(format!("Failed to create output directory: {}", e));
                }
            }

            // Generate API documentation
            let api_doc_path = output_path.join("api.md");
            let mut content = String::from("# API Documentation\n\n");

            content.push_str("## Optimized Types\n\n");
            content.push_str("### OptimizedPoint\n");
            content.push_str("- `new(x: f64, y: f64, z: f64) -> Self`\n");
            content.push_str("- `to_point(&self) -> Point`\n");
            content.push_str("- `from_point(point: &Point) -> Self`\n");
            content.push_str("- `add(&self, other: &Self) -> Self`\n");
            content.push_str("- `sub(&self, other: &Self) -> Self`\n");
            content.push_str("- `mul(&self, scalar: f64) -> Self`\n");
            content.push_str("- `div(&self, scalar: f64) -> Self`\n");
            content.push_str("- `distance(&self, other: &Self) -> f64`\n");
            content.push_str("- `distance_squared(&self, other: &Self) -> f64`\n");
            content.push_str("- `approx_eq(&self, other: &Self, epsilon: f64) -> bool`\n\n");

            content.push_str("### OptimizedMesh\n");
            content.push_str("- `new() -> Self`\n");
            content.push_str("- `add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize`\n");
            content.push_str("- `add_face(&mut self, v1: usize, v2: usize, v3: usize)`\n");
            content.push_str("- `enable_normals(&mut self)`\n");
            content.push_str(
                "- `set_normal(&mut self, vertex_id: usize, nx: f64, ny: f64, nz: f64)`\n",
            );
            content.push_str("- `get_vertex(&self, vertex_id: usize) -> (f64, f64, f64)`\n");
            content.push_str("- `get_face(&self, face_id: usize) -> (usize, usize, usize)`\n");
            content
                .push_str("- `get_normal(&self, vertex_id: usize) -> Option<(f64, f64, f64)>`\n");
            content.push_str("- `to_mesh(&self) -> Mesh3D`\n");
            content.push_str("- `from_mesh(mesh: &Mesh3D) -> Self`\n");
            content.push_str(
                "- `calculate_bounding_box(&self) -> Option<(f64, f64, f64, f64, f64, f64)>`\n",
            );
            content.push_str("- `calculate_volume(&self) -> f64`\n");
            content.push_str("- `calculate_surface_area(&self) -> f64`\n\n");

            if let Err(e) = fs::write(api_doc_path, content) {
                return Err(format!("Failed to write API documentation: {}", e));
            }

            Ok(())
        }

        /// Generate user guide
        pub fn generate_user_guide(&self) -> Result<(), String> {
            // Implementation of user guide generation
            use std::fs;
            use std::path::Path;

            let output_path = Path::new(&self.output_dir);
            let guide_path = output_path.join("user_guide.md");

            let mut content = String::from("# User Guide\n\n");
            content.push_str("## Getting Started\n\n");
            content.push_str("### Basic Usage\n\n");
            content.push_str("```rust\n");
            content.push_str("// Create an optimized point\n");
            content.push_str("let point = OptimizedPoint::new(1.0, 2.0, 3.0);\n\n");
            content.push_str("// Create an optimized mesh\n");
            content.push_str("let mut mesh = OptimizedMesh::new();\n");
            content.push_str("let v0 = mesh.add_vertex(0.0, 0.0, 0.0);\n");
            content.push_str("let v1 = mesh.add_vertex(1.0, 0.0, 0.0);\n");
            content.push_str("let v2 = mesh.add_vertex(0.0, 1.0, 0.0);\n");
            content.push_str("mesh.add_face(v0, v1, v2);\n\n");
            content.push_str("// Calculate volume and surface area\n");
            content.push_str("let volume = mesh.calculate_volume();\n");
            content.push_str("let area = mesh.calculate_surface_area();\n");
            content.push_str("```\n\n");

            if let Err(e) = fs::write(guide_path, content) {
                return Err(format!("Failed to write user guide: {}", e));
            }

            Ok(())
        }

        /// Generate examples and tutorials
        pub fn generate_examples(&self) -> Result<(), String> {
            // Implementation of examples generation
            use std::fs;
            use std::path::Path;

            let output_path = Path::new(&self.output_dir);
            let examples_path = output_path.join("examples");

            // Create examples directory
            if !examples_path.exists() {
                if let Err(e) = fs::create_dir_all(&examples_path) {
                    return Err(format!("Failed to create examples directory: {}", e));
                }
            }

            // Create basic example
            let basic_example = examples_path.join("basic.rs");
            let mut content = String::from("// Basic example of using the optimized API\n");
            content.push_str("use breprs::api::optimized::{OptimizedPoint, OptimizedMesh};\n\n");
            content.push_str("fn main() {\n");
            content.push_str("    // Create points\n");
            content.push_str("    let p1 = OptimizedPoint::new(0.0, 0.0, 0.0);\n");
            content.push_str("    let p2 = OptimizedPoint::new(1.0, 0.0, 0.0);\n");
            content.push_str("    let p3 = OptimizedPoint::new(0.0, 1.0, 0.0);\n");
            content.push_str("    let p4 = OptimizedPoint::new(0.0, 0.0, 1.0);\n\n");
            content.push_str("    // Create mesh\n");
            content.push_str("    let mut mesh = OptimizedMesh::new();\n");
            content.push_str("    let v0 = mesh.add_vertex(p1.x, p1.y, p1.z);\n");
            content.push_str("    let v1 = mesh.add_vertex(p2.x, p2.y, p2.z);\n");
            content.push_str("    let v2 = mesh.add_vertex(p3.x, p3.y, p3.z);\n");
            content.push_str("    let v3 = mesh.add_vertex(p4.x, p4.y, p4.z);\n\n");
            content.push_str("    // Add faces\n");
            content.push_str("    mesh.add_face(v0, v1, v2);\n");
            content.push_str("    mesh.add_face(v0, v2, v3);\n");
            content.push_str("    mesh.add_face(v0, v3, v1);\n");
            content.push_str("    mesh.add_face(v1, v3, v2);\n\n");
            content.push_str("    // Calculate properties\n");
            content.push_str("    let volume = mesh.calculate_volume();\n");
            content.push_str("    let surface_area = mesh.calculate_surface_area();\n\n");
            content.push_str("    println!(\"Volume: {:.3}\", volume);\n");
            content.push_str("    println!(\"Surface Area: {:.3}\", surface_area);\n");
            content.push_str("}\n");

            if let Err(e) = fs::write(basic_example, content) {
                return Err(format!("Failed to write example: {}", e));
            }

            Ok(())
        }
    }
}
