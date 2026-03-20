//! JSON Serialization Support
//!
//! This module provides JSON serialization/deserialization for BrepRs types.

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
        self.writer.write_all(b"{")?;
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
    position: usize,
}

impl<R: std::io::Read> JsonStreamDeserializer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
            position: 0,
        }
    }

    pub fn read_next<T: for<'de> Deserialize<'de>>(
        &mut self,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        loop {
            // Try to find a complete JSON object in the buffer
            if let Some(json_str) = self.extract_json_object() {
                // Try to parse the extracted JSON
                match serde_json::from_str::<T>(json_str) {
                    Ok(value) => return Ok(Some(value)),
                    Err(e) => {
                        // If parsing fails, continue reading more data
                        eprintln!("JSON parse error: {}, trying to read more data", e);
                    }
                }
            }

            // Read more data from the reader
            let mut buf = [0u8; 4096];
            match self.reader.read(&mut buf) {
                Ok(0) => {
                    // End of stream - try to parse remaining buffer
                    if self.position < self.buffer.len() {
                        let remaining = &self.buffer[self.position..].trim();
                        if !remaining.is_empty() {
                            match serde_json::from_str::<T>(remaining) {
                                Ok(value) => {
                                    self.position = self.buffer.len();
                                    return Ok(Some(value));
                                }
                                Err(_) => return Ok(None),
                            }
                        }
                    }
                    return Ok(None);
                }
                Ok(n) => {
                    self.buffer.push_str(&String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }

    /// Extract a complete JSON object from the buffer
    fn extract_json_object(&mut self) -> Option<&str> {
        let remaining = &self.buffer[self.position..];
        let trimmed = remaining.trim_start();

        // Skip whitespace
        let start_offset = remaining.len() - trimmed.len();
        let start_pos = self.position + start_offset;

        if trimmed.is_empty() {
            return None;
        }

        // Look for the start of a JSON value
        let first_char = trimmed.chars().next()?;
        match first_char {
            '{' => self.extract_balanced_object(start_pos, '{', '}'),
            '[' => self.extract_balanced_object(start_pos, '[', ']'),
            '"' => self.extract_string(start_pos),
            't' | 'f' => self.extract_boolean(start_pos),
            'n' => self.extract_null(start_pos),
            c if c.is_ascii_digit() || c == '-' => self.extract_number(start_pos),
            _ => None,
        }
    }

    /// Extract a balanced object (object or array)
    fn extract_balanced_object(
        &mut self,
        start_pos: usize,
        open: char,
        close: char,
    ) -> Option<&str> {
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let chars: Vec<char> = self.buffer[start_pos..].chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match c {
                '\\' if in_string => escape_next = true,
                '"' => in_string = !in_string,
                _ if !in_string => {
                    if c == open {
                        depth += 1;
                    } else if c == close {
                        depth -= 1;
                        if depth == 0 {
                            let end_pos =
                                start_pos + chars[..=i].iter().map(|c| c.len_utf8()).sum::<usize>();
                            let result = &self.buffer[start_pos..end_pos];
                            self.position = end_pos;
                            return Some(result);
                        }
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// Extract a JSON string
    fn extract_string(&mut self, start_pos: usize) -> Option<&str> {
        let chars: Vec<char> = self.buffer[start_pos..].chars().collect();
        if *chars.first()? != '"' {
            return None;
        }

        let mut escape_next = false;
        for (i, &c) in chars.iter().skip(1).enumerate() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match c {
                '\\' => escape_next = true,
                '"' => {
                    let end_pos =
                        start_pos + chars[..=i + 1].iter().map(|c| c.len_utf8()).sum::<usize>();
                    let result = &self.buffer[start_pos..end_pos];
                    self.position = end_pos;
                    return Some(result);
                }
                _ => {}
            }
        }

        None
    }

    /// Extract a JSON boolean
    fn extract_boolean(&mut self, start_pos: usize) -> Option<&str> {
        let remaining = &self.buffer[start_pos..];
        if remaining.starts_with("true") {
            self.position = start_pos + 4;
            Some(&self.buffer[start_pos..start_pos + 4])
        } else if remaining.starts_with("false") {
            self.position = start_pos + 5;
            Some(&self.buffer[start_pos..start_pos + 5])
        } else {
            None
        }
    }

    /// Extract a JSON null
    fn extract_null(&mut self, start_pos: usize) -> Option<&str> {
        let remaining = &self.buffer[start_pos..];
        if remaining.starts_with("null") {
            self.position = start_pos + 4;
            Some(&self.buffer[start_pos..start_pos + 4])
        } else {
            None
        }
    }

    /// Extract a JSON number
    fn extract_number(&mut self, start_pos: usize) -> Option<&str> {
        let chars: Vec<char> = self.buffer[start_pos..].chars().collect();
        let mut end = 0;

        // Optional leading minus
        if chars.get(end) == Some(&'-') {
            end += 1;
        }

        // Integer part
        while let Some(&c) = chars.get(end) {
            if c.is_ascii_digit() {
                end += 1;
            } else {
                break;
            }
        }

        // Optional decimal part
        if chars.get(end) == Some(&'.') {
            end += 1;
            while let Some(&c) = chars.get(end) {
                if c.is_ascii_digit() {
                    end += 1;
                } else {
                    break;
                }
            }
        }

        // Optional exponent
        if let Some(&'e' | &'E') = chars.get(end) {
            end += 1;
            if let Some(&'+' | &'-') = chars.get(end) {
                end += 1;
            }
            while let Some(&c) = chars.get(end) {
                if c.is_ascii_digit() {
                    end += 1;
                } else {
                    break;
                }
            }
        }

        if end > 0 {
            let end_pos = start_pos + chars[..end].iter().map(|c| c.len_utf8()).sum::<usize>();
            let result = &self.buffer[start_pos..end_pos];
            self.position = end_pos;
            Some(result)
        } else {
            None
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
    use crate::geometry::Point;

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
