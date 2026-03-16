//! Attributes module
//! 
//! This module provides standard attributes for BrepRs objects.

use std::collections::HashMap;

/// Standard attributes for objects
pub struct Attributes {
    attributes: HashMap<String, String>,
}

impl Attributes {
    /// Create a new attributes object
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// Add an attribute
    pub fn add(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Get an attribute
    pub fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Remove an attribute
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
    }

    /// Get all attributes
    pub fn all(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Clear all attributes
    pub fn clear(&mut self) {
        self.attributes.clear();
    }

    /// Check if an attribute exists
    pub fn contains(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Get the number of attributes
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if the attributes are empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard attribute keys
pub mod keys {
    /// Color attribute
    pub const COLOR: &str = "color";
    /// Material attribute
    pub const MATERIAL: &str = "material";
    /// Layer attribute
    pub const LAYER: &str = "layer";
    /// Name attribute
    pub const NAME: &str = "name";
    /// Description attribute
    pub const DESCRIPTION: &str = "description";
    /// Author attribute
    pub const AUTHOR: &str = "author";
    /// Creation date attribute
    pub const CREATION_DATE: &str = "creation_date";
    /// Last modified attribute
    pub const LAST_MODIFIED: &str = "last_modified";
    /// Version attribute
    pub const VERSION: &str = "version";
    /// UUID attribute
    pub const UUID: &str = "uuid";
}

/// Standard attribute values
pub mod values {
    /// Default color
    pub const DEFAULT_COLOR: &str = "#FFFFFF";
    /// Default material
    pub const DEFAULT_MATERIAL: &str = "default";
    /// Default layer
    pub const DEFAULT_LAYER: &str = "0";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attributes() {
        let mut attrs = Attributes::new();
        assert_eq!(attrs.len(), 0);
        assert!(attrs.is_empty());

        attrs.add(keys::COLOR, values::DEFAULT_COLOR);
        attrs.add(keys::MATERIAL, values::DEFAULT_MATERIAL);

        assert_eq!(attrs.len(), 2);
        assert!(!attrs.is_empty());
        assert_eq!(attrs.get(keys::COLOR), Some(&values::DEFAULT_COLOR.to_string()));
        assert_eq!(attrs.get(keys::MATERIAL), Some(&values::DEFAULT_MATERIAL.to_string()));

        attrs.remove(keys::COLOR);
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs.get(keys::COLOR), None);

        attrs.clear();
        assert_eq!(attrs.len(), 0);
        assert!(attrs.is_empty());
    }
}
