use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{topods_location::TopoDsLocation, topods_shape::TopoDsShape};
use serde::{Deserialize, Serialize};

/// Represents a compound in topological structure
///
/// A compound is a collection of shapes that can be of different types.
/// It's used to group shapes together without imposing any topological
/// constraints.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TopoDsCompound {
    shape: TopoDsShape,
    components: Vec<Handle<TopoDsShape>>,
}

impl TopoDsCompound {
    /// Create a new empty compound
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound),
            components: Vec::new(),
        }
    }

    /// Create a new compound with specified components
    pub fn with_components(components: Vec<Handle<TopoDsShape>>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound),
            components,
        }
    }

    /// Add a component to the compound
    pub fn add_component(&mut self, component: Handle<TopoDsShape>) {
        self.components.push(component);
    }

    /// Get the components of the compound
    pub fn components(&self) -> &[Handle<TopoDsShape>] {
        &self.components
    }

    /// Get mutable reference to the components of the compound
    pub fn components_mut(&mut self) -> &mut [Handle<TopoDsShape>] {
        &mut self.components
    }

    /// Get the number of components in the compound
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Get the location of the compound
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the compound
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if the compound is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Clear all components from the compound
    pub fn clear(&mut self) {
        self.components.clear();
    }

    /// Get the unique identifier of the compound
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the compound
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this compound is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the compound
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the compound contains a specific component
    pub fn contains(&self, component: &Handle<TopoDsShape>) -> bool {
        self.components.contains(component)
    }

    /// Remove a component from the compound
    pub fn remove_component(&mut self, component: &Handle<TopoDsShape>) {
        self.components.retain(|c| c != component);
    }

    /// Get the bounding box of the compound
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.components.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for component in &self.components {
            if let Some((min, max)) = self.component_bounding_box(component) {
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

    /// Get the bounding box of a component
    fn component_bounding_box(&self, component: &Handle<TopoDsShape>) -> Option<(Point, Point)> {
        // Check if component is a vertex
        if component.is_vertex() {
            // SAFETY: This is safe because we verified the shape is a vertex
            let vertex = unsafe {
                &*(component as *const _ as *const crate::topology::topods_vertex::TopoDsVertex)
            };
            let point = vertex.point();
            Some((*point, *point))
        }
        // Check if component is an edge
        else if component.is_edge() {
            // SAFETY: This is safe because we verified the shape is an edge
            let edge = unsafe {
                &*(component as *const _ as *const crate::topology::topods_edge::TopoDsEdge)
            };
            let v1 = edge.vertex1().point();
            let v2 = edge.vertex2().point();
            Some((
                Point::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z)),
                Point::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z)),
            ))
        }
        // Check if component is a wire
        else if component.is_wire() {
            // SAFETY: This is safe because we verified the shape is a wire
            let wire = unsafe {
                &*(component as *const _ as *const crate::topology::topods_wire::TopoDsWire)
            };
            wire.bounding_box()
        }
        // Check if component is a face
        else if component.is_face() {
            // SAFETY: This is safe because we verified the shape is a face
            let face = unsafe {
                &*(component as *const _ as *const crate::topology::topods_face::TopoDsFace)
            };
            face.bounding_box()
        }
        // Check if component is a shell
        else if component.is_shell() {
            // SAFETY: This is safe because we verified the shape is a shell
            let shell = unsafe {
                &*(component as *const _ as *const crate::topology::topods_shell::TopoDsShell)
            };
            shell.bounding_box()
        }
        // Check if component is a solid
        else if component.is_solid() {
            // SAFETY: This is safe because we verified the shape is a solid
            let solid = unsafe {
                &*(component as *const _ as *const crate::topology::topods_solid::TopoDsSolid)
            };
            solid.bounding_box()
        }
        // Check if component is a compound
        else if component.is_compound() {
            // SAFETY: This is safe because we verified the shape is a compound
            let compound = unsafe { &*(component as *const _ as *const TopoDsCompound) };
            compound.bounding_box()
        }
        // Check if component is a compsolid
        else if component.is_compsolid() {
            // SAFETY: This is safe because we verified the shape is a compsolid
            let compsolid = unsafe { &*(component as *const _ as *const TopoDsCompSolid) };
            compsolid.bounding_box()
        } else {
            None
        }
    }

    /// Get all components of a specific type
    pub fn components_of_type(
        &self,
        shape_type: crate::topology::shape_enum::ShapeType,
    ) -> Vec<Handle<TopoDsShape>> {
        self.components
            .iter()
            .filter(|c| c.shape_type() == shape_type)
            .cloned()
            .collect()
    }
}

impl Default for TopoDsCompound {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDsCompound {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            components: self.components.clone(),
        }
    }
}

impl PartialEq for TopoDsCompound {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

/// Represents a composite solid in topological structure
///
/// A composite solid is a collection of solids that are connected
/// by shared faces. It's used to represent assemblies of solids.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TopoDsCompSolid {
    shape: TopoDsShape,
    solids: Vec<Handle<crate::topology::topods_solid::TopoDsSolid>>,
}

impl TopoDsCompSolid {
    /// Create a new empty composite solid
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::CompSolid),
            solids: Vec::new(),
        }
    }

    /// Create a new composite solid with specified solids
    pub fn with_solids(solids: Vec<Handle<crate::topology::topods_solid::TopoDsSolid>>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::CompSolid),
            solids,
        }
    }

    /// Add a solid to the composite solid
    pub fn add_solid(&mut self, solid: Handle<crate::topology::topods_solid::TopoDsSolid>) {
        self.solids.push(solid);
    }

    /// Get the solids of the composite solid
    pub fn solids(&self) -> &[Handle<crate::topology::topods_solid::TopoDsSolid>] {
        &self.solids
    }

    /// Get mutable reference to the solids of the composite solid
    pub fn solids_mut(&mut self) -> &mut [Handle<crate::topology::topods_solid::TopoDsSolid>] {
        &mut self.solids
    }

    /// Get the number of solids in the composite solid
    pub fn num_solids(&self) -> usize {
        self.solids.len()
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Get the location of the composite solid
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the composite solid
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if the composite solid is empty
    pub fn is_empty(&self) -> bool {
        self.solids.is_empty()
    }

    /// Clear all solids from the composite solid
    pub fn clear(&mut self) {
        self.solids.clear();
    }

    /// Get the total volume of the composite solid
    pub fn volume(&self) -> f64 {
        self.solids.iter().map(|s| s.volume()).sum()
    }

    /// Get the total surface area of the composite solid
    pub fn area(&self) -> f64 {
        self.solids.iter().map(|s| s.area()).sum()
    }

    /// Get the unique identifier of the composite solid
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the composite solid
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this composite solid is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the composite solid
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the composite solid contains a specific solid
    pub fn contains(&self, solid: &Handle<crate::topology::topods_solid::TopoDsSolid>) -> bool {
        self.solids.contains(solid)
    }

    /// Get the bounding box of the composite solid
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.solids.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for solid in &self.solids {
            if let Some((min, max)) = solid.bounding_box() {
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

    /// Get all faces in the composite solid
    pub fn faces(&self) -> Vec<Handle<crate::topology::topods_face::TopoDsFace>> {
        use std::collections::HashSet;

        let mut face_set = HashSet::new();
        let mut faces = Vec::new();

        for solid in &self.solids {
            for face in solid.faces() {
                if face_set.insert(face.shape_id()) {
                    faces.push(face);
                }
            }
        }

        faces
    }

    /// Get all edges in the composite solid
    pub fn edges(&self) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        use std::collections::HashSet;

        let mut edge_set = HashSet::new();
        let mut edges = Vec::new();

        for solid in &self.solids {
            for edge in solid.edges() {
                if edge_set.insert(edge.shape_id()) {
                    edges.push(edge);
                }
            }
        }

        edges
    }

    /// Get all vertices in the composite solid
    pub fn vertices(&self) -> Vec<Handle<crate::topology::topods_vertex::TopoDsVertex>> {
        use std::collections::HashSet;

        let mut vertex_set = HashSet::new();
        let mut vertices = Vec::new();

        for solid in &self.solids {
            for vertex in solid.vertices() {
                if vertex_set.insert(vertex.shape_id()) {
                    vertices.push(vertex);
                }
            }
        }

        vertices
    }

    /// Get the number of faces in the composite solid
    pub fn num_faces(&self) -> usize {
        self.faces().len()
    }

    /// Get the number of edges in the composite solid
    pub fn num_edges(&self) -> usize {
        self.edges().len()
    }

    /// Get the number of vertices in the composite solid
    pub fn num_vertices(&self) -> usize {
        self.vertices().len()
    }
}

impl Default for TopoDsCompSolid {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDsCompSolid {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            solids: self.solids.clone(),
        }
    }
}

impl PartialEq for TopoDsCompSolid {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_creation() {
        let compound = TopoDsCompound::new();
        assert!(compound.is_empty());
        assert_eq!(compound.num_components(), 0);
    }

    #[test]
    fn test_compound_add_component() {
        let mut compound = TopoDsCompound::new();
        let shape = Handle::new(std::sync::Arc::new(TopoDsShape::new(
            crate::topology::shape_enum::ShapeType::Vertex,
        )));

        compound.add_component(shape);
        assert_eq!(compound.num_components(), 1);
    }

    #[test]
    fn test_compound_clear() {
        let shape = Handle::new(std::sync::Arc::new(TopoDsShape::new(
            crate::topology::shape_enum::ShapeType::Vertex,
        )));
        let mut compound = TopoDsCompound::with_components(vec![shape]);
        assert!(!compound.is_empty());

        compound.clear();
        assert!(compound.is_empty());
    }

    #[test]
    fn test_compound_shape_id() {
        let mut compound = TopoDsCompound::new();
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = compound.shape_id();
        assert!(initial_id > 0);

        compound.set_shape_id(42);
        assert_eq!(compound.shape_id(), 42);
    }

    #[test]
    fn test_compsolid_creation() {
        let compsolid = TopoDsCompSolid::new();
        assert!(compsolid.is_empty());
        assert_eq!(compsolid.num_solids(), 0);
    }

    #[test]
    fn test_compsolid_add_solid() {
        let mut compsolid = TopoDsCompSolid::new();
        let solid = Handle::new(std::sync::Arc::new(
            crate::topology::topods_solid::TopoDsSolid::new(),
        ));

        compsolid.add_solid(solid);
        assert_eq!(compsolid.num_solids(), 1);
    }

    #[test]
    fn test_compsolid_clear() {
        let solid = Handle::new(std::sync::Arc::new(
            crate::topology::topods_solid::TopoDsSolid::new(),
        ));
        let mut compsolid = TopoDsCompSolid::with_solids(vec![solid]);
        assert!(!compsolid.is_empty());

        compsolid.clear();
        assert!(compsolid.is_empty());
    }

    #[test]
    fn test_compsolid_shape_id() {
        let mut compsolid = TopoDsCompSolid::new();
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = compsolid.shape_id();
        assert!(initial_id > 0);

        compsolid.set_shape_id(42);
        assert_eq!(compsolid.shape_id(), 42);
    }
}
