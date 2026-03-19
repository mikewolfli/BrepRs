use crate::foundation::types::StandardReal;
#[cfg(test)]
use crate::foundation::types::STANDARD_REAL_EPSILON;
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{MeshFace, MeshVertex};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    pub vertices: Vec<MeshVertex>,
    pub faces: Vec<MeshFace>,
    pub bbox: (Point, Point),
    pub quality: HashMap<String, f64>,
    pub metadata: HashMap<String, String>,
}

impl TriangleMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            bbox: (Point::origin(), Point::origin()),
            quality: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, point: Point) -> usize {
        let id = self.vertices.len();
        self.vertices.push(MeshVertex::new(id, point));
        id
    }

    pub fn add_vertex_with_bbox(&mut self, point: Point) -> usize {
        let id = self.add_vertex(point);
        self.update_bbox();
        id
    }

    pub fn add_triangle(&mut self, v1: usize, v2: usize, v3: usize) -> usize {
        let id = self.faces.len();
        self.faces.push(MeshFace::new(id, vec![v1, v2, v3]));
        id
    }

    pub fn add_triangles(&mut self, triangles: &[[usize; 3]]) -> Vec<usize> {
        let mut face_ids = Vec::with_capacity(triangles.len());
        for triangle in triangles {
            let face_id = self.add_triangle(triangle[0], triangle[1], triangle[2]);
            face_ids.push(face_id);
        }
        face_ids
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.faces.len()
    }

    pub fn update_bbox(&mut self) {
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

    pub fn calculate_face_normal(&mut self, face_id: usize) {
        if face_id >= self.faces.len() {
            return;
        }

        let face = &self.faces[face_id];
        if face.vertices.len() != 3 {
            return;
        }

        let v0 = &self.vertices[face.vertices[0]];
        let v1 = &self.vertices[face.vertices[1]];
        let v2 = &self.vertices[face.vertices[2]];

        let vec1 = Vector::from_point(&v0.point, &v1.point);
        let vec2 = Vector::from_point(&v0.point, &v2.point);
        let normal = vec1.cross(&vec2).normalized();

        self.faces[face_id].set_normal([normal.x, normal.y, normal.z]);
    }

    pub fn calculate_normals(&mut self) {
        for i in 0..self.faces.len() {
            self.calculate_face_normal(i);
        }

        // Calculate vertex normals as average of adjacent face normals
        for vertex in &mut self.vertices {
            let mut normal = Vector::new(0.0, 0.0, 0.0);
            let mut count = 0;

            for face in &self.faces {
                if face.vertices.contains(&vertex.id) {
                    if let Some(face_normal) = face.normal {
                        normal.x += face_normal[0];
                        normal.y += face_normal[1];
                        normal.z += face_normal[2];
                        count += 1;
                    }
                }
            }

            if count > 0 {
                normal = normal * (1.0 / count as StandardReal);
                normal = normal.normalized();
                vertex.set_normal([normal.x, normal.y, normal.z]);
            }
        }
    }

    pub fn compute_quality(&mut self) {
        let mut total_area = 0.0;
        let mut total_aspect_ratio = 0.0;

        for face in &self.faces {
            if face.vertices.len() == 3 {
                let v0 = &self.vertices[face.vertices[0]].point;
                let v1 = &self.vertices[face.vertices[1]].point;
                let v2 = &self.vertices[face.vertices[2]].point;

                let a = v0.distance(v1);
                let b = v1.distance(v2);
                let c = v2.distance(v0);
                let s = (a + b + c) / 2.0;
                let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
                total_area += area;

                let aspect_ratio = (a * a + b * b + c * c) / (4.0 * area);
                total_aspect_ratio += aspect_ratio;
            }
        }

        let triangle_count = self.faces.len() as f64;
        if triangle_count > 0.0 {
            self.quality.insert("total_area".to_string(), total_area);
            self.quality.insert("average_aspect_ratio".to_string(), total_aspect_ratio / triangle_count);
        }
    }

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    pub fn from_mesh3d(mesh: &crate::mesh::mesh_data::Mesh3D) -> Self {
        let mut triangle_mesh = Self::new();

        // Copy vertices
        for vertex in &mesh.vertices {
            triangle_mesh.vertices.push(vertex.clone());
        }

        // Copy only triangular faces
        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                triangle_mesh.faces.push(face.clone());
            }
        }

        triangle_mesh.update_bbox();
        triangle_mesh.quality = mesh.quality.clone();
        triangle_mesh.metadata = mesh.metadata.clone();

        triangle_mesh
    }

    pub fn to_mesh3d(&self) -> crate::mesh::mesh_data::Mesh3D {
        let mut mesh = crate::mesh::mesh_data::Mesh3D::new();

        // Copy vertices
        for vertex in &self.vertices {
            mesh.vertices.push(vertex.clone());
        }

        // Copy faces
        for face in &self.faces {
            mesh.faces.push(face.clone());
        }

        mesh.bbox = self.bbox.clone();
        mesh.quality = self.quality.clone();
        mesh.metadata = self.metadata.clone();

        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_mesh_creation() {
        let mut mesh = TriangleMesh::new();
        
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        
        let face_id = mesh.add_triangle(v0, v1, v2);
        
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.faces[face_id].vertices, vec![0, 1, 2]);
    }

    #[test]
    fn test_triangle_mesh_normals() {
        let mut mesh = TriangleMesh::new();
        
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        
        mesh.add_triangle(v0, v1, v2);
        mesh.calculate_normals();
        
        assert!(mesh.faces[0].normal.is_some());
        let face_normal = mesh.faces[0].normal.unwrap();
        assert!((face_normal[0] - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((face_normal[1] - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((face_normal[2] - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_triangle_mesh_bounding_box() {
        let mut mesh = TriangleMesh::new();
        
        mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        mesh.add_vertex(Point::new(1.0, 2.0, 3.0));
        mesh.add_vertex(Point::new(0.5, 1.0, 1.5));
        mesh.add_triangle(0, 1, 2);
        
        mesh.update_bbox();
        let (min, max) = mesh.bbox;
        
        assert!((min.x - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((min.y - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((min.z - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((max.x - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((max.y - 2.0).abs() < STANDARD_REAL_EPSILON);
        assert!((max.z - 3.0).abs() < STANDARD_REAL_EPSILON);
    }
}
