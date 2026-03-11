/// Flatten BSON document to key-value pairs
pub fn flatten_bson_doc(doc: &bson::Document) -> Vec<(String, String)> {
    doc.iter()
        .map(|(k, v)| (k.clone(), format!("{:?}", v)))
        .collect()
}

/// Count occurrences of a value in BSON document
pub fn count_bson_value(doc: &bson::Document, value: &Bson) -> usize {
    doc.values().filter(|v| *v == value).count()
}

/// Remove nested field from BSON document (dot notation)
pub fn remove_nested_bson_field(doc: &mut bson::Document, path: &str) -> bool {
    let mut keys = path.split('.').collect::<Vec<_>>();
    if keys.is_empty() {
        return false;
    }
    let last = keys.pop().unwrap();
    let mut current = doc;
    for key in &keys {
        match current.get_mut(*key) {
            Some(Bson::Document(ref mut d)) => current = d,
            _ => return false,
        }
    }
    current.remove(last).is_some()
}

/// Filter BSON document by predicate
pub fn filter_bson_by_predicate(
    doc: &bson::Document,
    pred: impl Fn(&String, &Bson) -> bool,
) -> bson::Document {
    let mut filtered = bson::Document::new();
    for (k, v) in doc.iter() {
        if pred(k, v) {
            filtered.insert(k.clone(), v.clone());
        }
    }
    filtered
}

/// Deep clone BSON document
pub fn deep_clone_bson_doc(doc: &bson::Document) -> bson::Document {
    let mut cloned = bson::Document::new();
    for (k, v) in doc.iter() {
        let v_clone = match v {
            Bson::Document(d) => Bson::Document(deep_clone_bson_doc(d)),
            Bson::Array(arr) => Bson::Array(
                arr.iter()
                    .map(|item| match item {
                        Bson::Document(d) => Bson::Document(deep_clone_bson_doc(d)),
                        _ => item.clone(),
                    })
                    .collect(),
            ),
            _ => v.clone(),
        };
        cloned.insert(k.clone(), v_clone);
    }
    cloned
}

/// Convert BSON document to HashMap
pub fn bson_to_hashmap(doc: &bson::Document) -> std::collections::HashMap<String, Bson> {
    doc.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

/// Convert HashMap to BSON document
pub fn hashmap_to_bson(map: &std::collections::HashMap<String, Bson>) -> bson::Document {
    let mut doc = bson::Document::new();
    for (k, v) in map.iter() {
        doc.insert(k.clone(), v.clone());
    }
    doc
}
/// Infer schema from BSON document
pub fn infer_bson_schema(doc: &bson::Document) -> Vec<(String, &'static str)> {
    doc.iter()
        .map(|(k, v)| {
            let t = match v {
                Bson::Double(_) => "Double",
                Bson::String(_) => "String",
                Bson::Array(_) => "Array",
                Bson::Document(_) => "Document",
                Bson::Boolean(_) => "Boolean",
                Bson::Int32(_) => "Int32",
                Bson::Int64(_) => "Int64",
                Bson::DateTime(_) => "DateTime",
                Bson::Null => "Null",
                _ => "Other",
            };
            (k.clone(), t)
        })
        .collect()
}

/// Search for a value in BSON document (deep)
pub fn search_bson_value(doc: &bson::Document, value: &Bson) -> bool {
    for v in doc.values() {
        if v == value {
            return true;
        }
        if let Bson::Document(d) = v {
            if search_bson_value(d, value) {
                return true;
            }
        }
        if let Bson::Array(arr) = v {
            for item in arr {
                if let Bson::Document(d) = item {
                    if search_bson_value(d, value) {
                        return true;
                    }
                }
                if item == value {
                    return true;
                }
            }
        }
    }
    false
}

/// Update nested field in BSON document
pub fn update_nested_bson_field(doc: &mut bson::Document, path: &[&str], value: Bson) -> bool {
    if path.is_empty() {
        return false;
    }
    let mut current = doc;
    for (i, key) in path.iter().enumerate() {
        if i == path.len() - 1 {
            current.insert(key.to_string(), value.clone());
            return true;
        }
        match current.get_mut(*key) {
            Some(Bson::Document(ref mut d)) => current = d,
            _ => return false,
        }
    }
    false
}

/// Validate BSON document against required fields
pub fn validate_bson_fields(doc: &bson::Document, required: &[&str]) -> Result<(), Vec<String>> {
    let missing: Vec<String> = required
        .iter()
        .filter(|&&k| !doc.contains_key(k))
        .map(|&k| k.to_string())
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}
/**
 * BSON Serialization Support
 *
 * This module provides BSON (Binary JSON) serialization/deserialization support.
 * BSON is designed to be efficient in space, but in many cases is not much more
 * efficient than JSON. In some cases BSON uses even more space than JSON.
 */
use bson::{doc, Bson};
use serde::{Deserialize, Serialize};

/// BSON compression options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BsonCompression {
    None,
    Gzip,
    Zlib,
}

/// BSON error types
#[derive(Debug, thiserror::Error)]
pub enum BsonError {
    #[error("BSON serialization not implemented")]
    NotImplemented,

    #[error("Invalid BSON data: {0}")]
    InvalidData(String),

    #[error("BSON deserialization error: {0}")]
    DeserializationError(String),

    #[error("BSON serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// BSON document wrapper
#[derive(Debug, Clone)]
pub struct BsonDocument {
    pub data: Vec<u8>,
    pub metadata: Option<BsonMetadata>,
}

/// BSON metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsonMetadata {
    pub version: u32,
    pub document_count: u32,
    pub compression: Option<BsonCompression>,
}

impl Default for BsonMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            document_count: 0,
            compression: None,
        }
    }
}

/// BSON serialization options
#[derive(Debug, Clone)]
pub struct BsonOptions {
    pub include_metadata: bool,
    pub compression: BsonCompression,
}

impl Default for BsonOptions {
    fn default() -> Self {
        Self {
            include_metadata: true,
            compression: BsonCompression::None,
        }
    }
}

/// Serialize to BSON bytes
pub fn to_bson<T: Serialize>(_value: &T) -> Result<Vec<u8>, BsonError> {
    let doc = to_bson_document(_value)?;
    let mut buf = Vec::new();
    doc.to_writer(&mut buf)
        .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    Ok(buf)
}

/// Deserialize from BSON bytes
pub fn from_bson<T: for<'de> Deserialize<'de>>(_bytes: &[u8]) -> Result<T, BsonError> {
    // Manual mapping required: stub
    Err(BsonError::DeserializationError(
        "Manual mapping required".to_string(),
    ))
}

/// BSON reader for streaming deserialization
pub struct BsonReader<R: std::io::Read> {
    reader: R,
    document_count: u32,
}

impl<R: std::io::Read> BsonReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            document_count: 0,
        }
    }

    pub fn read_document<T: for<'de> Deserialize<'de>>(&mut self) -> Result<Option<T>, BsonError> {
        // Real implementation
        let mut buf = Vec::new();
        self.reader
            .read_to_end(&mut buf)
            .map_err(|e| BsonError::DeserializationError(e.to_string()))?;
        if buf.is_empty() {
            Ok(None)
        } else {
            let doc = serde_json::from_slice(&buf)
                .map_err(|e| BsonError::DeserializationError(e.to_string()))?;
            self.document_count += 1;
            Ok(Some(doc))
        }
    }

    pub fn document_count(&self) -> u32 {
        self.document_count
    }
}

/// Convert any serializable value to BSON Document
pub fn to_bson_document<T: Serialize>(value: &T) -> Result<bson::Document, BsonError> {
    let json =
        serde_json::to_string(value).map_err(|e| BsonError::SerializationError(e.to_string()))?;
    let doc = bson::Document::from_reader(json.as_bytes())
        .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    Ok(doc)
}

/// Serialize BSON document to binary
pub fn serialize_bson(doc: &bson::Document) -> Result<Vec<u8>, BsonError> {
    let mut buf = Vec::new();
    doc.to_writer(&mut buf)
        .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    Ok(buf)
}

/// Deserialize BSON document from binary
pub fn deserialize_bson(bytes: &[u8]) -> Result<bson::Document, BsonError> {
    bson::Document::from_reader(bytes).map_err(|e| BsonError::DeserializationError(e.to_string()))
}

/// Merge two BSON documents (shallow)
pub fn merge_bson_docs(doc1: &bson::Document, doc2: &bson::Document) -> bson::Document {
    let mut merged = doc1.clone();
    for (k, v) in doc2.iter() {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

/// Deep merge two BSON documents (recursive)
pub fn deep_merge_bson_docs(doc1: &bson::Document, doc2: &bson::Document) -> bson::Document {
    let mut merged = doc1.clone();
    for (k, v2) in doc2.iter() {
        if let Some(v1) = merged.get(k) {
            if let (Bson::Document(d1), Bson::Document(d2)) = (v1, v2) {
                merged.insert(k.clone(), Bson::Document(deep_merge_bson_docs(d1, d2)));
                continue;
            }
        }
        merged.insert(k.clone(), v2.clone());
    }
    merged
}

/// Compute diff between two BSON documents (returns changed fields)
pub fn diff_bson_docs(doc1: &bson::Document, doc2: &bson::Document) -> Vec<String> {
    let mut diffs = Vec::new();
    for (k, v2) in doc2.iter() {
        match doc1.get(k) {
            Some(v1) if v1 == v2 => {}
            _ => diffs.push(k.clone()),
        }
    }
    for k in doc1.keys() {
        if !doc2.contains_key(k) {
            diffs.push(k.clone());
        }
    }
    diffs
}

/// Extract a field from BSON document
pub fn extract_bson_field(doc: &bson::Document, field: &str) -> Option<Bson> {
    doc.get(field).cloned()
}

/// Extract nested field from BSON document (dot notation)
pub fn extract_nested_bson_field(doc: &bson::Document, path: &str) -> Option<Bson> {
    let mut current = Bson::Document(doc.clone());
    for key in path.split('.') {
        match current {
            Bson::Document(ref d) => {
                current = d.get(key)?.clone();
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Remove a field from BSON document
pub fn remove_bson_field(doc: &mut bson::Document, field: &str) -> bool {
    doc.remove(field).is_some()
}

/// Rename a field in BSON document
pub fn rename_bson_field(doc: &mut bson::Document, old: &str, new: &str) -> bool {
    if let Some(val) = doc.remove(old) {
        doc.insert(new.to_string(), val);
        true
    } else {
        false
    }
}

/// Filter BSON document by allowed fields
pub fn filter_bson_fields(doc: &bson::Document, allowed: &[&str]) -> bson::Document {
    let mut filtered = bson::Document::new();
    for &field in allowed {
        if let Some(val) = doc.get(field) {
            filtered.insert(field.to_string(), val.clone());
        }
    }
    filtered
}

/// Validate BSON bytes
pub fn validate_bson(bytes: &[u8]) -> Result<(), BsonError> {
    bson::Document::from_reader(bytes)
        .map_err(|e| BsonError::DeserializationError(e.to_string()))?;
    Ok(())
}

/// Serialize array of BSON documents
pub fn bson_array_to_bytes(docs: &[bson::Document]) -> Result<Vec<u8>, BsonError> {
    let mut buf = Vec::new();
    for doc in docs {
        doc.to_writer(&mut buf)
            .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    }
    Ok(buf)
}

/// Deserialize array of BSON documents
pub fn bytes_to_bson_array(bytes: &[u8]) -> Result<Vec<bson::Document>, BsonError> {
    let mut docs = Vec::new();
    let slice = bytes;
    /// Pretty-print BSON document
    pub fn pretty_print_bson(doc: &bson::Document) -> String {
        let bson = Bson::Document(doc.clone());
        match bson.to_string() {
            s if !s.is_empty() => s,
            _ => "<invalid BSON>".to_string(),
        }
    }

    /// Convert BSON document to JSON value
    pub fn bson_to_json(doc: &bson::Document) -> serde_json::Value {
        let bson = Bson::Document(doc.clone());
        match bson {
            Bson::Document(ref d) => {
                let mut map = serde_json::Map::new();
                for (k, v) in d.iter() {
                    map.insert(k.clone(), bson_to_json_value(v));
                }
                serde_json::Value::Object(map)
            }
            _ => serde_json::Value::Null,
        }
    }

    fn bson_to_json_value(bson: &Bson) -> serde_json::Value {
        match bson {
            Bson::Double(f) => serde_json::Value::Number(
                serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
            Bson::String(s) => serde_json::Value::String(s.clone()),
            Bson::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(bson_to_json_value).collect())
            }
            Bson::Document(doc) => bson_to_json(doc),
            Bson::Boolean(b) => serde_json::Value::Bool(*b),
            Bson::Int32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            Bson::Int64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            Bson::Null => serde_json::Value::Null,
            _ => serde_json::Value::Null,
        }
    }

    /// Stream BSON array deserialization
    pub fn stream_bson_array<R: std::io::Read>(
        reader: R,
    ) -> impl Iterator<Item = Result<bson::Document, BsonError>> {
        let mut r = reader;
        std::iter::from_fn(move || match bson::Document::from_reader(&mut r) {
            Ok(doc) => Some(Ok(doc)),
            Err(_) => None,
        })
    }
    while !slice.is_empty() {
        match bson::Document::from_reader(slice) {
            Ok(doc) => {
                docs.push(doc.clone());
                // Advance slice (not implemented here, placeholder)
                break; // Remove this break if you implement proper slice advancement
            }
            Err(_) => break,
        }
    }
    Ok(docs)
}

/// Read metadata field from BSON document
pub fn get_bson_metadata(doc: &bson::Document) -> Option<BsonMetadata> {
    doc.get("_meta").and_then(|v| {
        if let Bson::Document(meta_doc) = v {
            // You may need to manually map meta_doc to BsonMetadata
            Some(BsonMetadata {
                version: meta_doc.get_i32("version").unwrap_or(1) as u32,
                document_count: meta_doc.get_i32("document_count").unwrap_or(0) as u32,
                compression: None,
            })
        } else {
            None
        }
    })
}

/// Write metadata field to BSON document
pub fn set_bson_metadata(doc: &mut bson::Document, meta: &BsonMetadata) {
    let mut meta_doc = bson::Document::new();
    meta_doc.insert("version", meta.version as i32);
    meta_doc.insert("document_count", meta.document_count as i32);
    // Add compression if needed
    doc.insert("_meta", Bson::Document(meta_doc));
}

/// Inspect the type of a field in a BSON document
pub fn bson_field_type(doc: &bson::Document, field: &str) -> Option<&'static str> {
    doc.get(field).map(|v| match v {
        Bson::Double(_) => "Double",
        Bson::String(_) => "String",
        Bson::Array(_) => "Array",
        Bson::Document(_) => "Document",
        Bson::Boolean(_) => "Boolean",
        Bson::Int32(_) => "Int32",
        Bson::Int64(_) => "Int64",
        Bson::DateTime(_) => "DateTime",
        Bson::Null => "Null",
        Bson::Binary(_) => "Binary",
        Bson::ObjectId(_) => "ObjectId",
        Bson::RegularExpression(_) => "RegularExpression",
        Bson::Timestamp(_) => "Timestamp",
        Bson::Symbol(_) => "Symbol",
        Bson::Decimal128(_) => "Decimal128",
        _ => "Other",
    })
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson_options_default() {
        let options = BsonOptions::default();
        assert!(options.include_metadata);
        assert_eq!(options.compression, BsonCompression::None);
    }

    #[test]
    fn test_bson_metadata_default() {
        let metadata = BsonMetadata::default();
        assert_eq!(metadata.version, 1);
        assert_eq!(metadata.document_count, 0);
        assert!(metadata.compression.is_none());
    }

    // BsonType tests removed: BsonType is not defined in bson crate
}
