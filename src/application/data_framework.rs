//! Data framework for application
//! 
//! This module provides the data framework for BrepRs applications,
//! including data storage, management, and synchronization.

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::topology::TopoDsShape;

/// Data object trait
pub trait DataObject: Send + Sync {
    /// Get object ID
    fn id(&self) -> String;
    /// Get object name
    fn name(&self) -> String;
    /// Set object name
    fn set_name(&mut self, name: String);
    /// Get object type
    fn type_name(&self) -> String;
    /// Clone the object
    fn clone_object(&self) -> Box<dyn DataObject>;
    /// Get as Any for downcasting
    fn as_any(&self) -> &dyn Any;
    /// Get mutable as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Data container for holding data objects
pub struct DataContainer {
    objects: HashMap<String, Arc<RwLock<Box<dyn DataObject>>>>,
    name_index: HashMap<String, HashSet<String>>,
}

impl DataContainer {
    /// Create a new data container
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            name_index: HashMap::new(),
        }
    }

    /// Add an object to the container
    pub fn add_object(&mut self, object: Box<dyn DataObject>) -> String {
        let id = object.id();
        let name = object.name();
        
        self.objects.insert(id.clone(), Arc::new(RwLock::new(object)));
        
        // Update name index
        self.name_index
            .entry(name)
            .or_insert_with(HashSet::new)
            .insert(id.clone());
        
        id
    }

    /// Get an object by ID
    pub fn get_object(&self, id: &str) -> Option<Arc<RwLock<Box<dyn DataObject>>>> {
        self.objects.get(id).cloned()
    }

    /// Get objects by name
    pub fn get_objects_by_name(&self, name: &str) -> Vec<Arc<RwLock<Box<dyn DataObject>>>> {
        self.name_index
            .get(name)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.objects.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove an object by ID
    pub fn remove_object(&mut self, id: &str) -> Option<Arc<RwLock<Box<dyn DataObject>>>> {
        if let Some(object) = self.objects.remove(id) {
            let name = object.read().unwrap().name();
            if let Some(ids) = self.name_index.get_mut(&name) {
                ids.remove(id);
                if ids.is_empty() {
                    self.name_index.remove(&name);
                }
            }
            Some(object)
        } else {
            None
        }
    }

    /// Get all objects
    pub fn get_all_objects(&self) -> Vec<Arc<RwLock<Box<dyn DataObject>>>> {
        self.objects.values().cloned().collect()
    }

    /// Clear all objects
    pub fn clear(&mut self) {
        self.objects.clear();
        self.name_index.clear();
    }

    /// Get object count
    pub fn count(&self) -> usize {
        self.objects.len()
    }
}

impl Default for DataContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Shape data object
pub struct ShapeData {
    id: String,
    name: String,
    shape: TopoDsShape,
    attributes: HashMap<String, String>,
}

impl ShapeData {
    /// Create a new shape data object
    pub fn new(id: String, name: String, shape: TopoDsShape) -> Self {
        Self {
            id,
            name,
            shape,
            attributes: HashMap::new(),
        }
    }

    /// Get the shape
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable shape
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Get an attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Remove an attribute
    pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
    }
}

impl DataObject for ShapeData {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn type_name(&self) -> String {
        "ShapeData".to_string()
    }

    fn clone_object(&self) -> Box<dyn DataObject> {
        Box::new(ShapeData {
            id: format!("{}-clone", self.id),
            name: format!("{} (Clone)", self.name),
            shape: self.shape.clone(),
            attributes: self.attributes.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Data transaction for atomic operations
pub struct DataTransaction<'a> {
    container: &'a mut DataContainer,
    operations: Vec<TransactionOperation>,
}

/// Transaction operation
enum TransactionOperation {
    Add(String, Box<dyn DataObject>),
    Remove(String),
    Update(String, Box<dyn DataObject>),
}

impl<'a> DataTransaction<'a> {
    /// Create a new transaction
    pub fn new(container: &'a mut DataContainer) -> Self {
        Self {
            container,
            operations: Vec::new(),
        }
    }

    /// Add an object
    pub fn add(&mut self, object: Box<dyn DataObject>) {
        let id = object.id();
        self.operations.push(TransactionOperation::Add(id, object));
    }

    /// Remove an object
    pub fn remove(&mut self, id: &str) {
        self.operations.push(TransactionOperation::Remove(id.to_string()));
    }

    /// Update an object
    pub fn update(&mut self, id: &str, object: Box<dyn DataObject>) {
        self.operations.push(TransactionOperation::Update(id.to_string(), object));
    }

    /// Commit the transaction
    pub fn commit(self) {
        for op in self.operations {
            match op {
                TransactionOperation::Add(id, object) => {
                    self.container.add_object(object);
                }
                TransactionOperation::Remove(id) => {
                    self.container.remove_object(&id);
                }
                TransactionOperation::Update(id, object) => {
                    self.container.remove_object(&id);
                    self.container.add_object(object);
                }
            }
        }
    }

    /// Rollback the transaction
    pub fn rollback(self) {
        // For simplicity, we just don't commit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::TopoDsShape;
    use crate::topology::ShapeType;

    #[test]
    fn test_data_container() {
        let mut container = DataContainer::new();
        
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let shape_data = ShapeData::new(
            "shape1".to_string(),
            "Test Shape".to_string(),
            shape
        );
        
        let id = container.add_object(Box::new(shape_data));
        assert_eq!(id, "shape1");
        assert_eq!(container.count(), 1);
        
        let retrieved = container.get_object(&id);
        assert!(retrieved.is_some());
        
        let objects_by_name = container.get_objects_by_name("Test Shape");
        assert_eq!(objects_by_name.len(), 1);
        
        container.remove_object(&id);
        assert_eq!(container.count(), 0);
    }

    #[test]
    fn test_shape_data() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let mut shape_data = ShapeData::new(
            "shape1".to_string(),
            "Test Shape".to_string(),
            shape
        );
        
        assert_eq!(shape_data.id(), "shape1");
        assert_eq!(shape_data.name(), "Test Shape");
        assert_eq!(shape_data.type_name(), "ShapeData");
        
        shape_data.add_attribute("color", "red");
        assert_eq!(shape_data.get_attribute("color"), Some(&"red".to_string()));
        
        shape_data.set_name("Renamed Shape".to_string());
        assert_eq!(shape_data.name(), "Renamed Shape");
    }

    #[test]
    fn test_data_transaction() {
        let mut container = DataContainer::new();
        
        let shape1 = TopoDsShape::new(ShapeType::Vertex);
        let shape_data1 = ShapeData::new(
            "shape1".to_string(),
            "Shape 1".to_string(),
            shape1
        );
        
        let shape2 = TopoDsShape::new(ShapeType::Edge);
        let shape_data2 = ShapeData::new(
            "shape2".to_string(),
            "Shape 2".to_string(),
            shape2
        );
        
        let mut transaction = DataTransaction::new(&mut container);
        transaction.add(Box::new(shape_data1));
        transaction.add(Box::new(shape_data2));
        transaction.commit();
        
        assert_eq!(container.count(), 2);
    }
}
