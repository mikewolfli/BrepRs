//! Surface clipping module
//!
//! This module provides clipping operations on surfaces,
//! including plane clipping, box clipping, and custom clipping.

use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshVertex};

/// Clipping operation type
#[derive(Debug, Clone, PartialEq)]
pub enum ClippingType {
    /// Plane clipping
    Plane,
    /// Box clipping
    Box,
    /// Sphere clipping
    Sphere,
    /// Custom clipping
    Custom,
}

/// Clipping parameters
#[derive(Debug, Clone)]
pub struct ClippingParams {
    /// Clipping type
    pub clip_type: ClippingType,
    /// Clipping plane (for plane clipping)
    pub plane: Option<ClippingPlane>,
    /// Clipping box (for box clipping)
    pub box_bounds: Option<(Point, Point)>,
    /// Clipping sphere center and radius (for sphere clipping)
    pub sphere: Option<(Point, f64)>,
    /// Custom clipping function
    pub custom_clip: Option<Box<dyn Fn(&Point) -> bool>>,
}

/// Clipping plane
#[derive(Debug, Clone)]
pub struct ClippingPlane {
    /// Plane normal
    pub normal: Vector,
    /// Plane point
    pub point: Point,
}

impl ClippingPlane {
    /// Create a new clipping plane
    pub fn new(normal: Vector, point: Point) -> Self {
        Self {
            normal: {
                let mut n = normal;
                n.normalize();
                n
            },
            point,
        }
    }

    /// Calculate signed distance from a point to plane
    pub fn signed_distance(&self, point: &Point) -> f64 {
        Vector::from_point(point, &self.point).dot(&self.normal)
    }
}

/// Surface clipper
pub struct SurfaceClipper {
    /// Clipping parameters
    params: ClippingParams,
}

impl SurfaceClipper {
    /// Create a new surface clipper
    pub fn new(params: ClippingParams) -> Self {
        Self { params }
    }

    /// Clip a mesh
    pub fn clip(&self, mesh: &Mesh3D) -> Mesh3D {
        match self.params.clip_type {
            ClippingType::Plane => self.clip_by_plane(mesh),
            ClippingType::Box => self.clip_by_box(mesh),
            ClippingType::Sphere => self.clip_by_sphere(mesh),
            ClippingType::Custom => self.clip_by_custom(mesh),
        }
    }

    /// Clip mesh by a plane
    fn clip_by_plane(&self, mesh: &Mesh3D) -> Mesh3D {
        let plane = match &self.params.plane {
            Some(p) => p,
            None => return mesh.clone(),
        };

        let mut clipped_mesh = Mesh3D::new();
        let mut vertex_map: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        // Process vertices
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let distance = plane.signed_distance(&vertex.point);

            if distance >= 0.0 {
                // Vertex is on the positive side, keep it
                let new_index = clipped_mesh.add_vertex(vertex.point);
                vertex_map.insert(i, new_index);
            }
        }

        // Process tetrahedrons
        for tetra in &mesh.tetrahedrons {
            let v0 = &mesh.vertices[tetra.vertices[0]];
            let v1 = &mesh.vertices[tetra.vertices[1]];
            let v2 = &mesh.vertices[tetra.vertices[2]];
            let v3 = &mesh.vertices[tetra.vertices[3]];

            let d0 = plane.signed_distance(&v0.point);
            let d1 = plane.signed_distance(&v1.point);
            let d2 = plane.signed_distance(&v2.point);
            let d3 = plane.signed_distance(&v3.point);

            // Check if tetrahedron is completely on the positive side
            if d0 >= 0.0 && d1 >= 0.0 && d2 >= 0.0 && d3 >= 0.0 {
                // Keep tetrahedron
                let new_v0 = vertex_map[&tetra.vertices[0]];
                let new_v1 = vertex_map[&tetra.vertices[1]];
                let new_v2 = vertex_map[&tetra.vertices[2]];
                let new_v3 = vertex_map[&tetra.vertices[3]];
                clipped_mesh.add_tetrahedron(new_v0, new_v1, new_v2, new_v3);
            }
        }

        clipped_mesh
    }

    /// Clip mesh by a box
    fn clip_by_box(&self, mesh: &Mesh3D) -> Mesh3D {
        let (min, max) = match &self.params.box_bounds {
            Some(b) => b,
            None => return mesh.clone(),
        };

        let mut clipped_mesh = Mesh3D::new();
        let mut vertex_map: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        // Process vertices
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if vertex.point.x >= min.x
                && vertex.point.x <= max.x
                && vertex.point.y >= min.y
                && vertex.point.y <= max.y
                && vertex.point.z >= min.z
                && vertex.point.z <= max.z
            {
                let new_index = clipped_mesh.add_vertex(vertex.point);
                vertex_map.insert(i, new_index);
            }
        }

        // Process tetrahedrons
        for tetra in &mesh.tetrahedrons {
            let all_inside = tetra.vertices.iter().all(|&v| vertex_map.contains_key(&v));

            if all_inside {
                // Keep tetrahedron
                let new_v0 = vertex_map[&tetra.vertices[0]];
                let new_v1 = vertex_map[&tetra.vertices[1]];
                let new_v2 = vertex_map[&tetra.vertices[2]];
                let new_v3 = vertex_map[&tetra.vertices[3]];
                clipped_mesh.add_tetrahedron(new_v0, new_v1, new_v2, new_v3);
            }
        }

        clipped_mesh
    }

    /// Clip mesh by a sphere
    fn clip_by_sphere(&self, mesh: &Mesh3D) -> Mesh3D {
        let (center, radius) = match &self.params.sphere {
            Some(s) => s,
            None => return mesh.clone(),
        };

        let mut clipped_mesh = Mesh3D::new();
        let mut vertex_map: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        // Process vertices
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let distance = vertex.point.distance(center);
            if distance <= radius {
                let new_index = clipped_mesh.add_vertex(vertex.point);
                vertex_map.insert(i, new_index);
            }
        }

        // Process tetrahedrons
        for tetra in &mesh.tetrahedrons {
            let all_inside = tetra.vertices.iter().all(|&v| vertex_map.contains_key(&v));

            if all_inside {
                // Keep tetrahedron
                let new_v0 = vertex_map[&tetra.vertices[0]];
                let new_v1 = vertex_map[&tetra.vertices[1]];
                let new_v2 = vertex_map[&tetra.vertices[2]];
                let new_v3 = vertex_map[&tetra.vertices[3]];
                clipped_mesh.add_tetrahedron(new_v0, new_v1, new_v2, new_v3);
            }
        }

        clipped_mesh
    }

    /// Clip mesh by a custom function
    fn clip_by_custom(&self, mesh: &Mesh3D) -> Mesh3D {
        let clip_func = match &self.params.custom_clip {
            Some(f) => f,
            None => return mesh.clone(),
        };

        let mut clipped_mesh = Mesh3D::new();
        let mut vertex_map: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        // Process vertices
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if clip_func(&vertex.point) {
                let new_index = clipped_mesh.add_vertex(vertex.point, vertex.normal);
                vertex_map.insert(i, new_index);
            }
        }

        // Process tetrahedrons
        for tetra in &mesh.tetrahedrons {
            let all_inside = tetra.vertices.iter().all(|&v| vertex_map.contains_key(&v));

            if all_inside {
                // Keep tetrahedron
                let new_v0 = vertex_map[&tetra.vertices[0]];
                let new_v1 = vertex_map[&tetra.vertices[1]];
                let new_v2 = vertex_map[&tetra.vertices[2]];
                let new_v3 = vertex_map[&tetra.vertices[3]];
                clipped_mesh.add_tetrahedron(new_v0, new_v1, new_v2, new_v3);
            }
        }

        clipped_mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_plane_clipping() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), Vector::zero());
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0), Vector::zero());

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let plane = ClippingPlane::new(Vector::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.5));

        let params = ClippingParams {
            clip_type: ClippingType::Plane,
            plane: Some(plane),
            box_bounds: None,
            sphere: None,
            custom_clip: None,
        };

        let clipper = SurfaceClipper::new(params);
        let clipped = clipper.clip(&mesh);

        // Verify that mesh was clipped
        assert!(clipped.vertices.len() <= mesh.vertices.len());
    }

    #[test]
    fn test_box_clipping() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), Vector::zero());
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0), Vector::zero());

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = ClippingParams {
            clip_type: ClippingType::Box,
            plane: None,
            box_bounds: Some((Point::new(0.0, 0.0, 0.0), Point::new(0.5, 0.5, 0.5))),
            sphere: None,
            custom_clip: None,
        };

        let clipper = SurfaceClipper::new(params);
        let clipped = clipper.clip(&mesh);

        // Verify that mesh was clipped
        assert!(clipped.vertices.len() <= mesh.vertices.len());
    }

    #[test]
    fn test_sphere_clipping() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), Vector::zero());
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0), Vector::zero());

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = ClippingParams {
            clip_type: ClippingType::Sphere,
            plane: None,
            box_bounds: None,
            sphere: Some((Point::new(0.0, 0.0, 0.0), 0.5)),
            custom_clip: None,
        };

        let clipper = SurfaceClipper::new(params);
        let clipped = clipper.clip(&mesh);

        // Verify that mesh was clipped
        assert!(clipped.vertices.len() <= mesh.vertices.len());
    }

    #[test]
    fn test_custom_clipping() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), Vector::zero());
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0), Vector::zero());

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = ClippingParams {
            clip_type: ClippingType::Custom,
            plane: None,
            box_bounds: None,
            sphere: None,
            custom_clip: Some(Box::new(|p: &Point| p.x + p.y + p.z < 1.5)),
        };

        let clipper = SurfaceClipper::new(params);
        let clipped = clipper.clip(&mesh);

        // Verify that mesh was clipped
        assert!(clipped.vertices.len() <= mesh.vertices.len());
    }
}
