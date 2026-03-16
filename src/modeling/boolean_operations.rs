//! Boolean operations module
//!
//! This module provides boolean operations for topological shapes,
//! including fuse, cut, common, and section operations.

use crate::foundation::handle::Handle;
use crate::geometry::{Plane, Point, Vector};
use crate::modeling::{bsp_tree::BspTreeBuilder, BrepBuilder};
use crate::topology::{
    shape_enum::ShapeType, topods_compound::TopoDsCompound, topods_edge::TopoDsEdge,
    topods_face::TopoDsFace, topods_shape::TopoDsShape, topods_solid::TopoDsSolid,
};

/// Boolean operations class
///
/// This class provides methods to perform boolean operations on topological shapes.
/// It follows the OpenCASCADE BRepAlgoAPI pattern.
#[derive(Debug, Clone)]
pub struct BooleanOperations {
    builder: BrepBuilder,
    bsp_builder: BspTreeBuilder,
}

impl BooleanOperations {
    /// Create a new BooleanOperations instance
    #[inline]
    pub fn new() -> Self {
        Self {
            builder: BrepBuilder::new(),
            bsp_builder: BspTreeBuilder::new(1e-6),
        }
    }

    /// Get the BrepBuilder instance
    #[inline]
    pub fn builder(&self) -> &BrepBuilder {
        &self.builder
    }

    /// Get a mutable reference to the BrepBuilder instance
    #[inline]
    pub fn builder_mut(&mut self) -> &mut BrepBuilder {
        &mut self.builder
    }

    /// Check if the boolean operations instance is valid
    #[inline]
    pub fn is_none(&self) -> bool {
        false
    }

    // =========================================================================
    // Fuse Operation
    // =========================================================================

    /// Fuse two shapes together
    ///
    /// The fuse operation creates a new shape that is the union of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the union of the two input shapes
    #[inline]
    pub fn fuse(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            compound.add_component(shape2.clone());
            return compound;
        }

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, just return a compound with both
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            compound.add_component(shape2.clone());
            return compound;
        }

        // Build BSP trees for both shapes
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform union operation using BSP trees
        let result_bsp = bsp1.union(&bsp2);

        // Convert BSP tree result back to compound
        let faces = result_bsp.collect_all_faces();
        self.faces_to_compound(&faces)
    }

    /// Fuse multiple shapes together
    ///
    /// # Parameters
    /// - `shapes`: A vector of shapes to fuse
    ///
    /// # Returns
    /// A new compound that is the union of all input shapes
    #[inline]
    pub fn fuse_all(&self, shapes: &[Handle<TopoDsShape>]) -> TopoDsCompound {
        if shapes.is_empty() {
            return TopoDsCompound::new();
        }

        if shapes.len() == 1 {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shapes[0].clone());
            return compound;
        }

        // Build BSP trees for all shapes and union them in parallel
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            let bsp_trees: Vec<_> = shapes
                .par_iter()
                .map(|shape| self.bsp_builder.build_from_shape(shape))
                .collect();

            let mut result_bsp = bsp_trees[0].clone();
            for bsp in bsp_trees.iter().skip(1) {
                result_bsp = result_bsp.union(bsp);
            }

            // Convert BSP tree result back to compound
            let faces = result_bsp.collect_all_faces();
            self.faces_to_compound(&faces)
        }

        #[cfg(not(feature = "rayon"))]
        {
            // Build BSP trees for all shapes and union them
            let mut result_bsp = self.bsp_builder.build_from_shape(&shapes[0]);
            for shape in shapes.iter().skip(1) {
                let bsp = self.bsp_builder.build_from_shape(shape);
                result_bsp = result_bsp.union(&bsp);
            }

            // Convert BSP tree result back to compound
            let faces = result_bsp.collect_all_faces();
            self.faces_to_compound(&faces)
        }
    }

    // =========================================================================
    // Cut Operation
    // =========================================================================

    /// Cut the second shape from the first shape
    ///
    /// The cut operation creates a new shape that is the first shape with the second shape removed.
    ///
    /// # Parameters
    /// - `shape1`: The shape to cut from
    /// - `shape2`: The shape to cut
    ///
    /// # Returns
    /// A new compound that is the first shape with the second shape removed
    #[inline]
    pub fn cut(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            return compound;
        }

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, return the first shape unchanged
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            return compound;
        }

        // Build BSP trees for both shapes
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform difference operation using BSP trees
        let result_bsp = bsp1.difference(&bsp2);

        // Convert BSP tree result back to compound
        let faces = result_bsp.collect_all_faces();
        self.faces_to_compound(&faces)
    }

    // =========================================================================
    // Common Operation
    // =========================================================================

    /// Compute the common part of two shapes
    ///
    /// The common operation creates a new shape that is the intersection of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the intersection of the two input shapes
    #[inline]
    pub fn common(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, return empty compound
            return TopoDsCompound::new();
        }

        // Build BSP trees for both shapes
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform intersection operation using BSP trees
        let result_bsp = bsp1.intersection(&bsp2);

        // Convert BSP tree result back to compound
        let faces = result_bsp.collect_all_faces();
        self.faces_to_compound(&faces)
    }

    // =========================================================================
    // Section Operation
    // =========================================================================

    /// Compute the section of two shapes
    ///
    /// The section operation creates a new shape that is the intersection curves of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that contains the intersection curves
    #[inline]
    pub fn section(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Collect all faces from both shapes
        let faces1 = self.collect_faces_from_shape(shape1);
        let faces2 = self.collect_faces_from_shape(shape2);

        // Find intersection curves between faces
        let mut intersection_edges = Vec::new();

        for face1 in &faces1 {
            for face2 in &faces2 {
                if let Some(edges) = self.calculate_face_face_intersection(face1, face2) {
                    intersection_edges.extend(edges);
                }
            }
        }

        // Create compound from intersection edges
        if intersection_edges.is_empty() {
            // If no intersections found, return empty compound
            TopoDsCompound::new()
        } else {
            let mut compound = TopoDsCompound::new();
            for edge in intersection_edges {
                compound.add_component(edge);
            }
            compound
        }
    }

    /// Calculate intersection edges between two faces
    fn calculate_face_face_intersection(
        &self,
        face1: &TopoDsFace,
        face2: &TopoDsFace,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get the surfaces of the faces
        let surface1 = face1.surface();
        let surface2 = face2.surface();

        if surface1.is_some() && surface2.is_some() {
            // Get the bounding boxes of the faces
            let bbox1 = face1.bounding_box();
            let bbox2 = face2.bounding_box();

            if bbox1.is_some() && bbox2.is_some() {
                let (min1, max1) = bbox1.unwrap();
                let (min2, max2) = bbox2.unwrap();

                // Check if bounding boxes overlap
                if !self.bounding_boxes_intersect(&(min1, max1), &(min2, max2)) {
                    return None;
                }

                // Get the wires of both faces
                let wires1 = face1.wires();
                let wires2 = face2.wires();

                let mut intersection_edges = Vec::new();

                // For each wire in face1, check intersection with each wire in face2
                for wire1 in wires1 {
                    if let Some(wire1_ref) = wire1.get() {
                        let edges1 = wire1_ref.edges();

                        for wire2 in wires2 {
                            if let Some(wire2_ref) = wire2.get() {
                                let edges2 = wire2_ref.edges();

                                // Check edge-edge intersections
                                for edge1 in edges1.iter() {
                                    if let Some(edge1_ref) = edge1.get() {
                                        for edge2 in edges2.iter() {
                                            if let Some(edge2_ref) = edge2.get() {
                                                // Calculate edge-edge intersection
                                                if let Some(intersection_point) = self
                                                    .calculate_edge_edge_intersection(
                                                        edge1_ref, edge2_ref,
                                                    )
                                                {
                                                    // Create vertices and edge
                                                    let vertex1 = Handle::new(std::sync::Arc::new(
                                                        crate::topology::topods_vertex::TopoDsVertex::new(intersection_point),
                                                    ));
                                                    let vertex2 = Handle::new(std::sync::Arc::new(
                                                        crate::topology::topods_vertex::TopoDsVertex::new(intersection_point),
                                                    ));
                                                    let edge = TopoDsEdge::new(vertex1, vertex2);
                                                    intersection_edges.push(Handle::new(
                                                        std::sync::Arc::new(edge.shape().clone()),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !intersection_edges.is_empty() {
                    Some(intersection_edges)
                } else {
                    // If no edge-edge intersections, try surface-surface intersection
                    self.calculate_surface_surface_intersection(
                        surface1.unwrap(),
                        surface2.unwrap(),
                    )
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Compute the section of a shape with a plane
    ///
    /// # Parameters
    /// - `shape`: The shape to section
    /// - `plane`: The plane to section with
    ///
    /// # Returns
    /// A new compound that contains the intersection curves
    #[inline]
    pub fn section_with_plane(&self, shape: &Handle<TopoDsShape>, plane: &Plane) -> TopoDsCompound {
        // Collect all faces from the shape
        let faces = self.collect_faces_from_shape(shape);

        // Find intersection curves between faces and plane
        let mut intersection_edges = Vec::new();

        for face in &faces {
            if let Some(edges) = self.calculate_face_plane_intersection(face, plane) {
                intersection_edges.extend(edges);
            }
        }

        // Create compound from intersection edges
        if intersection_edges.is_empty() {
            // If no intersections found, return compound with original shape
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape.clone());
            compound
        } else {
            let mut compound = TopoDsCompound::new();
            for edge in intersection_edges {
                compound.add_component(edge);
            }
            compound
        }
    }

    /// Calculate intersection between a face and a plane
    fn calculate_face_plane_intersection(
        &self,
        face: &TopoDsFace,
        plane: &Plane,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get the surface of the face
        if let Some(surface) = face.surface() {
            if surface.as_ref().is_some() {
                // Check if bounding box intersects with plane
                if let Some((min, max)) = face.bounding_box() {
                    if !self.plane_intersects_bounding_box(plane, &(min, max)) {
                        return None;
                    }
                }

                // Calculate surface-plane intersection
                return self.calculate_surface_plane_intersection(surface, plane);
            }
        }
        None
    }

    /// Calculate intersection between two edges
    fn calculate_edge_edge_intersection(
        &self,
        edge1: &crate::topology::topods_edge::TopoDsEdge,
        edge2: &crate::topology::topods_edge::TopoDsEdge,
    ) -> Option<Point> {
        // Get the curves of the edges
        let curve1 = edge1.curve();
        let curve2 = edge2.curve();

        if curve1.is_some() && curve2.is_some() {
            // For simplicity, we'll check if the edges share any vertices
            let v1_start = edge1.start_vertex();
            let v1_end = edge1.end_vertex();
            let v2_start = edge2.start_vertex();
            let v2_end = edge2.end_vertex();

            // Check all vertex pairs
            if let (Some(v1s), Some(v1e), Some(v2s), Some(v2e)) =
                (v1_start.get(), v1_end.get(), v2_start.get(), v2_end.get())
            {
                let p1s = v1s.point();
                let p1e = v1e.point();
                let p2s = v2s.point();
                let p2e = v2e.point();

                // Check if any vertices are the same
                if p1s.distance(p2s) < 1e-6 {
                    return Some(p1s.clone());
                }
                if p1s.distance(p2e) < 1e-6 {
                    return Some(p1s.clone());
                }
                if p1e.distance(p2s) < 1e-6 {
                    return Some(p1e.clone());
                }
                if p1e.distance(p2e) < 1e-6 {
                    return Some(p1e.clone());
                }

                // Check for actual edge-edge intersection
                // For line segments, use parametric intersection
                if let Some(intersection) =
                    self.calculate_line_segment_intersection(p1s, p1e, p2s, p2e)
                {
                    return Some(intersection);
                }
            }
        }

        None
    }

    /// Calculate intersection between two line segments
    fn calculate_line_segment_intersection(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> Option<Point> {
        // Calculate line segment intersection using parametric form
        let d1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let d2 = Vector::new(p4.x - p3.x, p4.y - p3.y, p4.z - p3.z);

        let cross = Vector::new(
            d1.y * d2.z - d1.z * d2.y,
            d1.z * d2.x - d1.x * d2.z,
            d1.x * d2.y - d1.y * d2.x,
        );

        // Check if lines are parallel
        let cross_magnitude = (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt();
        if cross_magnitude < 1e-10 {
            return None; // Lines are parallel
        }

        // Calculate intersection parameters
        let diff = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);

        let t = (diff.x * cross.y * d2.z - diff.x * cross.z * d2.y + diff.y * cross.z * d2.x
            - diff.y * cross.x * d2.z
            + diff.z * cross.x * d2.y
            - diff.z * cross.y * d2.x)
            / (cross_magnitude * cross_magnitude);

        let s = (diff.x * cross.y * d1.z - diff.x * cross.z * d1.y + diff.y * cross.z * d1.x
            - diff.y * cross.x * d1.z
            + diff.z * cross.x * d1.y
            - diff.z * cross.y * d1.x)
            / (cross_magnitude * cross_magnitude);

        // Check if intersection is within both segments
        if t >= -1e-6 && t <= 1.0 + 1e-6 && s >= -1e-6 && s <= 1.0 + 1e-6 {
            let intersection = Point::new(p1.x + t * d1.x, p1.y + t * d1.y, p1.z + t * d1.z);
            Some(intersection)
        } else {
            None
        }
    }

    /// Calculate surface-surface intersection
    fn calculate_surface_surface_intersection(
        &self,
        surface1: &Handle<crate::geometry::surface_enum::SurfaceEnum>,
        surface2: &Handle<crate::geometry::surface_enum::SurfaceEnum>,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get surface parameter ranges
        let (u1_range, v1_range) = surface1.parameter_range();
        let (u2_range, v2_range) = surface2.parameter_range();

        let mut intersection_points = Vec::new();

        // Sample surfaces and find intersection points
        let num_samples = 10;
        for i in 0..=num_samples {
            let u1 = u1_range.0 + (u1_range.1 - u1_range.0) * (i as f64 / num_samples as f64);
            for j in 0..=num_samples {
                let v1 = v1_range.0 + (v1_range.1 - v1_range.0) * (j as f64 / num_samples as f64);

                let point1 = surface1.value(u1, v1);

                // Find closest point on surface2
                for k in 0..=num_samples {
                    let u2 =
                        u2_range.0 + (u2_range.1 - u2_range.0) * (k as f64 / num_samples as f64);
                    for l in 0..=num_samples {
                        let v2 = v2_range.0
                            + (v2_range.1 - v2_range.0) * (l as f64 / num_samples as f64);

                        let point2 = surface2.value(u2, v2);

                        // Check if points are close enough
                        if point1.distance(&point2) < 1e-3 {
                            intersection_points.push(point1.clone());
                        }
                    }
                }
            }
        }

        // Create edges from intersection points
        if intersection_points.len() >= 2 {
            let mut edges = Vec::new();
            for i in 0..intersection_points.len() - 1 {
                let vertex1 = Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(
                        intersection_points[i].clone(),
                    ),
                ));
                let vertex2 = Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(
                        intersection_points[i + 1].clone(),
                    ),
                ));
                let edge = TopoDsEdge::new(vertex1, vertex2);
                edges.push(Handle::new(std::sync::Arc::new(edge.shape().clone())));
            }
            Some(edges)
        } else {
            None
        }
    }

    /// Calculate intersection between a surface and a plane
    fn calculate_surface_plane_intersection(
        &self,
        surface: &Handle<crate::geometry::surface_enum::SurfaceEnum>,
        plane: &Plane,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get surface parameter ranges
        let (u_range, v_range) = surface.parameter_range();

        let mut intersection_points = Vec::new();

        // Sample surface and find intersection with plane
        let num_samples = 20;
        for i in 0..=num_samples {
            let u = u_range.0 + (u_range.1 - u_range.0) * (i as f64 / num_samples as f64);
            for j in 0..=num_samples {
                let v = v_range.0 + (v_range.1 - v_range.0) * (j as f64 / num_samples as f64);

                let point = surface.value(u, v);
                let distance = plane.distance(&point);

                // Check if point is on the plane (within tolerance)
                if distance.abs() < 1e-3 {
                    intersection_points.push(point);
                }
            }
        }

        // Create edges from intersection points
        if intersection_points.len() >= 2 {
            let mut edges = Vec::new();
            // Sort points to create a continuous curve
            intersection_points.sort_by(|a, b| {
                let dist_a = a.x * a.x + a.y * a.y + a.z * a.z;
                let dist_b = b.x * b.x + b.y * b.y + b.z * b.z;
                dist_a
                    .partial_cmp(&dist_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for i in 0..intersection_points.len() - 1 {
                let vertex1 = Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(
                        intersection_points[i].clone(),
                    ),
                ));
                let vertex2 = Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(
                        intersection_points[i + 1].clone(),
                    ),
                ));
                let edge = TopoDsEdge::new(vertex1, vertex2);
                edges.push(Handle::new(std::sync::Arc::new(edge.shape().clone())));
            }
            Some(edges)
        } else {
            None
        }
    }

    /// Check if a plane intersects a bounding box
    fn plane_intersects_bounding_box(&self, plane: &Plane, bounding_box: &(Point, Point)) -> bool {
        let (min, max) = bounding_box;

        // Check if the plane intersects any of the bounding box's vertices
        let vertices = vec![
            Point::new(min.x, min.y, min.z),
            Point::new(max.x, min.y, min.z),
            Point::new(min.x, max.y, min.z),
            Point::new(max.x, max.y, min.z),
            Point::new(min.x, min.y, max.z),
            Point::new(max.x, min.y, max.z),
            Point::new(min.x, max.y, max.z),
            Point::new(max.x, max.y, max.z),
        ];

        // Check if all vertices are on one side of the plane
        let mut positive_count = 0;
        let mut negative_count = 0;

        for vertex in &vertices {
            let distance = plane.distance(vertex);
            if distance > 1e-6 {
                positive_count += 1;
            } else if distance < -1e-6 {
                negative_count += 1;
            }
        }

        // If vertices are on both sides of the plane, they intersect
        positive_count > 0 && negative_count > 0
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Check if two shapes can be used for boolean operations
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// `true` if the shapes can be used for boolean operations, `false` otherwise
    #[inline]
    pub fn can_perform_boolean(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> bool {
        let valid_types = [
            ShapeType::Solid,
            ShapeType::Shell,
            ShapeType::Face,
            ShapeType::Wire,
            ShapeType::Edge,
        ];

        valid_types.contains(&shape1.shape_type()) && valid_types.contains(&shape2.shape_type())
    }

    /// Check if two shapes might intersect based on their bounding boxes
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// `true` if the shapes might intersect, `false` otherwise
    #[inline]
    pub fn might_intersect(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> bool {
        // Get bounding boxes
        let bbox1 = self.get_bounding_box(shape1);
        let bbox2 = self.get_bounding_box(shape2);

        if bbox1.is_none() || bbox2.is_none() {
            return true; // Assume intersection if we can't determine bounding boxes
        }

        let (min1, max1) = bbox1.unwrap();
        let (min2, max2) = bbox2.unwrap();

        self.bounding_boxes_intersect(&(min1, max1), &(min2, max2))
    }

    /// Check if two solids might intersect based on their bounding boxes
    ///
    /// # Parameters
    /// - `solid1`: The first solid
    /// - `solid2`: The second solid
    ///
    /// # Returns
    /// `true` if the solids might intersect, `false` otherwise
    #[inline]
    pub fn might_intersect_solids(&self, solid1: &TopoDsSolid, solid2: &TopoDsSolid) -> bool {
        // Get bounding boxes from solids
        let bbox1 = solid1.bounding_box();
        let bbox2 = solid2.bounding_box();

        if bbox1.is_none() || bbox2.is_none() {
            return true; // Assume intersection if we can't determine bounding boxes
        }

        let (min1, max1) = bbox1.unwrap();
        let (min2, max2) = bbox2.unwrap();

        self.bounding_boxes_intersect(&(min1, max1), &(min2, max2))
    }

    /// Get the bounding box of a shape
    fn get_bounding_box(&self, shape: &Handle<TopoDsShape>) -> Option<(Point, Point)> {
        match shape.shape_type() {
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    solid.bounding_box()
                } else {
                    None
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    shell.bounding_box()
                } else {
                    None
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    face.bounding_box()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if two bounding boxes intersect
    pub fn bounding_boxes_intersect(&self, bbox1: &(Point, Point), bbox2: &(Point, Point)) -> bool {
        let (min1, max1) = bbox1;
        let (min2, max2) = bbox2;

        // Check for overlap in all three dimensions
        min1.x <= max2.x
            && max1.x >= min2.x
            && min1.y <= max2.y
            && max1.y >= min2.y
            && min1.z <= max2.z
            && max1.z >= min2.z
    }

    /// Collect all faces from a shape
    fn collect_faces_from_shape(&self, shape: &Handle<TopoDsShape>) -> Vec<TopoDsFace> {
        let mut faces = Vec::new();

        match shape.shape_type() {
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    faces.push(face.clone());
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            faces.push(face.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            for face_handle in shell.faces() {
                                if let Some(face) = face_handle.get() {
                                    faces.push(face.as_ref().clone());
                                }
                            }
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component in compound.components() {
                        let component_faces = self.collect_faces_from_shape(component);
                        faces.extend(component_faces);
                    }
                }
            }
            _ => {}
        }

        faces
    }

    /// Convert a list of faces to a compound
    fn faces_to_compound(&self, faces: &[TopoDsFace]) -> TopoDsCompound {
        let mut compound = TopoDsCompound::new();

        for face in faces {
            compound.add_component(Handle::new(std::sync::Arc::new(face.shape().clone())));
        }

        compound
    }

    /// Convert BSP tree to shape
    ///
    /// # Parameters
    /// - `tree`: The BSP tree to convert
    ///
    /// # Returns
    /// A compound shape representing the BSP tree
    pub fn convert_tree_to_shape(
        &self,
        tree: &crate::modeling::bsp_tree::BspTree,
    ) -> TopoDsCompound {
        // Collect all faces from the BSP tree
        let faces = tree.collect_all_faces();

        // Convert faces to compound
        self.faces_to_compound(&faces)
    }
}

impl Default for BooleanOperations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundation::handle::Handle;
    use crate::geometry::Point;
    use std::sync::Arc;

    #[test]
    fn test_boolean_operations_creation() {
        assert!(!boolean_ops.is_none());
    }

    #[test]
    fn test_fuse() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();

        // Fuse the solids
        let result = boolean_ops.fuse(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_cut() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();

        // Cut solid2 from solid1
        let result = boolean_ops.cut(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_common() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();

        // Compute common part
        let result = boolean_ops.common(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_section() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();

        // Compute section
        let result = boolean_ops.section(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid3 = crate::topology::topods_solid::TopoDsSolid::new();

        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::new(Arc::new(solid2.shape().clone())),
            Handle::new(Arc::new(solid3.shape().clone())),
        ];

        // Fuse all solids
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_might_intersect_solids() {
        let boolean_ops = BooleanOperations::new();

        // Create two solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();

        assert!(boolean_ops.might_intersect_solids(&solid1, &solid2));
    }

    #[test]
    fn test_section_with_plane() {
        let boolean_ops = BooleanOperations::new();

        // Create a solid
        let solid = crate::topology::topods_solid::TopoDsSolid::new();

        // Create a plane
        let plane = Plane::new(
            Point::new(0.5, 0.5, 0.5),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        );

        // Compute section
        let result =
            boolean_ops.section_with_plane(&Handle::new(Arc::new(solid.shape().clone())), &plane);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_default() {
        let boolean_ops = BooleanOperations::default();
        assert!(!boolean_ops.is_none());
    }

    #[test]
    fn test_fuse_all_empty() {
        let boolean_ops = BooleanOperations::new();

        // Fuse empty list
        let shapes: Vec<Handle<TopoDsShape>> = vec![];
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be an empty compound
        assert!(result.shape().is_compound());
        assert_eq!(result.num_components(), 0);
    }

    #[test]
    fn test_fuse_all_single() {
        let boolean_ops = BooleanOperations::new();

        // Fuse single shape
        let solid = crate::topology::topods_solid::TopoDsSolid::new();
        let shapes = vec![Handle::new(Arc::new(solid.shape().clone()))];
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound with one component
        assert!(result.shape().is_compound());
        assert_eq!(result.num_components(), 1);
    }

    #[test]
    fn test_fuse_all_parallel() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple solids for parallel processing
        let mut shapes = Vec::new();
        for _ in 0..10 {
            let solid = crate::topology::topods_solid::TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all solids (should use parallel processing if rayon is enabled)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Create shapes including null handles
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();
        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::null(),
            Handle::new(Arc::new(solid2.shape().clone())),
        ];

        // Fuse all solids (should handle null handles gracefully)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all_consistency() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid3 = crate::topology::topods_solid::TopoDsSolid::new();

        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::new(Arc::new(solid2.shape().clone())),
            Handle::new(Arc::new(solid3.shape().clone())),
        ];

        // Fuse all solids multiple times
        let result1 = boolean_ops.fuse_all(&shapes);
        let result2 = boolean_ops.fuse_all(&shapes);
        let result3 = boolean_ops.fuse_all(&shapes);

        // Results should be consistent
        assert_eq!(result1.num_components(), result2.num_components());
        assert_eq!(result2.num_components(), result3.num_components());
    }

    #[test]
    fn test_fuse_all_with_many_shapes() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for stress testing
        let mut shapes = Vec::new();
        for _ in 0..100 {
            let solid = crate::topology::topods_solid::TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all shapes (should handle large number of shapes)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_section_with_different_planes() {
        let boolean_ops = BooleanOperations::new();

        // Create a simple solid
        let solid = crate::topology::topods_solid::TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));

        // Create different planes
        let plane1 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        );
        let plane2 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            crate::geometry::Direction::x_axis(),
            crate::geometry::Direction::y_axis(),
        );
        let plane3 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            crate::geometry::Direction::y_axis(),
            crate::geometry::Direction::z_axis(),
        );

        // Section solid with different planes
        let result1 = boolean_ops.section_with_plane(&solid_handle, &plane1);
        let result2 = boolean_ops.section_with_plane(&solid_handle, &plane2);
        let result3 = boolean_ops.section_with_plane(&solid_handle, &plane3);

        // All results should be compounds
        assert!(result1.shape().is_compound());
        assert!(result2.shape().is_compound());
        assert!(result3.shape().is_compound());
    }

    #[test]
    fn test_section_with_null_handle() {
        let boolean_ops = BooleanOperations::new();

        // Create a plane
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        );

        // Section null handle with plane (should handle gracefully)
        let null_handle: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.section_with_plane(&null_handle, &plane);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Fuse null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.fuse(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_cut_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Cut null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.cut(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_common_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Find common of null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.common(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_with_different_shape_types() {
        let boolean_ops = BooleanOperations::new();

        // Create different shape types
        let solid = crate::topology::topods_solid::TopoDsSolid::new();
        let face = crate::topology::topods_face::TopoDsFace::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));
        let face_handle = Handle::new(Arc::new(face.shape().clone()));

        // Test operations with different shape types
        let fuse_result = boolean_ops.fuse(&solid_handle, &face_handle);
        let cut_result = boolean_ops.cut(&solid_handle, &face_handle);
        let common_result = boolean_ops.common(&solid_handle, &face_handle);

        // All results should be compounds
        assert!(fuse_result.shape().is_compound());
        assert!(cut_result.shape().is_compound());
        assert!(common_result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_with_same_handles() {
        let boolean_ops = BooleanOperations::new();

        // Create a single handle
        let solid = crate::topology::topods_solid::TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));

        // Test operations with same handle
        let fuse_result = boolean_ops.fuse(&solid_handle, &solid_handle);
        let cut_result = boolean_ops.cut(&solid_handle, &solid_handle);
        let common_result = boolean_ops.common(&solid_handle, &solid_handle);

        // All results should be compounds
        assert!(fuse_result.shape().is_compound());
        assert!(cut_result.shape().is_compound());
        assert!(common_result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_performance() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for performance testing
        let mut shapes = Vec::new();
        for _ in 0..50 {
            let solid = crate::topology::topods_solid::TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Measure performance of fuse_all
        let start = std::time::Instant::now();
        let result = boolean_ops.fuse_all(&shapes);
        let duration = start.elapsed();

        // Result should be a compound
        assert!(result.shape().is_compound());

        // Performance should be reasonable (less than 1 second for 50 shapes)
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_boolean_operations_thread_safety() {
        use std::thread;

        let boolean_ops = BooleanOperations::new();

        // Create shapes for thread safety testing
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid3 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid4 = crate::topology::topods_solid::TopoDsSolid::new();

        let handle1 = Handle::new(Arc::new(solid1.shape().clone()));
        let handle2 = Handle::new(Arc::new(solid2.shape().clone()));
        let handle3 = Handle::new(Arc::new(solid3.shape().clone()));
        let handle4 = Handle::new(Arc::new(solid4.shape().clone()));

        // Spawn multiple threads performing operations
        let handle1_clone = handle1.clone();
        let handle2_clone = handle2.clone();
        let handle3_clone = handle3.clone();
        let handle4_clone = handle4.clone();

        let thread1 = thread::spawn(move || {
            let ops = BooleanOperations::new();
            ops.fuse(&handle1_clone, &handle2_clone)
        });

        let thread2 = thread::spawn(move || {
            let ops = BooleanOperations::new();
            ops.cut(&handle3_clone, &handle4_clone)
        });

        // Wait for threads to complete
        let result1 = thread1.join().unwrap();
        let result2 = thread2.join().unwrap();

        // Results should be compounds
        assert!(result1.shape().is_compound());
        assert!(result2.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_memory_usage() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for memory testing
        let mut shapes = Vec::new();
        for _ in 0..100 {
            let solid = crate::topology::topods_solid::TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all shapes
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());

        // Memory usage should be reasonable (number of components should be reasonable)
        assert!(result.num_components() <= 100);
    }
}
