//! JSON Serialization Support
//!
//! This module provides JSON serialization/deserialization for BrepRs types.

use crate::geometry::Point;
use serde::{Deserialize, Serialize};

/// JSON serializer configuration
pub struct JsonConfig {
    pub pretty: bool,
    pub include_nulls: bool,
}

impl Default for JsonConfig {
    fn default() -> Self {
        Self {
            pretty: true,
            include_nulls: false,
        }
    }
}

/// Serialize to JSON with custom configuration
pub fn to_json_with_config<T: Serialize>(
    value: &T,
    config: &JsonConfig,
) -> Result<String, serde_json::Error> {
    if config.pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    }
}

/// JSON schema for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    pub schema_version: String,
    pub title: String,
    pub description: String,
    pub properties: serde_json::Value,
    pub required: Vec<String>,
}

impl JsonSchema {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            schema_version: "http://json-schema.org/draft-07/schema#".to_string(),
            title: title.to_string(),
            description: description.to_string(),
            properties: serde_json::Value::Object(serde_json::Map::new()),
            required: Vec::new(),
        }
    }

    pub fn add_property(&mut self, name: &str, property_type: &str, required: bool) {
        if let serde_json::Value::Object(ref mut props) = self.properties {
            let mut prop = serde_json::Map::new();
            prop.insert(
                "type".to_string(),
                serde_json::Value::String(property_type.to_string()),
            );
            props.insert(name.to_string(), serde_json::Value::Object(prop));
        }

        if required {
            self.required.push(name.to_string());
        }
    }
}

/// JSON streaming serializer for large datasets
pub struct JsonStreamSerializer<W: std::io::Write> {
    writer: W,
    first: bool,
}

impl<W: std::io::Write> JsonStreamSerializer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first: true,
        }
    }

    pub fn begin_array(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b"[")
    }

    pub fn end_array(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b"]")
    }

    pub fn begin_object(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b"{");
        self.first = true;
        Ok(())
    }

    pub fn end_object(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b"}")
    }

    pub fn write_element<T: Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.first {
            self.writer.write_all(b",")?;
        }
        self.first = false;

        let json = serde_json::to_string(value)?;
        self.writer.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn write_property<T: Serialize>(
        &mut self,
        name: &str,
        value: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.first {
            self.writer.write_all(b",")?;
        }
        self.first = false;

        let json = serde_json::to_string(name)?;
        self.writer.write_all(json.as_bytes())?;
        self.writer.write_all(b":")?;

        let value_json = serde_json::to_string(value)?;
        self.writer.write_all(value_json.as_bytes())?;
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// JSON streaming deserializer for large datasets
pub struct JsonStreamDeserializer<R: std::io::Read> {
    reader: R,
    buffer: String,
}

impl<R: std::io::Read> JsonStreamDeserializer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
        }
    }

    pub fn read_next<T: for<'de> Deserialize<'de>>(
        &mut self,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        // Simplified implementation - in production, use a proper streaming JSON parser
        let mut buf = [0u8; 1024];
        match self.reader.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.buffer.push_str(&String::from_utf8_lossy(&buf[..n]));
                // Parse complete JSON objects from buffer
                // This is a simplified placeholder
                Ok(None)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// Serialize a collection to JSON array
pub fn serialize_collection<T: Serialize>(
    items: &[T],
    pretty: bool,
) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(items)
    } else {
        serde_json::to_string(items)
    }
}

/// Deserialize a JSON array to a collection
pub fn deserialize_collection<T: for<'de> Deserialize<'de>>(
    json: &str,
) -> Result<Vec<T>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serialize with type information for polymorphic types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedJson<T> {
    #[serde(rename = "type")]
    pub type_name: String,
    pub data: T,
}

impl<T> TypedJson<T> {
    pub fn new(type_name: &str, data: T) -> Self {
        Self {
            type_name: type_name.to_string(),
            data,
        }
    }
}

/// JSON diff utility for comparing two JSON values
pub fn json_diff(old: &serde_json::Value, new: &serde_json::Value) -> JsonDiff {
    let mut diff = JsonDiff::new();
    compute_diff("", old, new, &mut diff);
    diff
}

/// JSON diff result
#[derive(Debug, Clone, Default)]
pub struct JsonDiff {
    pub added: Vec<(String, serde_json::Value)>,
    pub removed: Vec<(String, serde_json::Value)>,
    pub modified: Vec<(String, serde_json::Value, serde_json::Value)>,
}

impl JsonDiff {
    fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }

    pub fn has_changes(&self) -> bool {
        !self.is_empty()
    }
}

fn compute_diff(path: &str, old: &serde_json::Value, new: &serde_json::Value, diff: &mut JsonDiff) {
    match (old, new) {
        (serde_json::Value::Object(old_map), serde_json::Value::Object(new_map)) => {
            // Check for added and modified keys
            for (key, new_val) in new_map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                match old_map.get(key) {
                    None => diff.added.push((new_path, new_val.clone())),
                    Some(old_val) if old_val != new_val => {
                        compute_diff(&new_path, old_val, new_val, diff);
                    }
                    _ => {}
                }
            }

            // Check for removed keys
            for (key, old_val) in old_map {
                if !new_map.contains_key(key) {
                    let old_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    diff.removed.push((old_path, old_val.clone()));
                }
            }
        }
        (old_val, new_val) if old_val != new_val => {
            diff.modified
                .push((path.to_string(), old.clone(), new.clone()));
        }
        _ => {}
    }
}

/// JSON merge utility
pub fn json_merge(base: &mut serde_json::Value, overlay: &serde_json::Value) {
    match (base, overlay) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(overlay_map)) => {
            for (key, overlay_val) in overlay_map {
                if let Some(base_val) = base_map.get_mut(key) {
                    json_merge(base_val, overlay_val);
                } else {
                    base_map.insert(key.clone(), overlay_val.clone());
                }
            }
        }
        (base_val, overlay_val) => {
            *base_val = overlay_val.clone();
        }
    }
}

/// JSON path query utility
pub fn json_path<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;

    for part in parts {
        match current {
            serde_json::Value::Object(map) => {
                current = map.get(part)?;
            }
            serde_json::Value::Array(arr) => {
                let index: usize = part.parse().ok()?;
                current = arr.get(index)?;
            }
            _ => return None,
        }
    }

    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_config() {
        let config = JsonConfig::default();
        assert!(config.pretty);
        assert!(!config.include_nulls);
    }

    #[test]
    fn test_json_schema() {
        let mut schema = JsonSchema::new("Point", "A 3D point");
        schema.add_property("x", "number", true);
        schema.add_property("y", "number", true);
        schema.add_property("z", "number", true);

        assert_eq!(schema.required.len(), 3);
    }

    #[test]
    fn test_typed_json() {
        let point = Point::new(1.0, 2.0, 3.0);
        let typed = TypedJson::new("Point", point);

        assert_eq!(typed.type_name, "Point");
        assert!((typed.data.x() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_json_diff() {
        let old = serde_json::json!({
            "name": "test",
            "value": 10,
            "removed": true
        });

        let new = serde_json::json!({
            "name": "test",
            "value": 20,
            "added": true
        });

        let diff = json_diff(&old, &new);
        assert!(!diff.is_empty());
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.modified.len(), 1);
    }

    #[test]
    fn test_json_merge() {
        let mut base = serde_json::json!({
            "a": 1,
            "b": { "c": 2 }
        });

        let overlay = serde_json::json!({
            "b": { "d": 3 },
            "e": 4
        });

        json_merge(&mut base, &overlay);

        assert_eq!(base["a"], 1);
        assert_eq!(base["b"]["c"], 2);
        assert_eq!(base["b"]["d"], 3);
        assert_eq!(base["e"], 4);
    }

    #[test]
    fn test_json_path() {
        let value = serde_json::json!({
            "user": {
                "name": "John",
                "addresses": [
                    { "city": "New York" },
                    { "city": "Los Angeles" }
                ]
            }
        });

        assert_eq!(
            json_path(&value, "user.name"),
            Some(&serde_json::json!("John"))
        );
        assert_eq!(
            json_path(&value, "user.addresses.0.city"),
            Some(&serde_json::json!("New York"))
        );
        assert_eq!(json_path(&value, "nonexistent"), None);
    }
}
