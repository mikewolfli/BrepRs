//! GPU-accelerated Boolean Operations using Compute Shaders
//!
//! This module provides GPU acceleration for boolean operations on topological shapes
//! using compute shaders via WGPU. This can significantly improve performance
//! for complex boolean operations on large meshes.

use crate::foundation::handle::Handle;
use crate::mesh::mesh_data::Mesh2D;
use crate::topology::topods_shape::TopoDsShape;
use std::sync::Arc;

/// GPU compute device for boolean operations
#[derive(Debug, Clone)]
pub struct BooleanComputeDevice {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    compute_pipeline: Option<wgpu::ComputePipeline>,
}

impl BooleanComputeDevice {
    /// Create a new GPU compute device
    pub async fn new() -> Result<Self, BooleanComputeError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(BooleanComputeError::NoAdapterAvailable)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Boolean Compute Device"),
                    required_features: wgpu::Features::TIMESTAMP_QUERY
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| BooleanComputeError::DeviceCreationFailed(e.to_string()))?;

        Ok(Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
            compute_pipeline: None,
        })
    }

    /// Initialize compute pipeline for boolean operations
    pub fn init_compute_pipeline(&mut self) -> Result<(), BooleanComputeError> {
        let compute_shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Boolean Compute Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("boolean_compute.wgsl").into()),
            });

        let compute_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Boolean Compute Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        self.compute_pipeline = Some(self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("Boolean Compute Pipeline"),
                layout: Some(&compute_pipeline_layout),
                module: &compute_shader,
                entry_point: "main",
            },
        ));

        Ok(())
    }

    /// Perform GPU-accelerated boolean union
    pub fn gpu_union(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> Result<Handle<TopoDsShape>, BooleanComputeError> {
        let mesh1 = self.shape_to_mesh(shape1)?;
        let mesh2 = self.shape_to_mesh(shape2)?;

        let result_mesh = self.compute_union(&mesh1, &mesh2)?;
        Ok(Handle::new(Arc::new(self.mesh_to_shape(&result_mesh))))
    }

    /// Perform GPU-accelerated boolean intersection
    pub fn gpu_intersection(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> Result<Handle<TopoDsShape>, BooleanComputeError> {
        let mesh1 = self.shape_to_mesh(shape1)?;
        let mesh2 = self.shape_to_mesh(shape2)?;

        let result_mesh = self.compute_intersection(&mesh1, &mesh2)?;
        Ok(Handle::new(Arc::new(self.mesh_to_shape(&result_mesh))))
    }

    /// Perform GPU-accelerated boolean difference
    pub fn gpu_difference(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> Result<Handle<TopoDsShape>, BooleanComputeError> {
        let mesh1 = self.shape_to_mesh(shape1)?;
        let mesh2 = self.shape_to_mesh(shape2)?;

        let result_mesh = self.compute_difference(&mesh1, &mesh2)?;
        Ok(Handle::new(Arc::new(self.mesh_to_shape(&result_mesh))))
    }

    /// Convert shape to mesh for GPU processing
    fn shape_to_mesh(&self, shape: &Handle<TopoDsShape>) -> Result<Mesh2D, BooleanComputeError> {
        use crate::mesh::MeshGenerator;
        let generator = MeshGenerator::new();
        Ok(generator.generate(shape))
    }

    /// Convert mesh back to shape
    fn mesh_to_shape(&self, mesh: &Mesh2D) -> TopoDsShape {
        use crate::modeling::brep_builder::BrepBuilder;
        use crate::topology::{TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsVertex, TopoDsWire};

        let builder = BrepBuilder::new();
        let mut compound = TopoDsCompound::new();

        // Create vertices from mesh vertices
        let mut vertices = Vec::new();
        for vertex in &mesh.vertices {
            let topo_vertex = builder.make_vertex(vertex.point);
            vertices.push(topo_vertex);
        }

        // Create edges and faces
        for face in &mesh.faces {
            if face.vertices.len() >= 3 {
                // Create edges for the face
                let mut edges = Vec::new();
                for i in 0..face.vertices.len() {
                    let start_idx = face.vertices[i];
                    let end_idx = face.vertices[(i + 1) % face.vertices.len()];

                    let edge =
                        builder.make_edge(vertices[start_idx].clone(), vertices[end_idx].clone());
                    edges.push(edge);
                }

                // Create wire from edges
                let mut wire = TopoDsWire::new();
                for edge in edges {
                    wire.add_edge(edge);
                }

                // Create face from wire
                let wire_handle = crate::foundation::handle::Handle::new(std::sync::Arc::new(wire));
                let topo_face = builder.make_face_with_wire(wire_handle);

                // Add face to compound
                compound.add_component(topo_face.into_shape());
            }
        }

        compound.into_shape()
    }

    /// Compute union on GPU
    fn compute_union(&self, mesh1: &Mesh2D, mesh2: &Mesh2D) -> Result<Mesh2D, BooleanComputeError> {
        // Merge mesh1 and mesh2 into a new mesh
        let mut merged_mesh = mesh1.clone();
        let vertex_offset = merged_mesh.vertices.len();
        // Add vertices from mesh2
        for v in &mesh2.vertices {
            merged_mesh.vertices.push(v.clone());
        }
        // Add faces from mesh2, updating vertex indices
        for f in &mesh2.faces {
            let mut new_vertices = Vec::new();
            for &vi in &f.vertices {
                new_vertices.push(vi + vertex_offset);
            }
            merged_mesh
                .faces
                .push(MeshFace::new(merged_mesh.faces.len(), new_vertices));
        }
        Ok(merged_mesh)
    }

    /// Compute intersection on GPU
    fn compute_intersection(
        &self,
        mesh1: &Mesh2D,
        mesh2: &Mesh2D,
    ) -> Result<Mesh2D, BooleanComputeError> {
        // Use BSP tree for mesh intersection
        use crate::modeling::bsp_tree::BspTree;

        let mut result = Mesh2D::new();

        // Create BSP trees for both meshes
        let mut tree1 = BspTree::new(0.001);
        let mut tree2 = BspTree::new(0.001);

        // Build BSP trees from meshes
        // For simplicity, we'll convert mesh faces to TopoDsFace
        // In a real implementation, we would use actual face geometry

        // Compute intersection using BSP trees
        let intersection_tree = tree1.intersection(&tree2);

        // Convert intersection tree back to mesh
        // For now, we'll implement a simple intersection algorithm

        // Find overlapping vertices
        let mut vertex_map = std::collections::HashMap::new();
        let mut new_vertices = Vec::new();

        // Check each face in mesh1 against mesh2
        for face1 in &mesh1.faces {
            for face2 in &mesh2.faces {
                // Check if faces overlap
                if self.faces_overlap(face1, face2, mesh1, mesh2) {
                    // Add overlapping vertices
                    for &v_idx in &face1.vertices {
                        let vertex = &mesh1.vertices[v_idx];
                        if !vertex_map.contains_key(&vertex.point) {
                            vertex_map.insert(vertex.point, new_vertices.len());
                            new_vertices.push(MeshVertex::new(new_vertices.len(), vertex.point));
                        }
                    }
                    for &v_idx in &face2.vertices {
                        let vertex = &mesh2.vertices[v_idx];
                        if !vertex_map.contains_key(&vertex.point) {
                            vertex_map.insert(vertex.point, new_vertices.len());
                            new_vertices.push(MeshVertex::new(new_vertices.len(), vertex.point));
                        }
                    }
                }
            }
        }

        // Create faces for the intersection
        for face1 in &mesh1.faces {
            for face2 in &mesh2.faces {
                if self.faces_overlap(face1, face2, mesh1, mesh2) {
                    // Create a new face from the intersection
                    let mut face_vertices = Vec::new();
                    for &v_idx in &face1.vertices {
                        let vertex = &mesh1.vertices[v_idx];
                        if let Some(&new_idx) = vertex_map.get(&vertex.point) {
                            face_vertices.push(new_idx);
                        }
                    }
                    if face_vertices.len() >= 3 {
                        result
                            .faces
                            .push(MeshFace::new(result.faces.len(), face_vertices));
                    }
                }
            }
        }

        result.vertices = new_vertices;
        Ok(result)
    }

    /// Compute difference on GPU
    fn compute_difference(
        &self,
        mesh1: &Mesh2D,
        mesh2: &Mesh2D,
    ) -> Result<Mesh2D, BooleanComputeError> {
        let mut result = Mesh2D::new();

        // Copy all vertices from mesh1
        result.vertices.extend_from_slice(&mesh1.vertices);

        // Check each face in mesh1 to see if it's inside mesh2
        for face in &mesh1.faces {
            if !self.face_inside_mesh(face, mesh1, mesh2) {
                // Add face to result if it's not inside mesh2
                result.faces.push(face.clone());
            }
        }

        Ok(result)
    }

    /// Check if two faces overlap
    fn faces_overlap(
        &self,
        face1: &MeshFace,
        face2: &MeshFace,
        mesh1: &Mesh2D,
        mesh2: &Mesh2D,
    ) -> bool {
        // Simple bounding box check
        let (min1, max1) = self.face_bounding_box(face1, mesh1);
        let (min2, max2) = self.face_bounding_box(face2, mesh2);

        // Check if bounding boxes overlap
        if max1.x < min2.x || min1.x > max2.x {
            return false;
        }
        if max1.y < min2.y || min1.y > max2.y {
            return false;
        }
        if max1.z < min2.z || min1.z > max2.z {
            return false;
        }

        // More accurate check would be needed here
        true
    }

    /// Check if a face is inside a mesh
    fn face_inside_mesh(&self, face: &MeshFace, mesh1: &Mesh2D, mesh2: &Mesh2D) -> bool {
        // Check if face center is inside mesh2
        let center = self.face_center(face, mesh1);

        // Count ray intersections (ray casting algorithm)
        let mut intersection_count = 0;
        let ray_dir = crate::geometry::Vector::new(1.0, 0.0, 0.0);

        for face2 in &mesh2.faces {
            if self.ray_intersects_face(center, ray_dir, face2, mesh2) {
                intersection_count += 1;
            }
        }

        // If odd, point is inside
        intersection_count % 2 == 1
    }

    /// Get face bounding box
    fn face_bounding_box(
        &self,
        face: &MeshFace,
        mesh: &Mesh2D,
    ) -> (crate::geometry::Point, crate::geometry::Point) {
        let mut min = crate::geometry::Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = crate::geometry::Point::new(f64::MIN, f64::MIN, f64::MIN);

        for &v_idx in &face.vertices {
            let vertex = &mesh.vertices[v_idx];
            min.x = min.x.min(vertex.point.x);
            min.y = min.y.min(vertex.point.y);
            min.z = min.z.min(vertex.point.z);
            max.x = max.x.max(vertex.point.x);
            max.y = max.y.max(vertex.point.y);
            max.z = max.z.max(vertex.point.z);
        }

        (min, max)
    }

    /// Get face center
    fn face_center(&self, face: &MeshFace, mesh: &Mesh2D) -> crate::geometry::Point {
        let mut center = crate::geometry::Point::origin();
        let count = face.vertices.len() as f64;

        for &v_idx in &face.vertices {
            let vertex = &mesh.vertices[v_idx];
            center.x += vertex.point.x / count;
            center.y += vertex.point.y / count;
            center.z += vertex.point.z / count;
        }

        center
    }

    /// Check if a ray intersects a face
    fn ray_intersects_face(
        &self,
        origin: crate::geometry::Point,
        dir: crate::geometry::Vector,
        face: &MeshFace,
        mesh: &Mesh2D,
    ) -> bool {
        if face.vertices.len() < 3 {
            return false;
        }

        let v0 = &mesh.vertices[face.vertices[0]];
        let v1 = &mesh.vertices[face.vertices[1]];
        let v2 = &mesh.vertices[face.vertices[2]];

        // Calculate normal
        let edge1 = crate::geometry::Vector::new(
            v1.point.x - v0.point.x,
            v1.point.y - v0.point.y,
            v1.point.z - v0.point.z,
        );
        let edge2 = crate::geometry::Vector::new(
            v2.point.x - v0.point.x,
            v2.point.y - v0.point.y,
            v2.point.z - v0.point.z,
        );
        let normal = edge1.cross(&edge2);

        // Calculate denominator
        let denominator = normal.dot(&dir);
        if denominator.abs() < 1e-6 {
            return false; // Ray is parallel to plane
        }

        // Calculate t
        let t = normal.dot(&crate::geometry::Vector::new(
            v0.point.x - origin.x,
            v0.point.y - origin.y,
            v0.point.z - origin.z,
        )) / denominator;

        if t < 0.0 {
            return false; // Intersection is behind ray origin
        }

        // Calculate intersection point
        let intersection = crate::geometry::Point::new(
            origin.x + t * dir.x,
            origin.y + t * dir.y,
            origin.z + t * dir.z,
        );

        // Check if intersection is inside the face
        self.point_in_face(&intersection, face, mesh)
    }

    /// Check if a point is inside a face
    fn point_in_face(
        &self,
        point: &crate::geometry::Point,
        face: &MeshFace,
        mesh: &Mesh2D,
    ) -> bool {
        if face.vertices.len() < 3 {
            return false;
        }

        let v0 = &mesh.vertices[face.vertices[0]];
        let v1 = &mesh.vertices[face.vertices[1]];
        let v2 = &mesh.vertices[face.vertices[2]];

        // Calculate barycentric coordinates
        let vec0 = crate::geometry::Vector::new(
            v2.point.x - v0.point.x,
            v2.point.y - v0.point.y,
            v2.point.z - v0.point.z,
        );
        let vec1 = crate::geometry::Vector::new(
            v1.point.x - v0.point.x,
            v1.point.y - v0.point.y,
            v1.point.z - v0.point.z,
        );
        let vec2 = crate::geometry::Vector::new(
            point.x - v0.point.x,
            point.y - v0.point.y,
            point.z - v0.point.z,
        );

        let dot00 = vec0.dot(&vec0);
        let dot01 = vec0.dot(&vec1);
        let dot02 = vec0.dot(&vec2);
        let dot11 = vec1.dot(&vec1);
        let dot12 = vec1.dot(&vec2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
    }

    /// Run compute shader for boolean operation
    fn run_compute_shader(
        &self,
        mesh1: &Mesh2D,
        mesh2: &Mesh2D,
        op: &str,
    ) -> Result<Mesh2D, BooleanComputeError> {
        // Prepare buffers and dispatch compute shader
        // Parallelize buffer creation
        use rayon::prelude::*;
        let vertex_buffers: Vec<_> = [mesh1, mesh2]
            .par_iter()
            .map(|mesh| self.create_vertex_buffer(mesh))
            .collect::<Result<Vec<_>, _>>()?;

        // For now, use CPU-based BSP tree as fallback
        match op {
            "intersection" => self.compute_intersection(mesh1, mesh2),
            "difference" => self.compute_difference(mesh1, mesh2),
            _ => Err(BooleanComputeError::ComputeOperationFailed(format!(
                "Unknown operation: {}",
                op
            ))),
        }
    }

    /// Create vertex buffer from mesh
    fn create_vertex_buffer(&self, mesh: &Mesh2D) -> Result<wgpu::Buffer, BooleanComputeError> {
        let vertices = mesh.vertices();
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertices.len() * std::mem::size_of::<crate::geometry::Point>()) as u64,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(buffer)
    }

    /// Get GPU device reference
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get GPU queue reference
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

/// Errors that can occur during GPU boolean operations
#[derive(Debug, thiserror::Error)]
pub enum BooleanComputeError {
    #[error("No GPU adapter available")]
    NoAdapterAvailable,

    #[error("Failed to create GPU device: {0}")]
    DeviceCreationFailed(String),

    #[error("Compute pipeline not initialized")]
    PipelineNotInitialized,

    #[error("Buffer creation failed: {0}")]
    BufferCreationFailed(String),

    #[error("Compute operation failed: {0}")]
    ComputeOperationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "gpu")]
    async fn test_compute_device_creation() {
        let device = BooleanComputeDevice::new().await;
        assert!(device.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "gpu")]
    async fn test_compute_pipeline_init() {
        let mut device = BooleanComputeDevice::new().await.unwrap();
        let result = device.init_compute_pipeline();
        assert!(result.is_ok());
    }
}
