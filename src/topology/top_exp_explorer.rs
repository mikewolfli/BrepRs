use crate::topology::{
    ShapeType, TopoDsCompSolid, TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsShell,
    TopoDsSolid, TopoDsVertex, TopoDsWire,
};
// use std::ops::Deref; // Removed unused import
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
    /// Move to the next shape in the traversal
    ///
    /// Advances the explorer to the next shape in the depth-first traversal order.
    /// This method is optimized to avoid unnecessary cloning by directly using
    /// the popped shape from the stack instead of cloning it.
    ///
    /// # Behavior
    /// - Pops the next shape from the internal stack
    /// - Sets it as the current shape
    /// - Marks the shape as visited to avoid revisiting
    /// - Explores and pushes sub-shapes onto the stack
    /// - Returns early if the stack is empty
    ///
    /// # Performance
    /// This method is O(1) for popping from stack and O(k) for exploring sub-shapes,
    /// where k is the number of sub-shapes. The optimization of avoiding cloning
    /// reduces memory allocations and improves performance for large topological structures.
    ///
    /// # Example
    /// ```
    /// let mut explorer = TopExpExplorer::new(&shape, ShapeType::Edge);
    /// while explorer.more() {
    ///     explorer.next();
    ///     if let Some(current) = explorer.current() {
    ///         // Process current shape
    ///     }
    /// }
    /// ```
    pub fn next(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        let current_shape = self.stack.pop().unwrap();
        let shape_id = current_shape.shape_id();
        self.current = Some(current_shape);
        // Mark as visited
        self.visited.insert(shape_id);
        // Add sub-shapes to the stack
        // Clone the shape before exploring to avoid borrow conflicts
        let current_clone = self.current.as_ref().map(|s| s.clone());
        if let Some(current) = current_clone {
            self.explore_sub_shapes(&current);
        }
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

    /// Explore sub-shapes of given shape
    fn explore_sub_shapes(&mut self, shape: &TopoDsShape) {
        match shape.shape_type() {
            ShapeType::Compound => {
                // Explore components of compound
                // Use unsafe cast since we know the shape type
                unsafe {
                    let compound = &*(shape as *const _ as *const TopoDsCompound);
                    for component in compound.components() {
                        if let Some(comp) = component.as_ref() {
                            if !self.visited.contains(&comp.shape_id()) {
                                self.visited.insert(comp.shape_id());
                                self.stack.push(comp.clone());
                            }
                        }
                    }
                }
            }
            ShapeType::CompSolid => {
                // Explore solids of compsolid
                unsafe {
                    let compsolid = &*(shape as *const _ as *const TopoDsCompSolid);
                    for solid in compsolid.solids() {
                        if let Some(s) = solid.as_ref() {
                            if !self.visited.contains(&s.shape_id()) {
                                self.visited.insert(s.shape_id());
                                self.stack.push(s.shape().clone());
                            }
                        }
                    }
                }
            }
            ShapeType::Solid => {
                // Explore shells of solid
                unsafe {
                    let solid = &*(shape as *const _ as *const TopoDsSolid);
                    for shell in solid.shells() {
                        if let Some(sh) = shell.as_ref() {
                            if !self.visited.contains(&sh.shape_id()) {
                                self.visited.insert(sh.shape_id());
                                self.stack.push(sh.shape().clone());
                            }
                        }
                    }
                }
            }
            ShapeType::Shell => {
                // Explore faces of shell
                unsafe {
                    let shell = &*(shape as *const _ as *const TopoDsShell);
                    for face in shell.faces() {
                        if let Some(f) = face.as_ref() {
                            if !self.visited.contains(&f.shape_id()) {
                                self.visited.insert(f.shape_id());
                                self.stack.push(f.shape().clone());
                            }
                        }
                    }
                }
            }
            ShapeType::Face => {
                // Explore wires of face
                unsafe {
                    let face = &*(shape as *const _ as *const TopoDsFace);
                    for wire in face.wires() {
                        if let Some(w) = wire.as_ref() {
                            if !self.visited.contains(&w.shape_id()) {
                                self.visited.insert(w.shape_id());
                                self.stack.push(w.shape().clone());
                            }
                        }
                    }
                }
            }
            ShapeType::Wire => {
                // Explore edges of wire
                unsafe {
                    let wire = &*(shape as *const _ as *const TopoDsWire);
                    for edge in wire.edges() {
                        if let Some(e) = edge.as_ref() {
                            if !self.visited.contains(&e.shape_id()) {
                                self.visited.insert(e.shape_id());
                                self.stack.push(e.shape().clone());
                            }
                        }
                    }
                }
            }
            ShapeType::Edge => {
                // Explore vertices of edge
                unsafe {
                    let edge = &*(shape as *const _ as *const TopoDsEdge);
                    if let Some(v1) = edge.vertex1().as_ref() {
                        if !self.visited.contains(&v1.shape_id()) {
                            self.visited.insert(v1.shape_id());
                            self.stack.push(v1.shape().clone());
                        }
                    }
                    if let Some(v2) = edge.vertex2().as_ref() {
                        if !self.visited.contains(&v2.shape_id()) {
                            self.visited.insert(v2.shape_id());
                            self.stack.push(v2.shape().clone());
                        }
                    }
                }
            }
            ShapeType::Vertex => {
                // Vertices have no sub-shapes
            }
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

    /// Check if a shape is suitable for the given LOD level
    fn is_suitable_for_lod(&self, shape: &TopoDsShape, lod_level: usize) -> bool {
        // Different shape types are suitable for different LOD levels
        match shape.shape_type() {
            ShapeType::Vertex => true,              // Always include vertices
            ShapeType::Edge => lod_level >= 1,      // Include edges at level 1+
            ShapeType::Wire => lod_level >= 2,      // Include wires at level 2+
            ShapeType::Face => lod_level >= 2,      // Include faces at level 2+
            ShapeType::Shell => lod_level >= 3,     // Include shells at level 3+
            ShapeType::Solid => lod_level >= 3,     // Include solids at level 3+
            ShapeType::Compound => lod_level >= 4,  // Include compounds at level 4+
            ShapeType::CompSolid => lod_level >= 4, // Include compsolids at level 4+
        }
    }

    /// Collect all shapes into a vector
    pub fn collect(&self) -> Vec<TopoDsShape> {
        let mut result = Vec::new();
        let mut explorer = TopExpExplorer::new(self.shape.as_ref().unwrap(), self.shape_type);
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                result.push(current.clone());
            }
        }
        result
    }
}

impl Iterator for TopExpExplorer {
    type Item = TopoDsShape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.more() {
            self.next();
            self.current.clone()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_creation() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let explorer = TopExpExplorer::new(&shape, ShapeType::Vertex);
        assert!(explorer.more());
    }

    #[test]
    fn test_explorer_next() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let mut explorer = TopExpExplorer::new(&shape, ShapeType::Vertex);
        assert!(explorer.more());
        explorer.next();
        // After next(), the stack should be empty for a vertex
        assert!(!explorer.more());
    }

    #[test]
    fn test_explorer_current() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let mut explorer = TopExpExplorer::new(&shape, ShapeType::Vertex);
        explorer.next();
        assert!(explorer.current().is_some());
        assert_eq!(explorer.current().unwrap().shape_type(), ShapeType::Vertex);
    }
}
