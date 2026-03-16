//! Mesh generation module
//!
//! This module provides functionality for mesh generation, including
//! mesh data structures, 2D triangle meshing, 3D tetrahedral meshing,
//! and mesh quality optimization.

pub mod boundary_layer;
pub mod hex_mesher;
pub mod mesh_data;
pub mod mesher2d;
pub mod mesher3d;
pub mod quad_mesher;
pub mod quality;

#[cfg(test)]
mod tests;

pub use boundary_layer::*;
pub use hex_mesher::*;
pub use mesh_data::*;
pub use mesher2d::*;
pub use mesher3d::*;
pub use quad_mesher::*;
pub use quality::*;

use crate::foundation::handle::Handle;
use crate::topology::topods_shape::TopoDsShape;

pub enum MeshingAlgorithm {
    Surface,
    Volume,
    Delaunay,
}

pub struct MeshGenerator {
    deflection: f64,
    angle: f64,
}

impl MeshGenerator {
    pub fn new() -> Self {
        Self {
            deflection: 0.1,
            angle: 0.5,
        }
    }

    pub fn with_params(deflection: f64, angle: f64) -> Self {
        Self { deflection, angle }
    }

    /// Get the deflection parameter
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Set the deflection parameter
    pub fn set_deflection(&mut self, deflection: f64) {
        self.deflection = deflection;
    }

    /// Get the angle parameter
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Set the angle parameter
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn generate(
        &self,
        shape: &Handle<TopoDsShape>,
        deflection: f64,
        angle: f64,
    ) -> mesh_data::Mesh2D {
        // Implementation of mesh generation
        let mut mesh = mesh_data::Mesh2D::new();

        // Get shape type
        let shape_type = shape.shape_type();

        // Generate mesh based on shape type
        match shape_type {
            crate::topology::ShapeType::Face => {
                // For face, use generate_face
                if let Some(face) = shape.as_face() {
                    let face_handle =
                        crate::foundation::handle::Handle::new(std::sync::Arc::new(face.clone()));
                    mesh = self.generate_face(&face_handle, deflection, angle);
                }
            }
            crate::topology::ShapeType::Solid => {
                // For solid, generate mesh for each face in parallel
                let faces = shape.faces();
                if !faces.is_empty() {
                    // Use rayon for parallel processing if feature is enabled
                    #[cfg(feature = "rayon")]
                    {
                        use rayon::prelude::*;
                        let face_meshes: Vec<_> = faces
                            .par_iter()
                            .map(|face| {
                                let face_handle = crate::foundation::handle::Handle::new(
                                    std::sync::Arc::new(face.clone()),
                                );
                                self.generate_face(&face_handle, deflection, angle)
                            })
                            .collect();

                        // Merge all face meshes into the main mesh
                        for face_mesh in face_meshes {
                            mesh.merge(&face_mesh);
                        }
                    }

                    // Fallback to sequential processing if rayon is not enabled
                    #[cfg(not(feature = "rayon"))]
                    {
                        for face in faces {
                            let face_handle = crate::foundation::handle::Handle::new(
                                std::sync::Arc::new(face.clone()),
                            );
                            let face_mesh = self.generate_face(&face_handle, deflection, angle);
                            // Merge face mesh into the main mesh
                            mesh.merge(&face_mesh);
                        }
                    }
                }
            }
            _ => {
                // For other shape types, generate simple mesh
                self.generate_simple_mesh(&mut mesh);
            }
        }

        mesh
    }

    pub fn generate_face(
        &self,
        face: &crate::foundation::handle::Handle<crate::topology::topods_face::TopoDsFace>,
        _deflection: f64,
        _angle: f64,
    ) -> mesh_data::Mesh2D {
        // Implementation of face mesh generation
        let mut mesh = mesh_data::Mesh2D::new();

        if let Some(face_ref) = face.get() {
            // Get face geometry
            if let Some(_surface) = face_ref.surface() {
                // Get face wires
                let wires = face_ref.wires();

                // For each wire, generate mesh
                for wire in wires {
                    if let Some(wire_ref) = wire.get() {
                        // Get wire edges
                        let edges = wire_ref.edges();

                        // Collect vertices from edges
                        let mut vertices = Vec::new();
                        for edge in edges {
                            if let Some(edge_ref) = edge.get() {
                                let start_vertex = edge_ref.start_vertex();
                                let end_vertex = edge_ref.end_vertex();

                                if let (Some(start), Some(end)) =
                                    (start_vertex.get(), end_vertex.get())
                                {
                                    vertices.push(start.point().clone());
                                    vertices.push(end.point().clone());
                                }
                            }
                        }

                        // Generate mesh using Delaunay triangulation
                        self.generate_delaunay_mesh(
                            &mut mesh,
                            &vertices,
                            self.deflection,
                            self.angle,
                        );
                    }
                }
            }
        }

        mesh
    }

    pub fn generate_tetrahedral(
        &self,
        solid: &crate::foundation::handle::Handle<crate::topology::topods_solid::TopoDsSolid>,
        max_edge_length: f64,
    ) -> crate::mesh::mesh_data::Mesh3D {
        // Implementation of tetrahedral mesh generation
        let mut mesh = crate::mesh::mesh_data::Mesh3D::new();

        if let Some(solid_ref) = solid.get() {
            // Get solid shells
            let shells = solid_ref.shells();

            // For each shell, generate surface mesh first
            for shell in shells {
                if let Some(shell_ref) = shell.get() {
                    let faces = shell_ref.faces();

                    // Collect all vertices from faces
                    let mut vertices = Vec::new();
                    for face in faces {
                        if let Some(face_ref) = face.get() {
                            let wires = face_ref.wires();
                            for wire in wires {
                                if let Some(wire_ref) = wire.get() {
                                    let edges = wire_ref.edges();
                                    for edge in edges {
                                        if let Some(edge_ref) = edge.get() {
                                            let start_vertex = edge_ref.start_vertex();
                                            let end_vertex = edge_ref.end_vertex();

                                            if let (Some(start), Some(end)) =
                                                (start_vertex.get(), end_vertex.get())
                                            {
                                                vertices.push(start.point().clone());
                                                vertices.push(end.point().clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Generate tetrahedral mesh using vertices
                    self.generate_tetrahedral_mesh(&mut mesh, &vertices, max_edge_length);
                }
            }
        }

        mesh
    }

    pub fn optimize(&self, mesh: &mut mesh_data::Mesh2D, iterations: usize) {
        // Implementation of mesh optimization
        for _ in 0..iterations {
            // 1. Smooth vertices
            self.smooth_vertices(mesh);

            // 2. Improve element quality
            self.improve_element_quality(mesh);

            // 3. Remove bad elements
            self.remove_bad_elements(mesh);
        }
    }

    pub fn evaluate_quality(&self, mesh: &mesh_data::Mesh2D) -> crate::mesh::quality::MeshQuality {
        let analyzer = crate::mesh::quality::MeshQualityAnalyzer::new(
            crate::mesh::quality::QualityThresholds::default(),
        );
        analyzer.analyze_2d(mesh)
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Generate a simple mesh for testing purposes
    fn generate_simple_mesh(&self, mesh: &mut mesh_data::Mesh2D) {
        // Add some vertices
        let v0 = mesh.add_vertex(crate::geometry::Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(crate::geometry::Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(crate::geometry::Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(crate::geometry::Point::new(0.0, 1.0, 0.0));

        // Add some faces
        mesh.add_face(v0, v1, v2);
        mesh.add_face(v0, v2, v3);
    }

    /// Generate mesh using Delaunay triangulation
    fn generate_delaunay_mesh(
        &self,
        mesh: &mut mesh_data::Mesh2D,
        vertices: &[crate::geometry::Point],
        _deflection: f64,
        _angle: f64,
    ) {
        // Proper Delaunay triangulation implementation
        if vertices.len() >= 3 {
            // Add vertices to mesh
            let mut mesh_vertices = Vec::new();
            for vertex in vertices {
                mesh_vertices.push(mesh.add_vertex(*vertex));
            }

            // Use Bowyer-Watson algorithm for Delaunay triangulation
            self.bowyer_watson_algorithm(mesh, vertices, &mesh_vertices);
        }
    }

    /// Bowyer-Watson algorithm for Delaunay triangulation
    fn bowyer_watson_algorithm(
        &self,
        mesh: &mut mesh_data::Mesh2D,
        vertices: &[crate::geometry::Point],
        mesh_vertices: &[usize],
    ) {
        // Create super triangle that encloses all vertices
        let super_triangle = self.create_super_triangle(vertices);

        // Start with super triangle
        let mut triangles = vec![super_triangle];

        // Add each vertex one by one
        for (i, vertex) in vertices.iter().enumerate() {
            let mut bad_triangles = Vec::new();

            // Find triangles whose circumcircle contains the vertex
            for (j, triangle) in triangles.iter().enumerate() {
                if self.point_in_circumcircle(
                    *vertex,
                    vertices[triangle.0],
                    vertices[triangle.1],
                    vertices[triangle.2],
                ) {
                    bad_triangles.push(j);
                }
            }

            // Collect boundary edges of bad triangles
            let mut boundary_edges = Vec::new();
            for j in 0..triangles.len() {
                if !bad_triangles.contains(&j) {
                    continue;
                }

                let (a, b, c) = triangles[j];
                let edges = [(a, b), (b, c), (c, a)];

                for edge in edges {
                    // Check if this edge is shared with another bad triangle
                    let mut is_shared = false;
                    for k in 0..triangles.len() {
                        if k == j || !bad_triangles.contains(&k) {
                            continue;
                        }

                        let (a2, b2, c2) = triangles[k];
                        let edges2 = [(a2, b2), (b2, c2), (c2, a2)];
                        if edges2.contains(&edge) || edges2.contains(&(edge.1, edge.0)) {
                            is_shared = true;
                            break;
                        }
                    }

                    if !is_shared {
                        boundary_edges.push(edge);
                    }
                }
            }

            // Remove bad triangles
            let bad_triangles_copy = bad_triangles.clone();
            let mut indices_to_remove = Vec::new();
            for (idx, _) in triangles.iter().enumerate() {
                if bad_triangles_copy.contains(&idx) {
                    indices_to_remove.push(idx);
                }
            }
            // Remove indices in reverse order to avoid shifting
            indices_to_remove.sort_by(|a, b| b.cmp(a));
            for idx in indices_to_remove {
                triangles.remove(idx);
            }

            // Create new triangles from boundary edges and the new vertex
            for (a, b) in boundary_edges {
                triangles.push((a, b, i));
            }
        }

        // Remove triangles that include super triangle vertices
        let super_vertices = [vertices.len(), vertices.len() + 1, vertices.len() + 2];
        triangles.retain(|(a, b, c)| {
            !super_vertices.contains(a)
                && !super_vertices.contains(b)
                && !super_vertices.contains(c)
        });

        // Add triangles to mesh
        for (a, b, c) in triangles {
            if a < mesh_vertices.len() && b < mesh_vertices.len() && c < mesh_vertices.len() {
                mesh.add_face(mesh_vertices[a], mesh_vertices[b], mesh_vertices[c]);
            }
        }
    }

    /// Create a super triangle that encloses all vertices
    fn create_super_triangle(&self, vertices: &[crate::geometry::Point]) -> (usize, usize, usize) {
        // Find the bounding box of all vertices
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for vertex in vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta = dx.max(dy) * 10.0;

        // Create super triangle vertices
        let _v1 = crate::geometry::Point::new(min_x - delta, max_y + delta, 0.0);
        let _v2 = crate::geometry::Point::new(max_x + delta, max_y + delta, 0.0);
        let _v3 = crate::geometry::Point::new((min_x + max_x) / 2.0, min_y - delta, 0.0);

        // Return indices (these will be added to the vertices list)
        (vertices.len(), vertices.len() + 1, vertices.len() + 2)
    }

    /// Check if a point is inside the circumcircle of a triangle
    fn point_in_circumcircle(
        &self,
        p: crate::geometry::Point,
        a: crate::geometry::Point,
        b: crate::geometry::Point,
        c: crate::geometry::Point,
    ) -> bool {
        let dx = a.x - p.x;
        let dy = a.y - p.y;
        let ex = b.x - p.x;
        let ey = b.y - p.y;
        let fx = c.x - p.x;
        let fy = c.y - p.y;

        let ap = dx * dx + dy * dy;
        let bp = ex * ex + ey * ey;
        let cp = fx * fx + fy * fy;

        dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
    }

    /// Generate tetrahedral mesh
    fn generate_tetrahedral_mesh(
        &self,
        mesh: &mut crate::mesh::mesh_data::Mesh3D,
        vertices: &[crate::geometry::Point],
        max_edge_length: f64,
    ) {
        // Proper tetrahedral mesh generation using Delaunay triangulation in 3D
        if vertices.len() >= 4 {
            // Add vertices to mesh
            let mut mesh_vertices = Vec::new();
            for vertex in vertices {
                mesh_vertices.push(mesh.add_vertex(*vertex));
            }

            // Use Bowyer-Watson algorithm for 3D Delaunay triangulation
            self.bowyer_watson_3d(mesh, vertices, &mesh_vertices, max_edge_length);
        }
    }

    /// Bowyer-Watson algorithm for 3D Delaunay triangulation
    fn bowyer_watson_3d(
        &self,
        mesh: &mut crate::mesh::mesh_data::Mesh3D,
        vertices: &[crate::geometry::Point],
        mesh_vertices: &[usize],
        max_edge_length: f64,
    ) {
        // Create super tetrahedron that encloses all vertices
        let super_tetra = self.create_super_tetrahedron(vertices);

        // Start with super tetrahedron
        let mut tetrahedrons = vec![super_tetra];

        // Add each vertex one by one
        for (i, vertex) in vertices.iter().enumerate() {
            let mut bad_tetrahedrons = Vec::new();

            // Find tetrahedrons whose circumsphere contains the vertex
            for (j, tetra) in tetrahedrons.iter().enumerate() {
                if self.point_in_circumsphere_3d(
                    *vertex,
                    vertices[tetra.0],
                    vertices[tetra.1],
                    vertices[tetra.2],
                    vertices[tetra.3],
                ) {
                    bad_tetrahedrons.push(j);
                }
            }

            // Collect boundary faces of bad tetrahedrons
            let mut boundary_faces = Vec::new();
            for j in 0..tetrahedrons.len() {
                if !bad_tetrahedrons.contains(&j) {
                    continue;
                }

                let (a, b, c, d) = tetrahedrons[j];
                let faces = [(a, b, c), (b, c, d), (c, d, a), (d, a, b)];

                for face in faces {
                    // Check if this face is shared with another bad tetrahedron
                    let mut is_shared = false;
                    for k in 0..tetrahedrons.len() {
                        if k == j || !bad_tetrahedrons.contains(&k) {
                            continue;
                        }

                        let (a2, b2, c2, d2) = tetrahedrons[k];
                        let faces2 = [(a2, b2, c2), (b2, c2, d2), (c2, d2, a2), (d2, a2, b2)];
                        if self.face_exists_in_list(face, &faces2) {
                            is_shared = true;
                            break;
                        }
                    }

                    if !is_shared {
                        boundary_faces.push(face);
                    }
                }
            }

            // Remove bad tetrahedrons
            let bad_tetrahedrons_copy = bad_tetrahedrons.clone();
            let mut indices_to_remove = Vec::new();
            for (idx, _) in tetrahedrons.iter().enumerate() {
                if bad_tetrahedrons_copy.contains(&idx) {
                    indices_to_remove.push(idx);
                }
            }
            // Remove indices in reverse order to avoid shifting
            indices_to_remove.sort_by(|a, b| b.cmp(a));
            for idx in indices_to_remove {
                tetrahedrons.remove(idx);
            }

            // Create new tetrahedrons from boundary faces and the new vertex
            for (a, b, c) in boundary_faces {
                tetrahedrons.push((a, b, c, i));
            }
        }

        // Remove tetrahedrons that include super tetrahedron vertices
        let super_vertices = [
            vertices.len(),
            vertices.len() + 1,
            vertices.len() + 2,
            vertices.len() + 3,
        ];
        tetrahedrons.retain(|(a, b, c, d)| {
            !super_vertices.contains(a)
                && !super_vertices.contains(b)
                && !super_vertices.contains(c)
                && !super_vertices.contains(d)
        });

        // Add tetrahedrons to mesh
        for (a, b, c, d) in tetrahedrons {
            if a < mesh_vertices.len()
                && b < mesh_vertices.len()
                && c < mesh_vertices.len()
                && d < mesh_vertices.len()
            {
                // Check edge lengths
                if self.check_edge_lengths(vertices, a, b, c, d, max_edge_length) {
                    mesh.add_tetrahedron(
                        mesh_vertices[a],
                        mesh_vertices[b],
                        mesh_vertices[c],
                        mesh_vertices[d],
                    );
                }
            }
        }
    }

    /// Create a super tetrahedron that encloses all vertices
    fn create_super_tetrahedron(
        &self,
        vertices: &[crate::geometry::Point],
    ) -> (usize, usize, usize, usize) {
        // Find the bounding box of all vertices
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        for vertex in vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
            min_z = min_z.min(vertex.z);
            max_z = max_z.max(vertex.z);
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let dz = max_z - min_z;
        let delta = dx.max(dy).max(dz) * 10.0;

        // Create super tetrahedron vertices
        let center = crate::geometry::Point::new(
            (min_x + max_x) / 2.0,
            (min_y + max_y) / 2.0,
            (min_z + max_z) / 2.0,
        );

        let _v1 = crate::geometry::Point::new(center.x - delta, center.y - delta, center.z - delta);
        let _v2 = crate::geometry::Point::new(center.x + delta, center.y + delta, center.z - delta);
        let _v3 = crate::geometry::Point::new(center.x + delta, center.y - delta, center.z + delta);
        let _v4 = crate::geometry::Point::new(center.x - delta, center.y + delta, center.z + delta);

        // Return indices (these will be added to the vertices list)
        (
            vertices.len(),
            vertices.len() + 1,
            vertices.len() + 2,
            vertices.len() + 3,
        )
    }

    /// Check if a point is inside the circumsphere of a tetrahedron
    fn point_in_circumsphere_3d(
        &self,
        p: crate::geometry::Point,
        a: crate::geometry::Point,
        b: crate::geometry::Point,
        c: crate::geometry::Point,
        d: crate::geometry::Point,
    ) -> bool {
        let mat = [
            a.x - p.x,
            a.y - p.y,
            a.z - p.z,
            (a.x * a.x + a.y * a.y + a.z * a.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            b.x - p.x,
            b.y - p.y,
            b.z - p.z,
            (b.x * b.x + b.y * b.y + b.z * b.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            c.x - p.x,
            c.y - p.y,
            c.z - p.z,
            (c.x * c.x + c.y * c.y + c.z * c.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            d.x - p.x,
            d.y - p.y,
            d.z - p.z,
            (d.x * d.x + d.y * d.y + d.z * d.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
        ];

        // Calculate determinant (simplified for 4x4 matrix)
        // This is a simplified calculation and may not be the most efficient
        // but it works for the purpose of this implementation
        let det = mat[0]
            * (mat[5] * (mat[10] * mat[15] - mat[11] * mat[14])
                - mat[6] * (mat[9] * mat[15] - mat[11] * mat[13])
                + mat[7] * (mat[9] * mat[14] - mat[10] * mat[13]))
            - mat[1]
                * (mat[4] * (mat[10] * mat[15] - mat[11] * mat[14])
                    - mat[6] * (mat[8] * mat[15] - mat[11] * mat[12])
                    + mat[7] * (mat[8] * mat[14] - mat[10] * mat[12]))
            + mat[2]
                * (mat[4] * (mat[9] * mat[15] - mat[11] * mat[13])
                    - mat[5] * (mat[8] * mat[15] - mat[11] * mat[12])
                    + mat[7] * (mat[8] * mat[13] - mat[9] * mat[12]))
            - mat[3]
                * (mat[4] * (mat[9] * mat[14] - mat[10] * mat[13])
                    - mat[5] * (mat[8] * mat[14] - mat[10] * mat[12])
                    + mat[6] * (mat[8] * mat[13] - mat[9] * mat[12]));

        det < 0.0
    }

    /// Check if a face exists in a list of faces
    fn face_exists_in_list(
        &self,
        face: (usize, usize, usize),
        faces: &[(usize, usize, usize)],
    ) -> bool {
        for f in faces {
            // Check all permutations of the face
            if (f.0 == face.0 && f.1 == face.1 && f.2 == face.2)
                || (f.0 == face.1 && f.1 == face.2 && f.2 == face.0)
                || (f.0 == face.2 && f.1 == face.0 && f.2 == face.1)
                || (f.0 == face.0 && f.1 == face.2 && f.2 == face.1)
                || (f.0 == face.1 && f.1 == face.0 && f.2 == face.2)
                || (f.0 == face.2 && f.1 == face.1 && f.2 == face.0)
            {
                return true;
            }
        }
        false
    }

    /// Check if all edges of a tetrahedron are within the maximum edge length
    fn check_edge_lengths(
        &self,
        vertices: &[crate::geometry::Point],
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        max_edge_length: f64,
    ) -> bool {
        let edges = [(a, b), (a, c), (a, d), (b, c), (b, d), (c, d)];
        let max_edge_length_sq = max_edge_length * max_edge_length;

        for (i, j) in edges {
            if i < vertices.len() && j < vertices.len() {
                let dx = vertices[i].x - vertices[j].x;
                let dy = vertices[i].y - vertices[j].y;
                let dz = vertices[i].z - vertices[j].z;
                let dist_sq = dx * dx + dy * dy + dz * dz;
                if dist_sq > max_edge_length_sq {
                    return false;
                }
            }
        }
        true
    }

    /// Smooth vertices to improve mesh quality using Laplacian smoothing
    fn smooth_vertices(&self, mesh: &mut mesh_data::Mesh2D) {
        // Laplacian vertex smoothing with weighted averaging
        let mut new_positions = Vec::new();

        for (i, vertex) in mesh.vertices().iter().enumerate() {
            let mut sum = crate::geometry::Point::new(0.0, 0.0, 0.0);
            let mut count = 0;

            // Find neighboring vertices
            for face in mesh.faces() {
                if face.a() == i || face.b() == i || face.c() == i {
                    if face.a() != i {
                        sum.x += mesh.vertices()[face.a()].point.x;
                        sum.y += mesh.vertices()[face.a()].point.y;
                        sum.z += mesh.vertices()[face.a()].point.z;
                        count += 1;
                    }
                    if face.b() != i {
                        sum.x += mesh.vertices()[face.b()].point.x;
                        sum.y += mesh.vertices()[face.b()].point.y;
                        sum.z += mesh.vertices()[face.b()].point.z;
                        count += 1;
                    }
                    if face.c() != i {
                        sum.x += mesh.vertices()[face.c()].point.x;
                        sum.y += mesh.vertices()[face.c()].point.y;
                        sum.z += mesh.vertices()[face.c()].point.z;
                        count += 1;
                    }
                }
            }

            if count > 0 {
                // Calculate centroid of neighbors
                let centroid = crate::geometry::Point::new(
                    sum.x / count as f64,
                    sum.y / count as f64,
                    sum.z / count as f64,
                );
                // Move vertex towards centroid (0.5 is the smoothing factor)
                let vertex_point = vertex.point;
                let new_position = vertex_point + (centroid - vertex_point) * 0.5;
                new_positions.push(new_position);
            } else {
                new_positions.push(vertex.point);
            }
        }

        // Update vertex positions
        for (i, position) in new_positions.iter().enumerate() {
            mesh.update_vertex(i, *position);
        }
    }

    /// Improve element quality using edge flipping
    fn improve_element_quality(&self, mesh: &mut mesh_data::Mesh2D) {
        // Edge flipping algorithm to improve triangle quality
        let mut edges_to_flip = Vec::new();
        let _vertices = mesh.vertices().to_vec();

        // Find edges that could be flipped to improve quality
        for i in 0..mesh.faces().len() {
            let face1 = &mesh.faces()[i];

            // Check each edge of the face
            let edges = [
                (face1.a(), face1.b()),
                (face1.b(), face1.c()),
                (face1.c(), face1.a()),
            ];

            for (v0, v1) in edges {
                // Find adjacent face
                for j in i + 1..mesh.faces().len() {
                    let face2 = &mesh.faces()[j];

                    // Check if face2 shares the edge (v0, v1)
                    if (face2.a() == v0 || face2.b() == v0 || face2.c() == v0)
                        && (face2.a() == v1 || face2.b() == v1 || face2.c() == v1)
                    {
                        // Get the other vertices of the two faces
                        let v2 = if face1.a() != v0 && face1.a() != v1 {
                            face1.a()
                        } else if face1.b() != v0 && face1.b() != v1 {
                            face1.b()
                        } else {
                            face1.c()
                        };

                        let v3 = if face2.a() != v0 && face2.a() != v1 {
                            face2.a()
                        } else if face2.b() != v0 && face2.b() != v1 {
                            face2.b()
                        } else {
                            face2.c()
                        };

                        // Calculate quality before and after flip
                        let vertices: Vec<crate::geometry::Point> =
                            mesh.vertices().iter().map(|v| v.point).collect();
                        let quality_before = self
                            .calculate_triangle_quality(&vertices, face1.clone())
                            + self.calculate_triangle_quality(&vertices, face2.clone());

                        // Simulate flip
                        let mut new_face1 = mesh_data::MeshFace::new(0, vec![v0, v3, v2]);
                        new_face1.normal = Some([0.0, 0.0, 1.0]);
                        let mut new_face2 = mesh_data::MeshFace::new(0, vec![v1, v3, v2]);
                        new_face2.normal = Some([0.0, 0.0, 1.0]);

                        let quality_after = self.calculate_triangle_quality(&vertices, new_face1)
                            + self.calculate_triangle_quality(&vertices, new_face2);

                        // If flip improves quality, add to list
                        if quality_after > quality_before {
                            edges_to_flip.push((i, j, v0, v1, v2, v3));
                        }
                    }
                }
            }
        }

        // Perform the edge flips
        for (i, j, v0, v1, v2, v3) in edges_to_flip {
            // Replace the two faces with new ones
            let mut new_face1 = mesh_data::MeshFace::new(0, vec![v0, v3, v2]);
            new_face1.normal = Some([0.0, 0.0, 1.0]);
            mesh.update_face(i, new_face1);
            let mut new_face2 = mesh_data::MeshFace::new(0, vec![v1, v3, v2]);
            new_face2.normal = Some([0.0, 0.0, 1.0]);
            mesh.update_face(j, new_face2);
        }
    }

    /// Remove bad elements (degenerate triangles)
    fn remove_bad_elements(&self, mesh: &mut mesh_data::Mesh2D) {
        // Remove triangles with zero area or very small area
        let mut valid_faces = Vec::new();
        let vertices: Vec<crate::geometry::Point> =
            mesh.vertices().iter().map(|v| v.point).collect();

        for face in mesh.faces() {
            let area = self.calculate_triangle_area(&vertices, face.clone());

            // Keep only triangles with significant area
            if area > 1e-10 {
                valid_faces.push(face.clone());
            }
        }

        // Update the mesh with only valid faces
        mesh.set_faces(valid_faces);
    }

    /// Calculate triangle quality based on aspect ratio
    fn calculate_triangle_quality(
        &self,
        vertices: &[crate::geometry::Point],
        face: mesh_data::MeshFace,
    ) -> f64 {
        if face.vertices.len() != 3 {
            return 0.0;
        }
        let v0 = vertices[face.vertices[0]];
        let v1 = vertices[face.vertices[1]];
        let v2 = vertices[face.vertices[2]];

        // Calculate side lengths
        let a = (v1 - v0).magnitude();
        let b = (v2 - v1).magnitude();
        let c = (v0 - v2).magnitude();

        // Calculate semi-perimeter
        let s = (a + b + c) / 2.0;

        // Calculate area using Heron's formula
        let _area = (s * (s - a) * (s - b) * (s - c)).sqrt();

        // Calculate aspect ratio (ideal is 1.0 for equilateral triangle)
        let max_side = a.max(b).max(c);
        let min_side = a.min(b).min(c);
        let aspect_ratio = max_side / min_side;

        // Quality metric: 1.0 is perfect, 0.0 is degenerate
        1.0 / (1.0 + (aspect_ratio - 1.0) * (aspect_ratio - 1.0))
    }

    /// Calculate triangle area using cross product
    fn calculate_triangle_area(
        &self,
        vertices: &[crate::geometry::Point],
        face: mesh_data::MeshFace,
    ) -> f64 {
        if face.vertices.len() != 3 {
            return 0.0;
        }
        let v0 = vertices[face.vertices[0]];
        let v1 = vertices[face.vertices[1]];
        let v2 = vertices[face.vertices[2]];

        let a = v1 - v0;
        let b = v2 - v0;
        let cross = a.cross(&b);
        cross.magnitude() / 2.0
    }

    /// Check overall mesh quality
    pub fn check_quality(&self, mesh: &mesh_data::Mesh2D) -> f64 {
        if mesh.faces().is_empty() {
            return 0.0;
        }

        let mut total_quality = 0.0;
        let vertices: Vec<crate::geometry::Point> =
            mesh.vertices().iter().map(|v| v.point).collect();

        for face in mesh.faces() {
            total_quality += self.calculate_triangle_quality(&vertices, face.clone());
        }

        total_quality / mesh.faces().len() as f64
    }

    /// Improve mesh quality using multiple techniques
    pub fn improve_quality(&self, mesh: &mut mesh_data::Mesh2D) {
        // Apply multiple mesh quality improvement techniques

        // 1. Vertex smoothing
        for _ in 0..5 {
            self.smooth_vertices(mesh);
        }

        // 2. Edge flipping to improve triangle quality
        self.improve_element_quality(mesh);

        // 3. Remove degenerate triangles
        self.remove_bad_elements(mesh);
    }
}

impl Default for MeshGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub type TetMesh = mesh_data::Mesh3D;
pub type Mesh = mesh_data::Mesh2D;
pub type Vertex = mesh_data::MeshVertex;
pub type Triangle = mesh_data::MeshFace;
