use crate::topology::{
    ShapeType, TopoDsCompSolid, TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsShell,
    TopoDsSolid, TopoDsVertex, TopoDsWire,
};
// use std::ops::Deref; // 已移除未用import
use std::collections::HashSet;

/// Explorer for topological shapes
///
/// This class provides a way to explore the topology of a shape, allowing
/// traversal of sub-shapes of a specified type.
#[derive(Debug)]
pub struct TopExpExplorer {
    shape: Option<TopoDsShape>,
    shape_type: ShapeType,
    current: Option<TopoDsShape>,
    stack: Vec<TopoDsShape>,
    visited: HashSet<i32>, // shape_id
}

impl TopExpExplorer {
    /// Create a new explorer for the given shape and type
    pub fn new(shape: &TopoDsShape, shape_type: ShapeType) -> Self {
        let mut stack = Vec::new();
        stack.push(shape.clone());
        let mut visited = HashSet::new();
        visited.insert(shape.shape_id());
        Self {
            shape: Some(shape.clone()),
            shape_type,
            current: None,
            stack,
            visited,
        }
    }

    /// Reset the explorer to start traversal again
    pub fn init(&mut self, shape: &TopoDsShape, shape_type: ShapeType) {
        self.shape = Some(shape.clone());
        self.shape_type = shape_type;
        self.current = None;
        self.stack.clear();
        self.visited.clear();
        self.stack.push(shape.clone());
        self.visited.insert(shape.shape_id());
    }

    /// Check if there are more shapes to explore
    pub fn more(&self) -> bool {
        !self.stack.is_empty()
    }

    /// Move to the next shape
    pub fn next(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        let current_shape = self.stack.pop().unwrap();
        self.current = Some(current_shape.clone());
        // 标记已访问
        self.visited.insert(current_shape.shape_id());
        // Add sub-shapes to the stack
        self.explore_sub_shapes(&current_shape);
    }

    /// Get the current shape
    pub fn current(&self) -> Option<&TopoDsShape> {
        self.current.as_ref()
    }

    /// Get the current vertex (if current shape is a vertex)
    pub fn current_vertex(&self) -> Option<&TopoDsVertex> {
        if let Some(shape) = &self.current {
            if shape.is_vertex() {
                // SAFETY: This is safe because:
                // - We verified the shape is a vertex via is_vertex()
                // - TopoDsVertex is the concrete type for vertex shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsVertex)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current edge (if current shape is an edge)
    pub fn current_edge(&self) -> Option<&TopoDsEdge> {
        if let Some(shape) = &self.current {
            if shape.is_edge() {
                // SAFETY: This is safe because:
                // - We verified the shape is an edge via is_edge()
                // - TopoDsEdge is the concrete type for edge shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsEdge)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current wire (if current shape is a wire)
    pub fn current_wire(&self) -> Option<&TopoDsWire> {
        if let Some(shape) = &self.current {
            if shape.is_wire() {
                // SAFETY: This is safe because:
                // - We verified the shape is a wire via is_wire()
                // - TopoDsWire is the concrete type for wire shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsWire)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current face (if current shape is a face)
    pub fn current_face(&self) -> Option<&TopoDsFace> {
        if let Some(shape) = &self.current {
            if shape.is_face() {
                // SAFETY: This is safe because:
                // - We verified the shape is a face via is_face()
                // - TopoDsFace is the concrete type for face shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsFace)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current shell (if current shape is a shell)
    pub fn current_shell(&self) -> Option<&TopoDsShell> {
        if let Some(shape) = &self.current {
            if shape.is_shell() {
                // SAFETY: This is safe because:
                // - We verified the shape is a shell via is_shell()
                // - TopoDsShell is the concrete type for shell shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsShell)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current solid (if current shape is a solid)
    pub fn current_solid(&self) -> Option<&TopoDsSolid> {
        if let Some(shape) = &self.current {
            if shape.is_solid() {
                // SAFETY: This is safe because:
                // - We verified the shape is a solid via is_solid()
                // - TopoDsSolid is the concrete type for solid shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsSolid)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current compound (if current shape is a compound)
    pub fn current_compound(&self) -> Option<&TopoDsCompound> {
        if let Some(shape) = &self.current {
            if shape.is_compound() {
                // SAFETY: This is safe because:
                // - We verified the shape is a compound via is_compound()
                // - TopoDsCompound is the concrete type for compound shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsCompound)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the current compsolid (if current shape is a compsolid)
    pub fn current_compsolid(&self) -> Option<&TopoDsCompSolid> {
        if let Some(shape) = &self.current {
            if shape.is_compsolid() {
                // SAFETY: This is safe because:
                // - We verified the shape is a compsolid via is_compsolid()
                // - TopoDsCompSolid is the concrete type for compsolid shapes
                // - The pointer is valid and properly aligned
                // - The lifetime of the reference is tied to self
                unsafe { Some(&*(shape as *const _ as *const TopoDsCompSolid)) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Explore sub-shapes of the given shape
    fn explore_sub_shapes(&mut self, shape: &TopoDsShape) {
        // Explore sub-shapes based on the shape type
        match shape.shape_type() {
            // Edge has vertices as sub-shapes
            ShapeType::Edge => {
                // SAFETY: This is safe because we verified the shape is an edge
                let edge = unsafe { &*(shape as *const _ as *const TopoDsEdge) };
                let v1 = edge.vertex1();
                let v2 = edge.vertex2();

                if !v1.is_null() {
                    if let Some(vertex_ref) = v1.as_ref() {
                        self.stack.push(vertex_ref.shape().clone());
                    }
                }
                if !v2.is_null() {
                    if let Some(vertex_ref) = v2.as_ref() {
                        self.stack.push(vertex_ref.shape().clone());
                    }
                }
            }
            // Wire has edges as sub-shapes
            ShapeType::Wire => {
                // SAFETY: This is safe because we verified the shape is a wire
                let wire = unsafe { &*(shape as *const _ as *const TopoDsWire) };
                for edge in wire.edges() {
                    if !edge.is_null() {
                        if let Some(edge_ref) = edge.as_ref() {
                            self.stack.push(edge_ref.shape().clone());
                        }
                    }
                }
            }
            // Face has wires as sub-shapes
            ShapeType::Face => {
                // SAFETY: This is safe because we verified the shape is a face
                let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
                for wire in face.wires() {
                    if !wire.is_null() {
                        if let Some(wire_ref) = wire.as_ref() {
                            self.stack.push(wire_ref.shape().clone());
                        }
                    }
                }
            }
            // Shell has faces as sub-shapes
            ShapeType::Shell => {
                // SAFETY: This is safe because we verified the shape is a shell
                let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
                for face in shell.faces() {
                    if !face.is_null() {
                        if let Some(face_ref) = face.as_ref() {
                            self.stack.push(face_ref.shape().clone());
                        }
                    }
                }
            }
            // Solid has shells as sub-shapes
            ShapeType::Solid => {
                // SAFETY: This is safe because we verified the shape is a solid
                let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
                for shell in solid.shells() {
                    if !shell.is_null() {
                        if let Some(shell_ref) = shell.as_ref() {
                            self.stack.push(shell_ref.shape().clone());
                        }
                    }
                }
            }
            // Compound has components as sub-shapes
            ShapeType::Compound => {
                // SAFETY: This is safe because we verified the shape is a compound
                let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
                for component in compound.components() {
                    if !component.is_null() {
                        if let Some(shape_ref) = component.as_ref() {
                            self.stack.push(shape_ref.clone());
                        }
                    }
                }
            }
            // CompSolid has solids as sub-shapes
            ShapeType::CompSolid => {
                // SAFETY: This is safe because we verified the shape is a compsolid
                let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
                for solid in compsolid.solids() {
                    if !solid.is_null() {
                        if let Some(solid_ref) = solid.as_ref() {
                            self.stack.push(solid_ref.shape().clone());
                        }
                    }
                }
            }
            // Vertex has no sub-shapes
            ShapeType::Vertex => {}
        }
    }

    /// LOD-aware shape traversal
    pub fn explore_with_lod(&mut self, shape: &TopoDsShape, lod_level: usize) -> Vec<TopoDsShape> {
        // Implementation of LOD-aware shape traversal
        let mut result = Vec::new();
        let mut queue = Vec::new();
        let mut visited = std::collections::HashSet::new();

        queue.push(shape.clone());
        visited.insert(shape.shape_id());

        while let Some(current) = queue.pop() {
            if self.is_suitable_for_lod(&current, lod_level) {
                result.push(current.clone());

                // Add sub-shapes to the queue based on LOD level
                match current.shape_type() {
                    ShapeType::Edge => {
                        // For edges, add vertices only at lower LOD levels
                        if lod_level < 2 {
                            // Get vertices from edge
                            let explorer = TopExpExplorer::new(&current, ShapeType::Vertex);
                            let vertices: Vec<TopoDsShape> = explorer.collect();
                            for vertex in vertices {
                                if !visited.contains(&vertex.shape_id()) {
                                    visited.insert(vertex.shape_id());
                                    queue.push(vertex);
                                }
                            }
                        }
                    }
                    ShapeType::Wire => {
                        // For wires, add edges only at lower LOD levels
                        if lod_level < 2 {
                            // Get edges from wire
                            let explorer = TopExpExplorer::new(&current, ShapeType::Edge);
                            let edges: Vec<TopoDsShape> = explorer.collect();
                            for edge in edges {
                                if !visited.contains(&edge.shape_id()) {
                                    visited.insert(edge.shape_id());
                                    queue.push(edge);
                                }
                            }
                        }
                    }
                    ShapeType::Face => {
                        // For faces, add wires only at lower LOD levels
                        if lod_level < 3 {
                            // Get wires from face
                            let explorer = TopExpExplorer::new(&current, ShapeType::Wire);
                            let wires: Vec<TopoDsShape> = explorer.collect();
                            for wire in wires {
                                if !visited.contains(&wire.shape_id()) {
                                    visited.insert(wire.shape_id());
                                    queue.push(wire);
                                }
                            }
                        }
                    }
                    ShapeType::Shell => {
                        // For shells, add faces only at lower LOD levels
                        if lod_level < 3 {
                            // Get faces from shell
                            let explorer = TopExpExplorer::new(&current, ShapeType::Face);
                            let faces: Vec<TopoDsShape> = explorer.collect();
                            for face in faces {
                                if !visited.contains(&face.shape_id()) {
                                    visited.insert(face.shape_id());
                                    queue.push(face);
                                }
                            }
                        }
                    }
                    ShapeType::Solid => {
                        // For solids, add shells only at lower LOD levels
                        if lod_level < 4 {
                            // Get shells from solid
                            let explorer = TopExpExplorer::new(&current, ShapeType::Shell);
                            let shells: Vec<TopoDsShape> = explorer.collect();
                            for shell in shells {
                                if !visited.contains(&shell.shape_id()) {
                                    visited.insert(shell.shape_id());
                                    queue.push(shell);
                                }
                            }
                        }
                    }
                    ShapeType::Compound => {
                        // For compounds, add components based on LOD level
                        if lod_level < 4 {
                            // Get components from compound
                            let explorer = TopExpExplorer::new(&current, ShapeType::Compound);
                            let components: Vec<TopoDsShape> = explorer.collect();
                            for component in components {
                                if !visited.contains(&component.shape_id()) {
                                    visited.insert(component.shape_id());
                                    queue.push(component);
                                }
                            }
                        }
                    }
                    ShapeType::CompSolid => {
                        // For compsolids, add solids based on LOD level
                        if lod_level < 4 {
                            // Get solids from compsolid
                            let explorer = TopExpExplorer::new(&current, ShapeType::Solid);
                            let solids: Vec<TopoDsShape> = explorer.collect();
                            for solid in solids {
                                if !visited.contains(&solid.shape_id()) {
                                    visited.insert(solid.shape_id());
                                    queue.push(solid);
                                }
                            }
                        }
                    }
                    ShapeType::Vertex => {
                        // Vertices have no sub-shapes
                    }
                }
            }
        }

        result
    }

    /// LOD-aware shape simplification
    pub fn simplify_shape(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // Implementation of LOD-aware shape simplification
        match shape.shape_type() {
            ShapeType::Vertex => {
                // Vertices are already simple, no need to simplify
                Some(shape.clone())
            }
            ShapeType::Edge => {
                // For edges, we can simplify by reducing the number of control points
                if lod_level >= 2 {
                    // At higher LOD levels, keep the edge as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, we could potentially simplify the curve
                    // For now, return the original edge
                    Some(shape.clone())
                }
            }
            ShapeType::Wire => {
                // For wires, we can simplify by removing unnecessary edges
                if lod_level >= 3 {
                    // At higher LOD levels, keep the wire as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the wire
                    self.simplify_wire(shape, lod_level)
                }
            }
            ShapeType::Face => {
                // For faces, we can simplify by reducing the number of wires or simplifying the surface
                if lod_level >= 4 {
                    // At higher LOD levels, keep the face as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the face
                    self.simplify_face(shape, lod_level)
                }
            }
            ShapeType::Shell => {
                // For shells, we can simplify by removing unnecessary faces
                if lod_level >= 5 {
                    // At higher LOD levels, keep the shell as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the shell
                    self.simplify_shell(shape, lod_level)
                }
            }
            ShapeType::Solid => {
                // For solids, we can simplify by removing unnecessary shells
                if lod_level >= 6 {
                    // At higher LOD levels, keep the solid as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the solid
                    self.simplify_solid(shape, lod_level)
                }
            }
            ShapeType::Compound => {
                // For compounds, we can simplify by removing unnecessary components
                if lod_level >= 7 {
                    // At higher LOD levels, keep the compound as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the compound
                    self.simplify_compound(shape, lod_level)
                }
            }
            ShapeType::CompSolid => {
                // For compsolids, we can simplify by removing unnecessary solids
                if lod_level >= 8 {
                    // At higher LOD levels, keep the compsolid as is
                    Some(shape.clone())
                } else {
                    // At lower LOD levels, simplify the compsolid
                    self.simplify_compsolid(shape, lod_level)
                }
            }
        }
    }

    /// Simplify a wire by removing unnecessary edges
    fn simplify_wire(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a wire
        let wire = unsafe { &*(shape as *const _ as *const TopoDsWire) };
        let edges = wire.edges();

        if edges.len() <= 2 {
            // Wires with 2 or fewer edges are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only the most important edges
        let mut simplified_edges = Vec::new();
        let step = (lod_level + 1).min(edges.len());

        for i in (0..edges.len()).step_by(step) {
            simplified_edges.push(edges[i].clone());
        }

        if simplified_edges.is_empty() {
            Some(shape.clone())
        } else {
            let simplified_wire = TopoDsWire::with_edges(simplified_edges);
            Some(simplified_wire.shape().clone())
        }
    }

    /// Simplify a face by reducing the number of wires
    fn simplify_face(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a face
        let face = unsafe { &*(shape as *const _ as *const TopoDsFace) };
        let wires = face.wires();

        if wires.len() <= 1 {
            // Faces with only one wire are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only the outer wire
        let mut simplified_wires = Vec::new();
        if let Some(outer_wire) = face.outer_wire() {
            simplified_wires.push(outer_wire.clone());
        }

        // Add inner wires only at higher LOD levels
        if lod_level >= 1 && wires.len() > 1 {
            let inner_wires = wires.iter().skip(1).take(1); // Take only one inner wire for simplicity
            simplified_wires.extend(inner_wires.cloned());
        }

        // Create a new face with simplified wires
        let mut simplified_face = TopoDsFace::new();
        for (i, wire) in simplified_wires.iter().enumerate() {
            simplified_face.set_wire(i, wire.clone());
        }

        Some(simplified_face.shape().clone())
    }

    /// Simplify a shell by removing unnecessary faces
    fn simplify_shell(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a shell
        let shell = unsafe { &*(shape as *const _ as *const TopoDsShell) };
        let faces = shell.faces();

        if faces.len() <= 4 {
            // Shells with 4 or fewer faces are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only a subset of faces
        let mut simplified_faces = Vec::new();
        let step = (lod_level + 1).min(faces.len());

        for i in (0..faces.len()).step_by(step) {
            simplified_faces.push(faces[i].clone());
        }

        if simplified_faces.is_empty() {
            Some(shape.clone())
        } else {
            let mut simplified_shell = TopoDsShell::new();
            for face in simplified_faces {
                simplified_shell.add_face(face);
            }
            Some(simplified_shell.shape().clone())
        }
    }

    /// Simplify a solid by removing unnecessary shells
    fn simplify_solid(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a solid
        let solid = unsafe { &*(shape as *const _ as *const TopoDsSolid) };
        let shells = solid.shells();

        if shells.len() <= 1 {
            // Solids with only one shell are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only the outer shell
        let mut simplified_shells = Vec::new();
        if let Some(outer_shell) = solid.outer_shell() {
            simplified_shells.push(outer_shell.clone());
        }

        // Add inner shells only at higher LOD levels
        if lod_level >= 2 && shells.len() > 1 {
            let inner_shells = shells.iter().skip(1).take(1); // Take only one inner shell for simplicity
            simplified_shells.extend(inner_shells.cloned());
        }

        // Create a new solid with simplified shells
        let mut simplified_solid = TopoDsSolid::new();
        for shell in simplified_shells {
            simplified_solid.add_shell(shell);
        }

        Some(simplified_solid.shape().clone())
    }

    /// Simplify a compound by removing unnecessary components
    fn simplify_compound(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a compound
        let compound = unsafe { &*(shape as *const _ as *const TopoDsCompound) };
        let components = compound.components();

        if components.len() <= 2 {
            // Compounds with 2 or fewer components are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only a subset of components
        let mut simplified_components = Vec::new();
        let step = (lod_level + 1).min(components.len());

        for i in (0..components.len()).step_by(step) {
            simplified_components.push(components[i].clone());
        }

        if simplified_components.is_empty() {
            Some(shape.clone())
        } else {
            let mut simplified_compound = TopoDsCompound::new();
            for component in simplified_components {
                simplified_compound.add_component(component);
            }
            Some(simplified_compound.shape().clone())
        }
    }

    /// Simplify a compsolid by removing unnecessary solids
    fn simplify_compsolid(&self, shape: &TopoDsShape, lod_level: usize) -> Option<TopoDsShape> {
        // SAFETY: This is safe because we verified the shape is a compsolid
        let compsolid = unsafe { &*(shape as *const _ as *const TopoDsCompSolid) };
        let solids = compsolid.solids();

        if solids.len() <= 2 {
            // Compsolids with 2 or fewer solids are already simple
            return Some(shape.clone());
        }

        // For lower LOD levels, keep only a subset of solids
        let mut simplified_solids = Vec::new();
        let step = (lod_level + 1).min(solids.len());

        for i in (0..solids.len()).step_by(step) {
            simplified_solids.push(solids[i].clone());
        }

        if simplified_solids.is_empty() {
            Some(shape.clone())
        } else {
            let mut simplified_compsolid = TopoDsCompSolid::new();
            for solid in simplified_solids {
                simplified_compsolid.add_solid(solid);
            }
            Some(simplified_compsolid.shape().clone())
        }
    }

    /// Calculate LOD level based on distance
    pub fn calculate_lod_level(&self, shape: &TopoDsShape, distance: f64) -> usize {
        // Implementation of LOD level calculation based on distance
        // Consider both distance and shape size
        let (min_point, max_point) = shape.bounding_box();
        let shape_size = ((max_point.x - min_point.x)
            .max(max_point.y - min_point.y)
            .max(max_point.z - min_point.z))
        .max(0.001);

        // Calculate relative size: shape size compared to distance
        let relative_size = shape_size / distance.max(0.001);

        // Determine LOD level based on relative size
        if relative_size > 0.5 {
            // Very close, use highest detail
            0
        } else if relative_size > 0.2 {
            // Close, use high detail
            1
        } else if relative_size > 0.1 {
            // Medium distance, use medium detail
            2
        } else if relative_size > 0.05 {
            // Far, use low detail
            3
        } else if relative_size > 0.01 {
            // Very far, use very low detail
            4
        } else if relative_size > 0.001 {
            // Extremely far, use minimal detail
            5
        } else {
            // Too far to see details
            6
        }
    }

    /// Check if shape is suitable for given LOD level
    pub fn is_suitable_for_lod(&self, shape: &TopoDsShape, lod_level: usize) -> bool {
        // Implementation of LOD suitability check
        // Determine if the shape should be included based on its type and the LOD level
        match shape.shape_type() {
            ShapeType::Vertex => {
                // Vertices are only included at the highest detail levels
                lod_level <= 1
            }
            ShapeType::Edge => {
                // Edges are included at high to medium detail levels
                lod_level <= 2
            }
            ShapeType::Wire => {
                // Wires are included at medium detail levels
                lod_level <= 3
            }
            ShapeType::Face => {
                // Faces are included at medium to low detail levels
                lod_level <= 4
            }
            ShapeType::Shell => {
                // Shells are included at low detail levels
                lod_level <= 5
            }
            ShapeType::Solid => {
                // Solids are included at very low detail levels
                lod_level <= 6
            }
            ShapeType::Compound => {
                // Compounds are included at all detail levels
                true
            }
            ShapeType::CompSolid => {
                // Compsolids are included at all detail levels
                true
            }
        }
    }
}

impl Iterator for TopExpExplorer {
    type Item = TopoDsShape;

    fn next(&mut self) -> Option<Self::Item> {
        // First call - need to advance to first element
        if self.current.is_none() && self.more() {
            TopExpExplorer::next(self);
        }

        if self.more() {
            let current = self.current.clone();
            TopExpExplorer::next(self);
            current
        } else {
            None
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
    fn test_explorer_vertices() {
        // Create a simple edge
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        // Create explorer for vertices
        let mut explorer = TopExpExplorer::new(edge.shape(), ShapeType::Vertex);

        // Should find two vertices (edge itself is not counted, only its sub-shapes)
        let mut count = 0;
        loop {
            if !explorer.more() {
                break;
            }
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Vertex {
                    count += 1;
                }
            }
        }
        // Edge has 2 vertices as sub-shapes
        assert_eq!(count, 2);
    }

    #[test]
    fn test_explorer_edges() {
        // Create a simple wire with two edges
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let v3 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 1.0, 0.0))));
        let edge1 = Handle::new(Arc::new(TopoDsEdge::new(v1, v2.clone())));
        let edge2 = Handle::new(Arc::new(TopoDsEdge::new(v2, v3)));
        let wire = TopoDsWire::with_edges(vec![edge1, edge2]);

        // Create explorer for edges
        let mut explorer = TopExpExplorer::new(wire.shape(), ShapeType::Edge);

        // Should find two edges (wire itself is not counted, only its sub-shapes)
        let mut count = 0;
        loop {
            if !explorer.more() {
                break;
            }
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Edge {
                    count += 1;
                }
            }
        }
        // Wire has 2 edges as sub-shapes
        assert_eq!(count, 2);
    }

    #[test]
    fn test_explorer_as_iterator() {
        // Create a simple edge
        let v1 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = TopoDsEdge::new(v1, v2);

        // Use explorer as iterator
        let explorer = TopExpExplorer::new(edge.shape(), ShapeType::Vertex);
        let vertices: Vec<TopoDsShape> = explorer.collect();

        // Edge has 2 vertices as sub-shapes
        assert_eq!(vertices.len(), 2);
    }
}
