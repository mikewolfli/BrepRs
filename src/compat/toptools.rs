#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! Topology Tools Compatibility Module
//! 
//! Provides OpenCASCADE-compatible TopTools API for topology utilities.

use crate::foundation::handle::Handle;
use crate::topology::top_tools::{
    IndexedMapOfShape,
    ListOfShape,
    SequenceOfShape,
    DataMapOfShapeShape,
    DataMapOfShapeInteger,
    ShapeSet,
};
use crate::topology::TopoDsShape;

/// Indexed map of shapes (OpenCASCADE compatible)
pub struct TopTools_IndexedMapOfShape {
    inner: IndexedMapOfShape,
}

impl TopTools_IndexedMapOfShape {
    /// Create a new indexed map
    pub fn new() -> Self {
        Self {
            inner: IndexedMapOfShape::new(),
        }
    }
    
    /// Add a shape to the map
    pub fn Add(&mut self, S: &TopoDsShape) -> i32 {
        self.inner.add(S)
    }
    
    /// Find the index of a shape
    pub fn FindIndex(&self, S: &TopoDsShape) -> i32 {
        self.inner.find_index(S)
    }
    
    /// Find a shape by index
    pub fn FindFromIndex(&self, Index: i32) -> Option<Handle<TopoDsShape>> {
        self.inner.find_from_index(Index)
    }
    
    /// Check if the map contains a shape
    pub fn Contains(&self, S: &TopoDsShape) -> bool {
        self.inner.contains(S)
    }
    
    /// Get the number of shapes in the map
    pub fn Extent(&self) -> i32 {
        self.inner.extent()
    }
    
    /// Clear the map
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
    
    /// Remove a shape from the map
    pub fn Remove(&mut self, S: &TopoDsShape) -> bool {
        self.inner.remove(S)
    }
    
    /// Get all shapes in the map
    pub fn Shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.shapes()
    }
}

impl Default for TopTools_IndexedMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// List of shapes (OpenCASCADE compatible)
pub struct TopTools_ListOfShape {
    inner: ListOfShape,
}

impl TopTools_ListOfShape {
    /// Create a new list
    pub fn new() -> Self {
        Self {
            inner: ListOfShape::new(),
        }
    }
    
    /// Append a shape to the list
    pub fn Append(&mut self, S: &TopoDsShape) {
        self.inner.append(S)
    }
    
    /// Prepend a shape to the list
    pub fn Prepend(&mut self, S: &TopoDsShape) {
        self.inner.prepend(S)
    }
    
    /// Insert a shape at a specific position
    pub fn InsertBefore(&mut self, Index: i32, S: &TopoDsShape) {
        self.inner.insert_before(Index, S)
    }
    
    /// Insert a shape after a specific position
    pub fn InsertAfter(&mut self, Index: i32, S: &TopoDsShape) {
        self.inner.insert_after(Index, S)
    }
    
    /// Remove a shape from the list
    pub fn Remove(&mut self, S: &TopoDsShape) -> bool {
        self.inner.remove(S)
    }
    
    /// Remove a shape at a specific index
    pub fn RemoveAt(&mut self, Index: i32) -> bool {
        self.inner.remove_at(Index)
    }
    
    /// Get the first shape
    pub fn First(&self) -> Option<Handle<TopoDsShape>> {
        self.inner.first()
    }
    
    /// Get the last shape
    pub fn Last(&self) -> Option<Handle<TopoDsShape>> {
        self.inner.last()
    }
    
    /// Get a shape at a specific index
    pub fn Value(&self, Index: i32) -> Option<Handle<TopoDsShape>> {
        self.inner.value(Index)
    }
    
    /// Get the number of shapes in the list
    pub fn Extent(&self) -> i32 {
        self.inner.extent()
    }
    
    /// Check if the list is empty
    pub fn IsEmpty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Clear the list
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
    
    /// Get all shapes in the list
    pub fn Shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.shapes()
    }
    
    /// Reverse the list
    pub fn Reverse(&mut self) {
        self.inner.reverse()
    }
    
    /// Sort the list
    pub fn Sort(&mut self) {
        self.inner.sort()
    }
}

impl Default for TopTools_ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Sequence of shapes (OpenCASCADE compatible)
pub struct TopTools_SequenceOfShape {
    inner: SequenceOfShape,
}

impl TopTools_SequenceOfShape {
    /// Create a new sequence
    pub fn new() -> Self {
        Self {
            inner: SequenceOfShape::new(),
        }
    }
    
    /// Append a shape to the sequence
    pub fn Append(&mut self, S: &TopoDsShape) {
        self.inner.append(S)
    }
    
    /// Prepend a shape to the sequence
    pub fn Prepend(&mut self, S: &TopoDsShape) {
        self.inner.prepend(S)
    }
    
    /// Insert a shape at a specific position
    pub fn InsertBefore(&mut self, Index: i32, S: &TopoDsShape) {
        self.inner.insert_before(Index, S)
    }
    
    /// Insert a shape after a specific position
    pub fn InsertAfter(&mut self, Index: i32, S: &TopoDsShape) {
        self.inner.insert_after(Index, S)
    }
    
    /// Remove a shape from the sequence
    pub fn Remove(&mut self, S: &TopoDsShape) -> bool {
        self.inner.remove(S)
    }
    
    /// Remove a shape at a specific index
    pub fn RemoveAt(&mut self, Index: i32) -> bool {
        self.inner.remove_at(Index)
    }
    
    /// Get the first shape
    pub fn First(&self) -> Option<Handle<TopoDsShape>> {
        self.inner.first()
    }
    
    /// Get the last shape
    pub fn Last(&self) -> Option<Handle<TopoDsShape>> {
        self.inner.last()
    }
    
    /// Get a shape at a specific index
    pub fn Value(&self, Index: i32) -> Option<Handle<TopoDsShape>> {
        self.inner.value(Index)
    }
    
    /// Change the value at a specific index
    pub fn SetValue(&mut self, Index: i32, S: &TopoDsShape) {
        self.inner.set_value(Index, S)
    }
    
    /// Get the number of shapes in the sequence
    pub fn Length(&self) -> i32 {
        self.inner.length()
    }
    
    /// Check if the sequence is empty
    pub fn IsEmpty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Clear the sequence
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
    
    /// Get all shapes in the sequence
    pub fn Shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.shapes()
    }
    
    /// Reverse the sequence
    pub fn Reverse(&mut self) {
        self.inner.reverse()
    }
    
    /// Exchange two elements
    pub fn Exchange(&mut self, I: i32, J: i32) {
        self.inner.exchange(I, J)
    }
}

impl Default for TopTools_SequenceOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Data map of shapes (OpenCASCADE compatible)
pub struct TopTools_DataMapOfShapeShape {
    inner: DataMapOfShapeShape,
}

impl TopTools_DataMapOfShapeShape {
    /// Create a new data map
    pub fn new() -> Self {
        Self {
            inner: DataMapOfShapeShape::new(),
        }
    }
    
    /// Bind a shape to another shape
    pub fn Bind(&mut self, Key: &TopoDsShape, Value: &TopoDsShape) {
        self.inner.bind(Key, Value)
    }
    
    /// Find a shape by key
    pub fn Find(&self, Key: &TopoDsShape) -> Option<Handle<TopoDsShape>> {
        self.inner.find(Key)
    }
    
    /// Check if the map contains a key
    pub fn IsBound(&self, Key: &TopoDsShape) -> bool {
        self.inner.is_bound(Key)
    }
    
    /// Unbind a key
    pub fn UnBind(&mut self, Key: &TopoDsShape) -> bool {
        self.inner.unbind(Key)
    }
    
    /// Get the number of entries in the map
    pub fn Extent(&self) -> i32 {
        self.inner.extent()
    }
    
    /// Clear the map
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
    
    /// Get all keys in the map
    pub fn Keys(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.keys()
    }
    
    /// Get all values in the map
    pub fn Values(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.values()
    }
}

impl Default for TopTools_DataMapOfShapeShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Data map of shape to integer (OpenCASCADE compatible)
pub struct TopTools_DataMapOfShapeInteger {
    inner: DataMapOfShapeInteger,
}

impl TopTools_DataMapOfShapeInteger {
    /// Create a new data map
    pub fn new() -> Self {
        Self {
            inner: DataMapOfShapeInteger::new(),
        }
    }
    
    /// Bind a shape to an integer
    pub fn Bind(&mut self, Key: &TopoDsShape, Value: i32) {
        self.inner.bind(Key, Value)
    }
    
    /// Find an integer by key
    pub fn Find(&self, Key: &TopoDsShape) -> Option<i32> {
        self.inner.find(Key)
    }
    
    /// Check if the map contains a key
    pub fn IsBound(&self, Key: &TopoDsShape) -> bool {
        self.inner.is_bound(Key)
    }
    
    /// Unbind a key
    pub fn UnBind(&mut self, Key: &TopoDsShape) -> bool {
        self.inner.unbind(Key)
    }
    
    /// Get the number of entries in the map
    pub fn Extent(&self) -> i32 {
        self.inner.extent()
    }
    
    /// Clear the map
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
}

impl Default for TopTools_DataMapOfShapeInteger {
    fn default() -> Self {
        Self::new()
    }
}

/// Shape set for storing unique shapes (OpenCASCADE compatible)
pub struct TopTools_ShapeSet {
    inner: ShapeSet,
}

impl TopTools_ShapeSet {
    /// Create a new shape set
    pub fn new() -> Self {
        Self {
            inner: ShapeSet::new(),
        }
    }
    
    /// Add a shape to the set
    pub fn Add(&mut self, S: &TopoDsShape) {
        self.inner.add(S)
    }
    
    /// Remove a shape from the set
    pub fn Remove(&mut self, S: &TopoDsShape) -> bool {
        self.inner.remove(S)
    }
    
    /// Check if the set contains a shape
    pub fn Contains(&self, S: &TopoDsShape) -> bool {
        self.inner.contains(S)
    }
    
    /// Get the number of shapes in the set
    pub fn Extent(&self) -> i32 {
        self.inner.extent()
    }
    
    /// Clear the set
    pub fn Clear(&mut self) {
        self.inner.clear()
    }
    
    /// Check if the set is empty
    pub fn IsEmpty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Get all shapes in the set
    pub fn Shapes(&self) -> Vec<Handle<TopoDsShape>> {
        self.inner.shapes()
    }
    
    /// Get all shape IDs in the set
    pub fn ShapeIds(&self) -> Vec<i32> {
        self.inner.shape_ids()
    }
}

impl Default for TopTools_ShapeSet {
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
        let mut map = TopTools_IndexedMapOfShape::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        let index = map.Add(shape);
        assert_eq!(index, 0);
        assert_eq!(map.Extent(), 1);
        assert!(map.Contains(shape));
    }
    
    #[test]
    fn test_list_of_shape() {
        let mut list = TopTools_ListOfShape::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        list.Append(shape);
        assert_eq!(list.Extent(), 1);
        assert!(!list.IsEmpty());
        assert!(list.First().is_some());
    }
    
    #[test]
    fn test_shape_set() {
        let mut set = TopTools_ShapeSet::new();
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let shape = v1.shape();
        
        set.Add(shape);
        assert_eq!(set.Extent(), 1);
        assert!(set.Contains(shape));
        assert!(!set.IsEmpty());
    }
}
