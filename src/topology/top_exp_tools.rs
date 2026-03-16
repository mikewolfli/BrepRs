use crate::topology::{
    ShapeType, TopoDsCompSolid, TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsShell,
    TopoDsSolid, TopoDsVertex, TopoDsWire,
};

/// Collection of tools for topological exploration
pub struct TopExpTools;

impl TopExpTools {
    /// Find all vertices in a shape
    pub fn vertices(shape: &TopoDsShape) -> Vec<TopoDsVertex> {
        let mut result = Vec::new();
        Self::collect_vertices_recursive(shape, &mut result);
        result
    }

    /// Recursively collect vertices from a shape
    fn collect_vertices_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsVertex>) {
        match shape.shape_type() {
            ShapeType::Vertex => {
                if let Some(vertex) = shape.as_vertex() {
                    result.push(vertex.clone());
                }
            }
            ShapeType::Edge => {
                if let Some(edge) = shape.as_edge() {
                    let mut start_id = None;
                    if let Some(start) = edge.start_vertex().get() {
                        start_id = Some(start.shape_id());
                        result.push(start.as_ref().clone());
                    }
                    if let Some(end) = edge.end_vertex().get() {
                        if start_id != Some(end.shape_id()) {
                            result.push(end.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Wire => {
                if let Some(wire) = shape.as_wire() {
                    for edge_handle in wire.edges() {
                        if let Some(edge) = edge_handle.get() {
                            Self::collect_vertices_recursive(edge.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    for wire_handle in face.wires() {
                        if let Some(wire) = wire_handle.get() {
                            Self::collect_vertices_recursive(wire.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            Self::collect_vertices_recursive(face.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            Self::collect_vertices_recursive(shell.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_vertices_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                // CompSolid not yet implemented - skip for now
            }
        }
    }

    /// Find all edges in a shape
    pub fn edges(shape: &TopoDsShape) -> Vec<TopoDsEdge> {
        let mut result = Vec::new();
        Self::collect_edges_recursive(shape, &mut result);
        result
    }

    /// Recursively collect edges from a shape
    fn collect_edges_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsEdge>) {
        match shape.shape_type() {
            ShapeType::Edge => {
                if let Some(edge) = shape.as_edge() {
                    result.push(edge.clone());
                }
            }
            ShapeType::Wire => {
                if let Some(wire) = shape.as_wire() {
                    for edge_handle in wire.edges() {
                        if let Some(edge) = edge_handle.get() {
                            result.push(edge.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    for wire_handle in face.wires() {
                        if let Some(wire) = wire_handle.get() {
                            Self::collect_edges_recursive(wire.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            Self::collect_edges_recursive(face.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            Self::collect_edges_recursive(shell.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_edges_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                // CompSolid not yet implemented - skip for now
            }
            _ => {}
        }
    }

    /// Find all wires in a shape
    pub fn wires(shape: &TopoDsShape) -> Vec<TopoDsWire> {
        let mut result = Vec::new();
        Self::collect_wires_recursive(shape, &mut result);
        result
    }

    /// Recursively collect wires from a shape
    fn collect_wires_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsWire>) {
        match shape.shape_type() {
            ShapeType::Wire => {
                if let Some(wire) = shape.as_wire() {
                    result.push(wire.clone());
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    for wire_handle in face.wires() {
                        if let Some(wire) = wire_handle.get() {
                            result.push(wire.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            Self::collect_wires_recursive(face.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            Self::collect_wires_recursive(shell.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_wires_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                if let Some(compsolid) = shape.as_compsolid() {
                    for solid_handle in compsolid.solids() {
                        if let Some(solid) = solid_handle.get() {
                            Self::collect_wires_recursive(solid.shape(), result);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Find all faces in a shape
    pub fn faces(shape: &TopoDsShape) -> Vec<TopoDsFace> {
        let mut result = Vec::new();
        Self::collect_faces_recursive(shape, &mut result);
        result
    }

    /// Recursively collect faces from a shape
    fn collect_faces_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsFace>) {
        match shape.shape_type() {
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    result.push(face.clone());
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            result.push(face.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            Self::collect_faces_recursive(shell.shape(), result);
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_faces_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                if let Some(compsolid) = shape.as_compsolid() {
                    for solid_handle in compsolid.solids() {
                        if let Some(solid) = solid_handle.get() {
                            Self::collect_faces_recursive(solid.shape(), result);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Find all shells in a shape
    pub fn shells(shape: &TopoDsShape) -> Vec<TopoDsShell> {
        let mut result = Vec::new();
        Self::collect_shells_recursive(shape, &mut result);
        result
    }

    /// Recursively collect shells from a shape
    fn collect_shells_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsShell>) {
        match shape.shape_type() {
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    result.push(shell.clone());
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            result.push(shell.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_shells_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                if let Some(compsolid) = shape.as_compsolid() {
                    for solid_handle in compsolid.solids() {
                        if let Some(solid) = solid_handle.get() {
                            Self::collect_shells_recursive(solid.shape(), result);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Find all solids in a shape
    pub fn solids(shape: &TopoDsShape) -> Vec<TopoDsSolid> {
        let mut result = Vec::new();
        Self::collect_solids_recursive(shape, &mut result);
        result
    }

    /// Recursively collect solids from a shape
    fn collect_solids_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsSolid>) {
        match shape.shape_type() {
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    result.push(solid.clone());
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component_handle in compound.components() {
                        Self::collect_solids_recursive(component_handle, result);
                    }
                }
            }
            ShapeType::CompSolid => {
                if let Some(compsolid) = shape.as_compsolid() {
                    for solid_handle in compsolid.solids() {
                        if let Some(solid) = solid_handle.get() {
                            result.push(solid.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Find all compounds in a shape
    pub fn compounds(shape: &TopoDsShape) -> Vec<TopoDsCompound> {
        let mut result = Vec::new();
        Self::collect_compounds_recursive(shape, &mut result);
        result
    }

    /// Recursively collect compounds from a shape
    fn collect_compounds_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsCompound>) {
        if let Some(compound) = shape.as_compound() {
            result.push(compound.clone());
            for component_handle in compound.components() {
                Self::collect_compounds_recursive(component_handle, result);
            }
        }
    }

    /// Find all compsolids in a shape
    pub fn compsolids(shape: &TopoDsShape) -> Vec<TopoDsCompSolid> {
        let mut result = Vec::new();
        Self::collect_compsolids_recursive(shape, &mut result);
        result
    }

    /// Recursively collect compsolids from a shape
    fn collect_compsolids_recursive(shape: &TopoDsShape, result: &mut Vec<TopoDsCompSolid>) {
        if let Some(compsolid) = shape.as_compsolid() {
            result.push(compsolid.clone());
        }
        if let Some(compound) = shape.as_compound() {
            for component_handle in compound.components() {
                Self::collect_compsolids_recursive(component_handle, result);
            }
        }
    }

    /// Count the number of vertices in a shape
    pub fn count_vertices(shape: &TopoDsShape) -> usize {
        Self::vertices(shape).len()
    }

    /// Count the number of edges in a shape
    pub fn count_edges(shape: &TopoDsShape) -> usize {
        Self::edges(shape).len()
    }

    /// Count the number of wires in a shape
    pub fn count_wires(shape: &TopoDsShape) -> usize {
        Self::wires(shape).len()
    }

    /// Count the number of faces in a shape
    pub fn count_faces(shape: &TopoDsShape) -> usize {
        Self::faces(shape).len()
    }

    /// Count the number of shells in a shape
    pub fn count_shells(shape: &TopoDsShape) -> usize {
        Self::shells(shape).len()
    }

    /// Count the number of solids in a shape
    pub fn count_solids(shape: &TopoDsShape) -> usize {
        Self::solids(shape).len()
    }

    /// Count the number of compounds in a shape
    pub fn count_compounds(shape: &TopoDsShape) -> usize {
        Self::compounds(shape).len()
    }

    /// Count the number of compsolids in a shape
    pub fn count_compsolids(shape: &TopoDsShape) -> usize {
        Self::compsolids(shape).len()
    }
}

/// Analyzer for topological shapes
pub struct TopToolsAnalyzer;

impl TopToolsAnalyzer {
    /// Check if a shape is connected
    pub fn is_connected(shape: &TopoDsShape) -> bool {
        // A shape is connected if all its sub-shapes are connected
        // For now, return true for simple shapes and check compounds
        match shape.shape_type() {
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    // A compound is connected if it has only one component
                    // or if all components share common boundaries
                    compound.components().len() <= 1
                } else {
                    true
                }
            }
            ShapeType::CompSolid => {
                if let Some(compsolid) = shape.as_compsolid() {
                    // A compsolid is connected if all solids share faces
                    compsolid.solids().len() <= 1
                } else {
                    true
                }
            }
            _ => true,
        }
    }

    /// Check if a shape is closed
    pub fn is_closed(shape: &TopoDsShape) -> bool {
        match shape.shape_type() {
            ShapeType::Wire => {
                if let Some(wire) = shape.as_wire() {
                    wire.is_closed()
                } else {
                    false
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    shell.is_closed()
                } else {
                    false
                }
            }
            ShapeType::Solid => true,
            _ => false,
        }
    }

    /// Check if a shape is manifold
    pub fn is_manifold(shape: &TopoDsShape) -> bool {
        // A shape is manifold if it has a well-defined neighborhood at every point
        // For now, check basic conditions
        match shape.shape_type() {
            ShapeType::Solid => true,
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    shell.is_closed()
                } else {
                    false
                }
            }
            ShapeType::Face => true,
            ShapeType::Edge => true,
            ShapeType::Vertex => true,
            _ => false,
        }
    }

    /// Check if a shape is oriented
    pub fn is_oriented(shape: &TopoDsShape) -> bool {
        // A shape is oriented if all its sub-shapes have consistent orientation
        match shape.shape_type() {
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    face.orientation() != 0
                } else {
                    false
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    shell
                        .faces()
                        .iter()
                        .all(|f| f.get().map(|face| face.orientation() != 0).unwrap_or(false))
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    /// Check if a shape is valid
    pub fn is_valid(shape: &TopoDsShape) -> bool {
        // A shape is valid if it meets basic topological requirements
        if shape.shape_id() <= 0 {
            return false;
        }

        match shape.shape_type() {
            ShapeType::Edge => {
                if let Some(edge) = shape.as_edge() {
                    edge.start_vertex().get().is_some() && edge.end_vertex().get().is_some()
                } else {
                    false
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    !face.wires().is_empty()
                } else {
                    false
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    !shell.faces().is_empty()
                } else {
                    false
                }
            }
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    !solid.shells().is_empty()
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    /// Get the complexity of a shape
    pub fn complexity(shape: &TopoDsShape) -> usize {
        // Complexity is the total number of sub-shapes
        TopExpTools::count_vertices(shape)
            + TopExpTools::count_edges(shape)
            + TopExpTools::count_faces(shape)
    }

    /// Get the bounding box of a shape
    pub fn bounding_box(shape: &TopoDsShape) -> Option<(Point, Point)> {
        let vertices = TopExpTools::vertices(shape);
        if vertices.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        for vertex in &vertices {
            let point = vertex.point();
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Get the center of mass of a shape
    pub fn center_of_mass(shape: &TopoDsShape) -> Option<Point> {
        let vertices = TopExpTools::vertices(shape);
        if vertices.is_empty() {
            return None;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for vertex in &vertices {
            let point = vertex.point();
            sum_x += point.x;
            sum_y += point.y;
            sum_z += point.z;
        }

        let count = vertices.len() as f64;
        Some(Point::new(sum_x / count, sum_y / count, sum_z / count))
    }

    /// Get the volume of a shape
    pub fn volume(shape: &TopoDsShape) -> f64 {
        if let Some(solid) = shape.as_solid() {
            solid.volume()
        } else {
            0.0
        }
    }

    /// Get the surface area of a shape
    pub fn surface_area(shape: &TopoDsShape) -> f64 {
        let faces = TopExpTools::faces(shape);
        let mut total_area = 0.0;

        for face in &faces {
            total_area += face.area();
        }

        total_area
    }

    /// Get the length of a shape
    pub fn length(shape: &TopoDsShape) -> f64 {
        let edges = TopExpTools::edges(shape);
        let mut total_length = 0.0;

        for edge in &edges {
            total_length += edge.length();
        }

        total_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::geometry::Point;
    use std::sync::Arc;

    #[test]
    fn test_top_exp_tools_vertices() {
        // Create a simple edge
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let vertices = TopExpTools::vertices(edge.shape());
        assert_eq!(vertices.len(), 2);
    }

    #[test]
    fn test_top_exp_tools_edges() {
        // Create a simple edge
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let edges = TopExpTools::edges(edge.shape());
        assert_eq!(edges.len(), 1);
    }

    #[test]
    fn test_top_tools_analyzer_validity() {
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        assert!(TopToolsAnalyzer::is_valid(edge.shape()));
    }

    #[test]
    fn test_top_tools_analyzer_complexity() {
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let complexity = TopToolsAnalyzer::complexity(edge.shape());
        assert!(complexity > 0);
    }

    #[test]
    fn test_top_tools_analyzer_bounding_box() {
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 1.0, 1.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let bbox = TopToolsAnalyzer::bounding_box(edge.shape());
        assert!(bbox.is_some());
    }

    #[test]
    fn test_top_tools_analyzer_center_of_mass() {
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let center = TopToolsAnalyzer::center_of_mass(edge.shape());
        assert!(center.is_some());
    }

    #[test]
    fn test_top_tools_analyzer_length() {
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        let length = TopToolsAnalyzer::length(edge.shape());
        assert!(length > 0.0);
    }
}
