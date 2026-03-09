//! Standard attributes for application
//! 
//! This module provides standard attributes for BrepRs objects,
//! including name, color, layer, material, and custom attributes.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Color attribute
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from RGB (alpha = 255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Predefined colors
    pub fn red() -> Self {
        Self::from_rgb(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::from_rgb(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::from_rgb(0, 0, 255)
    }

    pub fn white() -> Self {
        Self::from_rgb(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::from_rgb(0, 0, 0)
    }

    pub fn gray() -> Self {
        Self::from_rgb(128, 128, 128)
    }

    pub fn yellow() -> Self {
        Self::from_rgb(255, 255, 0)
    }

    pub fn cyan() -> Self {
        Self::from_rgb(0, 255, 255)
    }

    pub fn magenta() -> Self {
        Self::from_rgb(255, 0, 255)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

/// Material attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Diffuse color
    pub diffuse: Color,
    /// Specular color
    pub specular: Color,
    /// Ambient color
    pub ambient: Color,
    /// Emissive color
    pub emissive: Color,
    /// Shininess
    pub shininess: f32,
    /// Transparency
    pub transparency: f32,
}

impl Material {
    /// Create a new material
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            diffuse: Color::white(),
            specular: Color::gray(),
            ambient: Color::gray(),
            emissive: Color::black(),
            shininess: 32.0,
            transparency: 0.0,
        }
    }

    /// Set diffuse color
    pub fn with_diffuse(mut self, color: Color) -> Self {
        self.diffuse = color;
        self
    }

    /// Set specular color
    pub fn with_specular(mut self, color: Color) -> Self {
        self.specular = color;
        self
    }

    /// Set shininess
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    /// Set transparency
    pub fn with_transparency(mut self, transparency: f32) -> Self {
        self.transparency = transparency;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new("Default")
    }
}

/// Layer attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    /// Layer name
    pub name: String,
    /// Layer ID
    pub id: u32,
    /// Visibility
    pub visible: bool,
    /// Locked
    pub locked: bool,
    /// Color
    pub color: Color,
}

impl Layer {
    /// Create a new layer
    pub fn new(name: &str, id: u32) -> Self {
        Self {
            name: name.to_string(),
            id,
            visible: true,
            locked: false,
            color: Color::white(),
        }
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set locked
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new("Default", 0)
    }
}

/// Attribute storage
pub struct AttributeStorage {
    attributes: HashMap<String, AttributeValue>,
}

/// Attribute value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Color value
    Color(Color),
    /// Material value
    Material(Material),
    /// Layer value
    Layer(Layer),
    /// Array of values
    Array(Vec<AttributeValue>),
    /// Object of values
    Object(HashMap<String, AttributeValue>),
}

impl AttributeStorage {
    /// Create a new attribute storage
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// Set an attribute
    pub fn set(&mut self, key: &str, value: AttributeValue) {
        self.attributes.insert(key.to_string(), value);
    }

    /// Get an attribute
    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Remove an attribute
    pub fn remove(&mut self, key: &str) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Check if an attribute exists
    pub fn contains(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Get all attributes
    pub fn get_all(&self) -> &HashMap<String, AttributeValue> {
        &self.attributes
    }

    /// Clear all attributes
    pub fn clear(&mut self) {
        self.attributes.clear();
    }

    /// Get attribute count
    pub fn count(&self) -> usize {
        self.attributes.len()
    }
}

impl Default for AttributeStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard attributes trait
pub trait HasAttributes {
    /// Get attribute storage
    fn attributes(&self) -> &AttributeStorage;
    
    /// Get mutable attribute storage
    fn attributes_mut(&mut self) -> &mut AttributeStorage;

    /// Set an attribute
    fn set_attribute(&mut self, key: &str, value: AttributeValue) {
        self.attributes_mut().set(key, value);
    }

    /// Get an attribute
    fn get_attribute(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes().get(key)
    }

    /// Remove an attribute
    fn remove_attribute(&mut self, key: &str) -> Option<AttributeValue> {
        self.attributes_mut().remove(key)
    }

    /// Get name attribute
    fn name(&self) -> String {
        self.get_attribute("name").and_then(|v| {
            if let AttributeValue::String(name) = v {
                Some(name.clone())
            } else {
                None
            }
        }).unwrap_or_else(|| "Unnamed".to_string())
    }

    /// Set name attribute
    fn set_name(&mut self, name: &str) {
        self.set_attribute("name", AttributeValue::String(name.to_string()));
    }

    /// Get color attribute
    fn color(&self) -> Color {
        self.get_attribute("color").and_then(|v| {
            if let AttributeValue::Color(color) = v {
                Some(*color)
            } else {
                None
            }
        }).unwrap_or_default()
    }

    /// Set color attribute
    fn set_color(&mut self, color: Color) {
        self.set_attribute("color", AttributeValue::Color(color));
    }

    /// Get material attribute
    fn material(&self) -> Material {
        self.get_attribute("material").and_then(|v| {
            if let AttributeValue::Material(material) = v {
                Some(material.clone())
            } else {
                None
            }
        }).unwrap_or_default()
    }

    /// Set material attribute
    fn set_material(&mut self, material: Material) {
        self.set_attribute("material", AttributeValue::Material(material));
    }

    /// Get layer attribute
    fn layer(&self) -> Layer {
        self.get_attribute("layer").and_then(|v| {
            if let AttributeValue::Layer(layer) = v {
                Some(layer.clone())
            } else {
                None
            }
        }).unwrap_or_default()
    }

    /// Set layer attribute
    fn set_layer(&mut self, layer: Layer) {
        self.set_attribute("layer", AttributeValue::Layer(layer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let red = Color::red();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);
        assert_eq!(red.a, 255);
    }

    #[test]
    fn test_material() {
        let material = Material::new("Steel")
            .with_diffuse(Color::gray())
            .with_shininess(64.0);
        assert_eq!(material.name, "Steel");
        assert_eq!(material.shininess, 64.0);
    }

    #[test]
    fn test_layer() {
        let layer = Layer::new("Layer1", 1)
            .with_visible(true)
            .with_color(Color::blue());
        assert_eq!(layer.name, "Layer1");
        assert_eq!(layer.id, 1);
        assert!(layer.visible);
    }

    #[test]
    fn test_attribute_storage() {
        let mut storage = AttributeStorage::new();
        storage.set("name", AttributeValue::String("Test Object".to_string()));
        storage.set("value", AttributeValue::Integer(42));
        storage.set("active", AttributeValue::Boolean(true));
        
        assert_eq!(storage.count(), 3);
        assert!(storage.contains("name"));
        assert!(storage.contains("value"));
        assert!(storage.contains("active"));
        
        storage.remove("value");
        assert_eq!(storage.count(), 2);
        assert!(!storage.contains("value"));
    }
}