use std::collections::HashMap;
use std::sync::Arc;

use crate::foundation::handle::Handle;
use crate::topology::topods_shape::TopoDsShape;

/// Topological naming system to track shape modifications
pub struct TopoDsNaming {
    /// Map of shape IDs to their names
    name_map: HashMap<usize, String>,
    /// Map of names to shape IDs
    id_map: HashMap<String, usize>,
    /// History of shape modifications
    history: Vec<NamingEvent>,
}

/// Naming event types
pub enum NamingEvent {
    /// Shape created
    Created { shape_id: usize, name: String },
    /// Shape renamed
    Renamed {
        shape_id: usize,
        old_name: String,
        new_name: String,
    },
    /// Shape modified
    Modified { shape_id: usize, operation: String },
    /// Shape deleted
    Deleted { shape_id: usize, name: String },
}

impl TopoDsNaming {
    /// Create a new topological naming system
    pub fn new() -> Self {
        Self {
            name_map: HashMap::new(),
            id_map: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Assign a name to a shape
    pub fn assign_name(&mut self, shape: &TopoDsShape, name: String) {
        let shape_id = shape.shape_id();

        // Remove old name if exists
        if let Some(old_name) = self.name_map.get(&shape_id) {
            self.id_map.remove(old_name);
            self.history.push(NamingEvent::Renamed {
                shape_id,
                old_name: old_name.clone(),
                new_name: name.clone(),
            });
        } else {
            self.history.push(NamingEvent::Created {
                shape_id,
                name: name.clone(),
            });
        }

        self.name_map.insert(shape_id, name.clone());
        self.id_map.insert(name, shape_id);
    }

    /// Get the name of a shape
    pub fn get_name(&self, shape: &TopoDsShape) -> Option<&String> {
        self.name_map.get(&shape.shape_id())
    }

    /// Get a shape by name
    pub fn get_shape_by_name(&self, name: &str) -> Option<usize> {
        self.id_map.get(name).copied()
    }

    /// Rename a shape
    pub fn rename(&mut self, shape: &TopoDsShape, new_name: String) {
        let shape_id = shape.shape_id();

        if let Some(old_name) = self.name_map.get(&shape_id) {
            self.id_map.remove(old_name);
            self.name_map.insert(shape_id, new_name.clone());
            self.id_map.insert(new_name.clone(), shape_id);

            self.history.push(NamingEvent::Renamed {
                shape_id,
                old_name: old_name.clone(),
                new_name,
            });
        }
    }

    /// Record a shape modification
    pub fn record_modification(&mut self, shape: &TopoDsShape, operation: String) {
        let shape_id = shape.shape_id();
        self.history.push(NamingEvent::Modified {
            shape_id,
            operation,
        });
    }

    /// Remove a shape from the naming system
    pub fn remove_shape(&mut self, shape: &TopoDsShape) {
        let shape_id = shape.shape_id();

        if let Some(name) = self.name_map.remove(&shape_id) {
            self.id_map.remove(&name);
            self.history.push(NamingEvent::Deleted { shape_id, name });
        }
    }

    /// Get the history of naming events
    pub fn history(&self) -> &Vec<NamingEvent> {
        &self.history
    }

    /// Clear the naming history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Trait for objects that can be named
pub trait Nameable {
    /// Get the name of the object
    fn name(&self) -> Option<String>;

    /// Set the name of the object
    fn set_name(&mut self, name: String);
}

/// Extension trait for TopoDsShape to support naming
pub trait NamingExt {
    /// Assign a name to the shape
    fn assign_name(&self, name: String);

    /// Get the name of the shape
    fn get_name(&self) -> Option<String>;

    /// Rename the shape
    fn rename(&self, new_name: String);
}

/// Topological history manager
pub struct TopoHistory {
    /// Naming system
    naming: TopoDsNaming,
    /// History of operations
    operations: Vec<TopoOperation>,
}

/// Topological operation types
pub enum TopoOperation {
    /// Boolean operation
    Boolean {
        operation: String,
        operands: Vec<usize>,
        result: usize,
    },
    /// Fillet operation
    Fillet {
        edges: Vec<usize>,
        radius: f64,
        result: usize,
    },
    /// Chamfer operation
    Chamfer {
        edges: Vec<usize>,
        distance: f64,
        result: usize,
    },
    /// Offset operation
    Offset {
        shape: usize,
        distance: f64,
        result: usize,
    },
    /// Transform operation
    Transform {
        shape: usize,
        transform: String,
        result: usize,
    },
    /// Other operation
    Other {
        name: String,
        operands: Vec<usize>,
        result: usize,
    },
}

impl TopoHistory {
    /// Create a new topological history manager
    pub fn new() -> Self {
        Self {
            naming: TopoDsNaming::new(),
            operations: Vec::new(),
        }
    }

    /// Get the naming system
    pub fn naming(&self) -> &TopoDsNaming {
        &self.naming
    }

    /// Get mutable access to the naming system
    pub fn naming_mut(&mut self) -> &mut TopoDsNaming {
        &mut self.naming
    }

    /// Record a boolean operation
    pub fn record_boolean(&mut self, operation: String, operands: Vec<usize>, result: usize) {
        self.operations.push(TopoOperation::Boolean {
            operation,
            operands,
            result,
        });
    }

    /// Record a fillet operation
    pub fn record_fillet(&mut self, edges: Vec<usize>, radius: f64, result: usize) {
        self.operations.push(TopoOperation::Fillet {
            edges,
            radius,
            result,
        });
    }

    /// Record a chamfer operation
    pub fn record_chamfer(&mut self, edges: Vec<usize>, distance: f64, result: usize) {
        self.operations.push(TopoOperation::Chamfer {
            edges,
            distance,
            result,
        });
    }

    /// Record an offset operation
    pub fn record_offset(&mut self, shape: usize, distance: f64, result: usize) {
        self.operations.push(TopoOperation::Offset {
            shape,
            distance,
            result,
        });
    }

    /// Record a transform operation
    pub fn record_transform(&mut self, shape: usize, transform: String, result: usize) {
        self.operations.push(TopoOperation::Transform {
            shape,
            transform,
            result,
        });
    }

    /// Record another operation
    pub fn record_other(&mut self, name: String, operands: Vec<usize>, result: usize) {
        self.operations.push(TopoOperation::Other {
            name,
            operands,
            result,
        });
    }

    /// Get the operation history
    pub fn operations(&self) -> &Vec<TopoOperation> {
        &self.operations
    }

    /// Clear the operation history
    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }
}
