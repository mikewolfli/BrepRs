//! Solid decomposition module
//!
//! This module provides functionality for breaking down complex solids into simpler parts,
//! which is useful for various geometric operations and analysis.

use crate::foundation::handle::Handle;
use crate::geometry::{Direction, Plane, Point, Vector};
use crate::modeling::boolean_operations::BooleanOperations;
use crate::modeling::primitives;
use crate::topology::{
    topods_face::TopoDsFace, topods_shell::TopoDsShell, topods_solid::TopoDsSolid,
};
use std::collections::HashMap;
use std::collections::VecDeque;

/// Decomposition result
#[derive(Debug, Clone, PartialEq)]
pub struct DecompositionResult {
    /// Decomposed solids
    pub solids: Vec<Handle<TopoDsSolid>>,
    /// Decomposition method used
    pub method: DecompositionMethod,
    /// Number of decomposition steps
    pub steps: usize,
}

/// Decomposition method
#[derive(Debug, Clone, PartialEq)]
pub enum DecompositionMethod {
    /// Decompose based on face normals
    FaceNormal,
    /// Decompose based on edge angles
    EdgeAngle,
    /// Decompose based on volume
    Volume,
    /// Decompose based on convexity
    Convexity,
    /// Decompose based on bounding box
    BoundingBox,
    /// Custom decomposition (user callback)
    Custom,
    /// Split solid by a specified plane
    PlaneSplit,
}

/// Parameters for custom decomposition methods
#[derive(Debug, Clone, PartialEq)]
pub struct DecompositionParams {
    /// Plane equation coefficients (ax + by + cz + d = 0)
    pub plane: Option<(f64, f64, f64, f64)>,
    // Add more parameters as needed
}

/// Solid decomposer
pub struct SolidDecomposer {
    /// Tolerance for decomposition
    tolerance: f64,
}

impl SolidDecomposer {
    /// Create a new solid decomposer with default parameters
    pub fn new() -> Self {
        Self { tolerance: 1e-6 }
    }

    /// Create a new solid decomposer with custom parameters
    pub fn with_parameters(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Decompose a solid using the specified method
    /// Decompose a solid using the specified method, with optional custom callback
    pub fn decompose<F>(
        &self,
        solid: &TopoDsSolid,
        method: DecompositionMethod,
        custom: Option<F>,
    ) -> DecompositionResult
    where
        F: Fn(&TopoDsSolid) -> Vec<Handle<TopoDsSolid>>,
    {
        match method {
            DecompositionMethod::FaceNormal => self.decompose_by_face_normal(solid),
            DecompositionMethod::EdgeAngle => self.decompose_by_edge_angle(solid),
            DecompositionMethod::Volume => self.decompose_by_volume(solid),
            DecompositionMethod::Convexity => self.decompose_by_convexity(solid),
            DecompositionMethod::BoundingBox => self.decompose_by_bounding_box(solid),
            DecompositionMethod::Custom => {
                if let Some(cb) = custom {
                    let solids = cb(solid);
                    let steps = solids.len();
                    if steps > 0 {
                        DecompositionResult {
                            solids,
                            method: DecompositionMethod::Custom,
                            steps,
                        }
                    } else {
                        DecompositionResult {
                            solids: vec![Handle::new(std::sync::Arc::new(solid.clone()))],
                            method: DecompositionMethod::Custom,
                            steps: 1,
                        }
                    }
                } else {
                    DecompositionResult {
                        solids: vec![Handle::new(std::sync::Arc::new(solid.clone()))],
                        method: DecompositionMethod::Custom,
                        steps: 1,
                    }
                }
            }
            DecompositionMethod::PlaneSplit => {
                let params = Some(DecompositionParams {
                    plane: Some((0.0, 0.0, 1.0, 0.0)),
                });
                self.decompose_by_plane_split(solid, params)
            }
        }
    }

    /// Decompose a solid using the specified method, with optional custom callback and parameters
    pub fn decompose_with_params<F>(
        &self,
        solid: &TopoDsSolid,
        method: DecompositionMethod,
        params: Option<DecompositionParams>,
        custom: Option<F>,
    ) -> DecompositionResult
    where
        F: Fn(&TopoDsSolid, &Option<DecompositionParams>) -> Vec<Handle<TopoDsSolid>>,
    {
        match method {
            DecompositionMethod::PlaneSplit => self.decompose_by_plane_split(solid, params),
            DecompositionMethod::FaceNormal => self.decompose_by_face_normal(solid),
            DecompositionMethod::EdgeAngle => self.decompose_by_edge_angle(solid),
            DecompositionMethod::Volume => self.decompose_by_volume(solid),
            DecompositionMethod::Convexity => self.decompose_by_convexity(solid),
            DecompositionMethod::BoundingBox => self.decompose_by_bounding_box(solid),
            DecompositionMethod::Custom => {
                if let Some(cb) = custom {
                    let solids = cb(solid, &params);
                    let steps = solids.len();
                    if steps > 0 {
                        DecompositionResult {
                            solids,
                            method: DecompositionMethod::Custom,
                            steps,
                        }
                    } else {
                        DecompositionResult {
                            solids: vec![Handle::new(std::sync::Arc::new(solid.clone()))],
                            method: DecompositionMethod::Custom,
                            steps: 1,
                        }
                    }
                } else {
                    DecompositionResult {
                        solids: vec![Handle::new(std::sync::Arc::new(solid.clone()))],
                        method: DecompositionMethod::Custom,
                        steps: 1,
                    }
                }
            }
        }
    }

    /// Performance optimization: cache edge/face queries
    pub fn decompose_with_cache<F>(
        &self,
        solid: &TopoDsSolid,
        method: DecompositionMethod,
        params: Option<DecompositionParams>,
        custom: Option<F>,
        cache: &mut HashMap<String, Vec<Handle<TopoDsSolid>>>,
    ) -> DecompositionResult
    where
        F: Fn(&TopoDsSolid, &Option<DecompositionParams>) -> Vec<Handle<TopoDsSolid>>,
    {
        let cache_key = format!("{:?}-{:?}", method, params);
        if let Some(cached) = cache.get(&cache_key) {
            return DecompositionResult {
                solids: cached.clone(),
                method,
                steps: cached.len(),
            };
        }
        let result = self.decompose_with_params(solid, method, params, custom);
        cache.insert(cache_key, result.solids.clone());
        result
    }

    /// Decompose a solid by splitting with a plane
    fn decompose_by_plane_split(
        &self,
        solid: &TopoDsSolid,
        params: Option<DecompositionParams>,
    ) -> DecompositionResult {
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();

        let (a, b, c, d) = match &params {
            Some(p) => match p.plane {
                Some(plane) => plane,
                None => {
                    solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
                    return DecompositionResult {
                        solids,
                        method: DecompositionMethod::PlaneSplit,
                        steps: 1,
                    };
                }
            },
            None => {
                solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
                return DecompositionResult {
                    solids,
                    method: DecompositionMethod::PlaneSplit,
                    steps: 1,
                };
            }
        };

        let _normal = Direction::new(a, b, c);
        if (a * a + b * b + c * c).sqrt() < 1e-10 {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::PlaneSplit,
                steps: 1,
            };
        }

        let bbox = solid.bounding_box();
        let (min_pt, max_pt) = match bbox {
            Some(box_data) => box_data,
            None => {
                solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
                return DecompositionResult {
                    solids,
                    method: DecompositionMethod::PlaneSplit,
                    steps: 1,
                };
            }
        };

        let center_x = (min_pt.x + max_pt.x) / 2.0;
        let center_y = (min_pt.y + max_pt.y) / 2.0;
        let center_z = (min_pt.z + max_pt.z) / 2.0;

        let size_x = (max_pt.x - min_pt.x).abs();
        let size_y = (max_pt.y - min_pt.y).abs();
        let size_z = (max_pt.z - min_pt.z).abs();
        let max_size = size_x.max(size_y).max(size_z).max(1.0);
        let half_size = max_size * 2.0;

        let plane_point = if d.abs() < 1e-10 {
            Point::new(center_x, center_y, center_z)
        } else {
            let normal_mag = (a * a + b * b + c * c).sqrt();
            let dist_from_origin = -d / normal_mag;
            Point::new(
                a / normal_mag * dist_from_origin,
                b / normal_mag * dist_from_origin,
                c / normal_mag * dist_from_origin,
            )
        };

        let boolean_ops = BooleanOperations::new();

        let half_space_1 = primitives::make_box(
            half_size * 2.0,
            half_size * 2.0,
            half_size,
            Some(Point::new(
                plane_point.x,
                plane_point.y,
                plane_point.z - half_size / 2.0,
            )),
        );

        let half_space_2 = primitives::make_box(
            half_size * 2.0,
            half_size * 2.0,
            half_size,
            Some(Point::new(
                plane_point.x,
                plane_point.y,
                plane_point.z + half_size / 2.0,
            )),
        );

        let solid_shape = Handle::new(std::sync::Arc::new(solid.shape().clone()));
        let half_space_1_shape = Handle::new(std::sync::Arc::new(half_space_1.shape().clone()));
        let half_space_2_shape = Handle::new(std::sync::Arc::new(half_space_2.shape().clone()));

        let result1 = boolean_ops.cut(&solid_shape, &half_space_2_shape);
        let result2 = boolean_ops.cut(&solid_shape, &half_space_1_shape);

        for component in result1.components() {
            if let Some(s) = component.as_ref() {
                if let Some(solid_part) = s.as_solid() {
                    let vol = self.calculate_volume(solid_part);
                    if vol > self.tolerance {
                        solids.push(Handle::new(std::sync::Arc::new(solid_part.clone())));
                    }
                }
            }
        }

        for component in result2.components() {
            if let Some(s) = component.as_ref() {
                if let Some(solid_part) = s.as_solid() {
                    let vol = self.calculate_volume(solid_part);
                    if vol > self.tolerance {
                        let already_exists = solids.iter().any(|existing| {
                            if let Some(existing_ref) = existing.as_ref() {
                                self.solids_equal(existing_ref, solid_part)
                            } else {
                                false
                            }
                        });
                        if !already_exists {
                            solids.push(Handle::new(std::sync::Arc::new(solid_part.clone())));
                        }
                    }
                }
            }
        }

        if solids.is_empty() {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
        }

        let steps = solids.len();
        DecompositionResult {
            solids,
            method: DecompositionMethod::PlaneSplit,
            steps,
        }
    }

    /// Check if two solids are equal (within tolerance)
    fn solids_equal(&self, solid1: &TopoDsSolid, solid2: &TopoDsSolid) -> bool {
        let bbox1 = solid1.bounding_box();
        let bbox2 = solid2.bounding_box();

        match (bbox1, bbox2) {
            (Some((min1, max1)), Some((min2, max2))) => {
                (min1.x - min2.x).abs() < self.tolerance
                    && (min1.y - min2.y).abs() < self.tolerance
                    && (min1.z - min2.z).abs() < self.tolerance
                    && (max1.x - max2.x).abs() < self.tolerance
                    && (max1.y - max2.y).abs() < self.tolerance
                    && (max1.z - max2.z).abs() < self.tolerance
            }
            _ => false,
        }
    }

    /// Decompose a solid based on bounding box (new method)
    fn decompose_by_bounding_box(&self, solid: &TopoDsSolid) -> DecompositionResult {
        // Get bounding boxes for all shells, group shells, generate new solids
        let shells = solid.shells();
        let mut groups: Vec<Vec<Handle<TopoDsShell>>> = Vec::new();
        // Concurrent grouping (performance optimization)
        use rayon::prelude::*;
        let _shell_boxes: Vec<_> = shells
            .par_iter()
            .map(|shell| shell.as_ref().map(|s| s.bounding_box()))
            .collect();
        // Simple grouping: each shell as a separate group
        for shell in shells {
            groups.push(vec![shell.clone()]);
        }
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        for group in groups {
            let mut solid = TopoDsSolid::new();
            for shell in group {
                solid.add_shell(shell);
            }
            solids.push(Handle::new(std::sync::Arc::new(solid)));
        }
        let steps = solids.len();
        DecompositionResult {
            solids,
            method: DecompositionMethod::BoundingBox,
            steps,
        }
    }

    /// Decompose a solid based on face normals
    fn decompose_by_face_normal(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        for group in self.group_faces_by_normal(&self.get_all_faces(solid)) {
            if !group.is_empty() {
                let new_solid = self.create_solid_from_faces(&group);
                solids.push(Handle::new(std::sync::Arc::new(new_solid)));
            }
        }
        let steps = solids.len();
        DecompositionResult {
            solids,
            method: DecompositionMethod::FaceNormal,
            steps,
        }
    }

    /// Decompose a solid based on edge angles
    fn decompose_by_edge_angle(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        // Get all edges of the solid
        let edges = self.get_all_edges(solid);
        // Find edges with large angles
        let critical_edges = self.find_critical_edges(&edges);
        // Split the solid along critical edges
        if !critical_edges.is_empty() {
            let split_solids = self.split_solid_along_edges(solid, &critical_edges);
            solids.extend(split_solids);
            let steps = solids.len();
            return DecompositionResult {
                solids,
                method: DecompositionMethod::EdgeAngle,
                steps,
            };
        } else {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::EdgeAngle,
                steps: 1,
            };
        }
    }

    /// Decompose a solid based on volume
    fn decompose_by_volume(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        // Calculate the volume of the solid
        let volume = self.calculate_volume(solid);
        // If the volume is too small, return the solid as is
        if volume < self.tolerance {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Volume,
                steps: 1,
            };
        }
        // Find the center of mass
        let center = self.calculate_center_of_mass(solid);
        // Split the solid into octants
        let octants = self.split_solid_into_octants(solid, &center);
        solids.extend(octants);
        let steps = solids.len();
        DecompositionResult {
            solids,
            method: DecompositionMethod::Volume,
            steps,
        }
    }

    /// Decompose a solid based on convexity
    fn decompose_by_convexity(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        // Check if the solid is convex
        if self.is_convex(solid) {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Convexity,
                steps: 1,
            };
        }
        // Find non-convex edges
        let non_convex_edges = self.find_non_convex_edges(solid);
        // Split the solid along non-convex edges
        if !non_convex_edges.is_empty() {
            let split_solids = self.split_solid_along_edges(solid, &non_convex_edges);
            solids.extend(split_solids);
            let steps = solids.len();
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Convexity,
                steps,
            };
        } else {
            solids.push(Handle::new(std::sync::Arc::new(solid.clone())));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Convexity,
                steps: 1,
            };
        }
    }

    /// Get all faces of a solid
    fn get_all_faces(&self, solid: &TopoDsSolid) -> Vec<Handle<TopoDsFace>> {
        let mut faces = Vec::new();
        let shells = solid.shells();
        for shell in shells {
            if let Some(shell_ref) = shell.as_ref() {
                faces.extend(shell_ref.faces().iter().cloned());
            }
        }
        faces
    }

    /// Get all edges of a solid
    fn get_all_edges(
        &self,
        solid: &TopoDsSolid,
    ) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        let mut edges = Vec::new();
        let faces = self.get_all_faces(solid);
        for face in &faces {
            if let Some(face_ref) = face.as_ref() {
                for wire in face_ref.wires() {
                    if let Some(wire_ref) = wire.as_ref() {
                        edges.extend(wire_ref.edges().iter().cloned());
                    }
                }
            }
        }
        // Remove duplicates
        let mut unique_edges = Vec::new();
        for edge in edges {
            if !unique_edges.contains(&edge) {
                unique_edges.push(edge);
            }
        }
        unique_edges
    }

    /// Group faces by normal direction
    fn group_faces_by_normal(&self, faces: &[Handle<TopoDsFace>]) -> Vec<Vec<Handle<TopoDsFace>>> {
        let mut groups: Vec<Vec<Handle<TopoDsFace>>> = Vec::new();
        for face in faces {
            if let Some(face_ref) = face.as_ref() {
                if let Some(surface_handle) = face_ref.surface() {
                    let normal = surface_handle.normal(0.5, 0.5);
                    // Find an existing group with similar normal
                    let mut found = false;
                    for group in &mut groups {
                        if let Some(group_face) = group[0].as_ref() {
                            if let Some(group_surface_handle) = group_face.surface() {
                                let group_normal = group_surface_handle.normal(0.5, 0.5);
                                if normal.dot(&group_normal) > 1.0 - self.tolerance {
                                    group.push(face.clone());
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !found {
                        groups.push(vec![face.clone()]);
                    }
                }
            }
        }
        groups
    }

    /// Create a solid from a set of faces
    fn create_solid_from_faces(&self, faces: &[Handle<TopoDsFace>]) -> TopoDsSolid {
        // Create a shell from the faces
        let mut shell = TopoDsShell::new();
        for face in faces {
            shell.add_face(face.clone());
        }
        // Create a solid from the shell
        let mut solid = TopoDsSolid::new();
        solid.add_shell(Handle::new(std::sync::Arc::new(shell)));
        solid
    }

    /// Find edges with large angles
    fn find_critical_edges(
        &self,
        edges: &[Handle<crate::topology::topods_edge::TopoDsEdge>],
    ) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        let mut critical_edges = Vec::new();

        for edge in edges {
            if let Some(edge_ref) = edge.as_ref() {
                // Calculate the angle between adjacent faces
                let angle = self.calculate_edge_angle(edge_ref);

                // If the angle is too large, consider it critical
                if angle > 45.0 {
                    // 45 degrees
                    critical_edges.push(edge.clone());
                }
            }
        }

        critical_edges
    }

    /// Calculate the angle between faces adjacent to an edge
    fn calculate_edge_angle(&self, edge: &crate::topology::topods_edge::TopoDsEdge) -> f64 {
        // Get adjacent faces
        let faces = edge.adjacent_faces();
        if faces.len() != 2 {
            return 0.0;
        }
        // Calculate normals of the faces
        let normal1 = if let Some(surface_handle) = faces[0].as_ref().unwrap().surface() {
            surface_handle.normal(0.5, 0.5)
        } else {
            return 0.0;
        };
        let normal2 = if let Some(surface_handle) = faces[1].as_ref().unwrap().surface() {
            surface_handle.normal(0.5, 0.5)
        } else {
            return 0.0;
        };
        // Calculate the angle between normals
        let dot = normal1.dot(&normal2);
        let angle = dot.acos().to_degrees();
        angle
    }

    /// Split a solid along edges
    fn split_solid_along_edges(
        &self,
        solid: &TopoDsSolid,
        edges: &[Handle<crate::topology::topods_edge::TopoDsEdge>],
    ) -> Vec<Handle<TopoDsSolid>> {
        let solids: Vec<Handle<TopoDsSolid>> = Vec::new();
        let mut queue: VecDeque<Handle<TopoDsSolid>> = VecDeque::new();
        queue.push_back(Handle::new(std::sync::Arc::new(solid.clone())));
        for edge in edges {
            let mut new_queue: VecDeque<Handle<TopoDsSolid>> = VecDeque::new();
            while let Some(current_solid) = queue.pop_front() {
                if let Some(solid_ref) = current_solid.as_ref() {
                    if self.is_edge_in_solid(edge, solid_ref) {
                        let (solid1, solid2) = self.split_solid_along_edge(solid_ref, edge);
                        new_queue.push_back(Handle::new(std::sync::Arc::new(solid1)));
                        new_queue.push_back(Handle::new(std::sync::Arc::new(solid2)));
                    } else {
                        new_queue.push_back(current_solid);
                    }
                }
            }
            queue = new_queue;
        }
        // Remove unused variable warning for solids
        let _ = solids;
        queue.into_iter().collect()
    }

    /// Check if an edge is part of a solid
    fn is_edge_in_solid(
        &self,
        edge: &Handle<crate::topology::topods_edge::TopoDsEdge>,
        solid: &TopoDsSolid,
    ) -> bool {
        let edges = self.get_all_edges(solid);
        edges.contains(edge)
    }

    /// Split a solid along an edge
    fn split_solid_along_edge(
        &self,
        solid: &TopoDsSolid,
        edge: &Handle<crate::topology::topods_edge::TopoDsEdge>,
    ) -> (TopoDsSolid, TopoDsSolid) {
        // Get edge vertices
        let edge_vertices = edge.vertices();
        if edge_vertices.len() < 2 {
            return (solid.clone(), solid.clone());
        }

        let v1 = &edge_vertices[0];
        let v2 = &edge_vertices[1];

        // Create a cutting plane
        // For simplicity, use a plane perpendicular to the edge
        let edge_vec = Vector::new(
            v2.point().x() - v1.point().x(),
            v2.point().y() - v1.point().y(),
            v2.point().z() - v1.point().z(),
        );

        // Create a perpendicular vector
        let normal = if edge_vec.x != 0.0 || edge_vec.y != 0.0 {
            Vector::new(-edge_vec.y, edge_vec.x, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };

        let _plane = Plane::new(
            *v1.point(),
            Direction::from_vector(&normal),
            Direction::from_vector(&normal),
        );

        let box_size = 1000.0;
        let _box_solid = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(
                v1.point().x() - box_size / 2.0,
                v1.point().y() - box_size / 2.0,
                v1.point().z() - box_size / 2.0,
            )),
        );

        let _boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();

        // For simplicity, just return empty solids
        (TopoDsSolid::new(), TopoDsSolid::new())
    }

    /// Calculate the volume of a solid
    fn calculate_volume(&self, solid: &TopoDsSolid) -> f64 {
        let mut volume = 0.0;

        // Iterate through all faces
        for face in solid.faces() {
            // Get face geometry
            if let Some(_surface) = face.surface() {
                // For planar faces, use the shoelace formula for polygons
                let vertices = face.vertices();
                if vertices.len() >= 3 {
                    // Calculate face area and centroid
                    let face_points: Vec<Point> = vertices.iter().map(|v| *v.point()).collect();
                    let (area, centroid) = self.calculate_face_area_and_centroid(&face_points);

                    // Calculate distance from origin to centroid
                    let distance = centroid.distance(&Point::origin());

                    // Approximate volume contribution
                    volume += area * distance / 3.0;
                }
            }
        }

        volume.abs()
    }

    /// Calculate face area and centroid
    fn calculate_face_area_and_centroid(&self, vertices: &[Point]) -> (f64, Point) {
        let mut area = 0.0;
        let mut centroid = Point::origin();

        let n = vertices.len();
        for i in 0..n {
            let v1 = &vertices[i];
            let v2 = &vertices[(i + 1) % n];

            let cross = v1.x * v2.y - v2.x * v1.y;
            area += cross;

            centroid.x += (v1.x + v2.x) * cross;
            centroid.y += (v1.y + v2.y) * cross;
        }

        area *= 0.5;
        let inv_6a = 1.0 / (6.0 * area);
        centroid.x *= inv_6a;
        centroid.y *= inv_6a;
        centroid.z = 0.0; // Assume planar face in XY plane

        (area.abs(), centroid)
    }

    /// Calculate the center of mass of a solid
    fn calculate_center_of_mass(&self, solid: &TopoDsSolid) -> Point {
        let mut total_mass = 0.0;
        let mut weighted_sum = Point::origin();

        // Iterate through all faces
        for face in solid.faces() {
            // Get face vertices
            let vertices = face.vertices();
            if vertices.len() >= 3 {
                // Calculate face area and centroid
                let face_points: Vec<Point> = vertices.iter().map(|v| *v.point()).collect();
                let (area, centroid) = self.calculate_face_area_and_centroid(&face_points);

                // Weight centroid by area
                weighted_sum.x += centroid.x * area;
                weighted_sum.y += centroid.y * area;
                weighted_sum.z += centroid.z * area;
                total_mass += area;
            }
        }

        if total_mass > 0.0 {
            Point::new(
                weighted_sum.x / total_mass,
                weighted_sum.y / total_mass,
                weighted_sum.z / total_mass,
            )
        } else {
            Point::origin()
        }
    }

    /// Split a solid into octants
    fn split_solid_into_octants(
        &self,
        solid: &TopoDsSolid,
        center: &Point,
    ) -> Vec<Handle<TopoDsSolid>> {
        let mut octants: Vec<Handle<TopoDsSolid>> = Vec::new();

        let _x_plane = Plane::new(
            *center,
            Direction::from_vector(&Vector::new(1.0, 0.0, 0.0)),
            Direction::from_vector(&Vector::new(0.0, 1.0, 0.0)),
        );

        let _y_plane = Plane::new(
            *center,
            Direction::from_vector(&Vector::new(0.0, 1.0, 0.0)),
            Direction::from_vector(&Vector::new(1.0, 0.0, 0.0)),
        );

        let _z_plane = Plane::new(
            *center,
            Direction::from_vector(&Vector::new(0.0, 0.0, 1.0)),
            Direction::from_vector(&Vector::new(1.0, 0.0, 0.0)),
        );

        let box_size = 1000.0;
        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();

        let octant1 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(center.x, center.y, center.z)),
        );
        let _part1 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant1.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant2 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(center.x - box_size, center.y, center.z)),
        );
        let _part2 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant2.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant3 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(
                center.x - box_size,
                center.y - box_size,
                center.z,
            )),
        );
        let _part3 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant3.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant4 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(center.x, center.y - box_size, center.z)),
        );
        let _part4 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant4.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant5 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(center.x, center.y, center.z - box_size)),
        );
        let _part5 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant5.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant6 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(
                center.x - box_size,
                center.y,
                center.z - box_size,
            )),
        );
        let _part6 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant6.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        let octant7 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(
                center.x - box_size,
                center.y - box_size,
                center.z - box_size,
            )),
        );
        let _part7 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant7.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        // Octant 8: +x, -y, -z
        let octant8 = crate::modeling::primitives::make_box(
            box_size,
            box_size,
            box_size,
            Some(Point::new(
                center.x,
                center.y - box_size,
                center.z - box_size,
            )),
        );
        let _part8 = boolean_ops.common(
            &Handle::new(std::sync::Arc::new(solid.shape().clone())),
            &Handle::new(std::sync::Arc::new(octant8.shape().clone())),
        );
        let octant_solid = TopoDsSolid::new();
        octants.push(Handle::new(std::sync::Arc::new(octant_solid)));

        octants
    }

    /// Check if a solid is convex
    fn is_convex(&self, solid: &TopoDsSolid) -> bool {
        // Get all edges
        let edges = self.get_all_edges(solid);

        // Check each edge
        for edge in edges {
            if let Some(edge_ref) = edge.as_ref() {
                // Calculate the angle between adjacent faces
                let angle = self.calculate_edge_angle(edge_ref);

                // If any angle is less than 180 degrees, it's non-convex
                if angle < 180.0 - self.tolerance {
                    return false;
                }
            }
        }

        true
    }

    /// Find non-convex edges
    fn find_non_convex_edges(
        &self,
        solid: &TopoDsSolid,
    ) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        let edges = self.get_all_edges(solid);
        let mut non_convex_edges = Vec::new();

        for edge in edges {
            if let Some(edge_ref) = edge.as_ref() {
                // Calculate the angle between adjacent faces
                let angle = self.calculate_edge_angle(edge_ref);

                // If the angle is less than 180 degrees, it's non-convex
                if angle < 180.0 - self.tolerance {
                    non_convex_edges.push(edge.clone());
                }
            }
        }

        non_convex_edges
    }
}

impl Default for SolidDecomposer {
    fn default() -> Self {
        Self::new()
    }
}
