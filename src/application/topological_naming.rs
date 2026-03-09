//! Topological naming and history for application
//! 
//! This module provides topological naming and history functionality for BrepRs,
//! allowing for persistent naming of topological entities and tracking of modeling operations.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use crate::topology::TopoDsShape;
use crate::topology::ShapeType;

/// Topological name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopologicalName {
    /// Name components
    components: Vec<String>,
}

impl TopologicalName {
    /// Create a new topological name
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Create a topological name from components
    pub fn from_components(components: Vec<String>) -> Self {
        Self {
            components,
        }
    }

    /// Create a topological name from a string path
    pub fn from_path(path: &str) -> Self {
        let components = path.split('/').map(|s| s.to_string()).collect();
        Self {
            components,
        }
    }

    /// Add a component to the name
    pub fn add_component(&mut self, component: &str) {
        self.components.push(component.to_string());
    }

    /// Get the parent name
    pub fn parent(&self) -> Option<Self> {
        if self.components.len() > 1 {
            Some(Self {
                components: self.components[0..self.components.len()-1].to_vec(),
            })
        } else {
            None
        }
    }

    /// Get the last component
    pub fn last_component(&self) -> Option<&String> {
        self.components.last()
    }

    /// Get all components
    pub fn components(&self) -> &Vec<String> {
        &self.components
    }

    /// Convert to path string
    pub fn to_path(&self) -> String {
        self.components.join("/")
    }

    /// Check if the name is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Get name length
    pub fn len(&self) -> usize {
        self.components.len()
    }
}

impl Default for TopologicalName {
    fn default() -> Self {
        Self::new()
    }
}

/// Topological naming manager
pub struct TopologicalNamingManager {
    /// Shape ID to name mapping
    shape_id_to_name: HashMap<i32, TopologicalName>,
    /// Name to shape ID mapping
    name_to_shape_id: HashMap<TopologicalName, i32>,
    /// Shape ID to shape mapping
    shape_id_to_shape: HashMap<i32, Arc<TopoDsShape>>,
    /// Operation history
    history: VecDeque<ModelingOperation>,
    /// Maximum history size
    max_history_size: usize,
}

/// Modeling operation
#[derive(Debug, Clone)]
pub struct ModelingOperation {
    /// Operation type
    pub operation_type: String,
    /// Input shapes
    pub input_shapes: Vec<Arc<TopoDsShape>>,
    /// Output shapes
    pub output_shapes: Vec<Arc<TopoDsShape>>,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ModelingOperation {
    /// Create a new modeling operation
    pub fn new(operation_type: &str) -> Self {
        Self {
            operation_type: operation_type.to_string(),
            input_shapes: Vec::new(),
            output_shapes: Vec::new(),
            parameters: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add input shape
    pub fn add_input(&mut self, shape: Arc<TopoDsShape>) {
        self.input_shapes.push(shape);
    }

    /// Add output shape
    pub fn add_output(&mut self, shape: Arc<TopoDsShape>) {
        self.output_shapes.push(shape);
    }

    /// Set parameter
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.to_string(), value.to_string());
    }
}

impl TopologicalNamingManager {
    /// Create a new topological naming manager
    pub fn new() -> Self {
        Self {
            shape_id_to_name: HashMap::new(),
            name_to_shape_id: HashMap::new(),
            shape_id_to_shape: HashMap::new(),
            history: VecDeque::new(),
            max_history_size: 100,
        }
    }

    /// Register a shape with a name
    pub fn register_shape(&mut self, shape: Arc<TopoDsShape>, name: TopologicalName) {
        let shape_id = shape.shape_id();
        
        // Remove existing mappings
        if let Some(existing_name) = self.shape_id_to_name.get(&shape_id) {
            self.name_to_shape_id.remove(existing_name);
        }
        
        // Add new mappings
        self.shape_id_to_name.insert(shape_id, name.clone());
        self.name_to_shape_id.insert(name, shape_id);
        self.shape_id_to_shape.insert(shape_id, shape);
    }

    /// Get name for a shape
    pub fn get_name(&self, shape: &TopoDsShape) -> Option<&TopologicalName> {
        let shape_id = shape.shape_id();
        self.shape_id_to_name.get(&shape_id)
    }

    /// Get shape by name
    pub fn get_shape(&self, name: &TopologicalName) -> Option<&Arc<TopoDsShape>> {
        self.name_to_shape_id.get(name).and_then(|shape_id| {
            self.shape_id_to_shape.get(shape_id)
        })
    }

    /// Remove a shape
    pub fn remove_shape(&mut self, shape: &TopoDsShape) {
        let shape_id = shape.shape_id();
        if let Some(name) = self.shape_id_to_name.remove(&shape_id) {
            self.name_to_shape_id.remove(&name);
            self.shape_id_to_shape.remove(&shape_id);
        }
    }

    /// Record a modeling operation
    pub fn record_operation(&mut self, operation: ModelingOperation) {
        self.history.push_back(operation);
        
        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.pop_front();
        }
    }

    /// Get operation history
    pub fn history(&self) -> &VecDeque<ModelingOperation> {
        &self.history
    }

    /// Undo last operation
    pub fn undo(&mut self) -> Option<ModelingOperation> {
        self.history.pop_back()
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Set maximum history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        
        // Trim history if necessary
        while self.history.len() > size {
            self.history.pop_front();
        }
    }

    /// Generate a name for a new shape
    pub fn generate_name(&self, base_name: &str, shape_type: ShapeType) -> TopologicalName {
        let type_prefix = match shape_type {
            ShapeType::Vertex => "v",
            ShapeType::Edge => "e",
            ShapeType::Wire => "w",
            ShapeType::Face => "f",
            ShapeType::Shell => "sh",
            ShapeType::Solid => "s",
            ShapeType::Compound => "c",
            ShapeType::CompSolid => "cs",
        };
        
        let mut counter = 1;
        let mut name_components = vec![base_name.to_string()];
        
        loop {
            let component = format!("{}{}", type_prefix, counter);
            name_components.push(component);
            let name = TopologicalName::from_components(name_components.clone());
            
            if !self.name_to_shape_id.contains_key(&name) {
                return name;
            }
            
            name_components.pop();
            counter += 1;
        }
    }

    /// Rename a shape
    pub fn rename_shape(&mut self, shape: &TopoDsShape, new_name: TopologicalName) -> Result<(), String> {
        let shape_id = shape.shape_id();
        
        // Check if shape is registered
        let old_name = match self.shape_id_to_name.get(&shape_id) {
            Some(name) => name.clone(),
            None => return Err("Shape not registered".to_string()),
        };
        
        // Check if new name is already used
        if self.name_to_shape_id.contains_key(&new_name) {
            return Err("Name already in use".to_string());
        }
        
        // Update mappings
        self.shape_id_to_name.insert(shape_id, new_name.clone());
        self.name_to_shape_id.remove(&old_name);
        self.name_to_shape_id.insert(new_name, shape_id);
        
        Ok(())
    }

    /// Get all shapes
    pub fn get_all_shapes(&self) -> Vec<Arc<TopoDsShape>> {
        self.shape_id_to_shape.values().cloned().collect()
    }

    /// Get all names
    pub fn get_all_names(&self) -> Vec<TopologicalName> {
        self.name_to_shape_id.keys().cloned().collect()
    }
}

impl Default for TopologicalNamingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Topological history tracker
pub struct TopologicalHistoryTracker {
    /// Naming manager
    naming_manager: TopologicalNamingManager,
    /// Shape dependencies (shape ID to set of dependency shape IDs)
    dependencies: HashMap<i32, HashSet<i32>>,
    /// Shape dependents (shape ID to set of dependent shape IDs)
    dependents: HashMap<i32, HashSet<i32>>,
}

impl TopologicalHistoryTracker {
    /// Create a new topological history tracker
    pub fn new() -> Self {
        Self {
            naming_manager: TopologicalNamingManager::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Get naming manager
    pub fn naming_manager(&self) -> &TopologicalNamingManager {
        &self.naming_manager
    }

    /// Get mutable naming manager
    pub fn naming_manager_mut(&mut self) -> &mut TopologicalNamingManager {
        &mut self.naming_manager
    }

    /// Add dependency between shapes
    pub fn add_dependency(&mut self, dependent: Arc<TopoDsShape>, dependency: Arc<TopoDsShape>) {
        let dependent_id = dependent.shape_id();
        let dependency_id = dependency.shape_id();
        
        // Add to dependencies
        self.dependencies
            .entry(dependent_id)
            .or_insert_with(HashSet::new)
            .insert(dependency_id);
        
        // Add to dependents
        self.dependents
            .entry(dependency_id)
            .or_insert_with(HashSet::new)
            .insert(dependent_id);
    }

    /// Get dependencies for a shape
    pub fn get_dependencies(&self, shape: &TopoDsShape) -> Option<Vec<Arc<TopoDsShape>>> {
        let shape_id = shape.shape_id();
        self.dependencies.get(&shape_id).map(|dependency_ids| {
            dependency_ids.iter()
                .filter_map(|id| self.naming_manager.shape_id_to_shape.get(id))
                .cloned()
                .collect()
        })
    }

    /// Get dependents for a shape
    pub fn get_dependents(&self, shape: &TopoDsShape) -> Option<Vec<Arc<TopoDsShape>>> {
        let shape_id = shape.shape_id();
        self.dependents.get(&shape_id).map(|dependent_ids| {
            dependent_ids.iter()
                .filter_map(|id| self.naming_manager.shape_id_to_shape.get(id))
                .cloned()
                .collect()
        })
    }

    /// Remove a shape and its dependencies
    pub fn remove_shape(&mut self, shape: &TopoDsShape) {
        let shape_id = shape.shape_id();
        
        // Remove from naming manager
        self.naming_manager.remove_shape(shape);
        
        // Remove dependencies
        if let Some(dependency_ids) = self.dependencies.remove(&shape_id) {
            for dependency_id in dependency_ids {
                if let Some(dependents) = self.dependents.get_mut(&dependency_id) {
                    dependents.remove(&shape_id);
                    if dependents.is_empty() {
                        self.dependents.remove(&dependency_id);
                    }
                }
            }
        }
        
        // Remove dependents
        if let Some(dependent_ids) = self.dependents.remove(&shape_id) {
            for dependent_id in dependent_ids {
                if let Some(dependencies) = self.dependencies.get_mut(&dependent_id) {
                    dependencies.remove(&shape_id);
                    if dependencies.is_empty() {
                        self.dependencies.remove(&dependent_id);
                    }
                }
            }
        }
    }

    /// Record a modeling operation with dependencies
    pub fn record_operation(&mut self, operation: ModelingOperation) {
        // Register all input and output shapes
        for input_shape in &operation.input_shapes {
            let name = self.naming_manager.generate_name("input", input_shape.shape_type());
            self.naming_manager.register_shape(input_shape.clone(), name);
        }
        
        for output_shape in &operation.output_shapes {
            let name = self.naming_manager.generate_name("output", output_shape.shape_type());
            self.naming_manager.register_shape(output_shape.clone(), name);
        }
        
        // Add dependencies
        for output_shape in &operation.output_shapes {
            for input_shape in &operation.input_shapes {
                self.add_dependency(output_shape.clone(), input_shape.clone());
            }
        }
        
        // Record operation
        self.naming_manager.record_operation(operation);
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.naming_manager.clear_history();
        self.dependencies.clear();
        self.dependents.clear();
    }
}

impl Default for TopologicalHistoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::TopoDsShape;
    use crate::topology::ShapeType;

    #[test]
    fn test_topological_name() {
        let mut name = TopologicalName::new();
        name.add_component("part1");
        name.add_component("f1");
        
        assert_eq!(name.to_path(), "part1/f1");
        assert_eq!(name.len(), 2);
        
        let parent = name.parent().unwrap();
        assert_eq!(parent.to_path(), "part1");
    }

    #[test]
    fn test_topological_naming_manager() {
        let mut manager = TopologicalNamingManager::new();
        let shape = Arc::new(TopoDsShape::new(ShapeType::Face));
        let name = TopologicalName::from_path("part1/f1");
        
        manager.register_shape(shape.clone(), name.clone());
        
        assert_eq!(manager.get_name(&shape), Some(&name));
        assert_eq!(manager.get_shape(&name), Some(&shape));
    }

    #[test]
    fn test_generate_name() {
        let manager = TopologicalNamingManager::new();
        let name = manager.generate_name("part1", ShapeType::Face);
        assert!(name.to_path().starts_with("part1/f"));
    }

    #[test]
    fn test_modeling_operation() {
        let mut operation = ModelingOperation::new("extrude");
        let input_shape = Arc::new(TopoDsShape::new(ShapeType::Face));
        let output_shape = Arc::new(TopoDsShape::new(ShapeType::Solid));
        
        operation.add_input(input_shape);
        operation.add_output(output_shape);
        operation.set_parameter("distance", "10.0");
        
        assert_eq!(operation.operation_type, "extrude");
        assert_eq!(operation.input_shapes.len(), 1);
        assert_eq!(operation.output_shapes.len(), 1);
        assert_eq!(operation.parameters.get("distance"), Some(&"10.0".to_string()));
    }

    #[test]
    fn test_topological_history_tracker() {
        let mut tracker = TopologicalHistoryTracker::new();
        let input_shape = Arc::new(TopoDsShape::new(ShapeType::Face));
        let output_shape = Arc::new(TopoDsShape::new(ShapeType::Solid));
        
        let mut operation = ModelingOperation::new("extrude");
        operation.add_input(input_shape.clone());
        operation.add_output(output_shape.clone());
        
        tracker.record_operation(operation);
        
        let dependencies = tracker.get_dependencies(&output_shape).unwrap();
        assert!(dependencies.contains(&input_shape));
        
        let dependents = tracker.get_dependents(&input_shape).unwrap();
        assert!(dependents.contains(&output_shape));
    }
}