//! Interactive Generation Module
//!
//! This module provides functionality for multimodal prompts and interactive generation,
//! allowing users to collaborate with AI to create and refine 3D models.

use std::collections::HashMap;

use crate::ai_ml::protocol::{AiProtocolError, AiResult};
use crate::ai_ml::text_to_3d::{TextTo3DGenerator, TextTo3DSettings};
use crate::geometry::Point;
use crate::mesh::mesh_data::Mesh3D;

/// Multimodal Prompt
pub struct MultimodalPrompt {
    pub text: String,
    pub sketch_paths: Vec<String>,
    pub image_paths: Vec<String>,
    pub references: Vec<Mesh3D>,
}

impl MultimodalPrompt {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            sketch_paths: Vec::new(),
            image_paths: Vec::new(),
            references: Vec::new(),
        }
    }

    pub fn with_sketch(mut self, sketch_path: &str) -> Self {
        self.sketch_paths.push(sketch_path.to_string());
        self
    }

    pub fn with_image(mut self, image_path: &str) -> Self {
        self.image_paths.push(image_path.to_string());
        self
    }

    pub fn with_reference(mut self, reference: Mesh3D) -> Self {
        self.references.push(reference);
        self
    }
}

/// Interactive Session
pub struct InteractiveSession {
    id: String,
    current_mesh: Mesh3D,
    history: Vec<Interaction>,
    settings: TextTo3DSettings,
    generator: TextTo3DGenerator,
}

/// Interaction
pub enum Interaction {
    TextPrompt(String),
    SketchUpload(String),
    ImageUpload(String),
    ReferenceAddition(Mesh3D),
    MeshModification(String, Mesh3D),
    Undo,
    Redo,
}

impl InteractiveSession {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        
        Self {
            id: format!("session_{}", rng.random::<u64>()),
            current_mesh: Mesh3D::new(),
            history: Vec::new(),
            settings: TextTo3DSettings::default(),
            generator: TextTo3DGenerator::new(),
        }
    }

    pub fn with_settings(mut self, settings: TextTo3DSettings) -> Self {
        self.settings = settings.clone();
        self.generator = TextTo3DGenerator::new().with_settings(settings);
        self
    }

    /// Process multimodal prompt
    pub fn process_multimodal_prompt(&mut self, prompt: &MultimodalPrompt) -> AiResult<Mesh3D> {
        // Start with text generation
        let result = self.generator.generate(&prompt.text)?;
        let mut current_mesh = result.mesh;

        // Incorporate sketches if provided
        for sketch_path in &prompt.sketch_paths {
            let sketch_result = self.generator.generate_from_sketch(sketch_path)?;
            current_mesh = self.merge_meshes(&current_mesh, &sketch_result.mesh)?;
        }

        // Incorporate images if provided
        for image_path in &prompt.image_paths {
            let image_result = self.generator.generate_from_image(image_path)?;
            current_mesh = self.merge_meshes(&current_mesh, &image_result.mesh)?;
        }

        // Incorporate references if provided
        for reference in &prompt.references {
            current_mesh = self.merge_meshes(&current_mesh, reference)?;
        }

        // Update current mesh and history
        self.current_mesh = current_mesh.clone();
        self.history
            .push(Interaction::TextPrompt(prompt.text.clone()));

        Ok(current_mesh)
    }

    /// Handle user feedback
    pub fn handle_feedback(&mut self, feedback: &str) -> AiResult<Mesh3D> {
        // Process feedback and update mesh
        // Real implementation: use feedback to refine the mesh
        let refined_prompt = format!("Refine the mesh based on feedback: {}\nCurrent mesh features: {:.2} vertices, {:.2} faces", 
            feedback, 
            self.current_mesh.vertices.len(), 
            self.current_mesh.faces.len()
        );
        
        let result = self.generator.generate(&refined_prompt)?;
        
        // Use mesh morphing to smoothly transition between current and new mesh
        let updated_mesh = self.morph_meshes(&self.current_mesh, &result.mesh, 0.7)?;

        // Update current mesh and history
        self.current_mesh = updated_mesh.clone();
        self.history.push(Interaction::MeshModification(
            feedback.to_string(),
            updated_mesh.clone(),
        ));

        Ok(updated_mesh)
    }

    /// Undo last interaction
    pub fn undo(&mut self) -> AiResult<Mesh3D> {
        if self.history.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "No history to undo".to_string(),
            ));
        }

        // Remove last interaction
        self.history.pop();

        // Restore previous state
        if let Some(last_interaction) = self.history.last() {
            match last_interaction {
                Interaction::MeshModification(_, mesh) => {
                    self.current_mesh = mesh.clone();
                }
                _ => {
                    // For other interactions, we'll just return the current mesh
                }
            }
        } else {
            // If no history left, reset to empty mesh
            self.current_mesh = Mesh3D::new();
        }

        self.history.push(Interaction::Undo);
        Ok(self.current_mesh.clone())
    }

    /// Redo last undone action
    pub fn redo(&mut self) -> AiResult<Mesh3D> {
        // Real implementation: restore the mesh to the state before undo
        if self.history.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "No history to redo".to_string(),
            ));
        }

        // Look for the last undo interaction
        let mut undo_index = None;
        for (i, interaction) in self.history.iter().rev().enumerate() {
            if let Interaction::Undo = interaction {
                undo_index = Some(self.history.len() - 1 - i);
                break;
            }
        }

        if let Some(undo_idx) = undo_index {
            // Remove the undo interaction
            self.history.remove(undo_idx);

            // Restore the state before undo
            if undo_idx > 0 {
                let previous_interaction = &self.history[undo_idx - 1];
                if let Interaction::MeshModification(_, mesh) = previous_interaction {
                    self.current_mesh = mesh.clone();
                }
            }
        }

        self.history.push(Interaction::Redo);
        Ok(self.current_mesh.clone())
    }

    /// Morph between two meshes
    fn morph_meshes(&self, mesh1: &Mesh3D, mesh2: &Mesh3D, weight: f64) -> AiResult<Mesh3D> {
        // Real implementation: mesh morphing
        let mut morphed_mesh = Mesh3D::new();

        // Get the minimum number of vertices
        let min_vertices = mesh1.vertices.len().min(mesh2.vertices.len());

        // Morph vertices
        for i in 0..min_vertices {
            let v1 = &mesh1.vertices[i];
            let v2 = &mesh2.vertices[i];
            
            let morphed_point = Point::new(
                v1.point.x * (1.0 - weight) + v2.point.x * weight,
                v1.point.y * (1.0 - weight) + v2.point.y * weight,
                v1.point.z * (1.0 - weight) + v2.point.z * weight
            );
            
            morphed_mesh.add_vertex(morphed_point);
        }

        // Add remaining vertices from mesh1
        for i in min_vertices..mesh1.vertices.len() {
            morphed_mesh.add_vertex(mesh1.vertices[i].point);
        }

        // Use faces from mesh1
        for face in &mesh1.faces {
            morphed_mesh.add_face(face.vertices.clone());
        }

        Ok(morphed_mesh)
    }

    /// Get current mesh
    pub fn current_mesh(&self) -> &Mesh3D {
        &self.current_mesh
    }

    /// Get session history
    pub fn history(&self) -> &Vec<Interaction> {
        &self.history
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Merge two meshes
    fn merge_meshes(&self, mesh1: &Mesh3D, mesh2: &Mesh3D) -> AiResult<Mesh3D> {
        // Simple mesh merging implementation
        let mut merged_mesh = mesh1.clone();

        // Add vertices from mesh2 with offset
        let vertex_offset = merged_mesh.vertices.len();
        for vertex in &mesh2.vertices {
            merged_mesh.add_vertex(vertex.point);
        }

        // Add faces from mesh2 with vertex indices adjusted
        for face in &mesh2.faces {
            let mut adjusted_vertices = Vec::new();
            for &vid in &face.vertices {
                adjusted_vertices.push(vid + vertex_offset);
            }
            merged_mesh.add_face(adjusted_vertices);
        }

        Ok(merged_mesh)
    }
}

/// Interactive Generator
pub struct InteractiveGenerator {
    sessions: HashMap<String, InteractiveSession>,
    default_settings: TextTo3DSettings,
}

impl InteractiveGenerator {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            default_settings: TextTo3DSettings::default(),
        }
    }

    pub fn with_default_settings(mut self, settings: TextTo3DSettings) -> Self {
        self.default_settings = settings;
        self
    }

    /// Create a new interactive session
    pub fn create_session(&mut self) -> String {
        let session = InteractiveSession::new().with_settings(self.default_settings.clone());
        let session_id = session.id().to_string();
        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    /// Get session by ID
    pub fn get_session(&mut self, session_id: &str) -> Option<&mut InteractiveSession> {
        self.sessions.get_mut(session_id)
    }

    /// Process multimodal prompt for a session
    pub fn process_multimodal_prompt(
        &mut self,
        session_id: &str,
        prompt: &MultimodalPrompt,
    ) -> AiResult<Mesh3D> {
        match self.get_session(session_id) {
            Some(session) => session.process_multimodal_prompt(prompt),
            None => Err(AiProtocolError::InvalidData(format!(
                "Session '{}' not found",
                session_id
            ))),
        }
    }

    /// Handle user feedback for a session
    pub fn handle_feedback(&mut self, session_id: &str, feedback: &str) -> AiResult<Mesh3D> {
        match self.get_session(session_id) {
            Some(session) => session.handle_feedback(feedback),
            None => Err(AiProtocolError::InvalidData(format!(
                "Session '{}' not found",
                session_id
            ))),
        }
    }

    /// Undo last action for a session
    pub fn undo(&mut self, session_id: &str) -> AiResult<Mesh3D> {
        match self.get_session(session_id) {
            Some(session) => session.undo(),
            None => Err(AiProtocolError::InvalidData(format!(
                "Session '{}' not found",
                session_id
            ))),
        }
    }

    /// Redo last undone action for a session
    pub fn redo(&mut self, session_id: &str) -> AiResult<Mesh3D> {
        match self.get_session(session_id) {
            Some(session) => session.redo(),
            None => Err(AiProtocolError::InvalidData(format!(
                "Session '{}' not found",
                session_id
            ))),
        }
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<String> {
        self.sessions.keys().cloned().collect()
    }

    /// Remove session
    pub fn remove_session(&mut self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }
}

/// Extension methods for Mesh3D
pub trait InteractiveExt {
    /// Create interactive session from mesh
    fn create_interactive_session(settings: &TextTo3DSettings) -> String;

    /// Process multimodal prompt
    fn from_multimodal_prompt(
        prompt: &MultimodalPrompt,
        settings: &TextTo3DSettings,
    ) -> AiResult<Mesh3D>;
}

impl InteractiveExt for Mesh3D {
    fn create_interactive_session(settings: &TextTo3DSettings) -> String {
        let mut generator = InteractiveGenerator::new().with_default_settings((*settings).clone());
        generator.create_session()
    }

    fn from_multimodal_prompt(
        prompt: &MultimodalPrompt,
        settings: &TextTo3DSettings,
    ) -> AiResult<Mesh3D> {
        let mut generator = InteractiveGenerator::new().with_default_settings((*settings).clone());
        let session_id = generator.create_session();
        generator.process_multimodal_prompt(&session_id, prompt)
    }
}
