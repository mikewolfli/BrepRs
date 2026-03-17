use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::MeshVertex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PolygonFace {
    pub id: usize,
    pub vertices: Vec<usize>,
    pub normal: Option<[f64; 3]>,
    pub material_id: Option<usize>,
    pub data: HashMap<String, f64>,
}

impl PolygonFace {
    pub fn new(id: usize, vertices: Vec<usize>) -> Self {
        Self {
            id,
            vertices,
            normal: None,
            material_id: None,
            data: HashMap::new(),
        }
    }

    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = Some(normal);
    }

    pub fn set_material_id(&mut self, material_id: usize) {
        self.material_id = Some(material_id);
    }

    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_triangle(&self) -> bool {
        self.vertices.len() == 3
    }

    pub fn is_quad(&self) -> bool {
        self.vertices.len() == 4
    }

    pub fn calculate_normal(&mut self, vertices: &[MeshVertex]) {
        if self.vertices.len() < 3 {
            return;
        }

        // Calculate normal using the first three vertices
        let v0 = &vertices[self.vertices[0]].point;
        let v1 = &vertices[self.vertices[1]].point;
        let v2 = &vertices[self.vertices[2]].point;

        let vec1 = Vector::from_point(v0, v1);
        let vec2 = Vector::from_point(v0, v2);
        let normal = vec1.cross(&vec2).normalized();

        self.set_normal([normal.x, normal.y, normal.z]);
    }

    pub fn area(&self, vertices: &[MeshVertex]) -> StandardReal {
        if self.vertices.len() < 3 {
            return 0.0;
        }

        // Calculate area using the shoelace formula for polygons
        let mut area = 0.0;
        let n = self.vertices.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vi = &vertices[self.vertices[i]].point;
            let vj = &vertices[self.vertices[j]].point;

            area += (vi.x * vj.y) - (vj.x * vi.y);
        }

        area.abs() / 2.0
    }

    pub fn centroid(&self, vertices: &[MeshVertex]) -> Point {
        if self.vertices.is_empty() {
            return Point::origin();
        }

        let mut centroid = Point::origin();
        let n = self.vertices.len() as StandardReal;

        for &vertex_id in &self.vertices {
            let vertex = &vertices[vertex_id];
            centroid.x += vertex.point.x / n;
            centroid.y += vertex.point.y / n;
            centroid.z += vertex.point.z / n;
        }

        centroid
    }

    pub fn contains_point(&self, point: &Point, vertices: &[MeshVertex]) -> bool {
        if self.vertices.len() < 3 {
            return false;
        }

        // Simple point-in-polygon test using ray casting
        let mut inside = false;
        let n = self.vertices.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vi = &vertices[self.vertices[i]].point;
            let vj = &vertices[self.vertices[j]].point;

            if ((vi.y > point.y) != (vj.y > point.y)) && 
               (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
                inside = !inside;
            }
        }

        inside
    }

    pub fn triangulate(&self) -> Vec<[usize; 3]> {
        // Simple fan triangulation
        let mut triangles = Vec::new();
        let n = self.vertices.len();

        if n < 3 {
            return triangles;
        }

        for i in 1..n-1 {
            triangles.push([self.vertices[0], self.vertices[i], self.vertices[i+1]]);
        }

        triangles
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        let n = self.vertices.len();

        for i in 0..n {
            let j = (i + 1) % n;
            edges.push((self.vertices[i], self.vertices[j]));
        }

        edges
    }

    pub fn is_convex(&self, vertices: &[MeshVertex]) -> bool {
        if self.vertices.len() < 3 {
            return true;
        }

        // Check if all interior angles are less than 180 degrees
        let n = self.vertices.len();
        let mut previous_cross = 0.0;

        for i in 0..n {
            let v0 = &vertices[self.vertices[i]].point;
            let v1 = &vertices[self.vertices[(i + 1) % n]].point;
            let v2 = &vertices[self.vertices[(i + 2) % n]].point;

            let vec1 = Vector::from_point(v0, v1);
            let vec2 = Vector::from_point(v1, v2);
            let cross = vec1.cross(&vec2).z;

            if i == 0 {
                previous_cross = cross;
            } else if previous_cross * cross < -STANDARD_REAL_EPSILON {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_face_creation() {
        let vertices = vec![0, 1, 2, 3];
        let face = PolygonFace::new(0, vertices);
        
        assert_eq!(face.id, 0);
        assert_eq!(face.vertices, vec![0, 1, 2, 3]);
        assert_eq!(face.vertex_count(), 4);
        assert!(face.is_quad());
        assert!(!face.is_triangle());
    }

    #[test]
    fn test_polygon_face_triangulation() {
        let vertices = vec![0, 1, 2, 3, 4];
        let face = PolygonFace::new(0, vertices);
        
        let triangles = face.triangulate();
        assert_eq!(triangles.len(), 3); // 5-sided polygon should triangulate into 3 triangles
        assert_eq!(triangles[0], [0, 1, 2]);
        assert_eq!(triangles[1], [0, 2, 3]);
        assert_eq!(triangles[2], [0, 3, 4]);
    }

    #[test]
    fn test_polygon_face_edges() {
        let vertices = vec![0, 1, 2, 3];
        let face = PolygonFace::new(0, vertices);
        
        let edges = face.edges();
        assert_eq!(edges.len(), 4);
        assert_eq!(edges[0], (0, 1));
        assert_eq!(edges[1], (1, 2));
        assert_eq!(edges[2], (2, 3));
        assert_eq!(edges[3], (3, 0));
    }

    #[test]
    fn test_polygon_face_calculate_normal() {
        let mut face = PolygonFace::new(0, vec![0, 1, 2]);
        
        let vertices = vec![
            MeshVertex::new(0, Point::new(0.0, 0.0, 0.0)),
            MeshVertex::new(1, Point::new(1.0, 0.0, 0.0)),
            MeshVertex::new(2, Point::new(0.0, 1.0, 0.0)),
        ];
        
        face.calculate_normal(&vertices);
        assert!(face.normal.is_some());
        let normal = face.normal.unwrap();
        assert!((normal[0] - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((normal[1] - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((normal[2] - 1.0).abs() < STANDARD_REAL_EPSILON);
    }
}
