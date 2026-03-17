//! PyTorch Integration
//! 
//! This module provides utilities for integrating with PyTorch, including tensor conversion
//! between geometric data and PyTorch tensors, with optimized performance and GPU acceleration.

#[cfg(feature = "pytorch")]
use tch;

/// PyTorch Model Wrapper
#[cfg(feature = "pytorch")]
pub struct PyTorchModel {
    model: tch::CModule,
    device: tch::Device,
}

#[cfg(feature = "pytorch")]
impl PyTorchModel {
    /// Load PyTorch model from file
    pub fn load_from_file(path: &str, device: tch::Device) -> Result<Self, String> {
        let model = tch::CModule::load_on_device(path, device)
            .map_err(|e| format!("Failed to load model: {}", e))?;
        Ok(Self { model, device })
    }

    /// Execute model with input tensor
    pub fn execute(&self, input: &tch::Tensor) -> Result<tch::Tensor, String> {
        let output = self
            .model
            .forward_is_ok(&[input])
            .map_err(|e| format!("Model execution failed: {}", e))?;
        Ok(output)
    }

    /// Get device
    pub fn device(&self) -> &tch::Device {
        &self.device
    }
}

/// PyTorch Model Cache
#[cfg(feature = "pytorch")]
pub struct PyTorchModelCache {
    models: HashMap<String, Arc<PyTorchModel>>,
}

#[cfg(feature = "pytorch")]
impl PyTorchModelCache {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    /// Get or load model
    pub fn get_or_load(
        &mut self,
        path: &str,
        device: tch::Device,
    ) -> Result<Arc<PyTorchModel>, String> {
        if let Some(model) = self.models.get(path) {
            return Ok(model.clone());
        }

        let model = Arc::new(PyTorchModel::load_from_file(path, device)?);
        self.models.insert(path.to_string(), model.clone());
        Ok(model)
    }
}

/// Convert point to PyTorch tensor
#[cfg(feature = "pytorch")]
pub fn point_to_tensor(point: &Point) -> tch::Tensor {
    tch::Tensor::of_slice(&[point.x as f32, point.y as f32, point.z as f32])
}

/// Convert vector to PyTorch tensor
#[cfg(feature = "pytorch")]
pub fn vector_to_tensor(vector: &Vector) -> tch::Tensor {
    tch::Tensor::of_slice(&[vector.x as f32, vector.y as f32, vector.z as f32])
}

/// Convert mesh to PyTorch tensor (optimized)
#[cfg(feature = "pytorch")]
pub fn mesh_to_tensor(mesh: &Mesh3D) -> tch::Tensor {
    // Pre-allocate exact size to avoid reallocations
    let mut data = Vec::with_capacity(mesh.vertices.len() * 6);

    // Batch process vertices
    for vertex in &mesh.vertices {
        data.extend(&[
            vertex.point.x as f32,
            vertex.point.y as f32,
            vertex.point.z as f32,
        ]);
        if let Some(normal) = vertex.normal {
            data.extend(&[normal[0] as f32, normal[1] as f32, normal[2] as f32]);
        } else {
            data.extend(&[0.0, 0.0, 0.0]);
        }
    }

    tch::Tensor::of_slice(&data)
}

/// Convert batch of points to PyTorch tensor (optimized)
#[cfg(feature = "gpu")]
pub fn points_to_tensor(points: &[Point]) -> tch::Tensor {
    let mut data = Vec::with_capacity(points.len() * 3);
    for point in points {
        data.extend(&[point.x as f32, point.y as f32, point.z as f32]);
    }
    tch::Tensor::of_slice(&data).reshape(&[points.len() as i64, 3])
}

/// Convert PyTorch tensor to point
#[cfg(feature = "gpu")]
pub fn tensor_to_point(tensor: &tch::Tensor) -> Result<Point, String> {
    let data: Vec<f32> = tensor.to_vec();
    if data.len() < 3 {
        return Err("Tensor must have at least 3 elements for point".to_string());
    }
    Ok(Point::new(data[0] as f64, data[1] as f64, data[2] as f64))
}

/// Convert PyTorch tensor to mesh
#[cfg(feature = "gpu")]
pub fn tensor_to_mesh(tensor: &tch::Tensor) -> Result<Mesh3D, String> {
    let data: Vec<f32> = tensor.to_vec();
    if data.len() < 6 {
        return Err("Tensor must have at least 6 elements for mesh".to_string());
    }

    let mut vertices = Vec::new();
    let mut i = 0;
    while i + 5 < data.len() {
        let point = Point::new(data[i] as f64, data[i + 1] as f64, data[i + 2] as f64);

        vertices.push(MeshVertex::new(vertices.len(), point));
        i += 6;
    }

    // Create simple faces
    let mut faces = Vec::new();
    for j in 0..vertices.len() / 3 {
        let v0 = j * 3;
        let v1 = j * 3 + 1;
        let v2 = j * 3 + 2;
        if v2 < vertices.len() {
            faces.push(MeshFace::new(faces.len(), vec![v0, v1, v2]));
        }
    }

    let mut mesh = Mesh3D::new();
    mesh.vertices = vertices;
    mesh.faces = faces;
    Ok(mesh)
}

/// Move tensor to device (GPU if available)
#[cfg(feature = "gpu")]
pub fn move_to_device(tensor: &tch::Tensor) -> tch::Tensor {
    let device = if tch::Cuda::is_available() {
        tch::Device::Cuda(0)
    } else {
        tch::Device::Cpu
    };
    tensor.to_device(device)
}
