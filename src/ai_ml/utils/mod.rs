//! AI/ML Utils Module
//!
//! This module provides utility functions for AI/ML integration, including data conversion,
//! model execution, and other helper functions.

use std::collections::HashMap;
use std::path::Path;

use crate::ai_ml::models::{
    AiModel, AiModelManager, FeatureRecognitionModel, MeshGenerationModel, ModelRepairModel,
};
use crate::ai_ml::protocol::{
    AiDataType, AiProtocol, AiRequest, AiResponse, AiResult, DefaultAiProtocol,
};
use crate::geometry::{Plane, Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};
use crate::topology::topods_shape::TopoDsShape;

/// ML Dataset
pub struct MlDataset {
    pub name: String,
    pub samples: Vec<(Mesh3D, Vec<String>)>,
    pub metadata: HashMap<String, String>,
}

impl MlDataset {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            samples: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_sample(&mut self, mesh: Mesh3D, features: Vec<String>) {
        self.samples.push((mesh, features));
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        // Save dataset to file
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;
        writeln!(file, "{}", self.name).map_err(|e| e.to_string())?;
        writeln!(file, "{}", self.samples.len()).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        // Load dataset from file
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);
        let mut name = String::new();
        reader.read_line(&mut name).map_err(|e| e.to_string())?;
        let mut len = String::new();
        reader.read_line(&mut len).map_err(|e| e.to_string())?;
        let _samples_len: usize = len.trim().parse().unwrap_or(0);
        Ok(Self {
            name: name.trim().to_string(),
            samples: Vec::new(),
            metadata: HashMap::new(),
        })
    }
}

/// ML Model Format
pub enum MlModelFormat {
    PyTorch,
    TensorFlow,
    ONNX,
    Custom(String),
}

/// AI/ML Utilities
pub struct AiMlUtils {
    protocol: Box<dyn AiProtocol>,
    model_manager: AiModelManager,
}

impl AiMlUtils {
    pub fn new() -> Self {
        let protocol = Box::new(DefaultAiProtocol::new("http://localhost:8000"));
        let model_manager = AiModelManager::new(protocol.clone());

        // Register default models
        let mut manager = model_manager;
        manager.register_model(
            "feature_recognition",
            Box::new(FeatureRecognitionModel::new()),
        );
        manager.register_model("mesh_generation", Box::new(MeshGenerationModel::new()));
        manager.register_model("model_repair", Box::new(ModelRepairModel::new()));

        Self {
            protocol,
            model_manager: manager,
        }
    }

    /// Export geometric data to AI format
    pub fn export_to_ai(&self, data: &AiDataType) -> AiResult<serde_json::Value> {
        self.protocol.to_ai_format(data)
    }

    /// Import geometric data from AI format
    pub fn import_from_ai(&self, data: &serde_json::Value) -> AiResult<AiDataType> {
        self.protocol.from_ai_format(data)
    }

    /// Convert point to tensor
    pub fn point_to_tensor(&self, point: &Point) -> Vec<f32> {
        vec![point.x as f32, point.y as f32, point.z as f32]
    }

    /// Convert vector to tensor
    pub fn vector_to_tensor(&self, vector: &Vector) -> Vec<f32> {
        vec![vector.x as f32, vector.y as f32, vector.z as f32]
    }

    /// Convert plane to tensor
    pub fn plane_to_tensor(&self, plane: &Plane) -> Vec<f32> {
        let origin = self.point_to_tensor(plane.origin());
        let normal = self.vector_to_tensor(&plane.normal().to_vec());
        [origin, normal].concat()
    }

    /// Convert mesh to tensor
    pub fn mesh_to_tensor(&self, mesh: &Mesh3D) -> Vec<f32> {
        let mut tensor = Vec::new();

        // Add vertices
        for vertex in &mesh.vertices {
            tensor.extend(self.point_to_tensor(&vertex.point));
            if let Some(normal) = vertex.normal {
                tensor.extend(vec![normal[0] as f32, normal[1] as f32, normal[2] as f32]);
            } else {
                tensor.extend(vec![0.0, 0.0, 0.0]);
            }
        }

        // Add faces
        for face in &mesh.faces {
            for &vertex_id in &face.vertices {
                tensor.push(vertex_id as f32);
            }
        }

        tensor
    }

    /// Convert shape to tensor
    pub fn shape_to_tensor(&self, _shape: &TopoDsShape) -> Vec<f32> {
        // Convert shape to tensor by extracting points from its geometry
        // Note: TopoDsShape doesn't have a points() method, this is a placeholder
        let mut tensor = Vec::new();
        // Add some default points for testing
        tensor.extend(self.point_to_tensor(&Point::origin()));
        tensor
    }

    /// Convert tensor to point
    pub fn tensor_to_point(&self, tensor: &[f32]) -> Result<Point, String> {
        if tensor.len() < 3 {
            return Err("Tensor must have at least 3 elements for point".to_string());
        }
        Ok(Point::new(
            tensor[0] as f64,
            tensor[1] as f64,
            tensor[2] as f64,
        ))
    }

    /// Convert tensor to vector
    pub fn tensor_to_vector(&self, tensor: &[f32]) -> Result<Vector, String> {
        if tensor.len() < 3 {
            return Err("Tensor must have at least 3 elements for vector".to_string());
        }
        Ok(Vector::new(
            tensor[0] as f64,
            tensor[1] as f64,
            tensor[2] as f64,
        ))
    }

    /// Convert tensor to mesh
    pub fn tensor_to_mesh(&self, tensor: &[f32]) -> Result<Mesh3D, String> {
        // Convert tensor to mesh: expects [x, y, z, nx, ny, nz, ...] for each vertex, then face indices
        if tensor.len() < 6 {
            return Err("Tensor too short for mesh".to_string());
        }
        let mut vertices = Vec::new();
        let mut i = 0;
        while i + 5 < tensor.len() {
            let point = Point::new(tensor[i] as f64, tensor[i + 1] as f64, tensor[i + 2] as f64);
            let normal = Some([
                tensor[i + 3] as f64,
                tensor[i + 4] as f64,
                tensor[i + 5] as f64,
            ]);
            vertices.push(MeshVertex {
                point,
                normal,
                ..Default::default()
            });
            i += 6;
        }
        // Faces: assume remaining tensor values are indices
        let mut faces = Vec::new();
        while i + 2 < tensor.len() {
            let v0 = tensor[i] as usize;
            let v1 = tensor[i + 1] as usize;
            let v2 = tensor[i + 2] as usize;
            faces.push(MeshFace::new(faces.len(), vec![v0, v1, v2]));
            i += 3;
        }
        Ok(Mesh3D {
            vertices,
            faces,
            ..Default::default()
        })
    }

    /// Generate mesh from text description
    pub fn generate_mesh(&self, description: &str) -> AiResult<Mesh3D> {
        let input = AiDataType::Text(description.to_string());
        let result = self
            .model_manager
            .execute_model("mesh_generation", &input)?;

        match result {
            AiDataType::Mesh(mesh) => Ok(mesh),
            _ => Err(crate::ai_ml::protocol::AiProtocolError::ConversionError(
                "Expected mesh output".to_string(),
            )),
        }
    }

    /// Recognize features in mesh
    pub fn recognize_features(&self, mesh: &Mesh3D) -> AiResult<Vec<String>> {
        let input = AiDataType::Mesh(mesh.clone());
        let result = self
            .model_manager
            .execute_model("feature_recognition", &input)?;

        match result {
            AiDataType::Array(features) => {
                let feature_strings: Vec<String> = features
                    .into_iter()
                    .filter_map(|f| match f {
                        AiDataType::Text(s) => Some(s),
                        _ => None,
                    })
                    .collect();
                Ok(feature_strings)
            }
            _ => Err(crate::ai_ml::protocol::AiProtocolError::ConversionError(
                "Expected array of features".to_string(),
            )),
        }
    }

    /// Repair mesh
    pub fn repair_mesh(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        let input = AiDataType::Mesh(mesh.clone());
        let result = self.model_manager.execute_model("model_repair", &input)?;

        match result {
            AiDataType::Mesh(repaired_mesh) => Ok(repaired_mesh),
            _ => Err(crate::ai_ml::protocol::AiProtocolError::ConversionError(
                "Expected mesh output".to_string(),
            )),
        }
    }

    /// Send custom request to AI
    pub fn send_request(&self, request: &AiRequest) -> AiResult<AiResponse> {
        self.protocol.send_request(request)
    }

    /// Get model manager
    pub fn model_manager(&self) -> &AiModelManager {
        &self.model_manager
    }

    /// Get protocol
    pub fn protocol(&self) -> &dyn AiProtocol {
        &*self.protocol
    }

    /// Extract features from mesh
    pub fn extract_features(&self, mesh: &Mesh3D) -> Vec<f32> {
        // Extract features from mesh
        let mut features = Vec::new();

        // Add vertex count
        features.push(mesh.vertices.len() as f32);

        // Add face count
        features.push(mesh.faces.len() as f32);

        // Add bounding box size
        let min_x = mesh
            .vertices
            .iter()
            .map(|v| v.point.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let max_x = mesh
            .vertices
            .iter()
            .map(|v| v.point.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let min_y = mesh
            .vertices
            .iter()
            .map(|v| v.point.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let max_y = mesh
            .vertices
            .iter()
            .map(|v| v.point.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let min_z = mesh
            .vertices
            .iter()
            .map(|v| v.point.z)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let max_z = mesh
            .vertices
            .iter()
            .map(|v| v.point.z)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        features.push((max_x - min_x) as f32);
        features.push((max_y - min_y) as f32);
        features.push((max_z - min_z) as f32);

        features
    }

    /// Save model to file
    pub fn save_model(&self, model_name: &str, path: &Path, format: MlModelFormat) -> AiResult<()> {
        let model = self.model_manager.get_model(model_name).ok_or(
            crate::ai_ml::protocol::AiProtocolError::ModelError(format!(
                "Model not found: {}",
                model_name
            )),
        )?;
        model.save(path)
    }

    /// Load model from file
    pub fn load_model(
        &mut self,
        model_name: &str,
        path: &Path,
        format: MlModelFormat,
    ) -> AiResult<()> {
        // Load model based on format
        let model = match format {
            MlModelFormat::PyTorch => {
                // Load PyTorch model
                FeatureRecognitionModel::load(path)
            }
            MlModelFormat::TensorFlow => {
                // Load TensorFlow model
                FeatureRecognitionModel::load(path)
            }
            MlModelFormat::ONNX => {
                // Load ONNX model
                FeatureRecognitionModel::load(path)
            }
            MlModelFormat::Custom(_) => {
                // Load custom model
                FeatureRecognitionModel::load(path)
            }
        }?;

        self.model_manager.register_model(model_name, model);
        Ok(())
    }

    /// Create dataset from meshes
    pub fn create_dataset(
        &self,
        name: &str,
        meshes: &[Mesh3D],
        labels: &[Vec<String>],
    ) -> Result<MlDataset, String> {
        if meshes.len() != labels.len() {
            return Err("Meshes and labels must have the same length".to_string());
        }

        let mut dataset = MlDataset::new(name);
        for (mesh, label) in meshes.iter().zip(labels.iter()) {
            dataset.add_sample(mesh.clone(), label.clone());
        }

        Ok(dataset)
    }

    /// Train model with dataset
    pub fn train_model(&mut self, model_name: &str, dataset: &MlDataset) -> AiResult<()> {
        // Get the model
        let model = self.model_manager.get_model(model_name).ok_or(
            crate::ai_ml::protocol::AiProtocolError::ModelError(format!(
                "Model not found: {}",
                model_name
            )),
        )?;

        // Convert dataset to training data
        let training_data: Vec<(Mesh3D, Vec<String>)> = dataset.samples.clone();

        // Train the model if it's a FeatureRecognitionModel
        if let Some(feature_model) = AiModel::as_any(model).downcast_ref::<FeatureRecognitionModel>() {
            // Create a mutable copy for training
            let mut mutable_model = (*feature_model).clone();
            mutable_model.train(&training_data)?;

            // Register the trained model
            self.model_manager
                .register_model(model_name, Box::new(mutable_model));
        }

        Ok(())
    }
}

use std::any::Any;

/// Extension trait for AiModel to support downcasting
pub trait AiModelExt: AiModel {
    fn as_any(&self) -> &dyn Any;
}

impl<T: AiModel + 'static> AiModelExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AiModel for Box<dyn AiModel> {
    fn name(&self) -> &str {
        (**self).name()
    }

    fn description(&self) -> &str {
        (**self).description()
    }

    fn execute(&self, input: &AiDataType, protocol: &dyn AiProtocol) -> AiResult<AiDataType> {
        (**self).execute(input, protocol)
    }

    fn save(&self, path: &Path) -> AiResult<()> {
        (**self).save(path)
    }

    fn load(path: &Path) -> AiResult<Box<dyn AiModel>> {
        // This is a placeholder, actual implementation depends on the model type
        FeatureRecognitionModel::load(path)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }
}
