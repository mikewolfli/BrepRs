/**
 * BSON Serialization Support
 *
 * This module provides BSON (Binary JSON) serialization/deserialization support.
 * BSON is designed to be efficient in space, but in many cases is not much more
 * efficient than JSON. In some cases BSON uses even more space than JSON.
 */
#[cfg(feature = "serde")]
use bson::{de, doc, ser, Bson, Document};

#[cfg(not(feature = "serde"))]
use bson::{doc, Bson, Document};

use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_json;

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
    #[error("Invalid BSON data: {0}")]
    InvalidData(String),

    #[error("BSON deserialization error: {0}")]
    DeserializationError(String),

    #[error("BSON serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
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
    #[cfg(feature = "serde")]
    {
        ser::serialize_to_vec(_value).map_err(|e| BsonError::SerializationError(e.to_string()))
    }

    #[cfg(not(feature = "serde"))]
    {
        Err(BsonError::NotImplemented(
            "BSON serialization requires serde feature".to_string(),
        ))
    }
}

/// Deserialize from BSON bytes
pub fn from_bson<T: for<'de> Deserialize<'de>>(_bytes: &[u8]) -> Result<T, BsonError> {
    #[cfg(feature = "serde")]
    {
        de::deserialize_from_slice(_bytes)
            .map_err(|e| BsonError::DeserializationError(e.to_string()))
    }

    #[cfg(not(feature = "serde"))]
    {
        Err(BsonError::NotImplemented(
            "BSON deserialization requires serde feature".to_string(),
        ))
    }
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
        // Read document size (4 bytes, little-endian)
        let mut size_buf = [0u8; 4];
        match self.reader.read_exact(&mut size_buf) {
            Ok(_) => {
                let size = u32::from_le_bytes(size_buf) as usize;
                if size < 4 {
                    return Err(BsonError::InvalidData(
                        "Invalid BSON document size".to_string(),
                    ));
                }

                // Read entire document
                let mut doc_buf = vec![0u8; size];
                doc_buf[0..4].copy_from_slice(&size_buf);
                self.reader
                    .read_exact(&mut doc_buf[4..])
                    .map_err(|e| BsonError::DeserializationError(e.to_string()))?;

                // Deserialize document
                #[cfg(feature = "serde")]
                let value = de::deserialize_from_slice(&doc_buf)
                    .map_err(|e| BsonError::DeserializationError(e.to_string()))?;

                #[cfg(not(feature = "serde"))]
                return Err(BsonError::NotImplemented(
                    "BSON deserialization requires serde feature".to_string(),
                ));

                self.document_count += 1;
                Ok(Some(value))
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(BsonError::DeserializationError(e.to_string())),
        }
    }

    pub fn document_count(&self) -> u32 {
        self.document_count
    }
}

/// Convert any serializable value to BSON Document
#[cfg(feature = "serde")]
pub fn to_bson_document<T: Serialize>(value: &T) -> Result<Document, BsonError> {
    let bson_value =
        ser::serialize_to_bson(value).map_err(|e| BsonError::SerializationError(e.to_string()))?;
    match bson_value {
        Bson::Document(doc) => Ok(doc),
        _ => Err(BsonError::SerializationError(
            "Value is not a document".to_string(),
        )),
    }
}

#[cfg(not(feature = "serde"))]
pub fn to_bson_document<T: Serialize>(_value: &T) -> Result<bson::Document, BsonError> {
    Err(BsonError::NotImplemented(
        "BSON serialization requires serde feature".to_string(),
    ))
}

/// Serialize BSON document to binary
#[cfg(feature = "serde")]
pub fn serialize_bson(doc: &Document) -> Result<Vec<u8>, BsonError> {
    let mut buf = Vec::new();
    doc.to_writer(&mut buf)
        .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    Ok(buf)
}

#[cfg(not(feature = "serde"))]
pub fn serialize_bson(_doc: &bson::Document) -> Result<Vec<u8>, BsonError> {
    Err(BsonError::NotImplemented(
        "BSON serialization requires serde feature".to_string(),
    ))
}

/// Deserialize BSON document from binary
#[cfg(feature = "serde")]
pub fn deserialize_bson(bytes: &[u8]) -> Result<Document, BsonError> {
    Document::from_reader(bytes).map_err(|e| BsonError::DeserializationError(e.to_string()))
}

#[cfg(not(feature = "serde"))]
pub fn deserialize_bson(_bytes: &[u8]) -> Result<bson::Document, BsonError> {
    Err(BsonError::NotImplemented(
        "BSON deserialization requires serde feature".to_string(),
    ))
}

/// Merge two BSON documents (shallow)
#[cfg(feature = "serde")]
pub fn merge_bson_docs(doc1: &Document, doc2: &Document) -> Document {
    let mut merged = doc1.clone();
    for (k, v) in doc2.iter() {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

#[cfg(not(feature = "serde"))]
pub fn merge_bson_docs(_doc1: &bson::Document, _doc2: &bson::Document) -> bson::Document {
    bson::Document::new()
}

/// Deep merge two BSON documents (recursive)
#[cfg(feature = "serde")]
pub fn deep_merge_bson_docs(doc1: &Document, doc2: &Document) -> Document {
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

#[cfg(not(feature = "serde"))]
pub fn deep_merge_bson_docs(_doc1: &bson::Document, _doc2: &bson::Document) -> bson::Document {
    bson::Document::new()
}

/// Compute diff between two BSON documents (returns changed fields)
#[cfg(feature = "serde")]
pub fn diff_bson_docs(doc1: &Document, doc2: &Document) -> Vec<String> {
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

#[cfg(not(feature = "serde"))]
pub fn diff_bson_docs(_doc1: &bson::Document, _doc2: &bson::Document) -> Vec<String> {
    Vec::new()
}

/// Extract a field from BSON document
#[cfg(feature = "serde")]
pub fn extract_bson_field(doc: &Document, field: &str) -> Option<Bson> {
    doc.get(field).cloned()
}

#[cfg(not(feature = "serde"))]
pub fn extract_bson_field(_doc: &bson::Document, _field: &str) -> Option<Bson> {
    None
}

/// Extract nested field from BSON document (dot notation)
#[cfg(feature = "serde")]
pub fn extract_nested_bson_field(doc: &Document, path: &str) -> Option<Bson> {
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

#[cfg(not(feature = "serde"))]
pub fn extract_nested_bson_field(_doc: &bson::Document, _path: &str) -> Option<Bson> {
    None
}

/// Remove a field from BSON document
#[cfg(feature = "serde")]
pub fn remove_bson_field(doc: &mut Document, field: &str) -> bool {
    doc.remove(field).is_some()
}

#[cfg(not(feature = "serde"))]
pub fn remove_bson_field(_doc: &mut bson::Document, _field: &str) -> bool {
    false
}

/// Rename a field in BSON document
#[cfg(feature = "serde")]
pub fn rename_bson_field(doc: &mut Document, old: &str, new: &str) -> bool {
    if let Some(val) = doc.remove(old) {
        doc.insert(new.to_string(), val);
        true
    } else {
        false
    }
}

#[cfg(not(feature = "serde"))]
pub fn rename_bson_field(_doc: &mut bson::Document, _old: &str, _new: &str) -> bool {
    false
}

/// Filter BSON document by allowed fields
#[cfg(feature = "serde")]
pub fn filter_bson_fields(doc: &Document, allowed: &[&str]) -> Document {
    let mut filtered = Document::new();
    for &field in allowed {
        if let Some(val) = doc.get(field) {
            filtered.insert(field.to_string(), val.clone());
        }
    }
    filtered
}

#[cfg(not(feature = "serde"))]
pub fn filter_bson_fields(_doc: &bson::Document, _allowed: &[&str]) -> bson::Document {
    bson::Document::new()
}

/// Validate BSON bytes
#[cfg(feature = "serde")]
pub fn validate_bson(bytes: &[u8]) -> Result<(), BsonError> {
    Document::from_reader(bytes).map_err(|e| BsonError::DeserializationError(e.to_string()))?;
    Ok(())
}

#[cfg(not(feature = "serde"))]
pub fn validate_bson(_bytes: &[u8]) -> Result<(), BsonError> {
    Err(BsonError::NotImplemented(
        "BSON validation requires serde feature".to_string(),
    ))
}

/// Serialize array of BSON documents
#[cfg(feature = "serde")]
pub fn bson_array_to_bytes(docs: &[Document]) -> Result<Vec<u8>, BsonError> {
    let mut buf = Vec::new();
    for doc in docs {
        doc.to_writer(&mut buf)
            .map_err(|e| BsonError::SerializationError(e.to_string()))?;
    }
    Ok(buf)
}

#[cfg(not(feature = "serde"))]
pub fn bson_array_to_bytes(_docs: &[bson::Document]) -> Result<Vec<u8>, BsonError> {
    Err(BsonError::NotImplemented(
        "BSON serialization requires serde feature".to_string(),
    ))
}

/// Pretty-print BSON document
#[cfg(feature = "serde")]
pub fn pretty_print_bson(doc: &Document) -> String {
    let bson = Bson::Document(doc.clone());
    format!("{:#?}", bson)
}

#[cfg(not(feature = "serde"))]
pub fn pretty_print_bson(_doc: &bson::Document) -> String {
    String::new()
}

/// Convert BSON document to JSON value
#[cfg(feature = "serde")]
pub fn bson_to_json(doc: &Document) -> serde_json::Value {
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

#[cfg(not(feature = "serde"))]
pub fn bson_to_json(_doc: &bson::Document) -> serde_json::Value {
    serde_json::Value::Null
}

/// Convert BSON value to JSON value
#[cfg(feature = "serde")]
pub fn bson_to_json_value(bson: &Bson) -> serde_json::Value {
    match bson {
        Bson::Double(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        Bson::String(s) => serde_json::Value::String(s.clone()),
        Bson::Array(arr) => serde_json::Value::Array(arr.iter().map(bson_to_json_value).collect()),
        Bson::Document(doc) => bson_to_json(doc),
        Bson::Boolean(b) => serde_json::Value::Bool(*b),
        Bson::Int32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Bson::Int64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Bson::Null => serde_json::Value::Null,
        _ => serde_json::Value::Null,
    }
}

#[cfg(not(feature = "serde"))]
pub fn bson_to_json_value(_bson: &Bson) -> serde_json::Value {
    serde_json::Value::Null
}

/// Stream BSON array deserialization
#[cfg(feature = "serde")]
pub fn stream_bson_array<R: std::io::Read>(
    reader: R,
) -> impl Iterator<Item = Result<Document, BsonError>> {
    let mut r = reader;
    std::iter::from_fn(move || match Document::from_reader(&mut r) {
        Ok(doc) => Some(Ok(doc)),
        Err(_) => None,
    })
}

#[cfg(not(feature = "serde"))]
pub fn stream_bson_array<R: std::io::Read>(
    _reader: R,
) -> impl Iterator<Item = Result<bson::Document, BsonError>> {
    std::iter::empty()
}

/// Deserialize array of BSON documents
#[cfg(feature = "serde")]
pub fn bytes_to_bson_array(bytes: &[u8]) -> Result<Vec<Document>, BsonError> {
    let mut docs = Vec::new();
    let mut slice = bytes;
    while !slice.is_empty() {
        // Read document size
        if slice.len() < 4 {
            break;
        }
        let size = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as usize;
        if slice.len() < size {
            break;
        }

        // Parse document
        match Document::from_reader(&slice[0..size]) {
            Ok(doc) => {
                docs.push(doc);
                slice = &slice[size..];
            }
            Err(_) => break,
        }
    }
    Ok(docs)
}

#[cfg(not(feature = "serde"))]
pub fn bytes_to_bson_array(_bytes: &[u8]) -> Result<Vec<bson::Document>, BsonError> {
    Err(BsonError::NotImplemented(
        "BSON deserialization requires serde feature".to_string(),
    ))
}

/// Read metadata field from BSON document
#[cfg(feature = "serde")]
pub fn get_bson_metadata(doc: &Document) -> Option<BsonMetadata> {
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

#[cfg(not(feature = "serde"))]
pub fn get_bson_metadata(_doc: &bson::Document) -> Option<BsonMetadata> {
    None
}

/// Write metadata field to BSON document
#[cfg(feature = "serde")]
pub fn set_bson_metadata(doc: &mut Document, meta: &BsonMetadata) {
    let mut meta_doc = Document::new();
    meta_doc.insert("version", meta.version as i32);
    meta_doc.insert("document_count", meta.document_count as i32);
    // Add compression if needed
    doc.insert("_meta", Bson::Document(meta_doc));
}

#[cfg(not(feature = "serde"))]
pub fn set_bson_metadata(_doc: &mut bson::Document, _meta: &BsonMetadata) {
    // Do nothing when serde is not enabled
}

/// Inspect the type of a field in a BSON document
#[cfg(feature = "serde")]
pub fn bson_field_type(doc: &Document, field: &str) -> Option<&'static str> {
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

#[cfg(not(feature = "serde"))]
pub fn bson_field_type(_doc: &bson::Document, _field: &str) -> Option<&'static str> {
    None
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

    #[test]
    fn test_bson_serialization() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            name: String,
            age: u32,
        }

        let test = TestStruct {
            name: "Test".to_string(),
            age: 42,
        };

        let bson_bytes = to_bson(&test).unwrap();
        let deserialized: TestStruct = from_bson(&bson_bytes).unwrap();
        assert_eq!(test, deserialized);
    }

    // BsonType tests removed: BsonType is not defined in bson crate
}
