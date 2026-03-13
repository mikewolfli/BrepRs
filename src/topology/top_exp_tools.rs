use crate::topology::{
    ShapeType, TopoDsCompSolid, TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsShell,
    TopoDsSolid, TopoDsVertex, TopoDsWire,
};

/// Collection of tools for topological exploration
pub struct TopExpTools;

impl TopExpTools {
    /// Find all vertices in a shape
    pub fn vertices(shape: &TopoDsShape) -> Vec<TopoDsVertex> {
        let mut vertices = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_vertices(shape, &mut vertices, &mut visited);
        vertices
    }

    /// Recursively collect vertices from a shape
    fn collect_vertices(
        shape: &TopoDsShape,
        vertices: &mut Vec<TopoDsVertex>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Vertex => {
                // SAFETY: This is safe because we verified the shape is a vertex
                let vertex = unsafe { &*(shape as *const _ as *const TopoDsVertex) };
                vertices.push(vertex.clone());
            }
            ShapeType::Edge => {
                // SAFETY: This is safe because we verified the shape is an edge
                let edge = unsafe { &*(shape as *const _ as *const TopoDsEdge) };
                let v1 = edge.vertex1();
                let v2 = edge.vertex2();
                if !v1.is_null() {
                    Self::collect_vertices(v1.shape(), vertices, visited);
                }
                if !v2.is_null() {
                    Self::collect_vertices(v2.shape(), vertices, visited);
                }
            }
            ShapeType::Wire => {
                // SAFETY: This is safe because we verified the shape is a wire
                let wire = unsafe { &*(shape as *const _ as *const TopoDsWire) };
                for edge in wire.edges() {
                    if !edge.is_null() {
                        Self::collect_vertices(edge.shape(), vertices, visited);
                    }
                }
            }
            ShapeType::Face => {
                // SAFETY: This is safe because we verified the shape is a face
                let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
                for wire in face.wires() {
                    if !wire.is_null() {
                        Self::collect_vertices(wire.shape(), vertices, visited);
                    }
                }
            }
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                for face in shell.faces() {
                    if !face.is_null() {
                        Self::collect_vertices(face.shape(), vertices, visited);
                    }
                }
            }
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        Self::collect_vertices(shell.shape(), vertices, visited);
                    }
                }
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_vertices(comp_shape, vertices, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_vertices(solid.shape(), vertices, visited);
                    }
                }
            }
        }
    }

    /// Find all edges in a shape
    pub fn edges(shape: &TopoDsShape) -> Vec<TopoDsEdge> {
        let mut edges = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_edges(shape, &mut edges, &mut visited);
        edges
    }

    /// Recursively collect edges from a shape
    fn collect_edges(
        shape: &TopoDsShape,
        edges: &mut Vec<TopoDsEdge>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Edge => {
                // SAFETY: This is safe because we verified the shape is an edge
                let edge = unsafe { &*(shape as *const _ as *const TopoDsEdge) };
                edges.push(edge.clone());
            }
            ShapeType::Wire => {
                // SAFETY: This is safe because we verified the shape is a wire
                let wire = unsafe { &*(shape as *const _ as *const TopoDsWire) };
                for edge in wire.edges() {
                    if !edge.is_null() {
                        Self::collect_edges(edge.shape(), edges, visited);
                    }
                }
            }
            ShapeType::Face => {
                // SAFETY: This is safe because we verified the shape is a face
                let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
                for wire in face.wires() {
                    if !wire.is_null() {
                        Self::collect_edges(wire.shape(), edges, visited);
                    }
                }
            }
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                for face in shell.faces() {
                    if !face.is_null() {
                        Self::collect_edges(face.shape(), edges, visited);
                    }
                }
            }
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        Self::collect_edges(shell.shape(), edges, visited);
                    }
                }
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_edges(comp_shape, edges, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_edges(solid.shape(), edges, visited);
                    }
                }
            }
            ShapeType::Vertex => {}
        }
    }

    /// Find all wires in a shape
    pub fn wires(shape: &TopoDsShape) -> Vec<TopoDsWire> {
        let mut wires = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_wires(shape, &mut wires, &mut visited);
        wires
    }

    /// Recursively collect wires from a shape
    fn collect_wires(
        shape: &TopoDsShape,
        wires: &mut Vec<TopoDsWire>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Wire => {
                // SAFETY: This is safe because we verified the shape is a wire
                let wire = unsafe { &*(shape as *const _ as *const TopoDsWire) };
                wires.push(wire.clone());
            }
            ShapeType::Face => {
                // SAFETY: This is safe because we verified the shape is a face
                let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
                for wire in face.wires() {
                    if !wire.is_null() {
                        Self::collect_wires(wire.shape(), wires, visited);
                    }
                }
            }
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                for face in shell.faces() {
                    if !face.is_null() {
                        Self::collect_wires(face.shape(), wires, visited);
                    }
                }
            }
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        Self::collect_wires(shell.shape(), wires, visited);
                    }
                }
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_wires(comp_shape, wires, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_wires(solid.shape(), wires, visited);
                    }
                }
            }
            ShapeType::Vertex | ShapeType::Edge => {}
        }
    }

    /// Find all faces in a shape
    pub fn faces(shape: &TopoDsShape) -> Vec<TopoDsFace> {
        let mut faces = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_faces(shape, &mut faces, &mut visited);
        faces
    }

    /// Recursively collect faces from a shape
    fn collect_faces(
        shape: &TopoDsShape,
        faces: &mut Vec<TopoDsFace>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Face => {
                // SAFETY: This is safe because we verified the shape is a face
                let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
                faces.push(face.clone());
            }
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                for face in shell.faces() {
                    if !face.is_null() {
                        Self::collect_faces(face.shape(), faces, visited);
                    }
                }
            }
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        Self::collect_faces(shell.shape(), faces, visited);
                    }
                }
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_faces(comp_shape, faces, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_faces(solid.shape(), faces, visited);
                    }
                }
            }
            ShapeType::Vertex | ShapeType::Edge | ShapeType::Wire => {}
        }
    }

    /// Find all shells in a shape
    pub fn shells(shape: &TopoDsShape) -> Vec<TopoDsShell> {
        let mut shells = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_shells(shape, &mut shells, &mut visited);
        shells
    }

    /// Recursively collect shells from a shape
    fn collect_shells(
        shape: &TopoDsShape,
        shells: &mut Vec<TopoDsShell>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                shells.push(shell.clone());
            }
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        Self::collect_shells(shell.shape(), shells, visited);
                    }
                }
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_shells(comp_shape, shells, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_shells(solid.shape(), shells, visited);
                    }
                }
            }
            ShapeType::Vertex | ShapeType::Edge | ShapeType::Wire | ShapeType::Face => {}
        }
    }

    /// Find all solids in a shape
    pub fn solids(shape: &TopoDsShape) -> Vec<TopoDsSolid> {
        let mut solids = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_solids(shape, &mut solids, &mut visited);
        solids
    }

    /// Recursively collect solids from a shape
    fn collect_solids(
        shape: &TopoDsShape,
        solids: &mut Vec<TopoDsSolid>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                solids.push(solid.clone());
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_solids(comp_shape, solids, visited);
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        Self::collect_solids(solid.shape(), solids, visited);
                    }
                }
            }
            ShapeType::Vertex
            | ShapeType::Edge
            | ShapeType::Wire
            | ShapeType::Face
            | ShapeType::Shell => {}
        }
    }

    /// Find all compounds in a shape
    pub fn compounds(shape: &TopoDsShape) -> Vec<TopoDsCompound> {
        let mut compounds = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_compounds(shape, &mut compounds, &mut visited);
        compounds
    }

    /// Recursively collect compounds from a shape
    fn collect_compounds(
        shape: &TopoDsShape,
        compounds: &mut Vec<TopoDsCompound>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                compounds.push(compound.clone());

                // Recursively search inside this compound
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_compounds(comp_shape, compounds, visited);
                        }
                    }
                }
            }
            _ => {
                // Check if this shape is a component of a compound
                // For now, we just check direct sub-shapes
            }
        }
    }

    /// Find all compsolids in a shape
    pub fn compsolids(shape: &TopoDsShape) -> Vec<TopoDsCompSolid> {
        let mut compsolids = Vec::new();
        let mut visited = std::collections::HashSet::new();

        Self::collect_compsolids(shape, &mut compsolids, &mut visited);
        compsolids
    }

    /// Recursively collect compsolids from a shape
    fn collect_compsolids(
        shape: &TopoDsShape,
        compsolids: &mut Vec<TopoDsCompSolid>,
        visited: &mut std::collections::HashSet<i32>,
    ) {
        if visited.contains(&shape.shape_id()) {
            return;
        }
        visited.insert(shape.shape_id());

        match shape.shape_type() {
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                compsolids.push(compsolid.clone());
            }
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(comp_shape) = component.as_ref() {
                            Self::collect_compsolids(comp_shape, compsolids, visited);
                        }
                    }
                }
            }
            _ => {}
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

    /// Check if a shape contains any vertices
    pub fn has_vertices(shape: &TopoDsShape) -> bool {
        !Self::vertices(shape).is_empty()
    }

    /// Check if a shape contains any edges
    pub fn has_edges(shape: &TopoDsShape) -> bool {
        !Self::edges(shape).is_empty()
    }

    /// Check if a shape contains any wires
    pub fn has_wires(shape: &TopoDsShape) -> bool {
        !Self::wires(shape).is_empty()
    }

    /// Check if a shape contains any faces
    pub fn has_faces(shape: &TopoDsShape) -> bool {
        !Self::faces(shape).is_empty()
    }

    /// Check if a shape contains any shells
    pub fn has_shells(shape: &TopoDsShape) -> bool {
        !Self::shells(shape).is_empty()
    }

    /// Check if a shape contains any solids
    pub fn has_solids(shape: &TopoDsShape) -> bool {
        !Self::solids(shape).is_empty()
    }
}

/// Collection of topological analysis tools
pub struct TopToolsAnalyzer;

impl TopToolsAnalyzer {
    /// Check if a shape is a closed volume
    pub fn is_closed(shape: &TopoDsShape) -> bool {
        match shape.shape_type() {
            ShapeType::Solid => {
                // A solid is always closed
                true
            }
            ShapeType::Shell => {
                // Check if shell is closed
                let faces = TopExpTools::faces(shape);
                if faces.is_empty() {
                    return false;
                }

                // For a shell to be closed, every edge must be shared by two faces
                let edges = TopExpTools::edges(shape);
                let mut edge_counts = std::collections::HashMap::new();

                for face in &faces {
                    let face_edges = TopExpTools::edges(face.shape());
                    for edge in &face_edges {
                        *edge_counts.entry(edge.shape_id()).or_insert(0) += 1;
                    }
                }

                edges
                    .iter()
                    .all(|edge| edge_counts.get(&edge.shape_id()) == Some(&2))
            }
            _ => false,
        }
    }

    /// Check if a shape is manifold
    pub fn is_manifold(shape: &TopoDsShape) -> bool {
        match shape.shape_type() {
            ShapeType::Edge => {
                // An edge is always manifold
                true
            }
            ShapeType::Face => {
                // A face is always manifold
                true
            }
            ShapeType::Shell => {
                // Check if shell is manifold
                let edges = TopExpTools::edges(shape);
                let mut edge_counts = std::collections::HashMap::new();

                let faces = TopExpTools::faces(shape);
                for face in &faces {
                    let face_edges = TopExpTools::edges(face.shape());
                    for edge in &face_edges {
                        *edge_counts.entry(edge.shape_id()).or_insert(0) += 1;
                    }
                }

                // Each edge should be shared by at most two faces
                edges
                    .iter()
                    .all(|edge| edge_counts.get(&edge.shape_id()).unwrap_or(&0) <= &2)
            }
            ShapeType::Solid => {
                // A solid is always manifold
                true
            }
            _ => true,
        }
    }

    /// Check if a shape is connected
    pub fn is_connected(shape: &TopoDsShape) -> bool {
        match shape.shape_type() {
            ShapeType::Vertex | ShapeType::Edge | ShapeType::Face => {
                // These are always connected
                true
            }
            ShapeType::Wire => {
                // Check if wire is connected
                let edges = TopExpTools::edges(shape);
                if edges.len() <= 1 {
                    return true;
                }

                // Build adjacency list
                let mut adjacency = std::collections::HashMap::new();
                for edge in &edges {
                    let v1 = edge.vertex1();
                    let v2 = edge.vertex2();
                    adjacency
                        .entry(v1.shape_id())
                        .or_insert_with(Vec::new)
                        .push(v2.shape_id());
                    adjacency
                        .entry(v2.shape_id())
                        .or_insert_with(Vec::new)
                        .push(v1.shape_id());
                }

                // Perform BFS to check connectivity
                let start_vertex = edges[0].vertex1().shape_id();
                let mut visited = std::collections::HashSet::new();
                let mut queue = std::collections::VecDeque::new();

                queue.push_back(start_vertex);
                visited.insert(start_vertex);

                while let Some(current) = queue.pop_front() {
                    if let Some(neighbors) = adjacency.get(&current) {
                        for &neighbor in neighbors {
                            if !visited.contains(&neighbor) {
                                visited.insert(neighbor);
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }

                // Check if all vertices are visited
                let all_vertices = edges
                    .iter()
                    .flat_map(|edge| vec![edge.vertex1().shape_id(), edge.vertex2().shape_id()])
                    .collect::<std::collections::HashSet<_>>();

                visited == all_vertices
            }
            _ => {
                // For other types, assume connected for now
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundation::handle::Handle;
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
        // Create a simple wire with two edges
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let v3 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 1.0, 0.0))));
        let edge1 = Handle::new(Arc::new(TopoDsEdge::new(v1, v2.clone())));
        let edge2 = Handle::new(Arc::new(TopoDsEdge::new(v2, v3)));
        let wire = TopoDsWire::with_edges(vec![edge1, edge2]);

        let edges = TopExpTools::edges(wire.shape());
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_top_tools_analyzer_is_connected() {
        // Create a connected wire
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let v3 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 1.0, 0.0))));
        let edge1 = Handle::new(Arc::new(TopoDsEdge::new(v1, v2.clone())));
        let edge2 = Handle::new(Arc::new(TopoDsEdge::new(v2, v3)));
        let wire = TopoDsWire::with_edges(vec![edge1, edge2]);

        assert!(TopToolsAnalyzer::is_connected(wire.shape()));
    }

    #[test]
    fn test_top_tools_analyzer_is_closed() {
        // Create a solid (should be closed)
        let solid = TopoDsSolid::new();
        assert!(TopToolsAnalyzer::is_closed(solid.shape()));
    }

    #[test]
    fn test_top_tools_analyzer_is_manifold() {
        // Create a simple edge (should be manifold)
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        assert!(TopToolsAnalyzer::is_manifold(edge.shape()));
    }
}
