//! Geometric primitives module
//! 
//! This module provides implementations for various geometric primitives,
//! including prism, pyramid, and polyhedron.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{TopoDsShape, topods_solid::TopoDsSolid};

/// Prism primitive
#[derive(Debug, Clone)]
pub struct Prism {
    /// Base polygon vertices
    base_vertices: Vec<Point>,
    /// Height of the prism
    height: f64,
    /// Direction vector
    direction: Vector,
}

impl Prism {
    /// Create a new prism
    pub fn new(base_vertices: Vec<Point>, height: f64, direction: Vector) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }
        
        if height <= 0.0 {
            return Err("Height must be positive".to_string());
        }
        
        if direction.length() < 1e-10 {
            return Err("Direction vector must be non-zero".to_string());
        }
        
        Ok(Self {
            base_vertices,
            height,
            direction: direction.normalize(),
        })
    }

    /// Create a prism with default direction (along Z-axis)
    pub fn new_with_default_direction(base_vertices: Vec<Point>, height: f64) -> Result<Self, String> {
        Self::new(base_vertices, height, Vector::new(0.0, 0.0, 1.0))
    }

    /// Build the prism as a solid
    pub fn build(&self) -> Handle<TopoDsSolid> {
        // Create base face
        let base_face = self.create_face(&self.base_vertices);
        
        // Create top face by translating base vertices
        let top_vertices: Vec<Point> = self.base_vertices
            .iter()
            .map(|v| v + self.direction * self.height)
            .collect();
        let top_face = self.create_face(&top_vertices);
        
        // Create side faces
        let mut side_faces = Vec::new();
        for i in 0..self.base_vertices.len() {
            let v0 = self.base_vertices[i];
            let v1 = self.base_vertices[(i + 1) % self.base_vertices.len()];
            let v2 = top_vertices[(i + 1) % self.base_vertices.len()];
            let v3 = top_vertices[i];
            
            let side_face = self.create_face(&vec![v0, v1, v2, v3]);
            side_faces.push(side_face);
        }
        
        // Combine all faces into a solid
        self.create_solid(base_face, top_face, side_faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the face
        TopoDsShape::new()
    }

    /// Create a solid from faces
    fn create_solid(&self, base_face: TopoDsShape, top_face: TopoDsShape, side_faces: Vec<TopoDsShape>) -> Handle<TopoDsSolid> {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the solid
        Handle::new(TopoDsSolid::new())
    }

    /// Get the base vertices
    pub fn base_vertices(&self) -> &[Point] {
        &self.base_vertices
    }

    /// Get the height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get the direction
    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

/// Pyramid primitive
#[derive(Debug, Clone)]
pub struct Pyramid {
    /// Base polygon vertices
    base_vertices: Vec<Point>,
    /// Apex point
    apex: Point,
}

impl Pyramid {
    /// Create a new pyramid
    pub fn new(base_vertices: Vec<Point>, apex: Point) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }
        
        Ok(Self {
            base_vertices,
            apex,
        })
    }

    /// Create a pyramid with specified height
    pub fn new_with_height(base_vertices: Vec<Point>, height: f64) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }
        
        // Calculate centroid of base
        let centroid = base_vertices.iter().fold(Point::origin(), |sum, p| sum + *p) / base_vertices.len() as f64;
        
        // Calculate apex point
        let apex = centroid + Vector::new(0.0, 0.0, height);
        
        Ok(Self {
            base_vertices,
            apex,
        })
    }

    /// Build the pyramid as a solid
    pub fn build(&self) -> Handle<TopoDsSolid> {
        // Create base face
        let base_face = self.create_face(&self.base_vertices);
        
        // Create side faces
        let mut side_faces = Vec::new();
        for i in 0..self.base_vertices.len() {
            let v0 = self.base_vertices[i];
            let v1 = self.base_vertices[(i + 1) % self.base_vertices.len()];
            
            let side_face = self.create_face(&vec![v0, v1, self.apex]);
            side_faces.push(side_face);
        }
        
        // Combine all faces into a solid
        self.create_solid(base_face, side_faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the face
        TopoDsShape::new()
    }

    /// Create a solid from faces
    fn create_solid(&self, base_face: TopoDsShape, side_faces: Vec<TopoDsShape>) -> Handle<TopoDsSolid> {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the solid
        Handle::new(TopoDsSolid::new())
    }

    /// Get the base vertices
    pub fn base_vertices(&self) -> &[Point] {
        &self.base_vertices
    }

    /// Get the apex point
    pub fn apex(&self) -> &Point {
        &self.apex
    }
}

/// Polyhedron primitive
#[derive(Debug, Clone)]
pub struct Polyhedron {
    /// Vertices of the polyhedron
    vertices: Vec<Point>,
    /// Faces of the polyhedron (each face is a list of vertex indices)
    faces: Vec<Vec<usize>>,
}

impl Polyhedron {
    /// Create a new polyhedron
    pub fn new(vertices: Vec<Point>, faces: Vec<Vec<usize>>) -> Result<Self, String> {
        if vertices.len() < 4 {
            return Err("Polyhedron must have at least 4 vertices".to_string());
        }
        
        if faces.len() < 4 {
            return Err("Polyhedron must have at least 4 faces".to_string());
        }
        
        // Validate face indices
        for face in &faces {
            if face.len() < 3 {
                return Err("Each face must have at least 3 vertices".to_string());
            }
            
            for &index in face {
                if index >= vertices.len() {
                    return Err("Invalid vertex index in face".to_string());
                }
            }
        }
        
        Ok(Self {
            vertices,
            faces,
        })
    }

    /// Create a tetrahedron (simplest polyhedron)
    pub fn tetrahedron() -> Self {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
            Point::new(0.5, 0.5, 1.0),
        ];
        
        let faces = vec![
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![1, 2, 3],
            vec![2, 0, 3],
        ];
        
        Self { vertices, faces }
    }

    /// Create a cube as a polyhedron
    pub fn cube() -> Self {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
        ];
        
        let faces = vec![
            vec![0, 1, 2, 3], // Bottom face
            vec![4, 5, 6, 7], // Top face
            vec![0, 1, 5, 4], // Front face
            vec![2, 3, 7, 6], // Back face
            vec![0, 3, 7, 4], // Left face
            vec![1, 2, 6, 5], // Right face
        ];
        
        Self { vertices, faces }
    }

    /// Build the polyhedron as a solid
    pub fn build(&self) -> Handle<TopoDsSolid> {
        // Create faces
        let mut faces = Vec::new();
        for face_indices in &self.faces {
            let face_vertices: Vec<Point> = face_indices
                .iter()
                .map(|&i| self.vertices[i])
                .collect();
            let face = self.create_face(&face_vertices);
            faces.push(face);
        }
        
        // Combine all faces into a solid
        self.create_solid(faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the face
        TopoDsShape::new()
    }

    /// Create a solid from faces
    fn create_solid(&self, faces: Vec<TopoDsShape>) -> Handle<TopoDsSolid> {
        // This is a simplified implementation
        // In a real implementation, you would use BRepBuilder to create the solid
        Handle::new(TopoDsSolid::new())
    }

    /// Get the vertices
    pub fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    /// Get the faces
    pub fn faces(&self) -> &[Vec<usize>] {
        &self.faces
    }

    /// Calculate the volume of the polyhedron
    pub fn volume(&self) -> f64 {
        // Use the divergence theorem to calculate volume
        let mut volume = 0.0;
        
        for face in &self.faces {
            // Calculate centroid of the face
            let centroid = face.iter()
                .fold(Point::origin(), |sum, &i| sum + self.vertices[i])
                / face.len() as f64;
            
            // Calculate area vector of the face
            let area_vector = self.calculate_face_area_vector(face);
            
            // Add contribution to volume
            volume += centroid.dot(&area_vector) / 3.0;
        }
        
        volume.abs()
    }

    /// Calculate the area vector of a face
    fn calculate_face_area_vector(&self, face: &[usize]) -> Vector {
        let mut area_vector = Vector::zero();
        
        for i in 0..face.len() {
            let v0 = self.vertices[face[i]];
            let v1 = self.vertices[face[(i + 1) % face.len()]];
            
            area_vector += v0.cross(&v1);
        }
        
        area_vector * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_prism_creation() {
        let base_vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
        ];
        
        let result = Prism::new_with_default_direction(base_vertices, 1.0);
        assert!(result.is_ok());
        
        let prism = result.unwrap();
        assert_eq!(prism.base_vertices().len(), 3);
        assert!((prism.height() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_pyramid_creation() {
        let base_vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let result = Pyramid::new_with_height(base_vertices, 1.0);
        assert!(result.is_ok());
        
        let pyramid = result.unwrap();
        assert_eq!(pyramid.base_vertices().len(), 4);
        assert!((pyramid.apex().z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_polyhedron_tetrahedron() {
        let tetrahedron = Polyhedron::tetrahedron();
        assert_eq!(tetrahedron.vertices().len(), 4);
        assert_eq!(tetrahedron.faces().len(), 4);
        
        let volume = tetrahedron.volume();
        assert!((volume - 1.0 / 6.0).abs() < 1e-10); // Volume of a regular tetrahedron with edge length sqrt(2)
    }

    #[test]
    fn test_polyhedron_cube() {
        let cube = Polyhedron::cube();
        assert_eq!(cube.vertices().len(), 8);
        assert_eq!(cube.faces().len(), 6);
        
        let volume = cube.volume();
        assert!((volume - 1.0).abs() < 1e-10); // Volume of a unit cube
    }
}
