//! Solid decomposition module
//! 
//! This module provides functionality for breaking down complex solids into simpler parts,
//! which is useful for various geometric operations and analysis.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{TopoDsShape, topods_face::TopoDsFace, topods_solid::TopoDsSolid, topods_shell::TopoDsShell};
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
}

/// Solid decomposer
pub struct SolidDecomposer {
    /// Tolerance for decomposition
    tolerance: f64,
    /// Maximum number of decomposition steps
    max_steps: usize,
}

impl SolidDecomposer {
    /// Create a new solid decomposer with default parameters
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
            max_steps: 100,
        }
    }

    /// Create a new solid decomposer with custom parameters
    pub fn with_parameters(tolerance: f64, max_steps: usize) -> Self {
        Self {
            tolerance,
            max_steps,
        }
    }

    /// Decompose a solid using the specified method
    pub fn decompose(&self, solid: &TopoDsSolid, method: DecompositionMethod) -> DecompositionResult {
        match method {
            DecompositionMethod::FaceNormal => self.decompose_by_face_normal(solid),
            DecompositionMethod::EdgeAngle => self.decompose_by_edge_angle(solid),
            DecompositionMethod::Volume => self.decompose_by_volume(solid),
            DecompositionMethod::Convexity => self.decompose_by_convexity(solid),
        }
    }

    /// Decompose a solid based on face normals
    fn decompose_by_face_normal(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids = Vec::new();
        let mut steps = 0;
        
        // Get all faces of the solid
        let faces = self.get_all_faces(solid);
        
        // Group faces by normal direction
        let face_groups = self.group_faces_by_normal(&faces);
        
        // Create a solid for each group
        for group in face_groups {
            if !group.is_empty() {
                let new_solid = self.create_solid_from_faces(&group);
                solids.push(Handle::new(new_solid));
                steps += 1;
            }
        }
        
        DecompositionResult {
            solids,
            method: DecompositionMethod::FaceNormal,
            steps,
        }
    }

    /// Decompose a solid based on edge angles
    fn decompose_by_edge_angle(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids = Vec::new();
        let mut steps = 0;
        
        // Get all edges of the solid
        let edges = self.get_all_edges(solid);
        
        // Find edges with large angles
        let critical_edges = self.find_critical_edges(&edges);
        
        // Split the solid along critical edges
        if !critical_edges.is_empty() {
            let split_solids = self.split_solid_along_edges(solid, &critical_edges);
            solids.extend(split_solids);
            steps += split_solids.len();
        } else {
            solids.push(Handle::new(solid.clone()));
        }
        
        DecompositionResult {
            solids,
            method: DecompositionMethod::EdgeAngle,
            steps,
        }
    }

    /// Decompose a solid based on volume
    fn decompose_by_volume(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids = Vec::new();
        let mut steps = 0;
        
        // Calculate the volume of the solid
        let volume = self.calculate_volume(solid);
        
        // If the volume is too small, return the solid as is
        if volume < self.tolerance {
            solids.push(Handle::new(solid.clone()));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Volume,
                steps,
            };
        }
        
        // Find the center of mass
        let center = self.calculate_center_of_mass(solid);
        
        // Split the solid into octants
        let octants = self.split_solid_into_octants(solid, &center);
        solids.extend(octants);
        steps += octants.len();
        
        DecompositionResult {
            solids,
            method: DecompositionMethod::Volume,
            steps,
        }
    }

    /// Decompose a solid based on convexity
    fn decompose_by_convexity(&self, solid: &TopoDsSolid) -> DecompositionResult {
        let mut solids = Vec::new();
        let mut steps = 0;
        
        // Check if the solid is convex
        if self.is_convex(solid) {
            solids.push(Handle::new(solid.clone()));
            return DecompositionResult {
                solids,
                method: DecompositionMethod::Convexity,
                steps,
            };
        }
        
        // Find non-convex edges
        let non_convex_edges = self.find_non_convex_edges(solid);
        
        // Split the solid along non-convex edges
        if !non_convex_edges.is_empty() {
            let split_solids = self.split_solid_along_edges(solid, &non_convex_edges);
            solids.extend(split_solids);
            steps += split_solids.len();
        } else {
            solids.push(Handle::new(solid.clone()));
        }
        
        DecompositionResult {
            solids,
            method: DecompositionMethod::Convexity,
            steps,
        }
    }

    /// Get all faces of a solid
    fn get_all_faces(&self, solid: &TopoDsSolid) -> Vec<Handle<TopoDsFace>> {
        let mut faces = Vec::new();
        let shells = solid.shells();
        
        for shell in shells {
            if let Some(shell_ref) = shell.as_ref() {
                let shell_faces = shell_ref.faces();
                faces.extend(shell_faces);
            }
        }
        
        faces
    }

    /// Get all edges of a solid
    fn get_all_edges(&self, solid: &TopoDsSolid) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        let mut edges = Vec::new();
        let faces = self.get_all_faces(solid);
        
        for face in &faces {
            if let Some(face_ref) = face.as_ref() {
                let wires = face_ref.wires();
                for wire in &wires {
                    if let Some(wire_ref) = wire.as_ref() {
                        let wire_edges = wire_ref.edges();
                        edges.extend(wire_edges);
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
        let mut groups = Vec::new();
        
        for face in faces {
            if let Some(face_ref) = face.as_ref() {
                let normal = face_ref.normal(0.5, 0.5);
                
                // Find an existing group with similar normal
                let mut found = false;
                for group in &mut groups {
                    if let Some(group_face) = group[0].as_ref() {
                        let group_normal = group_face.normal(0.5, 0.5);
                        if normal.dot(&group_normal) > 1.0 - self.tolerance {
                            group.push(face.clone());
                            found = true;
                            break;
                        }
                    }
                }
                
                if !found {
                    groups.push(vec![face.clone()]);
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
        solid.add_shell(Handle::new(shell));
        
        solid
    }

    /// Find edges with large angles
    fn find_critical_edges(&self, edges: &[Handle<crate::topology::topods_edge::TopoDsEdge>]) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        let mut critical_edges = Vec::new();
        
        for edge in edges {
            if let Some(edge_ref) = edge.as_ref() {
                // Calculate the angle between adjacent faces
                let angle = self.calculate_edge_angle(edge_ref);
                
                // If the angle is too large, consider it critical
                if angle > 45.0 { // 45 degrees
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
        let normal1 = faces[0].as_ref().unwrap().normal(0.5, 0.5);
        let normal2 = faces[1].as_ref().unwrap().normal(0.5, 0.5);
        
        // Calculate the angle between normals
        let dot = normal1.dot(&normal2);
        let angle = dot.acos().to_degrees();
        
        angle
    }

    /// Split a solid along edges
    fn split_solid_along_edges(&self, solid: &TopoDsSolid, edges: &[Handle<crate::topology::topods_edge::TopoDsEdge>]) -> Vec<Handle<TopoDsSolid>> {
        let mut solids = Vec::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(Handle::new(solid.clone()));
        
        for edge in edges {
            let mut new_queue = VecDeque::new();
            
            while let Some(current_solid) = queue.pop_front() {
                if let Some(solid_ref) = current_solid.as_ref() {
                    // Check if the edge is part of the solid
                    if self.is_edge_in_solid(edge, solid_ref) {
                        // Split the solid along the edge
                        let (solid1, solid2) = self.split_solid_along_edge(solid_ref, edge);
                        new_queue.push_back(Handle::new(solid1));
                        new_queue.push_back(Handle::new(solid2));
                    } else {
                        new_queue.push_back(current_solid);
                    }
                }
            }
            
            queue = new_queue;
        }
        
        queue.into_iter().collect()
    }

    /// Check if an edge is part of a solid
    fn is_edge_in_solid(&self, edge: &Handle<crate::topology::topods_edge::TopoDsEdge>, solid: &TopoDsSolid) -> bool {
        let edges = self.get_all_edges(solid);
        edges.contains(edge)
    }

    /// Split a solid along an edge
    fn split_solid_along_edge(&self, solid: &TopoDsSolid, edge: &Handle<crate::topology::topods_edge::TopoDsEdge>) -> (TopoDsSolid, TopoDsSolid) {
        // For simplicity, return two copies of the solid
        // In a real implementation, this would perform an actual split
        (solid.clone(), solid.clone())
    }

    /// Calculate the volume of a solid
    fn calculate_volume(&self, solid: &TopoDsSolid) -> f64 {
        // For simplicity, return a dummy value
        // In a real implementation, this would calculate the actual volume
        1.0
    }

    /// Calculate the center of mass of a solid
    fn calculate_center_of_mass(&self, solid: &TopoDsSolid) -> Point {
        // For simplicity, return the origin
        // In a real implementation, this would calculate the actual center of mass
        Point::origin()
    }

    /// Split a solid into octants
    fn split_solid_into_octants(&self, solid: &TopoDsSolid, center: &Point) -> Vec<Handle<TopoDsSolid>> {
        let mut octants = Vec::new();
        
        // For simplicity, return the original solid
        // In a real implementation, this would perform an actual split
        octants.push(Handle::new(solid.clone()));
        
        octants
    }

    /// Check if a solid is convex
    fn is_convex(&self, solid: &TopoDsSolid) -> bool {
        // For simplicity, return true
        // In a real implementation, this would check actual convexity
        true
    }

    /// Find non-convex edges
    fn find_non_convex_edges(&self, solid: &TopoDsSolid) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_decompose_by_face_normal() {
        // Create a simple solid
        let solid = TopoDsSolid::new();
        
        let decomposer = SolidDecomposer::new();
        let result = decomposer.decompose(&solid, DecompositionMethod::FaceNormal);
        
        assert!(!result.solids.is_empty());
        assert_eq!(result.method, DecompositionMethod::FaceNormal);
    }

    #[test]
    fn test_decompose_by_convexity() {
        // Create a simple solid
        let solid = TopoDsSolid::new();
        
        let decomposer = SolidDecomposer::new();
        let result = decomposer.decompose(&solid, DecompositionMethod::Convexity);
        
        assert!(!result.solids.is_empty());
        assert_eq!(result.method, DecompositionMethod::Convexity);
    }
}
