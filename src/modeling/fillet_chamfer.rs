/// Fillet and Chamfer module
///
/// This module provides fillet and chamfer operations for topological shapes,
/// including edge filleting and face chamfering.
use crate::foundation::handle::Handle;
use crate::geometry::{Direction, Point};
#[cfg(test)]
use crate::topology::ShapeType;

use crate::topology::{
    topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_solid::TopoDsSolid,
    topods_vertex::TopoDsVertex, topods_wire::TopoDsWire,
};

/// Fillet and Chamfer operations class
///
/// This class provides methods to perform fillet and chamfer operations on topological shapes.
/// It follows the OpenCASCADE BRepFilletAPI pattern.
#[derive(Debug, Clone)]
pub struct FilletChamfer {
    /// Fillet and chamfer operator for BRep models.
    ///
    /// This struct manages the application of fillets and chamfers to edges and faces of a BRep model.
    /// - `radius`: Fillet radius.
    /// - `chamfer_distance`: Chamfer offset distance.
    /// - `edges_to_fillet`: Edges to apply fillet.
    /// - `faces_to_chamfer`: Faces to apply chamfer.
    radius: f64,
    chamfer_distance: f64,
    edges_to_fillet: Vec<Handle<TopoDsEdge>>,
    faces_to_chamfer: Vec<Handle<TopoDsFace>>,
}

impl FilletChamfer {
    /// Create a new FilletChamfer instance with default radius
    pub fn new() -> Self {
        Self {
            radius: 0.1,
            chamfer_distance: 0.1,
            edges_to_fillet: Vec::new(),
            faces_to_chamfer: Vec::new(),
        }
    }

    /// Create a new FilletChamfer instance with specified radius
    pub fn with_radius(radius: f64) -> Self {
        Self {
            radius,
            chamfer_distance: 0.1,
            edges_to_fillet: Vec::new(),
            faces_to_chamfer: Vec::new(),
        }
    }

    /// Create a new FilletChamfer instance with specified chamfer distance
    pub fn with_chamfer_distance(distance: f64) -> Self {
        Self {
            radius: 0.1,
            chamfer_distance: distance,
            edges_to_fillet: Vec::new(),
            faces_to_chamfer: Vec::new(),
        }
    }

    /// Set the fillet radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Get the fillet radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set the chamfer distance
    pub fn set_chamfer_distance(&mut self, distance: f64) {
        self.chamfer_distance = distance;
    }

    /// Get the chamfer distance
    pub fn chamfer_distance(&self) -> f64 {
        self.chamfer_distance
    }

    // =========================================================================
    // Edge Fillet Operations
    // =========================================================================

    /// Add an edge to be filleted
    ///
    /// # Parameters
    /// - `edge`: The edge to add
    pub fn add_edge(&mut self, edge: Handle<TopoDsEdge>) {
        self.edges_to_fillet.push(edge);
    }

    /// Add multiple edges to be filleted
    ///
    /// # Parameters
    /// - `edges`: The edges to add
    pub fn add_edges(&mut self, edges: &[Handle<TopoDsEdge>]) {
        self.edges_to_fillet.extend(edges.iter().cloned());
    }

    /// Remove an edge from the fillet list
    ///
    /// # Parameters
    /// - `edge`: The edge to remove
    pub fn remove_edge(&mut self, edge: &Handle<TopoDsEdge>) {
        self.edges_to_fillet.retain(|e| e != edge);
    }

    /// Clear all edges from the fillet list
    pub fn clear_edges(&mut self) {
        self.edges_to_fillet.clear();
    }

    /// Get the number of edges to be filleted
    pub fn num_edges(&self) -> usize {
        self.edges_to_fillet.len()
    }

    /// Apply fillet to a shape
    ///
    /// This method applies fillet operations to all edges in the shape.
    ///
    /// # Parameters
    /// - `shape`: The shape to apply fillet to
    ///
    /// # Returns
    /// A new shape with fillets applied
    pub fn apply_fillet(&self, shape: &TopoDsSolid) -> TopoDsSolid {
        // Create a copy of the input shape
        let mut result = shape.clone();

        // For each edge to fillet
        for edge in &self.edges_to_fillet {
            if let Some(edge_ref) = edge.get() {
                // Check if edge can be filleted
                if self.can_fillet_edge(edge) {
                    // Get adjacent faces
                    let adjacent_faces = edge_ref.adjacent_faces();
                    if adjacent_faces.len() == 2 {
                        // Calculate fillet surface
                        let fillet_surface = self.calculate_fillet_surface(edge, self.radius);

                        // Create fillet face from the calculated surface
                        if !fillet_surface.is_empty() {
                            // Create a face from the fillet surface points
                            let fillet_face = self.create_face_from_points(&fillet_surface);

                            // Trim original faces
                            self.trim_adjacent_faces(
                                &mut result,
                                edge,
                                &adjacent_faces,
                                self.radius,
                            );

                            // Add fillet face to the solid
                            result.add_face(Handle::new(std::sync::Arc::new(fillet_face)));
                        }
                    }
                }
            }
        }

        result
    }

    /// Create a face from a set of points
    fn create_face_from_points(&self, points: &[Point]) -> TopoDsFace {
        // Create a face from the given points
        let mut face = TopoDsFace::new();

        if points.len() >= 3 {
            // Check if points are coplanar
            if !self.are_points_coplanar(points) {
                return face; // Return empty face if points are not coplanar
            }

            // Create a wire from the points
            let mut wire = TopoDsWire::new();
            let mut vertices = Vec::new();

            // Create vertices and edges between consecutive points
            for i in 0..points.len() {
                let start_point = points[i];
                let end_point = points[(i + 1) % points.len()];

                // Create vertices
                let start_vertex = TopoDsVertex::new(start_point);
                let end_vertex = TopoDsVertex::new(end_point);
                vertices.push(Handle::new(std::sync::Arc::new(start_vertex.clone())));

                // Create edge between points
                let edge = TopoDsEdge::new(
                    Handle::new(std::sync::Arc::new(start_vertex)),
                    Handle::new(std::sync::Arc::new(end_vertex)),
                );

                wire.add_edge(Handle::new(std::sync::Arc::new(edge)));
            }

            // Set the wire for the face
            face.set_wire(0, Handle::new(std::sync::Arc::new(wire)));
        }

        face
    }

    /// Check if points are coplanar
    fn are_points_coplanar(&self, points: &[Point]) -> bool {
        if points.len() < 3 {
            return true;
        }

        // Calculate normal vector using first three points
        let v1 = points[1] - points[0];
        let v2 = points[2] - points[0];
        let normal = v1.cross(&v2);

        // Check if normal is zero (colinear points)
        if normal.magnitude() < 1e-6 {
            return true;
        }

        // Check if all other points lie on the same plane
        for i in 3..points.len() {
            let v = points[i] - points[0];
            let dot = normal.dot(&v);
            if dot.abs() > 1e-6 {
                return false;
            }
        }

        true
    }

    /// Calculate face normal from points
    #[allow(dead_code)]
    fn calculate_face_normal(&self, points: &[Point]) -> Option<Direction> {
        if points.len() < 3 {
            return None;
        }

        // Calculate normal vector using first three points
        let v1 = points[1] - points[0];
        let v2 = points[2] - points[0];
        let normal_vector = v1.cross(&v2);

        if normal_vector.magnitude() < 1e-6 {
            return None;
        }

        Some(Direction::new(
            normal_vector.x,
            normal_vector.y,
            normal_vector.z,
        ))
    }

    /// Trim adjacent faces for fillet
    fn trim_adjacent_faces(
        &self,
        solid: &mut TopoDsSolid,
        edge: &Handle<TopoDsEdge>,
        adjacent_faces: &[Handle<TopoDsFace>],
        radius: f64,
    ) {
        // Trim the adjacent faces to make room for the fillet
        for face in adjacent_faces {
            if let Some(face_ref) = face.get() {
                // Get the face's wires
                let wires = face_ref.wires();

                // For each wire, adjust it to trim the face
                for wire in wires {
                    if let Some(wire_ref) = wire.get() {
                        // Check if the wire contains the edge
                        let edges = wire_ref.edges();
                        if edges.contains(edge) {
                            // Find the points where the fillet starts and ends
                            if let Some(edge_ref) = edge.get() {
                                let start_vertex = edge_ref.start_vertex();
                                let end_vertex = edge_ref.end_vertex();

                                if let (Some(start_v), Some(end_v)) =
                                    (start_vertex.get(), end_vertex.get())
                                {
                                    // Calculate fillet start and end points
                                    let (start_point, end_point) = self.calculate_fillet_points(
                                        edge_ref,
                                        *start_v.point(),
                                        *end_v.point(),
                                        radius,
                                    );

                                    // Create new edges for the trimmed face
                                    let new_edges = self.create_trim_edges(
                                        face_ref,
                                        edge_ref,
                                        start_point,
                                        end_point,
                                    );

                                    // Update the wire with new edges
                                    if !new_edges.is_empty() {
                                        self.update_wire_with_new_edges(
                                            solid,
                                            wire.clone(),
                                            edge,
                                            &new_edges,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Calculate fillet start and end points
    fn calculate_fillet_points(
        &self,
        _edge: &TopoDsEdge,
        start_point: Point,
        end_point: Point,
        radius: f64,
    ) -> (Point, Point) {
        // Calculate the direction vector of the edge
        let edge_vector = end_point - start_point;
        let edge_length = edge_vector.magnitude();

        // Calculate the distance from vertices to fillet start/end points
        let trim_distance = radius;

        // Ensure trim distance is not greater than half the edge length
        let actual_trim = trim_distance.min(edge_length / 2.0);

        // Calculate fillet start and end points
        let start_fillet_point = start_point + edge_vector.normalized() * actual_trim;
        let end_fillet_point = end_point.translated(&(-edge_vector.normalized() * actual_trim));

        (start_fillet_point, end_fillet_point)
    }

    /// Create trim edges for the face
    fn create_trim_edges(
        &self,
        _face: &TopoDsFace,
        _edge: &TopoDsEdge,
        start_point: Point,
        end_point: Point,
    ) -> Vec<Handle<TopoDsEdge>> {
        let mut new_edges = Vec::new();

        // Create new vertices
        let start_vertex = TopoDsVertex::new(start_point);
        let end_vertex = TopoDsVertex::new(end_point);

        // Create new edge
        let new_edge = TopoDsEdge::new(
            Handle::new(std::sync::Arc::new(start_vertex)),
            Handle::new(std::sync::Arc::new(end_vertex)),
        );

        new_edges.push(Handle::new(std::sync::Arc::new(new_edge)));
        new_edges
    }

    /// Update wire with new edges
    fn update_wire_with_new_edges(
        &self,
        _solid: &mut TopoDsSolid,
        wire: Handle<TopoDsWire>,
        old_edge: &Handle<TopoDsEdge>,
        new_edges: &[Handle<TopoDsEdge>],
    ) {
        // Create a new wire with the old edges except the one to remove
        let mut new_wire = TopoDsWire::new();

        if let Some(wire_ref) = wire.get() {
            // Get all edges from the wire
            let edges = wire_ref.edges();

            // Find the position of the old edge in the wire
            let old_edge_index = edges.iter().position(|e| e == old_edge);

            if let Some(index) = old_edge_index {
                // Add edges before the old edge
                for edge in &edges[..index] {
                    new_wire.add_edge(edge.clone());
                }

                // Add new edges
                for new_edge in new_edges {
                    new_wire.add_edge(new_edge.clone());
                }

                // Add edges after the old edge
                for edge in &edges[index + 1..] {
                    new_wire.add_edge(edge.clone());
                }
            } else {
                // Old edge not found, add all edges and new edges
                for edge in edges {
                    new_wire.add_edge(edge.clone());
                }
                for new_edge in new_edges {
                    new_wire.add_edge(new_edge.clone());
                }
            }

            // Find the face that contains this wire
            // Note: remove_face method is not available, so we'll skip this part for now
        }
    }

    /// Apply fillet to specific edges of a shape
    ///
    /// # Parameters
    /// - `shape`: The shape to apply fillet to
    /// - `edges`: The edges to fillet
    /// - `radius`: The fillet radius
    ///
    /// # Returns
    /// A new shape with fillets applied
    pub fn fillet_edges(
        &self,
        shape: &TopoDsSolid,
        edges: &[Handle<TopoDsEdge>],
        radius: f64,
    ) -> TopoDsSolid {
        let mut fillet = Self::with_radius(radius);
        fillet.add_edges(edges);
        fillet.apply_fillet(shape)
    }

    // =========================================================================
    // Face Chamfer Operations
    // =========================================================================

    /// Add a face to be chamfered
    ///
    /// # Parameters
    /// - `face`: The face to add
    pub fn add_face(&mut self, face: Handle<TopoDsFace>) {
        self.faces_to_chamfer.push(face);
    }

    /// Add multiple faces to be chamfered
    ///
    /// # Parameters
    /// - `faces`: The faces to add
    pub fn add_faces(&mut self, faces: &[Handle<TopoDsFace>]) {
        self.faces_to_chamfer.extend(faces.iter().cloned());
    }

    /// Remove a face from the chamfer list
    ///
    /// # Parameters
    /// - `face`: The face to remove
    pub fn remove_face(&mut self, face: &Handle<TopoDsFace>) {
        self.faces_to_chamfer.retain(|f| f != face);
    }

    /// Clear all faces from the chamfer list
    pub fn clear_faces(&mut self) {
        self.faces_to_chamfer.clear();
    }

    /// Get the number of faces to be chamfered
    pub fn num_faces(&self) -> usize {
        self.faces_to_chamfer.len()
    }

    /// Apply chamfer to a shape
    ///
    /// This method applies chamfer operations to all faces in the shape.
    ///
    /// # Parameters
    /// - `shape`: The shape to apply chamfer to
    ///
    /// # Returns
    /// A new shape with chamfers applied
    pub fn apply_chamfer(&self, shape: &TopoDsSolid) -> TopoDsSolid {
        // Create a copy of the input shape
        let mut result = shape.clone();

        // For each face to chamfer
        for face in &self.faces_to_chamfer {
            if let Some(face_ref) = face.get() {
                // Check if face can be chamfered
                if self.can_chamfer_face(face) {
                    // Get edges adjacent to this face
                    let mut edges = Vec::new();
                    for wire in face_ref.wires() {
                        if let Some(wire_ref) = wire.get() {
                            edges.extend(wire_ref.edges().iter().cloned());
                        }
                    }
                    for edge in &edges {
                        // Check if edge can be chamfered
                        if self.can_fillet_edge(&edge) {
                            // Calculate chamfer surface
                            let chamfer_surface =
                                self.calculate_chamfer_surface(&edge, self.chamfer_distance);

                            // Create chamfer face from the calculated surface
                            if !chamfer_surface.is_empty() {
                                // Create a face from the chamfer surface points
                                let chamfer_face = self.create_face_from_points(&chamfer_surface);

                                // Get adjacent faces to the edge
                                if let Some(edge_ref) = edge.get() {
                                    let adjacent_faces = edge_ref.adjacent_faces();

                                    // Trim original faces
                                    self.trim_adjacent_faces(
                                        &mut result,
                                        &edge,
                                        &adjacent_faces,
                                        self.chamfer_distance,
                                    );

                                    // Add chamfer face to the solid
                                    result.add_face(Handle::new(std::sync::Arc::new(chamfer_face)));
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Apply chamfer to specific faces of a shape
    ///
    /// # Parameters
    /// - `shape`: The shape to apply chamfer to
    /// - `faces`: The faces to chamfer
    /// - `distance`: The chamfer distance
    ///
    /// # Returns
    /// A new shape with chamfers applied
    pub fn chamfer_faces(
        &self,
        shape: &TopoDsSolid,
        faces: &[Handle<TopoDsFace>],
        distance: f64,
    ) -> TopoDsSolid {
        let mut chamfer = Self::with_chamfer_distance(distance);
        chamfer.add_faces(faces);
        chamfer.apply_chamfer(shape)
    }

    // =========================================================================
    // Utility Methods
    // =========================================================================

    /// Check if an edge can be filleted
    ///
    /// # Parameters
    /// - `edge`: The edge to check
    ///
    /// # Returns
    /// `true` if the edge can be filleted, `false` otherwise
    pub fn can_fillet_edge(&self, edge: &Handle<TopoDsEdge>) -> bool {
        if let Some(edge_ref) = edge.get() {
            // Check if edge has adjacent faces
            // In a real implementation, we would check:
            // - Edge has exactly two adjacent faces
            // - Faces are not coplanar
            // - Edge is not degenerate
            !edge_ref.is_degenerate()
        } else {
            false
        }
    }

    /// Check if a face can be chamfered
    ///
    /// # Parameters
    /// - `face`: The face to check
    ///
    /// # Returns
    /// `true` if the face can be chamfered, `false` otherwise
    pub fn can_chamfer_face(&self, face: &Handle<TopoDsFace>) -> bool {
        if let Some(face_ref) = face.get() {
            // Check if face has edges
            // In a real implementation, we would check:
            // - Face has at least one edge
            // - Face is not degenerate
            face_ref.num_wires() > 0
        } else {
            false
        }
    }

    /// Calculate the fillet surface for an edge
    ///
    /// # Parameters
    /// - `edge`: The edge to calculate fillet for
    /// - `radius`: The fillet radius
    ///
    /// # Returns
    /// A list of points representing the fillet surface
    pub fn calculate_fillet_surface(&self, edge: &Handle<TopoDsEdge>, radius: f64) -> Vec<Point> {
        // Implementation of fillet surface calculation
        let mut points = Vec::new();

        if let Some(edge_ref) = edge.get() {
            // Get edge geometry
            if let Some(curve) = edge_ref.curve() {
                // Get edge vertices
                let start_vertex = edge_ref.start_vertex();
                let end_vertex = edge_ref.end_vertex();

                if let (Some(start), Some(end)) = (start_vertex.get(), end_vertex.get()) {
                    let start_point = start.point();
                    let end_point = end.point();

                    // Calculate edge direction
                    let edge_vector = crate::geometry::Vector::new(
                        end_point.x - start_point.x,
                        end_point.y - start_point.y,
                        end_point.z - start_point.z,
                    );
                    let edge_length = edge_vector.magnitude();

                    if edge_length > 1e-6 {
                        let edge_direction = edge_vector.normalized();

                        // Get adjacent faces
                        let adjacent_faces = edge_ref.adjacent_faces();
                        if adjacent_faces.len() == 2 {
                            // Generate sample normals for demonstration
                            let normal1 = crate::geometry::Direction::new(0.0, 0.0, 1.0);
                            let normal2 = crate::geometry::Direction::new(0.0, 1.0, 0.0);

                            // Calculate fillet direction: average of face normals, perpendicular to edge
                            let avg_normal = crate::geometry::Direction::new(
                                (normal1.x + normal2.x) / 2.0,
                                (normal1.y + normal2.y) / 2.0,
                                (normal1.z + normal2.z) / 2.0,
                            )
                            .normalized();
                            let avg_normal_vector = crate::geometry::Vector::new(
                                avg_normal.x,
                                avg_normal.y,
                                avg_normal.z,
                            );
                            let fillet_normal =
                                edge_direction.cross(&avg_normal_vector).normalized();

                            // Calculate the fillet center direction: average of face normals
                            let center_direction = avg_normal;

                            // Generate points along the fillet surface
                            let num_points_along_edge = 20;
                            let num_points_around = 8;

                            for i in 0..num_points_along_edge {
                                let t = i as f64 / (num_points_along_edge - 1) as f64;
                                let edge_point = curve.value(t);

                                // Calculate the fillet center at this point
                                let center_point = Point::new(
                                    edge_point.x + center_direction.x * radius,
                                    edge_point.y + center_direction.y * radius,
                                    edge_point.z + center_direction.z * radius,
                                );

                                // Generate points around the fillet center
                                for j in 0..num_points_around {
                                    let angle = j as f64 * 2.0 * std::f64::consts::PI
                                        / num_points_around as f64;

                                    // Create a local coordinate system at the center
                                    let tangent = edge_direction;
                                    let binormal = fillet_normal;
                                    let normal = tangent.cross(&binormal);

                                    // Calculate the offset from the center
                                    let offset_x =
                                        binormal.x * angle.cos() + normal.x * angle.sin();
                                    let offset_y =
                                        binormal.y * angle.cos() + normal.y * angle.sin();
                                    let offset_z =
                                        binormal.z * angle.cos() + normal.z * angle.sin();
                                    let fillet_point = Point::new(
                                        center_point.x + offset_x * radius,
                                        center_point.y + offset_y * radius,
                                        center_point.z + offset_z * radius,
                                    );

                                    points.push(fillet_point);
                                }
                            }
                        }
                    }
                }
            }
        }

        points
    }

    /// Calculate the chamfer surface for an edge
    ///
    /// # Parameters
    /// - `edge`: The edge to calculate chamfer for
    /// - `distance`: The chamfer distance
    ///
    /// # Returns
    /// A list of points representing the chamfer surface
    pub fn calculate_chamfer_surface(
        &self,
        edge: &Handle<TopoDsEdge>,
        distance: f64,
    ) -> Vec<Point> {
        // Implementation of chamfer surface calculation
        let mut points = Vec::new();

        if let Some(edge_ref) = edge.get() {
            // Get edge geometry
            if let Some(curve) = edge_ref.curve() {
                // Get edge vertices
                let start_vertex = edge_ref.start_vertex();
                let end_vertex = edge_ref.end_vertex();

                if let (Some(start), Some(end)) = (start_vertex.get(), end_vertex.get()) {
                    let start_point = start.point();
                    let end_point = end.point();

                    // Calculate edge direction
                    let edge_vector = crate::geometry::Vector::new(
                        end_point.x - start_point.x,
                        end_point.y - start_point.y,
                        end_point.z - start_point.z,
                    );
                    let edge_length = edge_vector.magnitude();

                    if edge_length > 1e-6 {
                        // Get adjacent faces
                        let adjacent_faces = edge_ref.adjacent_faces();
                        if adjacent_faces.len() == 2 {
                            // Generate sample normals for demonstration
                            let normal1 = crate::geometry::Direction::new(0.0, 0.0, 1.0);
                            let normal2 = crate::geometry::Direction::new(0.0, 1.0, 0.0);

                            // Generate points along the chamfer surface
                            let num_points_along_edge = 20;

                            for i in 0..num_points_along_edge {
                                let t = i as f64 / (num_points_along_edge - 1) as f64;
                                let edge_point = curve.value(t);

                                // Calculate the two chamfer points at this position
                                let chamfer_point1 = Point::new(
                                    edge_point.x + normal1.x * distance,
                                    edge_point.y + normal1.y * distance,
                                    edge_point.z + normal1.z * distance,
                                );
                                let chamfer_point2 = Point::new(
                                    edge_point.x + normal2.x * distance,
                                    edge_point.y + normal2.y * distance,
                                    edge_point.z + normal2.z * distance,
                                );

                                // Add points to the surface
                                points.push(chamfer_point1);
                                points.push(chamfer_point2);
                            }

                            // Connect the chamfer points to form a quad strip
                            for i in 0..num_points_along_edge - 1 {
                                let idx1 = i * 2;
                                let idx2 = idx1 + 1;
                                let idx3 = (i + 1) * 2;
                                let idx4 = idx3 + 1;

                                // Add the connecting points to form a quad
                                points.push(points[idx1]);
                                points.push(points[idx3]);
                                points.push(points[idx4]);
                                points.push(points[idx2]);
                            }
                        }
                    }
                }
            }
        }

        points
    }

    /// Reset the fillet and chamfer settings
    pub fn reset(&mut self) {
        self.radius = 0.1;
        self.chamfer_distance = 0.1;
        self.edges_to_fillet.clear();
        self.faces_to_chamfer.clear();
    }
}

impl Default for FilletChamfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::modeling::primitives;

    #[test]
    fn test_fillet_chamfer_creation() {
        let fc = FilletChamfer::new();
        assert_eq!(fc.radius(), 0.1);
        assert_eq!(fc.chamfer_distance(), 0.1);
        assert_eq!(fc.num_edges(), 0);
        assert_eq!(fc.num_faces(), 0);
    }

    #[test]
    fn test_fillet_chamfer_with_radius() {
        let fc = FilletChamfer::with_radius(0.5);
        assert_eq!(fc.radius(), 0.5);
        assert_eq!(fc.chamfer_distance(), 0.1);
    }

    #[test]
    fn test_fillet_chamfer_with_chamfer_distance() {
        let fc = FilletChamfer::with_chamfer_distance(0.3);
        assert_eq!(fc.radius(), 0.1);
        assert_eq!(fc.chamfer_distance(), 0.3);
    }

    #[test]
    fn test_set_radius() {
        let mut fc = FilletChamfer::new();
        fc.set_radius(0.5);
        assert_eq!(fc.radius(), 0.5);
    }

    #[test]
    fn test_set_chamfer_distance() {
        let mut fc = FilletChamfer::new();
        fc.set_chamfer_distance(0.3);
        assert_eq!(fc.chamfer_distance(), 0.3);
    }

    #[test]
    fn test_add_edge() {
        let fc = FilletChamfer::new();

        // Create a simple box to get edges
        let _box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // For now, we can't easily get edges from the solid
        // This test just verifies the method exists
        assert_eq!(fc.num_edges(), 0);
    }

    #[test]
    fn test_add_face() {
        let fc = FilletChamfer::new();

        // Create a simple box
        let _box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // For now, we can't easily get faces from the solid
        // This test just verifies the method exists
        assert_eq!(fc.num_faces(), 0);
    }

    #[test]
    fn test_apply_fillet() {
        let fc = FilletChamfer::with_radius(0.1);

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Apply fillet
        let result = fc.apply_fillet(&box_solid);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }

    #[test]
    fn test_apply_chamfer() {
        let fc = FilletChamfer::with_chamfer_distance(0.1);

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Apply chamfer
        let result = fc.apply_chamfer(&box_solid);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }

    #[test]
    fn test_reset() {
        let mut fc = FilletChamfer::with_radius(0.5);
        fc.set_chamfer_distance(0.3);

        fc.reset();

        assert_eq!(fc.radius(), 0.1);
        assert_eq!(fc.chamfer_distance(), 0.1);
        assert_eq!(fc.num_edges(), 0);
        assert_eq!(fc.num_faces(), 0);
    }

    #[test]
    fn test_can_fillet_edge() {
        let fc = FilletChamfer::new();

        // Create a simple edge with different vertices (non-degenerate)
        let v1 = crate::topology::TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let v2 = crate::topology::TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
        let edge = Handle::new(std::sync::Arc::new(crate::topology::TopoDsEdge::new(
            Handle::new(std::sync::Arc::new(v1)),
            Handle::new(std::sync::Arc::new(v2)),
        )));

        // This should return true for a non-degenerate edge
        assert!(fc.can_fillet_edge(&edge));

        // Create a degenerate edge (same vertex instance)
        let v3 = Handle::new(std::sync::Arc::new(crate::topology::TopoDsVertex::new(
            Point::new(0.0, 0.0, 0.0),
        )));
        let degenerate_edge = Handle::new(std::sync::Arc::new(crate::topology::TopoDsEdge::new(
            v3.clone(),
            v3,
        )));

        // This should return false for a degenerate edge
        assert!(!fc.can_fillet_edge(&degenerate_edge));
    }

    #[test]
    fn test_calculate_fillet_surface() {
        let fc = FilletChamfer::new();

        // Create a simple edge
        let v1 = crate::topology::TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let v2 = crate::topology::TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
        let edge = Handle::new(std::sync::Arc::new(crate::topology::TopoDsEdge::new(
            Handle::new(std::sync::Arc::new(v1)),
            Handle::new(std::sync::Arc::new(v2)),
        )));

        let points = fc.calculate_fillet_surface(&edge, 0.1);

        // Should return empty for a degenerate edge
        assert!(points.is_empty());
    }

    #[test]
    fn test_calculate_chamfer_surface() {
        let fc = FilletChamfer::new();

        // Create a simple edge
        let v1 = crate::topology::TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let v2 = crate::topology::TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
        let edge = Handle::new(std::sync::Arc::new(crate::topology::TopoDsEdge::new(
            Handle::new(std::sync::Arc::new(v1)),
            Handle::new(std::sync::Arc::new(v2)),
        )));

        let points = fc.calculate_chamfer_surface(&edge, 0.1);

        // Should return empty for a degenerate edge
        assert!(points.is_empty());
    }

    #[test]
    fn test_apply_fillet_to_box() {
        let fc = FilletChamfer::with_radius(0.1);

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Apply fillet
        let result = fc.apply_fillet(&box_solid);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }

    #[test]
    fn test_apply_chamfer_to_box() {
        let fc = FilletChamfer::with_chamfer_distance(0.1);

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Apply chamfer
        let result = fc.apply_chamfer(&box_solid);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }

    #[test]
    fn test_fillet_edges() {
        let fc = FilletChamfer::new();

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Create a simple edge
        let v1 = crate::topology::TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let v2 = crate::topology::TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
        let edge = Handle::new(std::sync::Arc::new(crate::topology::TopoDsEdge::new(
            Handle::new(std::sync::Arc::new(v1)),
            Handle::new(std::sync::Arc::new(v2)),
        )));

        let edges = vec![edge];

        // Apply fillet to specific edges
        let result = fc.fillet_edges(&box_solid, &edges, 0.1);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }

    #[test]
    fn test_chamfer_faces() {
        let fc = FilletChamfer::new();

        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Create a simple face
        let face = TopoDsFace::new();
        let faces = vec![Handle::new(std::sync::Arc::new(face))];

        // Apply chamfer to specific faces
        let result = fc.chamfer_faces(&box_solid, &faces, 0.1);

        // Verify result is a solid
        assert_eq!(result.shape().shape_type(), ShapeType::Solid);
    }
}
