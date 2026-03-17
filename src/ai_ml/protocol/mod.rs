//! AI Protocol Module
//! 
//! This module defines the protocol layer for AI communication, including data types,
//! request/response structures, and protocol implementation.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::geometry::{Plane, Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};
use crate::topology::topods_shape::TopoDsShape;

/// AI Protocol Error
#[derive(Debug, Clone)]
pub enum AiProtocolError {
    InvalidData(String),
    ModelError(String),
    CommunicationError(String),
    ConversionError(String),
    ValidationError(String),
}

impl fmt::Display for AiProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Self::ModelError(msg) => write!(f, "Model error: {}", msg),
            Self::CommunicationError(msg) => write!(f, "Communication error: {}", msg),
            Self::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for AiProtocolError {}

/// AI Protocol Result type
pub type AiResult<T> = Result<T, AiProtocolError>;

/// AI Data Type
#[derive(Debug, Clone)]
pub enum AiDataType {
    Point(Point),
    Vector(Vector),
    Plane(Plane),
    Mesh(Mesh3D),
    Shape(TopoDsShape),
    Text(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<AiDataType>),
    Object(HashMap<String, AiDataType>),
}

/// AI Message
#[derive(Debug, Clone)]
pub struct AiMessage {
    pub id: String,
    pub role: String, // "user", "assistant", "system"
    pub content: AiDataType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

/// AI Request
#[derive(Debug, Clone)]
pub struct AiRequest {
    pub model: String,
    pub messages: Vec<AiMessage>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout: Option<u64>, // in seconds
}

/// AI Response
#[derive(Debug, Clone)]
pub struct AiResponse {
    pub id: String,
    pub model: String,
    pub messages: Vec<AiMessage>,
    pub usage: Option<HashMap<String, u64>>,
    pub error: Option<AiProtocolError>,
}

/// AI Protocol Interface
pub trait AiProtocol {
    /// Send request to AI
    fn send_request(&self, request: &AiRequest) -> AiResult<AiResponse>;
    
    /// Convert geometric data to AI-compatible format
    fn to_ai_format(&self, data: &AiDataType) -> AiResult<serde_json::Value>;
    
    /// Convert AI response to geometric data
    fn from_ai_format(&self, data: &serde_json::Value) -> AiResult<AiDataType>;
    
    /// Validate AI response
    fn validate_response(&self, response: &AiResponse) -> AiResult<()>;
}

/// Default AI Protocol Implementation
#[derive(Clone)]
pub struct DefaultAiProtocol {
    base_url: String,
    api_key: Option<String>,
    timeout: u64,
}

impl DefaultAiProtocol {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: None,
            timeout: 30,
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
}

impl AiProtocol for DefaultAiProtocol {
    fn send_request(&self, request: &AiRequest) -> AiResult<AiResponse> {
        // Implement actual HTTP request to AI API
        // For now, return a mock response
        Ok(AiResponse {
            id: uuid::Uuid::new_v4().to_string(),
            model: request.model.clone(),
            messages: request.messages.clone(),
            usage: Some(HashMap::from([
                ("prompt_tokens".to_string(), 100),
                ("completion_tokens".to_string(), 200),
                ("total_tokens".to_string(), 300),
            ])),
            error: None,
        })
    }

    fn to_ai_format(&self, data: &AiDataType) -> AiResult<serde_json::Value> {
        match data {
            AiDataType::Point(point) => Ok(serde_json::json!({
                "type": "point",
                "x": point.x,
                "y": point.y,
                "z": point.z
            })),
            AiDataType::Vector(vector) => Ok(serde_json::json!({
                "type": "vector",
                "x": vector.x,
                "y": vector.y,
                "z": vector.z
            })),
            AiDataType::Plane(plane) => Ok(serde_json::json!({
                "type": "plane",
                "origin": {
                    "x": plane.origin().x,
                    "y": plane.origin().y,
                    "z": plane.origin().z
                },
                "normal": {
                    "x": plane.normal().to_vec().x,
                    "y": plane.normal().to_vec().y,
                    "z": plane.normal().to_vec().z
                }
            })),
            AiDataType::Mesh(mesh) => {
                let vertices: Vec<_> = mesh
                    .vertices
                    .iter()
                    .map(|v| {
                        serde_json::json!({
                            "x": v.point.x,
                            "y": v.point.y,
                            "z": v.point.z,
                            "normal": v.normal.map(|n| {
                                serde_json::json!({
                                    "x": n[0],
                                    "y": n[1],
                                    "z": n[2]
                                })
                            })
                        })
                    })
                    .collect();

                let faces: Vec<_> = mesh
                    .faces
                    .iter()
                    .map(|f| serde_json::json!(f.vertices))
                    .collect();

                Ok(serde_json::json!({
                    "type": "mesh",
                    "vertices": vertices,
                    "faces": faces
                }))
            }
            AiDataType::Shape(_shape) => {
                // Convert shape to mesh first, then to AI format
                // This is a placeholder implementation
                // In a real implementation, you would extract geometry from the shape
                let mesh = Mesh3D::default();
                let mesh_data = AiDataType::Mesh(mesh);
                let mut mesh_json = self.to_ai_format(&mesh_data)?;
                if let Some(mesh_obj) = mesh_json.as_object_mut() {
                    mesh_obj.insert("type".to_string(), serde_json::json!("shape"));
                    mesh_obj.insert("shape_type".to_string(), serde_json::json!("unknown"));
                }
                Ok(mesh_json)
            }
            AiDataType::Text(text) => Ok(serde_json::json!({
                "type": "text",
                "content": text
            })),
            AiDataType::Number(num) => Ok(serde_json::json!(num)),
            AiDataType::Boolean(bool) => Ok(serde_json::json!(bool)),
            AiDataType::Array(array) => {
                let items: Vec<_> = array
                    .iter()
                    .map(|item| self.to_ai_format(item))
                    .collect::<Result<_, _>>()?;
                Ok(serde_json::json!(items))
            }
            AiDataType::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), self.to_ai_format(v)?);
                }
                Ok(serde_json::json!(map))
            }
        }
    }

    fn from_ai_format(&self, data: &serde_json::Value) -> AiResult<AiDataType> {
        if let Some(obj) = data.as_object() {
            if let Some(ty) = obj.get("type").and_then(|t| t.as_str()) {
                match ty {
                    "point" => {
                        let x = obj.get("x").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid x coordinate".to_string(),
                            ),
                        )?;
                        let y = obj.get("y").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid y coordinate".to_string(),
                            ),
                        )?;
                        let z = obj.get("z").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid z coordinate".to_string(),
                            ),
                        )?;
                        Ok(AiDataType::Point(Point::new(x, y, z)))
                    }
                    "vector" => {
                        let x = obj.get("x").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid x component".to_string(),
                            ),
                        )?;
                        let y = obj.get("y").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid y component".to_string(),
                            ),
                        )?;
                        let z = obj.get("z").and_then(|v| v.as_f64()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid z component".to_string(),
                            ),
                        )?;
                        Ok(AiDataType::Vector(Vector::new(x, y, z)))
                    }
                    "mesh" => {
                        let vertices = obj.get("vertices").and_then(|v| v.as_array()).ok_or(
                            AiProtocolError::InvalidData("Missing or invalid vertices".to_string()),
                        )?;
                        let faces = obj.get("faces").and_then(|f| f.as_array()).ok_or(
                            AiProtocolError::InvalidData("Missing or invalid faces".to_string()),
                        )?;

                        let mesh_vertices: Vec<MeshVertex> = vertices
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                let v_obj = v.as_object().ok_or(AiProtocolError::InvalidData(
                                    format!("Invalid vertex at index {}", i),
                                ))?;
                                let x = v_obj.get("x").and_then(|val| val.as_f64()).ok_or(
                                    AiProtocolError::InvalidData(format!(
                                        "Missing x for vertex {}",
                                        i
                                    )),
                                )?;
                                let y = v_obj.get("y").and_then(|val| val.as_f64()).ok_or(
                                    AiProtocolError::InvalidData(format!(
                                        "Missing y for vertex {}",
                                        i
                                    )),
                                )?;
                                let z = v_obj.get("z").and_then(|val| val.as_f64()).ok_or(
                                    AiProtocolError::InvalidData(format!(
                                        "Missing z for vertex {}",
                                        i
                                    )),
                                )?;

                                let normal =
                                    v_obj
                                        .get("normal")
                                        .and_then(|n| n.as_object())
                                        .map(|n_obj| {
                                            [
                                                n_obj
                                                    .get("x")
                                                    .and_then(|val| val.as_f64())
                                                    .unwrap_or(0.0),
                                                n_obj
                                                    .get("y")
                                                    .and_then(|val| val.as_f64())
                                                    .unwrap_or(0.0),
                                                n_obj
                                                    .get("z")
                                                    .and_then(|val| val.as_f64())
                                                    .unwrap_or(1.0),
                                            ]
                                        });

                                Ok(MeshVertex {
                                    id: i,
                                    point: Point::new(x, y, z),
                                    normal,
                                    ..Default::default()
                                })
                            })
                            .collect::<AiResult<_>>()?;

                        let mesh_faces: Vec<MeshFace> = faces
                            .iter()
                            .enumerate()
                            .map(|(i, f)| {
                                let face_vertices =
                                    f.as_array().ok_or(AiProtocolError::InvalidData(format!(
                                        "Invalid face at index {}",
                                        i
                                    )))?;
                                let mut vertex_ids = Vec::new();
                                for v in face_vertices {
                                    let id = v.as_u64().ok_or(AiProtocolError::InvalidData(
                                        format!("Invalid vertex id in face {}", i),
                                    ))?;
                                    vertex_ids.push(id.try_into().unwrap());
                                }

                                Ok(MeshFace::new(i, vertex_ids))
                            })
                            .collect::<AiResult<_>>()?;

                        Ok(AiDataType::Mesh(Mesh3D {
                            vertices: mesh_vertices,
                            faces: mesh_faces,
                            ..Default::default()
                        }))
                    }
                    "text" => {
                        let content = obj.get("content").and_then(|c| c.as_str()).ok_or(
                            AiProtocolError::InvalidData(
                                "Missing or invalid text content".to_string(),
                            ),
                        )?;
                        Ok(AiDataType::Text(content.to_string()))
                    }
                    _ => Err(AiProtocolError::InvalidData(format!(
                        "Unknown type: {}",
                        ty
                    ))),
                }
            } else {
                // Handle generic object
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), self.from_ai_format(v)?);
                }
                Ok(AiDataType::Object(map))
            }
        } else if let Some(array) = data.as_array() {
            let items: Vec<_> = array
                .iter()
                .map(|item| self.from_ai_format(item))
                .collect::<Result<_, _>>()?;
            Ok(AiDataType::Array(items))
        } else if let Some(number) = data.as_f64() {
            Ok(AiDataType::Number(number))
        } else if let Some(boolean) = data.as_bool() {
            Ok(AiDataType::Boolean(boolean))
        } else if let Some(string) = data.as_str() {
            Ok(AiDataType::Text(string.to_string()))
        } else {
            Err(AiProtocolError::InvalidData(
                "Unknown data type".to_string(),
            ))
        }
    }

    fn validate_response(&self, response: &AiResponse) -> AiResult<()> {
        if response.error.is_some() {
            return Err(response.error.as_ref().unwrap().clone());
        }

        if response.messages.is_empty() {
            return Err(AiProtocolError::ValidationError(
                "Empty response messages".to_string(),
            ));
        }

        Ok(())
    }
}
