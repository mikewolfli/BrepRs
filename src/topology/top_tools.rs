//! Topology Tools
//! 
//! Provides utility classes for working with topological shapes, including
//! indexed maps, lists, sequences, and sets.

use crate::foundation::handle::Handle;
use crate::topology::TopoDsShape;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Indexed map of shapes
/// 
/// Provides efficient lookup by shape or index.
pub struct IndexedMapOfShape {
    map: HashMap<i32, Handle<TopoDsShape>>,
    index_to_id: Vec<i32>,
}

impl IndexedMapOfShape {
    /// Create a new indexed map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            index_to_id: Vec::new(),
        }
    }
    
    /// Add a shape to the map
    pub fn add(&mut self, shape: &TopoDsShape) -> i32 {
        let shape_id = shape.shape_id();
        if !self.map.contains_key(&shape_id) {
            self.index_to_id.push(shape_id);
            self.map.insert(shape_id, Handle::new(Arc::new(shape.clone())));
        }
        self.find_index(shape)
    }
    
    /// Find the index of a shape
    pub fn find_index(&self, shape: &TopoDsShape) -> i32 {
        let shape_id = shape.shape_id();
        self.index_to_id.iter()
            .position(|&id| id == shape_id)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
    
    /// Find a shape by index
    pub fn find_from_index(&self, index: i32) -> Option<Handle<TopoDsShape>> {
        if index >= 0 && (index as usize) < self.index_to_id.len() {
            let shape_id = self.index_to_id[index as usize];
            self.map.get(&shape_id).cloned()
        } else {
            None
        }
    }
    
    /// Check if the map contains a shape
    pub fn contains(&self, shape: &TopoDsShape) -> bool {
        self.map.contains_key(&shape.shape_id())
    }
    
    /// Get the number of shapes in the map
    pub fn extent(&self) -> i32 {
        self.map.len() as i32
    }
    
    /// Clear the map
    pub fn clear(&mut self) {
        self.map.clear();
        self.index_to_id.clear();
    }
    
    /// Remove a shape from the map
    pub fn remove(&mut self, shape: &TopoDsShape) -> bool {
        let shape_id = shape.shape_id();
        if let Some(index) = self.index_to_id.iter()
            .position(|&id| id == shape_id) {
            self.index_to_id.remove(index);
            self.map.remove(&shape_id).is_some()
        } else {
            false
        }
    }
    
    /// Get all shapes in the map
    pub fn shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.index_to_id.iter()
            .filter_map(|id| self.map.get(id).cloned())
            .collect()
    }
}

impl Default for IndexedMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// List of shapes
/// 
/// Provides ordered storage of shapes with various insertion and removal operations.
pub struct ListOfShape {
    shapes: Vec<Handle<TopoDsShape>>,
}

impl ListOfShape {
    /// Create a new list
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }
    
    /// Append a shape to the list
    pub fn append(&mut self, shape: &TopoDsShape) {
        self.shapes.push(Handle::new(Arc::new(shape.clone())));
    }
    
    /// Prepend a shape to the list
    pub fn prepend(&mut self, shape: &TopoDsShape) {
        self.shapes.insert(0, Handle::new(Arc::new(shape.clone())));
    }
    
    /// Insert a shape at a specific position
    pub fn insert_before(&mut self, index: i32, shape: &TopoDsShape) {
        if index >= 0 && (index as usize) <= self.shapes.len() {
            self.shapes.insert(index as usize, Handle::new(Arc::new(shape.clone())));
        }
    }
    
    /// Insert a shape after a specific position
    pub fn insert_after(&mut self, index: i32, shape: &TopoDsShape) {
        if index >= 0 && (index as usize) < self.shapes.len() {
            self.shapes.insert((index + 1) as usize, Handle::new(Arc::new(shape.clone())));
        }
    }
    
    /// Remove a shape from the list
    pub fn remove(&mut self, shape: &TopoDsShape) -> bool {
        let shape_id = shape.shape_id();
        if let Some(index) = self.shapes.iter()
            .position(|h| h.as_ref().map(|s| s.shape_id() == shape_id).unwrap_or(false)) {
            self.shapes.remove(index);
            true
        } else {
            false
        }
    }
    
    /// Remove a shape at a specific index
    pub fn remove_at(&mut self, index: i32) -> bool {
        if index >= 0 && (index as usize) < self.shapes.len() {
            self.shapes.remove(index as usize);
            true
        } else {
            false
        }
    }
    
    /// Get the first shape
    pub fn first(&self) -> Option<Handle<TopoDsShape>> {
        self.shapes.first().cloned()
    }
    
    /// Get the last shape
    pub fn last(&self) -> Option<Handle<TopoDsShape>> {
        self.shapes.last().cloned()
    }
    
    /// Get a shape at a specific index
    pub fn value(&self, index: i32) -> Option<Handle<TopoDsShape>> {
        if index >= 0 && (index as usize) < self.shapes.len() {
            Some(self.shapes[index as usize].clone())
        } else {
            None
        }
    }
    
    /// Get the number of shapes in the list
    pub fn extent(&self) -> i32 {
        self.shapes.len() as i32
    }
    
    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
    
    /// Clear the list
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
    
    /// Get all shapes in the list
    pub fn shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.shapes.clone()
    }
    
    /// Reverse the list
    pub fn reverse(&mut self) {
        self.shapes.reverse();
    }
    
    /// Sort the list
    pub fn sort(&mut self) {
        self.shapes.sort_by(|a, b| {
            let id_a = a.as_ref().map(|s| s.shape_id()).unwrap_or(0);
            let id_b = b.as_ref().map(|s| s.shape_id()).unwrap_or(0);
            id_a.cmp(&id_b)
        });
    }
}

impl Default for ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Sequence of shapes
/// 
/// Provides ordered storage with random access and modification capabilities.
pub struct SequenceOfShape {
    shapes: Vec<Handle<TopoDsShape>>,
}

impl SequenceOfShape {
    /// Create a new sequence
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }
    
    /// Append a shape to the sequence
    pub fn append(&mut self, shape: &TopoDsShape) {
        self.shapes.push(Handle::new(Arc::new(shape.clone())));
    }
    
    /// Prepend a shape to the sequence
    pub fn prepend(&mut self, shape: &TopoDsShape) {
        self.shapes.insert(0, Handle::new(Arc::new(shape.clone())));
    }
    
    /// Insert a shape at a specific position
    pub fn insert_before(&mut self, index: i32, shape: &TopoDsShape) {
        if index >= 0 && (index as usize) <= self.shapes.len() {
            self.shapes.insert(index as usize, Handle::new(Arc::new(shape.clone())));
        }
    }
    
    /// Insert a shape after a specific position
    pub fn insert_after(&mut self, index: i32, shape: &TopoDsShape) {
        if index >= 0 && (index as usize) < self.shapes.len() {
            self.shapes.insert((index + 1) as usize, Handle::new(Arc::new(shape.clone())));
        }
    }
    
    /// Remove a shape from the sequence
    pub fn remove(&mut self, shape: &TopoDsShape) -> bool {
        let shape_id = shape.shape_id();
        if let Some(index) = self.shapes.iter()
            .position(|h| h.as_ref().map(|s| s.shape_id() == shape_id).unwrap_or(false)) {
            self.shapes.remove(index);
            true
        } else {
            false
        }
    }
    
    /// Remove a shape at a specific index
    pub fn remove_at(&mut self, index: i32) -> bool {
        if index >= 0 && (index as usize) < self.shapes.len() {
            self.shapes.remove(index as usize);
            true
        } else {
            false
        }
    }
    
    /// Get the first shape
    pub fn first(&self) -> Option<Handle<TopoDsShape>> {
        self.shapes.first().cloned()
    }
    
    /// Get the last shape
    pub fn last(&self) -> Option<Handle<TopoDsShape>> {
        self.shapes.last().cloned()
    }
    
    /// Get a shape at a specific index
    pub fn value(&self, index: i32) -> Option<Handle<TopoDsShape>> {
        if index >= 0 && (index as usize) < self.shapes.len() {
            Some(self.shapes[index as usize].clone())
        } else {
            None
        }
    }
    
    /// Change the value at a specific index
    pub fn set_value(&mut self, index: i32, shape: &TopoDsShape) {
        if index >= 0 && (index as usize) < self.shapes.len() {
            self.shapes[index as usize] = Handle::new(Arc::new(shape.clone()));
        }
    }
    
    /// Get the number of shapes in the sequence
    pub fn length(&self) -> i32 {
        self.shapes.len() as i32
    }
    
    /// Check if the sequence is empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
    
    /// Clear the sequence
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
    
    /// Get all shapes in the sequence
    pub fn shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.shapes.clone()
    }
    
    /// Reverse the sequence
    pub fn reverse(&mut self) {
        self.shapes.reverse();
    }
    
    /// Exchange two elements
    pub fn exchange(&mut self, i: i32, j: i32) {
        if i >= 0 && j >= 0 && (i as usize) < self.shapes.len() && (j as usize) < self.shapes.len() {
            self.shapes.swap(i as usize, j as usize);
        }
    }
}

impl Default for SequenceOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Data map of shapes to shapes
/// 
/// Provides a mapping from one shape to another.
pub struct DataMapOfShapeShape {
    map: HashMap<i32, Handle<TopoDsShape>>,
    keys: Vec<Handle<TopoDsShape>>,
}

impl DataMapOfShapeShape {
    /// Create a new data map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }
    
    /// Bind a shape to another shape
    pub fn bind(&mut self, key: &TopoDsShape, value: &TopoDsShape) {
        let key_id = key.shape_id();
        if !self.map.contains_key(&key_id) {
            self.keys.push(Handle::new(Arc::new(key.clone())));
        }
        self.map.insert(key_id, Handle::new(Arc::new(value.clone())));
    }
    
    /// Find a shape by key
    pub fn find(&self, key: &TopoDsShape) -> Option<Handle<TopoDsShape>> {
        self.map.get(&key.shape_id()).cloned()
    }
    
    /// Check if the map contains a key
    pub fn is_bound(&self, key: &TopoDsShape) -> bool {
        self.map.contains_key(&key.shape_id())
    }
    
    /// Unbind a key
    pub fn unbind(&mut self, key: &TopoDsShape) -> bool {
        let key_id = key.shape_id();
        if self.map.remove(&key_id).is_some() {
            if let Some(index) = self.keys.iter()
                .position(|h| h.as_ref().map(|s| s.shape_id() == key_id).unwrap_or(false)) {
                self.keys.remove(index);
            }
            true
        } else {
            false
        }
    }
    
    /// Get the number of entries in the map
    pub fn extent(&self) -> i32 {
        self.map.len() as i32
    }
    
    /// Clear the map
    pub fn clear(&mut self) {
        self.map.clear();
        self.keys.clear();
    }
    
    /// Get all keys in the map
    pub fn keys(&self) -> Vec<Handle<TopoDsShape>> {
        self.keys.clone()
    }
    
    /// Get all values in the map
    pub fn values(&self) -> Vec<Handle<TopoDsShape>> {
        self.map.values().cloned().collect()
    }
}

impl Default for DataMapOfShapeShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Data map of shape to integer
/// 
/// Provides a mapping from shapes to integer values.
pub struct DataMapOfShapeInteger {
    map: HashMap<i32, i32>,
}

impl DataMapOfShapeInteger {
    /// Create a new data map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    
    /// Bind a shape to an integer
    pub fn bind(&mut self, key: &TopoDsShape, value: i32) {
        self.map.insert(key.shape_id(), value);
    }
    
    /// Find an integer by key
    pub fn find(&self, key: &TopoDsShape) -> Option<i32> {
        self.map.get(&key.shape_id()).copied()
    }
    
    /// Check if the map contains a key
    pub fn is_bound(&self, key: &TopoDsShape) -> bool {
        self.map.contains_key(&key.shape_id())
    }
    
    /// Unbind a key
    pub fn unbind(&mut self, key: &TopoDsShape) -> bool {
        self.map.remove(&key.shape_id()).is_some()
    }
    
    /// Get the number of entries in the map
    pub fn extent(&self) -> i32 {
        self.map.len() as i32
    }
    
    /// Clear the map
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for DataMapOfShapeInteger {
    fn default() -> Self {
        Self::new()
    }
}

/// Shape set for storing unique shapes
/// 
/// Provides efficient storage and lookup of unique shapes.
pub struct ShapeSet {
    shapes: HashSet<i32>,
    shape_handles: Vec<Handle<TopoDsShape>>,
}

impl ShapeSet {
    /// Create a new shape set
    pub fn new() -> Self {
        Self {
            shapes: HashSet::new(),
            shape_handles: Vec::new(),
        }
    }
    
    /// Add a shape to the set
    pub fn add(&mut self, shape: &TopoDsShape) {
        let shape_id = shape.shape_id();
        if !self.shapes.contains(&shape_id) {
            self.shapes.insert(shape_id);
            self.shape_handles.push(Handle::new(Arc::new(shape.clone())));
        }
    }
    
    /// Remove a shape from the set
    pub fn remove(&mut self, shape: &TopoDsShape) -> bool {
        let shape_id = shape.shape_id();
        if self.shapes.remove(&shape_id) {
            if let Some(index) = self.shape_handles.iter()
                .position(|h| h.as_ref().map(|s| s.shape_id() == shape_id).unwrap_or(false)) {
                self.shape_handles.remove(index);
            }
            true
        } else {
            false
        }
    }
    
    /// Check if the set contains a shape
    pub fn contains(&self, shape: &TopoDsShape) -> bool {
        self.shapes.contains(&shape.shape_id())
    }
    
    /// Get the number of shapes in the set
    pub fn extent(&self) -> i32 {
        self.shapes.len() as i32
    }
    
    /// Clear the set
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.shape_handles.clear();
    }
    
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
    
    /// Get all shapes in the set
    pub fn shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.shape_handles.clone()
    }
    
    /// Get all shape IDs in the set
    pub fn shape_ids(&self) -> Vec<i32> {
        self.shapes.iter().copied().collect()
    }
}

impl Default for ShapeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::modeling::brep_builder::BrepBuilder;
    
    #[test]
    fn test_indexed_map() {
        let mut map = IndexedMapOfShape::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        let index = map.add(shape);
        assert_eq!(index, 0);
        assert_eq!(map.extent(), 1);
        assert!(map.contains(shape));
    }
    
    #[test]
    fn test_list_of_shape() {
        let mut list = ListOfShape::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        list.append(shape);
        assert_eq!(list.extent(), 1);
        assert!(!list.is_empty());
        assert!(list.first().is_some());
    }
    
    #[test]
    fn test_shape_set() {
        let mut set = ShapeSet::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        set.add(shape);
        assert_eq!(set.extent(), 1);
        assert!(set.contains(shape));
        assert!(!set.is_empty());
    }
}
