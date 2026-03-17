//! Mesh data structures
//!
//! This module provides basic mesh data structures for 2D and 3D meshes.

use crate::geometry::Point;
use std::collections::HashMap;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Mesh vertex
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshVertex {
    /// Vertex ID
    pub id: usize,
    /// Vertex coordinates
    pub point: Point,
    /// Optional normal vector
    pub normal: Option<[f64; 3]>,
    /// Optional texture coordinates
    pub uv: Option<[f64; 2]>,
    /// Optional color
    pub color: Option<[f64; 4]>,
    /// Optional scalar field values
    pub field_values: HashMap<String, f64>,
    /// Optional BRep vertex ID
    pub brep_vertex_id: Option<i32>,
    /// Optional BRep face ID
    pub brep_face_id: Option<i32>,
}

impl MeshVertex {
    /// Create a new mesh vertex
    pub fn new(id: usize, point: Point) -> Self {
        Self {
            id,
            point,
            normal: None,
            uv: None,
            color: None,
            field_values: HashMap::new(),
            brep_vertex_id: None,
            brep_face_id: None,
        }
    }

    /// Set normal vector
    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = Some(normal);
    }

    /// Set texture coordinates
    pub fn set_uv(&mut self, uv: [f64; 2]) {
        self.uv = Some(uv);
    }

    /// Set color
    pub fn set_color(&mut self, color: [f64; 4]) {
        self.color = Some(color);
    }

    /// Add field value
    pub fn add_field_value(&mut self, name: &str, value: f64) {
        self.field_values.insert(name.to_string(), value);
    }

    /// Set BRep vertex ID
    pub fn set_brep_vertex_id(&mut self, id: i32) {
        self.brep_vertex_id = Some(id);
    }

    /// Set BRep face ID
    pub fn set_brep_face_id(&mut self, id: i32) {
        self.brep_face_id = Some(id);
    }

    /// Get x coordinate (alias for point.x)
    pub fn a(&self) -> f64 {
        self.point.x
    }

    /// Get y coordinate (alias for point.y)
    pub fn b(&self) -> f64 {
        self.point.y
    }

    /// Get z coordinate (alias for point.z)
    pub fn c(&self) -> f64 {
        self.point.z
    }

    /// Set x coordinate
    pub fn set_a(&mut self, a: f64) {
        self.point.x = a;
    }

    /// Set y coordinate
    pub fn set_b(&mut self, b: f64) {
        self.point.y = b;
    }

    /// Set z coordinate
    pub fn set_c(&mut self, c: f64) {
        self.point.z = c;
    }
}

/// Mesh edge
#[derive(Debug, Clone, PartialEq)]
pub struct MeshEdge {
    /// Edge ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 2],
    /// Optional edge data
    pub data: HashMap<String, f64>,
}

impl MeshEdge {
    /// Create a new mesh edge
    pub fn new(id: usize, v1: usize, v2: usize) -> Self {
        Self {
            id,
            vertices: [v1, v2],
            data: HashMap::new(),
        }
    }

    /// Add edge data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

/// Mesh face
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshFace {
    /// Face ID
    pub id: usize,
    /// Vertex indices
    pub vertices: Vec<usize>,
    /// Edge indices
    pub edges: Vec<usize>,
    /// Optional face normal
    pub normal: Option<[f64; 3]>,
    /// Optional material ID
    pub material_id: Option<usize>,
    /// Optional face data
    pub data: HashMap<String, f64>,
    /// Optional BRep face ID
    pub brep_face_id: Option<i32>,
}

impl MeshFace {
    /// Create a new mesh face
    pub fn new(id: usize, vertices: Vec<usize>) -> Self {
        Self {
            id,
            vertices,
            edges: Vec::new(),
            normal: None,
            material_id: None,
            data: HashMap::new(),
            brep_face_id: None,
        }
    }

    /// Set face normal
    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = Some(normal);
    }

    /// Set material ID
    pub fn set_material_id(&mut self, material_id: usize) {
        self.material_id = Some(material_id);
    }

    /// Add face data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }

    /// Set BRep face ID
    pub fn set_brep_face_id(&mut self, id: i32) {
        self.brep_face_id = Some(id);
    }

    /// Get vertex a (first vertex) for triangular faces
    pub fn a(&self) -> usize {
        if self.vertices.len() > 0 {
            self.vertices[0]
        } else {
            0
        }
    }

    /// Get vertex b (second vertex) for triangular faces
    pub fn b(&self) -> usize {
        if self.vertices.len() > 1 {
            self.vertices[1]
        } else {
            0
        }
    }

    /// Get vertex c (third vertex) for triangular faces
    pub fn c(&self) -> usize {
        if self.vertices.len() > 2 {
            self.vertices[2]
        } else {
            0
        }
    }

    /// Set vertex a (first vertex) for triangular faces
    pub fn set_a(&mut self, a: usize) {
        if self.vertices.len() > 0 {
            self.vertices[0] = a;
        }
    }

    /// Set vertex b (second vertex) for triangular faces
    pub fn set_b(&mut self, b: usize) {
        if self.vertices.len() > 1 {
            self.vertices[1] = b;
        }
    }

    /// Set vertex c (third vertex) for triangular faces
    pub fn set_c(&mut self, c: usize) {
        if self.vertices.len() > 2 {
            self.vertices[2] = c;
        }
    }
}

/// 2D mesh - AoS (Array of Structs) format
#[derive(Debug, Clone)]
pub struct Mesh2D {
    /// Vertices
    pub vertices: Vec<MeshVertex>,
    /// Edges
    pub edges: Vec<MeshEdge>,
    /// Faces (triangles)
    pub faces: Vec<MeshFace>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
}

/// 2D mesh - SoA (Struct of Arrays) format for better memory access
#[derive(Debug, Clone)]
pub struct Mesh2DSOA {
    /// Vertex positions
    pub positions: Vec<[f64; 3]>,
    /// Vertex normals
    pub normals: Vec<Option<[f64; 3]>>,
    /// Vertex UV coordinates
    pub uvs: Vec<Option<[f64; 2]>>,
    /// Vertex colors
    pub colors: Vec<Option<[f64; 4]>>,
    /// Face indices
    pub face_indices: Vec<[usize; 3]>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
}

impl Mesh2DSOA {
    /// Create from Mesh2D
    pub fn from_mesh2d(mesh: &Mesh2D) -> Self {
        let mut positions = Vec::with_capacity(mesh.vertices.len());
        let mut normals = Vec::with_capacity(mesh.vertices.len());
        let mut uvs = Vec::with_capacity(mesh.vertices.len());
        let mut colors = Vec::with_capacity(mesh.vertices.len());
        let mut face_indices = Vec::with_capacity(mesh.faces.len());

        for vertex in &mesh.vertices {
            positions.push([vertex.point.x, vertex.point.y, vertex.point.z]);
            normals.push(vertex.normal);
            uvs.push(vertex.uv);
            colors.push(vertex.color);
        }

        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                face_indices.push([face.vertices[0], face.vertices[1], face.vertices[2]]);
            }
        }

        Self {
            positions,
            normals,
            uvs,
            colors,
            face_indices,
            bbox: mesh.bbox.clone(),
            quality: mesh.quality.clone(),
        }
    }

    /// Convert back to Mesh2D
    pub fn to_mesh2d(&self) -> Mesh2D {
        let mut mesh = Mesh2D::new();

        for (i, pos) in self.positions.iter().enumerate() {
            let vertex = mesh.add_vertex(Point::new(pos[0], pos[1], pos[2]));
            if let Some(normal) = self.normals[i] {
                mesh.vertices[vertex].set_normal(normal);
            }
            if let Some(uv) = self.uvs[i] {
                mesh.vertices[vertex].set_uv(uv);
            }
            if let Some(color) = self.colors[i] {
                mesh.vertices[vertex].set_color(color);
            }
        }

        for indices in &self.face_indices {
            mesh.add_face(indices[0], indices[1], indices[2]);
        }

        mesh.quality = self.quality.clone();
        mesh
    }
}

impl Mesh2D {
    /// Create a new 2D mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            bbox: (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0)),
            quality: HashMap::new(),
        }
    }

    /// Add a vertex
    /// Add a vertex to the mesh
    ///
    /// Adds a new vertex at the specified point without updating the bounding box.
    /// This method is optimized for batch operations where multiple vertices are added
    /// sequentially, as it avoids the overhead of updating the bounding box on each call.
    ///
    /// # Arguments
    /// * `point` - The 3D coordinates of the vertex to add
    ///
    /// # Returns
    /// The index of the newly added vertex
    ///
    /// # Performance
    /// This method is O(1) for adding a single vertex. When adding multiple vertices,
    /// use this method for all vertices and then call update_bbox() once at the end
    /// for better performance.
    ///
    /// # Example
    /// ```
    /// let mut mesh = Mesh2D::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
    /// mesh.update_bbox(); // Update bounding box once after all vertices are added
    /// ```
    pub fn add_vertex(&mut self, point: Point) -> usize {
        let id = self.vertices.len();
        self.vertices.push(MeshVertex::new(id, point));
        id
    }

    /// Add a vertex and update bounding box
    ///
    /// Adds a new vertex at the specified point and immediately updates the mesh's bounding box.
    /// This method is useful when vertices are added infrequently or when the bounding box
    /// needs to be kept up-to-date after each addition.
    ///
    /// # Arguments
    /// * `point` - The 3D coordinates of the vertex to add
    ///
    /// # Returns
    /// The index of the newly added vertex
    ///
    /// # Performance
    /// This method is O(n) where n is the number of vertices, due to the bounding box update.
    /// For batch operations, consider using add_vertex() followed by a single update_bbox() call.
    ///
    /// # Example
    /// ```
    /// let mut mesh = Mesh2D::new();
    /// let v0 = mesh.add_vertex_with_bbox(Point::new(0.0, 0.0, 0.0));
    /// let v1 = mesh.add_vertex_with_bbox(Point::new(1.0, 0.0, 0.0));
    /// // Bounding box is automatically updated after each addition
    /// ```
    pub fn add_vertex_with_bbox(&mut self, point: Point) -> usize {
        let id = self.add_vertex(point);
        self.update_bbox();
        id
    }

    /// Add an edge
    pub fn add_edge(&mut self, v1: usize, v2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(MeshEdge::new(id, v1, v2));
        id
    }

    /// Add a face (triangle)
    pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) -> usize {
        let id = self.faces.len();
        self.faces.push(MeshFace::new(id, vec![v1, v2, v3]));
        id
    }

    /// Update vertex positions without changing topology
    pub fn update_vertex_positions(&mut self, new_positions: &[(usize, Point)]) {
        for (vertex_id, new_position) in new_positions {
            if *vertex_id < self.vertices.len() {
                self.vertices[*vertex_id].point = new_position.clone();
            }
        }
        self.update_bbox();
    }

    /// Create mesh from BRep shape
    pub fn from_brep(_shape: &crate::topology::topods_shape::TopoDsShape) -> Result<Self, String> {
        // Convert BRep shape to mesh by extracting points and faces
        // Placeholder implementation - actual implementation will depend on BRep structure
        let vertices = Vec::new();
        let edges = Vec::new();
        let faces = Vec::new();
        Ok(Self {
            vertices,
            edges,
            faces,
            bbox: (Point::default(), Point::default()),
            quality: std::collections::HashMap::new(),
        })
    }

    /// Convert mesh back to BRep shape
    pub fn to_brep(&self) -> Result<crate::topology::topods_shape::TopoDsShape, String> {
        // Convert mesh to BRep shape
        // Placeholder implementation - actual implementation will depend on BRep structure
        Ok(crate::topology::topods_shape::TopoDsShape::new(
            crate::topology::shape_enum::ShapeType::Compound,
        ))
    }

    /// Update bounding box
    fn update_bbox(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min_point = self.vertices[0].point.clone();
        let mut max_point = self.vertices[0].point.clone();

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.point.x);
            min_point.y = min_point.y.min(vertex.point.y);
            max_point.x = max_point.x.max(vertex.point.x);
            max_point.y = max_point.y.max(vertex.point.y);
        }

        self.bbox = (min_point, max_point);
    }

    /// Calculate face normal
    pub fn calculate_face_normal(&mut self, face_id: usize) {
        if face_id >= self.faces.len() {
            return;
        }

        let face = &self.faces[face_id];
        if face.vertices.len() < 3 {
            return;
        }

        let v0 = &self.vertices[face.vertices[0]];
        let v1 = &self.vertices[face.vertices[1]];
        let v2 = &self.vertices[face.vertices[2]];

        let vec1 = [v1.point.x - v0.point.x, v1.point.y - v0.point.y, 0.0];
        let vec2 = [v2.point.x - v0.point.x, v2.point.y - v0.point.y, 0.0];

        let normal = [0.0, 0.0, vec1[0] * vec2[1] - vec1[1] * vec2[0]];

        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > 1e-6 {
            let normalized_normal = [normal[0] / length, normal[1] / length, normal[2] / length];
            self.faces[face_id].set_normal(normalized_normal);
        }
    }

    /// Calculate all face normals
    pub fn calculate_normals(&mut self) {
        #[cfg(feature = "rayon")]
        {
            let faces = &self.faces;
            let vertices = &self.vertices;
            let mut results = vec![None; faces.len()];

            results.par_iter_mut().enumerate().for_each(|(i, result)| {
                let face = &faces[i];
                if face.vertices.len() >= 3 {
                    let v0 = &vertices[face.vertices[0]];
                    let v1 = &vertices[face.vertices[1]];
                    let v2 = &vertices[face.vertices[2]];

                    let vec1 = [v1.point.x - v0.point.x, v1.point.y - v0.point.y, 0.0];
                    let vec2 = [v2.point.x - v0.point.x, v2.point.y - v0.point.y, 0.0];

                    let normal = [0.0, 0.0, vec1[0] * vec2[1] - vec1[1] * vec2[0]];

                    let length =
                        (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2])
                            .sqrt();
                    if length > 1e-6 {
                        let normalized_normal =
                            [normal[0] / length, normal[1] / length, normal[2] / length];
                        *result = Some(normalized_normal);
                    }
                }
            });

            for (i, normal) in results.into_iter().enumerate() {
                if let Some(normal) = normal {
                    self.faces[i].set_normal(normal);
                }
            }
        }

        #[cfg(not(feature = "rayon"))]
        {
            for i in 0..self.faces.len() {
                self.calculate_face_normal(i);
            }
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    pub fn triangle_count(&self) -> usize {
        self.faces.len()
    }
    pub fn vertex(&self, i: usize) -> Option<&MeshVertex> {
        self.vertices.get(i)
    }
    pub fn triangle(&self, i: usize) -> Option<[usize; 3]> {
        self.faces.get(i).and_then(|face| {
            if face.vertices.len() == 3 {
                Some([face.vertices[0], face.vertices[1], face.vertices[2]])
            } else {
                None
            }
        })
    }

    /// Get vertices slice
    pub fn vertices(&self) -> &[MeshVertex] {
        &self.vertices
    }

    /// Get faces slice
    pub fn faces(&self) -> &[MeshFace] {
        &self.faces
    }

    /// Update a vertex position
    pub fn update_vertex(&mut self, index: usize, point: Point) {
        if index < self.vertices.len() {
            self.vertices[index].point = point;
            self.update_bbox();
        }
    }

    /// Update a face
    pub fn update_face(&mut self, index: usize, face: MeshFace) {
        if index < self.faces.len() {
            self.faces[index] = face;
        }
    }

    /// Set faces (replace all faces)
    pub fn set_faces(&mut self, faces: Vec<MeshFace>) {
        self.faces = faces;
    }

    /// Merge another mesh into this one
    ///
    /// Combines the vertices and faces from another mesh into this mesh.
    /// This method is optimized for performance by:
    /// - Reserving space upfront to avoid reallocations
    /// - Batch adding vertices without immediate bounding box updates
    /// - Updating vertex indices for faces to maintain connectivity
    /// - Updating bounding box only once after all operations complete
    ///
    /// # Arguments
    /// * `other` - Reference to the mesh to merge into this one
    ///
    /// # Performance
    /// This method is O(n + m) where n is the number of vertices and m is the number
    /// of faces in the other mesh. The bounding box update is O(n) but only performed once.
    ///
    /// # Example
    /// ```
    /// let mut mesh1 = Mesh2D::new();
    /// let mesh2 = Mesh2D::new();
    /// mesh1.merge(&mesh2);
    /// ```
    pub fn merge(&mut self, other: &Mesh2D) {
        let vertex_offset = self.vertices.len();

        // Reserve space for new vertices and faces to avoid reallocations
        self.vertices.reserve(other.vertices.len());
        self.faces.reserve(other.faces.len());

        // Add vertices from other mesh
        for vertex in &other.vertices {
            self.vertices
                .push(MeshVertex::new(self.vertices.len(), vertex.point));
        }

        // Add faces from other mesh with updated vertex indices
        for face in &other.faces {
            if face.vertices.len() == 3 {
                let v0 = face.vertices[0] + vertex_offset;
                let v1 = face.vertices[1] + vertex_offset;
                let v2 = face.vertices[2] + vertex_offset;
                self.faces
                    .push(MeshFace::new(self.faces.len(), vec![v0, v1, v2]));
            }
        }

        // Update bounding box once after all vertices are added
        self.update_bbox();
    }
}

/// 3D mesh - AoS (Array of Structs) format
#[derive(Debug, Clone, Default)]
pub struct Mesh3D {
    /// Vertices
    pub vertices: Vec<MeshVertex>,
    /// Edges
    pub edges: Vec<MeshEdge>,
    /// Faces
    pub faces: Vec<MeshFace>,
    /// Tetrahedrons
    pub tetrahedrons: Vec<MeshTetrahedron>,
    /// Hexahedrons
    pub hexahedrons: Vec<MeshHexahedron>,
    /// Prisms
    pub prisms: Vec<MeshPrism>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
    /// Mesh metadata
    pub metadata: HashMap<String, String>,
}

/// 3D mesh - SoA (Struct of Arrays) format for better memory access
#[derive(Debug, Clone)]
pub struct Mesh3DSOA {
    /// Vertex positions
    pub positions: Vec<[f64; 3]>,
    /// Vertex normals
    pub normals: Vec<Option<[f64; 3]>>,
    /// Vertex UV coordinates
    pub uvs: Vec<Option<[f64; 2]>>,
    /// Vertex colors
    pub colors: Vec<Option<[f64; 4]>>,
    /// Tetrahedron indices
    pub tetra_indices: Vec<[usize; 4]>,
    /// Hexahedron indices
    pub hex_indices: Vec<[usize; 8]>,
    /// Prism indices
    pub prism_indices: Vec<[usize; 6]>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
    /// Mesh metadata
    pub metadata: HashMap<String, String>,
}

impl Mesh3DSOA {
    /// Create from Mesh3D
    pub fn from_mesh3d(mesh: &Mesh3D) -> Self {
        let mut positions = Vec::with_capacity(mesh.vertices.len());
        let mut normals = Vec::with_capacity(mesh.vertices.len());
        let mut uvs = Vec::with_capacity(mesh.vertices.len());
        let mut colors = Vec::with_capacity(mesh.vertices.len());
        let mut tetra_indices = Vec::with_capacity(mesh.tetrahedrons.len());
        let mut hex_indices = Vec::with_capacity(mesh.hexahedrons.len());
        let mut prism_indices = Vec::with_capacity(mesh.prisms.len());

        for vertex in &mesh.vertices {
            positions.push([vertex.point.x, vertex.point.y, vertex.point.z]);
            normals.push(vertex.normal);
            uvs.push(vertex.uv);
            colors.push(vertex.color);
        }

        for tetra in &mesh.tetrahedrons {
            tetra_indices.push(tetra.vertices);
        }

        for hex in &mesh.hexahedrons {
            hex_indices.push(hex.vertices);
        }

        for prism in &mesh.prisms {
            prism_indices.push(prism.vertices);
        }

        Self {
            positions,
            normals,
            uvs,
            colors,
            tetra_indices,
            hex_indices,
            prism_indices,
            bbox: mesh.bbox.clone(),
            quality: mesh.quality.clone(),
            metadata: mesh.metadata.clone(),
        }
    }

    /// Convert back to Mesh3D
    pub fn to_mesh3d(&self) -> Mesh3D {
        let mut mesh = Mesh3D::new();

        for (i, pos) in self.positions.iter().enumerate() {
            let vertex = mesh.add_vertex(Point::new(pos[0], pos[1], pos[2]));
            if let Some(normal) = self.normals[i] {
                mesh.vertices[vertex].set_normal(normal);
            }
            if let Some(uv) = self.uvs[i] {
                mesh.vertices[vertex].set_uv(uv);
            }
            if let Some(color) = self.colors[i] {
                mesh.vertices[vertex].set_color(color);
            }
        }

        for indices in &self.tetra_indices {
            mesh.add_tetrahedron(indices[0], indices[1], indices[2], indices[3]);
        }

        for indices in &self.hex_indices {
            mesh.add_hexahedron(
                indices[0], indices[1], indices[2], indices[3], indices[4], indices[5], indices[6],
                indices[7],
            );
        }

        for indices in &self.prism_indices {
            mesh.add_prism(
                indices[0], indices[1], indices[2], indices[3], indices[4], indices[5],
            );
        }

        mesh.quality = self.quality.clone();
        mesh.metadata = self.metadata.clone();
        mesh
    }
}

impl Mesh3D {
    /// Create a new 3D mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            tetrahedrons: Vec::new(),
            hexahedrons: Vec::new(),
            prisms: Vec::new(),
            bbox: (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0)),
            quality: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, point: Point) -> usize {
        let id = self.vertices.len();
        self.vertices.push(MeshVertex::new(id, point));
        id
    }

    /// Add a vertex and update bounding box
    pub fn add_vertex_with_bbox(&mut self, point: Point) -> usize {
        let id = self.add_vertex(point);
        self.update_bbox();
        id
    }

    /// Add an edge
    pub fn add_edge(&mut self, v1: usize, v2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(MeshEdge::new(id, v1, v2));
        id
    }

    /// Add a face
    pub fn add_face(&mut self, vertices: Vec<usize>) -> usize {
        let id = self.faces.len();
        self.faces.push(MeshFace::new(id, vertices));
        id
    }

    /// Add a tetrahedron
    pub fn add_tetrahedron(&mut self, v1: usize, v2: usize, v3: usize, v4: usize) -> usize {
        let id = self.tetrahedrons.len();
        self.tetrahedrons
            .push(MeshTetrahedron::new(id, v1, v2, v3, v4));
        id
    }

    /// Add a hexahedron
    pub fn add_hexahedron(
        &mut self,
        v1: usize,
        v2: usize,
        v3: usize,
        v4: usize,
        v5: usize,
        v6: usize,
        v7: usize,
        v8: usize,
    ) -> usize {
        let id = self.hexahedrons.len();
        self.hexahedrons
            .push(MeshHexahedron::new(id, v1, v2, v3, v4, v5, v6, v7, v8));
        id
    }

    /// Add a prism
    pub fn add_prism(
        &mut self,
        v1: usize,
        v2: usize,
        v3: usize,
        v4: usize,
        v5: usize,
        v6: usize,
    ) -> usize {
        let id = self.prisms.len();
        self.prisms.push(MeshPrism::new(id, v1, v2, v3, v4, v5, v6));
        id
    }

    /// Update vertex positions without changing topology
    pub fn update_vertex_positions(&mut self, new_positions: &[(usize, Point)]) {
        for (vertex_id, new_position) in new_positions {
            if *vertex_id < self.vertices.len() {
                self.vertices[*vertex_id].point = new_position.clone();
            }
        }
        self.update_bbox();
    }

    /// Create mesh from BRep shape
    pub fn from_brep(_shape: &crate::topology::topods_shape::TopoDsShape) -> Result<Self, String> {
        // Implementation will be added in a future update
        Err("Not implemented yet".to_string())
    }

    /// Convert mesh back to BRep shape
    pub fn to_brep(&self) -> Result<crate::topology::topods_shape::TopoDsShape, String> {
        // Implementation will be added in a future update
        Err("Not implemented yet".to_string())
    }

    /// Update bounding box
    fn update_bbox(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min_point = self.vertices[0].point.clone();
        let mut max_point = self.vertices[0].point.clone();

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.point.x);
            min_point.y = min_point.y.min(vertex.point.y);
            min_point.z = min_point.z.min(vertex.point.z);
            max_point.x = max_point.x.max(vertex.point.x);
            max_point.y = max_point.y.max(vertex.point.y);
            max_point.z = max_point.z.max(vertex.point.z);
        }

        self.bbox = (min_point, max_point);
    }

    /// Calculate and return bounding box
    pub fn calculate_bounding_box(&self) -> (Point, Point) {
        if self.vertices.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_point = self.vertices[0].point.clone();
        let mut max_point = self.vertices[0].point.clone();

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.point.x);
            min_point.y = min_point.y.min(vertex.point.y);
            min_point.z = min_point.z.min(vertex.point.z);
            max_point.x = max_point.x.max(vertex.point.x);
            max_point.y = max_point.y.max(vertex.point.y);
            max_point.z = max_point.z.max(vertex.point.z);
        }

        (min_point, max_point)
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

/// Mesh tetrahedron
#[derive(Debug, Clone, PartialEq)]
pub struct MeshTetrahedron {
    /// Tetrahedron ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 4],
    /// Face indices
    pub faces: [usize; 4],
    /// Optional tetrahedron data
    pub data: HashMap<String, f64>,
}

impl MeshTetrahedron {
    /// Create a new mesh tetrahedron
    pub fn new(id: usize, v1: usize, v2: usize, v3: usize, v4: usize) -> Self {
        Self {
            id,
            vertices: [v1, v2, v3, v4],
            faces: [0, 0, 0, 0], // Will be filled later
            data: HashMap::new(),
        }
    }

    /// Add tetrahedron data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

/// Mesh hexahedron
#[derive(Debug, Clone, PartialEq)]
pub struct MeshHexahedron {
    /// Hexahedron ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 8],
    /// Face indices
    pub faces: [usize; 6],
    /// Optional hexahedron data
    pub data: HashMap<String, f64>,
}

impl MeshHexahedron {
    /// Create a new mesh hexahedron
    pub fn new(
        id: usize,
        v1: usize,
        v2: usize,
        v3: usize,
        v4: usize,
        v5: usize,
        v6: usize,
        v7: usize,
        v8: usize,
    ) -> Self {
        Self {
            id,
            vertices: [v1, v2, v3, v4, v5, v6, v7, v8],
            faces: [0, 0, 0, 0, 0, 0], // Will be filled later
            data: HashMap::new(),
        }
    }

    /// Add hexahedron data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

/// Mesh prism
#[derive(Debug, Clone, PartialEq)]
pub struct MeshPrism {
    /// Prism ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 6],
    /// Face indices
    pub faces: [usize; 5],
    /// Optional prism data
    pub data: HashMap<String, f64>,
}

impl MeshPrism {
    /// Create a new mesh prism
    pub fn new(
        id: usize,
        v1: usize,
        v2: usize,
        v3: usize,
        v4: usize,
        v5: usize,
        v6: usize,
    ) -> Self {
        Self {
            id,
            vertices: [v1, v2, v3, v4, v5, v6],
            faces: [0, 0, 0, 0, 0], // Will be filled later
            data: HashMap::new(),
        }
    }

    /// Add prism data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_vertex_creation() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = MeshVertex::new(0, point);
        assert_eq!(vertex.id, 0);
        assert_eq!(vertex.point.x, 1.0);
        assert_eq!(vertex.point.y, 2.0);
        assert_eq!(vertex.point.z, 3.0);
        assert!(vertex.normal.is_none());
        assert!(vertex.uv.is_none());
    }

    #[test]
    fn test_mesh_edge_creation() {
        let edge = MeshEdge::new(0, 0, 1);
        assert_eq!(edge.id, 0);
        assert_eq!(edge.vertices[0], 0);
        assert_eq!(edge.vertices[1], 1);
        assert!(edge.data.is_empty());
    }

    #[test]
    fn test_mesh_face_creation() {
        let face = MeshFace::new(0, vec![0, 1, 2]);
        assert_eq!(face.id, 0);
        assert_eq!(face.vertices, vec![0, 1, 2]);
        assert!(face.edges.is_empty());
        assert!(face.normal.is_none());
        assert!(face.material_id.is_none());
        assert!(face.data.is_empty());
    }

    #[test]
    fn test_mesh2d_creation() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn test_mesh3d_creation() {
        let mut mesh = Mesh3D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0));
        mesh.add_tetrahedron(v0, v1, v2, v3);

        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.tetrahedrons.len(), 1);
    }

    #[test]
    fn test_mesh_tetrahedron_creation() {
        let tetra = MeshTetrahedron::new(0, 0, 1, 2, 3);
        assert_eq!(tetra.id, 0);
        assert_eq!(tetra.vertices, [0, 1, 2, 3]);
        assert!(tetra.data.is_empty());
    }

    #[test]
    fn test_calculate_face_normal() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let face_id = mesh.add_face(v0, v1, v2);

        mesh.calculate_face_normal(face_id);
        let normal = mesh.faces[face_id].normal.unwrap();
        assert!((normal[0] - 0.0).abs() < 1e-6);
        assert!((normal[1] - 0.0).abs() < 1e-6);
        assert!((normal[2] - 1.0).abs() < 1e-6);
    }
}
