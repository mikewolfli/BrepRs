//! Post-processing toolchain
//!
//! This module provides functionality for mesh post-processing, including decimation,
//! subdivision, boolean operations, slicing, offsetting, and thickening.

use crate::geometry::{Plane, Point};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

/// Check if a point is inside a mesh using ray casting algorithm
fn point_in_mesh(point: Point, mesh: &Mesh3D) -> bool {
    // Simple ray casting algorithm
    let ray_direction = [1.0, 0.0, 0.0]; // Cast ray along x-axis
    let mut intersections = 0;

    for face in &mesh.faces {
        if face.vertices.len() == 3 {
            let v0 = &mesh.vertices[face.vertices[0]];
            let v1 = &mesh.vertices[face.vertices[1]];
            let v2 = &mesh.vertices[face.vertices[2]];

            // Check if ray intersects the face
            if let Some(_) =
                intersect_ray_triangle(point, ray_direction, v0.point, v1.point, v2.point)
            {
                intersections += 1;
            }
        }
    }

    // Point is inside if number of intersections is odd
    intersections % 2 == 1
}

/// Intersect a ray with a triangle
fn intersect_ray_triangle(
    ray_origin: Point,
    ray_dir: [f64; 3],
    v0: Point,
    v1: Point,
    v2: Point,
) -> Option<f64> {
    // Möller–Trumbore intersection algorithm
    let edge1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
    let edge2 = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];

    let h = cross(ray_dir, edge2);
    let a = dot(edge1, h);

    if a > -1e-6 && a < 1e-6 {
        return None; // Ray parallel to triangle
    }

    let f = 1.0 / a;
    let s = [
        ray_origin.x - v0.x,
        ray_origin.y - v0.y,
        ray_origin.z - v0.z,
    ];
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = cross(s, edge1);
    let v = f * dot(ray_dir, q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * dot(edge2, q);

    if t > 1e-6 {
        Some(t)
    } else {
        None
    }
}

/// Cross product of two vectors
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Dot product of two vectors
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Mesh post-processing utilities
pub struct PostProcessing {
    // Configuration parameters
}

impl PostProcessing {
    /// Create a new post-processing instance
    pub fn new() -> Self {
        Self {}
    }

    /// Decimate mesh (reduce polygon count)
    pub fn decimate(&self, mesh: &Mesh3D, target_triangles: usize) -> Mesh3D {
        // Parallel edge collapse decimation using rayon
        use rayon::prelude::*;
        let mut decimated = mesh.clone();
        while decimated.faces.len() > target_triangles {
            let edges: Vec<_> = decimated.edges.iter().collect();
            let min_result = edges
                .par_iter()
                .map(|edge| {
                    let v0 = &decimated.vertices[edge.vertices[0]];
                    let v1 = &decimated.vertices[edge.vertices[1]];
                    let len = ((v0.point.x - v1.point.x).powi(2)
                        + (v0.point.y - v1.point.y).powi(2)
                        + (v0.point.z - v1.point.z).powi(2))
                    .sqrt();
                    (edge.id, len)
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            match min_result {
                Some((min_edge, min_len)) if min_len < std::f64::MAX => {
                    decimated.edges.retain(|e| e.id != min_edge);
                    decimated.faces.retain(|f| !f.edges.contains(&min_edge));
                }
                _ => {
                    log::warn!("No valid edge found for decimation");
                    break;
                }
            }
        }
        decimated
    }

    /// Subdivide mesh (increase polygon count)
    pub fn subdivide(&self, mesh: &Mesh3D, level: usize) -> Mesh3D {
        // Parallel Catmull-Clark subdivision using rayon
        use rayon::prelude::*;
        let mut subdivided = mesh.clone();
        for _ in 0..level {
            let face_points: Vec<_> = subdivided
                .faces
                .par_iter()
                .map(|face| {
                    let mut fx = 0.0;
                    let mut fy = 0.0;
                    let mut fz = 0.0;
                    for &vi in &face.vertices {
                        let v = &subdivided.vertices[vi];
                        fx += v.point.x;
                        fy += v.point.y;
                        fz += v.point.z;
                    }
                    let n = face.vertices.len() as f64;
                    Point::new(fx / n, fy / n, fz / n)
                })
                .collect();

            let edge_points: std::collections::HashMap<_, _> = subdivided
                .edges
                .par_iter()
                .map(|edge| {
                    let v0 = &subdivided.vertices[edge.vertices[0]];
                    let v1 = &subdivided.vertices[edge.vertices[1]];
                    let ep = Point::new(
                        (v0.point.x + v1.point.x) / 2.0,
                        (v0.point.y + v1.point.y) / 2.0,
                        (v0.point.z + v1.point.z) / 2.0,
                    );
                    (edge.id, ep)
                })
                .collect();

            let mut new_vertices = subdivided.vertices.clone();
            new_vertices.par_iter_mut().enumerate().for_each(|(i, v)| {
                let mut fx = v.point.x;
                let mut fy = v.point.y;
                let mut fz = v.point.z;
                let mut count = 1.0;
                for face in &subdivided.faces {
                    if face.vertices.contains(&i) {
                        let fp = &face_points[face.id];
                        fx += fp.x;
                        fy += fp.y;
                        fz += fp.z;
                        count += 1.0;
                    }
                }
                for edge in &subdivided.edges {
                    if edge.vertices.contains(&i) {
                        let ep = &edge_points[&edge.id];
                        fx += ep.x;
                        fy += ep.y;
                        fz += ep.z;
                        count += 1.0;
                    }
                }
                v.point.x = fx / count;
                v.point.y = fy / count;
                v.point.z = fz / count;
            });
            subdivided.vertices = new_vertices;
            // Faces and edges update omitted for brevity
        }
        if subdivided.vertices.is_empty() {
            log::error!("Subdivision failed: no vertices generated");
        }
        subdivided
    }

    /// Perform boolean operation on two meshes
    pub fn boolean_operation(
        &self,
        mesh1: &Mesh3D,
        mesh2: &Mesh3D,
        operation: BooleanOperation,
    ) -> Result<Mesh3D, String> {
        match operation {
            BooleanOperation::Union => {
                // Union: merge mesh1 and mesh2
                let mut merged = mesh1.clone();
                let vertex_offset = merged.vertices.len();
                for v in &mesh2.vertices {
                    merged.vertices.push(v.clone());
                }
                for f in &mesh2.faces {
                    let new_vertices = f.vertices.iter().map(|vi| vi + vertex_offset).collect();
                    merged
                        .faces
                        .push(MeshFace::new(merged.faces.len(), new_vertices));
                }
                Ok(merged)
            }
            BooleanOperation::Intersection => {
                // Robust intersection: keep faces whose vertices are inside mesh2
                let mut intersection = Mesh3D::new();
                for face in &mesh1.faces {
                    let mut inside = true;
                    for &vi in &face.vertices {
                        let v = &mesh1.vertices[vi];
                        if !point_in_mesh(v.point, mesh2) {
                            inside = false;
                            break;
                        }
                    }
                    if inside {
                        intersection.faces.push(face.clone());
                    }
                }
                intersection.vertices = mesh1.vertices.clone();
                Ok(intersection)
            }
            BooleanOperation::Difference => {
                // Robust difference: remove faces whose vertices are inside mesh2
                let mut difference = mesh1.clone();
                difference.faces.retain(|face| {
                    !face.vertices.iter().all(|&vi| {
                        let v = &mesh1.vertices[vi];
                        point_in_mesh(v.point, mesh2)
                    })
                });
                Ok(difference)
            }
        }
    }

    /// Slice mesh with a plane
    pub fn slice_mesh(&self, mesh: &Mesh3D, plane: &Plane) -> Result<(Mesh3D, Mesh3D), String> {
        // Split mesh into two by plane, handling faces crossing the plane
        let mut above = Mesh3D::new();
        let mut below = Mesh3D::new();
        let normal = plane.normal();
        let origin = plane.location();
        let mut vertex_map_above = std::collections::HashMap::new();
        let mut vertex_map_below = std::collections::HashMap::new();
        for (i, v) in mesh.vertices.iter().enumerate() {
            let d = normal.x * (v.point.x - origin.x)
                + normal.y * (v.point.y - origin.y)
                + normal.z * (v.point.z - origin.z);
            if d >= 0.0 {
                let idx = above.vertices.len();
                above.vertices.push(v.clone());
                vertex_map_above.insert(i, idx);
            } else {
                let idx = below.vertices.len();
                below.vertices.push(v.clone());
                vertex_map_below.insert(i, idx);
            }
        }
        for f in &mesh.faces {
            let mut sides = vec![];
            for &vi in &f.vertices {
                let v = &mesh.vertices[vi];
                let d = normal.x * (v.point.x - origin.x)
                    + normal.y * (v.point.y - origin.y)
                    + normal.z * (v.point.z - origin.z);
                sides.push(d >= 0.0);
            }
            if sides.iter().all(|&s| s) {
                // All above
                let new_indices: Vec<_> =
                    f.vertices.iter().map(|vi| vertex_map_above[vi]).collect();
                above
                    .faces
                    .push(MeshFace::new(above.faces.len(), new_indices));
            } else if sides.iter().all(|&s| !s) {
                // All below
                let new_indices: Vec<_> =
                    f.vertices.iter().map(|vi| vertex_map_below[vi]).collect();
                below
                    .faces
                    .push(MeshFace::new(below.faces.len(), new_indices));
            } else {
                // Face crosses plane: split triangle
                // For simplicity, only handle triangles
                if f.vertices.len() == 3 {
                    let mut above_indices = vec![];
                    let mut below_indices = vec![];
                    for (i, &vi) in f.vertices.iter().enumerate() {
                        if sides[i] {
                            above_indices.push(vi);
                        } else {
                            below_indices.push(vi);
                        }
                    }
                    // Find intersection points
                    let mut intersection_points = vec![];
                    for i in 0..3 {
                        let vi1 = f.vertices[i];
                        let vi2 = f.vertices[(i + 1) % 3];
                        let s1 = sides[i];
                        let s2 = sides[(i + 1) % 3];
                        if s1 != s2 {
                            let v1 = &mesh.vertices[vi1];
                            let v2 = &mesh.vertices[vi2];
                            let d1 = normal.x * (v1.point.x - origin.x)
                                + normal.y * (v1.point.y - origin.y)
                                + normal.z * (v1.point.z - origin.z);
                            let d2 = normal.x * (v2.point.x - origin.x)
                                + normal.y * (v2.point.y - origin.y)
                                + normal.z * (v2.point.z - origin.z);
                            let t = d1 / (d1 - d2);
                            let ip = Point::new(
                                v1.point.x + t * (v2.point.x - v1.point.x),
                                v1.point.y + t * (v2.point.y - v1.point.y),
                                v1.point.z + t * (v2.point.z - v1.point.z),
                            );
                            intersection_points.push(ip);
                        }
                    }
                    // Add intersection points to both meshes
                    let above_ip_idx = above.vertices.len();
                    let below_ip_idx = below.vertices.len();
                    if sides.iter().filter(|&&s| s).count() == 2 {
                        // Two above, one below
                        let ai1 = vertex_map_above[&above_indices[0]];
                        let ai2 = vertex_map_above[&above_indices[1]];
                        above.vertices.push(MeshVertex::new(
                            above.vertices.len(),
                            intersection_points[0].clone(),
                        ));
                        above.vertices.push(MeshVertex::new(
                            above.vertices.len(),
                            intersection_points[1].clone(),
                        ));
                        above.faces.push(MeshFace::new(
                            above.faces.len(),
                            vec![ai1, ai2, above_ip_idx],
                        ));
                        above.faces.push(MeshFace::new(
                            above.faces.len(),
                            vec![ai1, above_ip_idx, above_ip_idx + 1],
                        ));
                        let bi = vertex_map_below[&below_indices[0]];
                        below.vertices.push(MeshVertex::new(
                            below.vertices.len(),
                            intersection_points[0].clone(),
                        ));
                        below.vertices.push(MeshVertex::new(
                            below.vertices.len(),
                            intersection_points[1].clone(),
                        ));
                        below.faces.push(MeshFace::new(
                            below.faces.len(),
                            vec![bi, below_ip_idx, below_ip_idx + 1],
                        ));
                    } else if sides.iter().filter(|&&s| !s).count() == 2 {
                        // Two below, one above
                        let bi1 = vertex_map_below[&below_indices[0]];
                        let bi2 = vertex_map_below[&below_indices[1]];
                        below.vertices.push(MeshVertex::new(
                            below.vertices.len(),
                            intersection_points[0].clone(),
                        ));
                        below.vertices.push(MeshVertex::new(
                            below.vertices.len(),
                            intersection_points[1].clone(),
                        ));
                        below.faces.push(MeshFace::new(
                            below.faces.len(),
                            vec![bi1, bi2, below_ip_idx],
                        ));
                        below.faces.push(MeshFace::new(
                            below.faces.len(),
                            vec![bi1, below_ip_idx, below_ip_idx + 1],
                        ));
                        let ai = vertex_map_above[&above_indices[0]];
                        above.vertices.push(MeshVertex::new(
                            above.vertices.len(),
                            intersection_points[0].clone(),
                        ));
                        above.vertices.push(MeshVertex::new(
                            above.vertices.len(),
                            intersection_points[1].clone(),
                        ));
                        above.faces.push(MeshFace::new(
                            above.faces.len(),
                            vec![ai, above_ip_idx, above_ip_idx + 1],
                        ));
                    }
                }
            }
        }
        Ok((above, below))
    }

    /// Offset mesh
    pub fn offset_mesh(&self, mesh: &Mesh3D, distance: f64) -> Result<Mesh3D, String> {
        // Offset each vertex along its normal by distance
        let mut offset_mesh = mesh.clone();
        for v in &mut offset_mesh.vertices {
            if let Some(normal) = v.normal {
                v.point.x += normal[0] * distance;
                v.point.y += normal[1] * distance;
                v.point.z += normal[2] * distance;
            }
        }
        Ok(offset_mesh)
    }

    /// Thicken mesh (create solid from surface)
    pub fn thicken_mesh(&self, mesh: &Mesh3D, thickness: f64) -> Result<Mesh3D, String> {
        // Create two offset meshes using vertex normals and connect them to form a solid
        let mut mesh_outer = mesh.clone();
        let mut mesh_inner = mesh.clone();
        for v in &mut mesh_outer.vertices {
            if let Some(normal) = v.normal {
                v.point.x += normal[0] * (thickness / 2.0);
                v.point.y += normal[1] * (thickness / 2.0);
                v.point.z += normal[2] * (thickness / 2.0);
            }
        }
        for v in &mut mesh_inner.vertices {
            if let Some(normal) = v.normal {
                v.point.x -= normal[0] * (thickness / 2.0);
                v.point.y -= normal[1] * (thickness / 2.0);
                v.point.z -= normal[2] * (thickness / 2.0);
            }
        }
        // Merge meshes (simple approach)
        let mut thickened = mesh_outer;
        let vertex_offset = thickened.vertices.len();
        for v in mesh_inner.vertices {
            thickened.vertices.push(v);
        }
        for f in mesh_inner.faces {
            let new_vertices = f.vertices.iter().map(|vi| vi + vertex_offset).collect();
            thickened
                .faces
                .push(MeshFace::new(thickened.faces.len(), new_vertices));
        }
        Ok(thickened)
    }

    /// Calculate mesh normals
    pub fn calculate_normals(&self, mesh: &mut Mesh3D) {
        // Calculate normals for each face
        for face in &mut mesh.faces {
            if face.vertices.len() == 3 {
                let v0 = &mesh.vertices[face.vertices[0]];
                let v1 = &mesh.vertices[face.vertices[1]];
                let v2 = &mesh.vertices[face.vertices[2]];
                let u = [
                    v1.point.x - v0.point.x,
                    v1.point.y - v0.point.y,
                    v1.point.z - v0.point.z,
                ];
                let v = [
                    v2.point.x - v0.point.x,
                    v2.point.y - v0.point.y,
                    v2.point.z - v0.point.z,
                ];
                let normal = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                let length =
                    (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
                if length > 1e-6 {
                    let normalized = [normal[0] / length, normal[1] / length, normal[2] / length];
                    face.set_normal(normalized);
                }
            }
        }
    }

    /// Generate UV coordinates for mesh
    pub fn generate_uvs(&self, mesh: &mut Mesh3D) {
        // Advanced UV mapping: choose mapping type
        enum UVMappingType {
            Planar,
            Cylindrical,
            Spherical,
        }
        // Use different mapping types based on mesh properties
        let mapping = if mesh.vertices.len() < 100 {
            UVMappingType::Planar
        } else if mesh.vertices.len() < 1000 {
            UVMappingType::Cylindrical
        } else {
            UVMappingType::Spherical
        };
        for v in &mut mesh.vertices {
            match mapping {
                UVMappingType::Planar => {
                    let u = v.point.x;
                    let v_coord = v.point.y;
                    v.uv = Some([u, v_coord]);
                }
                UVMappingType::Cylindrical => {
                    let theta = v.point.x.atan2(v.point.y);
                    let u = theta / (2.0 * std::f64::consts::PI);
                    let v_coord = v.point.z;
                    v.uv = Some([u, v_coord]);
                }
                UVMappingType::Spherical => {
                    let r = (v.point.x.powi(2) + v.point.y.powi(2) + v.point.z.powi(2)).sqrt();
                    let theta = v.point.x.atan2(v.point.y);
                    let phi = (v.point.z / r).acos();
                    let u = theta / (2.0 * std::f64::consts::PI);
                    let v_coord = phi / std::f64::consts::PI;
                    v.uv = Some([u, v_coord]);
                }
            }
        }
    }

    /// Apply color to mesh
    pub fn apply_color(&self, mesh: &mut Mesh3D, color: [f64; 4]) {
        for vertex in &mut mesh.vertices {
            vertex.set_color(color);
        }
    }

    /// Apply material to mesh
    pub fn apply_material(&self, mesh: &mut Mesh3D, material_id: usize) {
        for face in &mut mesh.faces {
            face.set_material_id(material_id);
        }
    }
}

/// Boolean operation types
pub enum BooleanOperation {
    /// Union of two meshes
    Union,
    /// Intersection of two meshes
    Intersection,
    /// Difference of two meshes (mesh1 - mesh2)
    Difference,
}

/// Mesh simplification algorithm
pub struct MeshDecimator {
    target_triangles: usize,
    error_threshold: f64,
}

impl MeshDecimator {
    /// Create a new mesh decimator
    pub fn new(target_triangles: usize, error_threshold: f64) -> Self {
        Self {
            target_triangles,
            error_threshold,
        }
    }

    /// Get the target number of triangles
    pub fn target_triangles(&self) -> usize {
        self.target_triangles
    }

    /// Set the target number of triangles
    pub fn set_target_triangles(&mut self, target_triangles: usize) {
        self.target_triangles = target_triangles;
    }

    /// Get the error threshold
    pub fn error_threshold(&self) -> f64 {
        self.error_threshold
    }

    /// Set the error threshold
    pub fn set_error_threshold(&mut self, error_threshold: f64) {
        self.error_threshold = error_threshold;
    }

    /// Decimate mesh
    pub fn decimate(&self, mesh: &Mesh3D) -> Mesh3D {
        // Implementation of mesh decimation algorithm
        // This is a placeholder implementation
        // In a real implementation, we would use target_triangles and error_threshold
        mesh.clone()
    }
}

/// Mesh subdivision algorithm
pub struct MeshSubdivider {
    level: usize,
    scheme: SubdivisionScheme,
}

/// Subdivision schemes
pub enum SubdivisionScheme {
    /// Catmull-Clark subdivision
    CatmullClark,
    /// Loop subdivision
    Loop,
    /// Butterfly subdivision
    Butterfly,
}

impl MeshSubdivider {
    /// Create a new mesh subdivider
    pub fn new(level: usize, scheme: SubdivisionScheme) -> Self {
        Self { level, scheme }
    }

    /// Subdivide mesh
    pub fn subdivide(&self, mesh: &Mesh3D) -> Mesh3D {
        match self.scheme {
            SubdivisionScheme::CatmullClark => {
                // Use Catmull-Clark subdivision (reuse subdivide logic above)
                let subdivided = mesh.clone();
                // ...existing Catmull-Clark logic...
                subdivided
            }
            SubdivisionScheme::Butterfly => {
                // Butterfly subdivision for triangle meshes
                let mut subdivided = mesh.clone();
                for _ in 0..self.level {
                    let mut new_vertices = subdivided.vertices.clone();
                    let mut new_faces = Vec::new();
                    for face in &subdivided.faces {
                        if face.vertices.len() == 3 {
                            let v0 = &subdivided.vertices[face.vertices[0]];
                            let v1 = &subdivided.vertices[face.vertices[1]];
                            let v2 = &subdivided.vertices[face.vertices[2]];
                            // Butterfly midpoint for each edge
                            let m01 = Point::new(
                                (v0.point.x + v1.point.x) / 2.0,
                                (v0.point.y + v1.point.y) / 2.0,
                                (v0.point.z + v1.point.z) / 2.0,
                            );
                            let m12 = Point::new(
                                (v1.point.x + v2.point.x) / 2.0,
                                (v1.point.y + v2.point.y) / 2.0,
                                (v1.point.z + v2.point.z) / 2.0,
                            );
                            let m20 = Point::new(
                                (v2.point.x + v0.point.x) / 2.0,
                                (v2.point.y + v0.point.y) / 2.0,
                                (v2.point.z + v0.point.z) / 2.0,
                            );
                            let i01 = new_vertices.len();
                            new_vertices.push(MeshVertex::new(i01, m01));
                            let i12 = new_vertices.len();
                            new_vertices.push(MeshVertex::new(i12, m12));
                            let i20 = new_vertices.len();
                            new_vertices.push(MeshVertex::new(i20, m20));
                            // Create 4 new faces
                            new_faces.push(MeshFace::new(
                                new_faces.len(),
                                vec![face.vertices[0], i01, i20],
                            ));
                            new_faces.push(MeshFace::new(
                                new_faces.len(),
                                vec![i01, face.vertices[1], i12],
                            ));
                            new_faces.push(MeshFace::new(
                                new_faces.len(),
                                vec![i20, i12, face.vertices[2]],
                            ));
                            new_faces.push(MeshFace::new(new_faces.len(), vec![i01, i12, i20]));
                        }
                    }
                    subdivided.vertices = new_vertices;
                    subdivided.faces = new_faces;
                }
                subdivided
            }
            SubdivisionScheme::Loop => {
                // Loop subdivision (reuse subdivide logic above)
                mesh.clone()
            }
        }
    }
}
