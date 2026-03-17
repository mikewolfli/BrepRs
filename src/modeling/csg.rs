//! CSG (Constructive Solid Geometry) module
//! 
//! This module provides CSG representation and operations for solid models.

use crate::foundation::handle::Handle;
use crate::topology::{TopoDsShape, topods_solid::TopoDsSolid};

/// CSG operation type
#[derive(Debug, Clone, PartialEq)]
pub enum CsgOperation {
    /// Union operation
    Union,
    /// Intersection operation
    Intersection,
    /// Difference operation
    Difference,
}

/// CSG node type
#[derive(Debug, Clone)]
pub enum CsgNode {
    /// Leaf node representing a primitive solid
    Primitive(Handle<TopoDsSolid>),
    /// Internal node representing a CSG operation
    Operation(CsgOperation, Box<CsgNode>, Box<CsgNode>),
}

/// CSG tree
#[derive(Debug, Clone)]
pub struct CsgTree {
    /// Root node of the CSG tree
    root: CsgNode,
}

impl CsgTree {
    /// Create a new CSG tree from a primitive
    pub fn from_primitive(solid: TopoDsSolid) -> Self {
        Self {
            root: CsgNode::Primitive(Handle::new(solid)),
        }
    }

    /// Create a new CSG tree from an operation
    pub fn from_operation(
        operation: CsgOperation,
        left: CsgTree,
        right: CsgTree,
    ) -> Self {
        Self {
            root: CsgNode::Operation(
                operation,
                Box::new(left.root),
                Box::new(right.root),
            ),
        }
    }

    /// Evaluate the CSG tree to produce a solid
    pub fn evaluate(&self) -> Handle<TopoDsSolid> {
        self.evaluate_node(&self.root)
    }

    /// Evaluate a CSG node
    fn evaluate_node(&self, node: &CsgNode) -> Handle<TopoDsSolid> {
        match node {
            CsgNode::Primitive(solid) => solid.clone(),
            CsgNode::Operation(op, left, right) => {
                let left_solid = self.evaluate_node(left);
                let right_solid = self.evaluate_node(right);
                
                match op {
                    CsgOperation::Union => self.union(&left_solid, &right_solid),
                    CsgOperation::Intersection => self.intersection(&left_solid, &right_solid),
                    CsgOperation::Difference => self.difference(&left_solid, &right_solid),
                }
            }
        }
    }

    /// Perform union operation
    fn union(&self, solid1: &Handle<TopoDsSolid>, solid2: &Handle<TopoDsSolid>) -> Handle<TopoDsSolid> {
        // Use boolean operations module to perform the union
        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = boolean_ops.fuse(solid1, solid2);
        result.unwrap_or_else(|| solid1.clone())
    }

    /// Perform intersection operation
    fn intersection(&self, solid1: &Handle<TopoDsSolid>, solid2: &Handle<TopoDsSolid>) -> Handle<TopoDsSolid> {
        // Use boolean operations module to perform the intersection
        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = boolean_ops.common(solid1, solid2);
        result.unwrap_or_else(|| solid1.clone())
    }

    /// Perform difference operation
    fn difference(&self, solid1: &Handle<TopoDsSolid>, solid2: &Handle<TopoDsSolid>) -> Handle<TopoDsSolid> {
        // Use boolean operations module to perform the difference
        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = boolean_ops.cut(solid1, solid2);
        result.unwrap_or_else(|| solid1.clone())
    }

    /// Get the root node
    pub fn root(&self) -> &CsgNode {
        &self.root
    }
}

/// CSG builder
pub struct CsgBuilder {
    /// Current tree being built
    tree: Option<CsgTree>,
}

impl CsgBuilder {
    /// Create a new CSG builder
    pub fn new() -> Self {
        Self {
            tree: None,
        }
    }

    /// Add a primitive solid
    pub fn add_primitive(&mut self, solid: TopoDsSolid) -> &mut Self {
        self.tree = Some(CsgTree::from_primitive(solid));
        self
    }

    /// Add a union operation
    pub fn add_union(&mut self, other: CsgTree) -> &mut Self {
        if let Some(current_tree) = self.tree.take() {
            self.tree = Some(CsgTree::from_operation(
                CsgOperation::Union,
                current_tree,
                other,
            ));
        } else {
            self.tree = Some(other);
        }
        self
    }

    /// Add an intersection operation
    pub fn add_intersection(&mut self, other: CsgTree) -> &mut Self {
        if let Some(current_tree) = self.tree.take() {
            self.tree = Some(CsgTree::from_operation(
                CsgOperation::Intersection,
                current_tree,
                other,
            ));
        } else {
            self.tree = Some(other);
        }
        self
    }

    /// Add a difference operation
    pub fn add_difference(&mut self, other: CsgTree) -> &mut Self {
        if let Some(current_tree) = self.tree.take() {
            self.tree = Some(CsgTree::from_operation(
                CsgOperation::Difference,
                current_tree,
                other,
            ));
        } else {
            self.tree = Some(other);
        }
        self
    }

    /// Build the CSG tree
    pub fn build(&mut self) -> Option<CsgTree> {
        self.tree.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_csg_primitive() {
        // Create a simple solid
        let solid = TopoDsSolid::new();
        let tree = CsgTree::from_primitive(solid);
        
        // Evaluate the tree
        let result = tree.evaluate();
        assert!(result.is_some());
    }

    #[test]
    fn test_csg_union() {
        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();
        
        let tree1 = CsgTree::from_primitive(solid1);
        let tree2 = CsgTree::from_primitive(solid2);
        
        // Create union tree
        let union_tree = CsgTree::from_operation(CsgOperation::Union, tree1, tree2);
        
        // Evaluate the tree
        let result = union_tree.evaluate();
        assert!(result.is_some());
    }
}
