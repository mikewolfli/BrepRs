use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_location::TopoDsLocation, topods_shape::TopoDsShape, topods_shell::TopoDsShell,
};
use serde::{Deserialize, Serialize};

/// Represents a solid in topological structure
///
/// A solid is a 3D region bounded by one or more shells.
/// The first shell is the outer boundary, and subsequent shells
/// are cavities or voids within the solid.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TopoDsSolid {
    shape: TopoDsShape,
    shells: Vec<Handle<TopoDsShell>>,
    tolerance: f64,
}

impl TopoDsSolid {
    /// Create a new empty solid
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Solid),
            shells: Vec::new(),
            tolerance: 0.001,
        }
    }

    /// Create a new solid with specified shells
    pub fn with_shells(shells: Vec<Handle<TopoDsShell>>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Solid),
            shells,
            tolerance: 0.001,
        }
    }

    /// Create a new solid with specified tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Solid),
            shells: Vec::new(),
            tolerance,
        }
    }

    /// Add a shell to the solid
    pub fn add_shell(&mut self, shell: Handle<TopoDsShell>) {
        self.shells.push(shell);
    }

    /// Get the shells of the solid
    pub fn shells(&self) -> &[Handle<TopoDsShell>] {
        &self.shells
    }

    /// Get the number of shells in the solid
    pub fn num_shells(&self) -> usize {
        self.shells.len()
    }

    /// Get the outer boundary shell (first shell)
    pub fn outer_shell(&self) -> Option<&Handle<TopoDsShell>> {
        self.shells.first()
    }

    /// Get the cavity shells (all shells except the first)
    pub fn cavity_shells(&self) -> &[Handle<TopoDsShell>] {
        if self.shells.len() <= 1 {
            return &[];
        }
        &self.shells[1..]
    }

    /// Set the outer boundary shell (first shell)
    pub fn set_outer_shell(&mut self, shell: Handle<TopoDsShell>) {
        if self.shells.is_empty() {
            self.shells.push(shell);
        } else {
            self.shells[0] = shell;
        }
    }

    /// Add a cavity shell to the solid
    pub fn add_cavity_shell(&mut self, shell: Handle<TopoDsShell>) {
        self.shells.push(shell);
    }

    /// Add a face to the outer shell (creates shell if needed)
    pub fn add_face(&mut self, face: Handle<crate::topology::topods_face::TopoDsFace>) {
        if self.shells.is_empty() {
            let shell = TopoDsShell::new();
            self.shells.push(Handle::new(std::sync::Arc::new(shell)));
        }

        if let Some(shell) = self.shells.first_mut() {
            if let Some(shell_mut) = Handle::as_mut(shell) {
                shell_mut.add_face(face);
            }
        }
    }

    /// Get the tolerance of the solid
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the solid
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Get the location of the solid
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the solid
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if the solid is empty (no shells)
    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    /// Check if the solid has cavities
    pub fn has_cavities(&self) -> bool {
        self.shells.len() > 1
    }

    /// Clear all shells from the solid
    pub fn clear(&mut self) {
        self.shells.clear();
    }

    /// Get the total volume of the solid
    pub fn volume(&self) -> f64 {
        if let Some(outer_shell) = self.outer_shell() {
            self.shell_volume(outer_shell)
        } else {
            0.0
        }
    }

    /// Calculate the volume bounded by a shell
    fn shell_volume(&self, shell: &Handle<TopoDsShell>) -> f64 {
        let faces = shell.faces();
        if faces.is_empty() {
            return 0.0;
        }

        let mut volume = 0.0;

        for face in faces {
            if let Some(outer_wire) = face.outer_wire() {
                let vertices = outer_wire.vertices();
                if vertices.len() >= 3 {
                    let v0 = vertices[0].point();
                    for i in 1..vertices.len() - 1 {
                        let v1 = vertices[i].point();
                        let v2 = vertices[i + 1].point();

                        let cross_x = (v1.y - v0.y) * (v2.z - v0.z) - (v1.z - v0.z) * (v2.y - v0.y);
                        let cross_y = (v1.z - v0.z) * (v2.x - v0.x) - (v1.x - v0.x) * (v2.z - v0.z);
                        let cross_z = (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x);

                        volume += v0.x * cross_x + v0.y * cross_y + v0.z * cross_z;
                    }
                }
            }
        }

        volume.abs() / 6.0
    }

    /// Get the total surface area of the solid
    pub fn area(&self) -> f64 {
        self.shells.iter().map(|s| s.area()).sum()
    }

    /// Get the centroid of the solid
    pub fn centroid(&self) -> Option<Point> {
        if let Some(outer_shell) = self.outer_shell() {
            self.shell_centroid(outer_shell)
        } else {
            None
        }
    }

    /// Calculate the centroid of a shell
    fn shell_centroid(&self, shell: &Handle<TopoDsShell>) -> Option<Point> {
        let faces = shell.faces();
        if faces.is_empty() {
            return None;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;
        let mut total_area = 0.0;

        for face in faces {
            let face_area = face.area();
            if let Some(face_centroid) = face.centroid() {
                sum_x += face_centroid.x * face_area;
                sum_y += face_centroid.y * face_area;
                sum_z += face_centroid.z * face_area;
                total_area += face_area;
            }
        }

        if total_area > 0.0 {
            Some(Point::new(
                sum_x / total_area,
                sum_y / total_area,
                sum_z / total_area,
            ))
        } else {
            None
        }
    }

    /// Get the unique identifier of the solid
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the solid
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this solid is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the solid
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the solid contains a specific shell
    pub fn contains_shell(&self, shell: &Handle<TopoDsShell>) -> bool {
        self.shells.contains(shell)
    }

    /// Get the bounding box of the solid
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.shells.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for shell in &self.shells {
            if let Some((min, max)) = shell.bounding_box() {
                min_x = min_x.min(min.x);
                min_y = min_y.min(min.y);
                min_z = min_z.min(min.z);
                max_x = max_x.max(max.x);
                max_y = max_y.max(max.y);
                max_z = max_z.max(max.z);
            }
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Get all faces in the solid
    pub fn faces(&self) -> Vec<Handle<crate::topology::topods_face::TopoDsFace>> {
        use std::collections::HashSet;

        let mut face_set = HashSet::new();
        let mut faces = Vec::new();

        for shell in &self.shells {
            for face in shell.faces() {
                if face_set.insert(face.shape_id()) {
                    faces.push(face.clone());
                }
            }
        }

        faces
    }

    /// Get all edges in the solid
    pub fn edges(&self) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        use std::collections::HashSet;

        let mut edge_set = HashSet::new();
        let mut edges = Vec::new();

        for shell in &self.shells {
            for edge in shell.edges() {
                if edge_set.insert(edge.shape_id()) {
                    edges.push(edge);
                }
            }
        }

        edges
    }

    /// Get all vertices in the solid
    pub fn vertices(&self) -> Vec<Handle<crate::topology::topods_vertex::TopoDsVertex>> {
        use std::collections::HashSet;

        let mut vertex_set = HashSet::new();
        let mut vertices = Vec::new();

        for shell in &self.shells {
            for vertex in shell.vertices() {
                if vertex_set.insert(vertex.shape_id()) {
                    vertices.push(vertex);
                }
            }
        }

        vertices
    }

    /// Get the number of faces in the solid
    pub fn num_faces(&self) -> usize {
        self.faces().len()
    }

    /// Get the number of edges in the solid
    pub fn num_edges(&self) -> usize {
        self.edges().len()
    }

    /// Get the number of vertices in the solid
    pub fn num_vertices(&self) -> usize {
        self.vertices().len()
    }

    /// Check if a point is inside the solid
    pub fn contains_point(&self, point: &Point) -> bool {
        if let Some(outer_shell) = self.outer_shell() {
            self.point_in_shell(point, outer_shell)
        } else {
            false
        }
    }

    /// Check if a point is inside a shell (using ray casting)
    fn point_in_shell(&self, point: &Point, shell: &Handle<TopoDsShell>) -> bool {
        let faces = shell.faces();
        if faces.is_empty() {
            return false;
        }

        let mut intersections = 0;
        let ray_dir = crate::geometry::Vector::new(1.0, 0.0, 0.0);

        for face in faces {
            if let Some(outer_wire) = face.outer_wire() {
                let vertices = outer_wire.vertices();
                if vertices.len() >= 3 {
                    if self.ray_intersects_face(point, &ray_dir, vertices) {
                        intersections += 1;
                    }
                }
            }
        }

        intersections % 2 == 1
    }

    /// Check if a ray intersects a face
    fn ray_intersects_face(
        &self,
        origin: &Point,
        direction: &crate::geometry::Vector,
        vertices: &[Handle<crate::topology::topods_vertex::TopoDsVertex>],
    ) -> bool {
        if vertices.len() < 3 {
            return false;
        }

        let v0 = vertices[0].point();
        let v1 = vertices[1].point();
        let v2 = vertices[2].point();

        let edge1 = crate::geometry::Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge2 = crate::geometry::Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);

        let normal = edge1.cross(&edge2);
        let normal_mag = normal.magnitude();

        if normal_mag < self.tolerance {
            return false;
        }

        let normal_unit = crate::geometry::Vector::new(
            normal.x / normal_mag,
            normal.y / normal_mag,
            normal.z / normal_mag,
        );

        let denom = normal_unit.dot(direction);
        if denom.abs() < self.tolerance {
            return false;
        }

        let d = -(normal_unit.x * v0.x + normal_unit.y * v0.y + normal_unit.z * v0.z);
        let t =
            -(normal_unit.x * origin.x + normal_unit.y * origin.y + normal_unit.z * origin.z + d)
                / denom;

        if t < 0.0 {
            return false;
        }

        let intersection = crate::geometry::Point::new(
            origin.x + t * direction.x,
            origin.y + t * direction.y,
            origin.z + t * direction.z,
        );

        self.point_in_triangle(&intersection, vertices)
    }

    /// Check if a point is inside a triangle
    fn point_in_triangle(
        &self,
        point: &Point,
        vertices: &[Handle<crate::topology::topods_vertex::TopoDsVertex>],
    ) -> bool {
        if vertices.len() < 3 {
            return false;
        }

        let v0 = vertices[0].point();
        let v1 = vertices[1].point();
        let v2 = vertices[2].point();

        let edge0 = crate::geometry::Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge1 = crate::geometry::Vector::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
        let edge2 = crate::geometry::Vector::new(v0.x - v2.x, v0.y - v2.y, v0.z - v2.z);

        let vp0 = crate::geometry::Vector::new(point.x - v0.x, point.y - v0.y, point.z - v0.z);
        let vp1 = crate::geometry::Vector::new(point.x - v1.x, point.y - v1.y, point.z - v1.z);
        let vp2 = crate::geometry::Vector::new(point.x - v2.x, point.y - v2.y, point.z - v2.z);

        let c0 = edge0.cross(&vp0);
        let c1 = edge1.cross(&vp1);
        let c2 = edge2.cross(&vp2);

        let d0 = c0.dot(&c1);
        let d1 = c0.dot(&c2);

        d0 >= 0.0 && d1 >= 0.0
    }
}

impl Default for TopoDsSolid {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDsSolid {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            shells: self.shells.clone(),
            tolerance: self.tolerance,
        }
    }
}

impl PartialEq for TopoDsSolid {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_creation() {
        let solid = TopoDsSolid::new();
        assert!(solid.is_empty());
        assert_eq!(solid.num_shells(), 0);
    }

    #[test]
    fn test_solid_add_shell() {
        let mut solid = TopoDsSolid::new();
        let shell = Handle::new(std::sync::Arc::new(TopoDsShell::new()));

        solid.add_shell(shell);
        assert_eq!(solid.num_shells(), 1);
        assert!(!solid.has_cavities());
    }

    #[test]
    fn test_solid_has_cavities() {
        let shell1 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let shell2 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let solid = TopoDsSolid::with_shells(vec![shell1, shell2]);

        assert!(solid.has_cavities());
    }

    #[test]
    fn test_solid_outer_shell() {
        let shell1 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let shell2 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let solid = TopoDsSolid::with_shells(vec![shell1.clone(), shell2]);

        assert_eq!(solid.outer_shell().unwrap().shape_id(), shell1.shape_id());
    }

    #[test]
    fn test_solid_cavity_shells() {
        let shell1 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let shell2 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let shell3 = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let solid = TopoDsSolid::with_shells(vec![shell1, shell2.clone(), shell3.clone()]);

        let cavities = solid.cavity_shells();
        assert_eq!(cavities.len(), 2);
        assert_eq!(cavities[0].shape_id(), shell2.shape_id());
        assert_eq!(cavities[1].shape_id(), shell3.shape_id());
    }

    #[test]
    fn test_solid_clear() {
        let shell = Handle::new(std::sync::Arc::new(TopoDsShell::new()));
        let mut solid = TopoDsSolid::with_shells(vec![shell]);
        assert!(!solid.is_empty());

        solid.clear();
        assert!(solid.is_empty());
    }

    #[test]
    fn test_solid_shape_id() {
        let mut solid = TopoDsSolid::new();
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = solid.shape_id();
        assert!(initial_id > 0);

        solid.set_shape_id(42);
        assert_eq!(solid.shape_id(), 42);
    }

    #[test]
    fn test_solid_mutable() {
        let mut solid = TopoDsSolid::new();
        assert!(!solid.is_mutable());

        solid.set_mutable(true);
        assert!(solid.is_mutable());
    }

    #[test]
    fn test_solid_clone() {
        let mut solid1 = TopoDsSolid::new();
        solid1.set_shape_id(10);

        let solid2 = solid1.clone();
        assert_eq!(solid2.shape_id(), 10);
        assert_eq!(solid1, solid2);
    }

    #[test]
    fn test_solid_tolerance() {
        let solid = TopoDsSolid::with_tolerance(0.01);
        assert_eq!(solid.tolerance(), 0.01);
    }
}
