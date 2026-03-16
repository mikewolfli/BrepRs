use crate::foundation::handle::Handle;
use crate::topology::{topods_shape::TopoDsShape, topods_solid::TopoDsSolid};
use serde::{Deserialize, Serialize};

/// Represents a compsolid in topological structure
///
/// A compsolid is a collection of solids grouped together.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TopoDsCompSolid {
    shape: TopoDsShape,
    solids: Vec<Handle<TopoDsSolid>>,
}

impl TopoDsCompSolid {
    /// Create a new empty compsolid
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::CompSolid),
            solids: Vec::new(),
        }
    }

    /// Create a new compsolid with specified solids
    pub fn with_solids(solids: Vec<Handle<TopoDsSolid>>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::CompSolid),
            solids,
        }
    }

    /// Add a solid to the compsolid
    pub fn add_solid(&mut self, solid: Handle<TopoDsSolid>) {
        self.solids.push(solid);
    }

    /// Get the solids of the compsolid
    pub fn solids(&self) -> &[Handle<TopoDsSolid>] {
        &self.solids
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }
}
