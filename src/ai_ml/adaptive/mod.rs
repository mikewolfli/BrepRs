//! Adaptive Model Module
//! 
//! This module provides functionality for adaptive 3D models that can automatically adjust their
//! detail level based on rendering performance, device capabilities, and user feedback.

use crate::ai_ml::model_optimization::{LodResult, ModelOptimizer};
use crate::ai_ml::visualization::PerformanceMetrics;
use crate::mesh::mesh_data::Mesh3D;
use std::time::Instant;

/// Device Capability Level
pub enum DeviceCapability {
    Low,    // Mobile devices, low-end GPUs
    Medium, // Mid-range desktops, integrated GPUs
    High,   // High-end desktops, dedicated GPUs
    Ultra,  // Workstations, high-end gaming PCs
}

/// User Feedback
pub struct UserFeedback {
    pub satisfaction: f64, // 0.0 to 1.0
    pub performance_issues: bool,
    pub visual_quality_issues: bool,
    pub comments: Option<String>,
}

impl Default for UserFeedback {
    fn default() -> Self {
        Self {
            satisfaction: 0.5,
            performance_issues: false,
            visual_quality_issues: false,
            comments: None,
        }
    }
}

/// Adaptive Model Settings
pub struct AdaptiveModelSettings {
    pub target_fps: f64,
    pub max_vertices: usize,
    pub max_faces: usize,
    pub quality_threshold: f64,
    pub adjustment_rate: f64, // How quickly to adjust detail level
}

impl Default for AdaptiveModelSettings {
    fn default() -> Self {
        Self {
            target_fps: 30.0,
            max_vertices: 100000,
            max_faces: 200000,
            quality_threshold: 0.7,
            adjustment_rate: 0.1,
        }
    }
}

/// Adaptive Model
pub struct AdaptiveModel {
    original_mesh: Mesh3D,
    lods: Vec<Mesh3D>,
    current_lod: usize,
    settings: AdaptiveModelSettings,
    device_capability: DeviceCapability,
    performance_history: Vec<PerformanceMetrics>,
    user_feedback: Vec<UserFeedback>,
    last_adjustment: Instant,
}

impl AdaptiveModel {
    pub fn new(mesh: Mesh3D, settings: AdaptiveModelSettings) -> Self {
        // Generate LODs
        let optimizer = ModelOptimizer::new();
        let lod_settings = crate::ai_ml::model_optimization::LodSettings {
            levels: 4,
            reduction_ratios: vec![0.8, 0.5, 0.3, 0.1],
        };
        
        let lod_result = optimizer.generate_lods(&mesh, &lod_settings).unwrap();
        let mut lods = lod_result.lods;
        lods.insert(0, mesh.clone()); // Add original mesh as highest LOD
        
        Self {
            original_mesh: mesh,
            lods,
            current_lod: 0,
            settings,
            device_capability: DeviceCapability::Medium,
            performance_history: Vec::new(),
            user_feedback: Vec::new(),
            last_adjustment: Instant::now(),
        }
    }

    pub fn with_device_capability(mut self, capability: DeviceCapability) -> Self {
        self.device_capability = capability;
        // Adjust initial LOD based on device capability
        match capability {
            DeviceCapability::Low => self.current_lod = 3,
            DeviceCapability::Medium => self.current_lod = 1,
            DeviceCapability::High => self.current_lod = 0,
            DeviceCapability::Ultra => self.current_lod = 0,
        }
        self
    }

    /// Get current mesh (at current LOD level)
    pub fn get_current_mesh(&self) -> &Mesh3D {
        &self.lods[self.current_lod]
    }

    /// Update performance metrics and adjust LOD if needed
    pub fn update_performance(&mut self, metrics: PerformanceMetrics) {
        self.performance_history.push(metrics);
        
        // Keep only the last 10 performance records
        if self.performance_history.len() > 10 {
            self.performance_history.remove(0);
        }
        
        // Check if we need to adjust LOD
        self.adjust_lod();
    }

    /// Add user feedback and adjust LOD if needed
    pub fn add_user_feedback(&mut self, feedback: UserFeedback) {
        self.user_feedback.push(feedback);
        
        // Keep only the last 5 feedback records
        if self.user_feedback.len() > 5 {
            self.user_feedback.remove(0);
        }
        
        // Check if we need to adjust LOD
        self.adjust_lod();
    }

    /// Adjust LOD based on performance and user feedback
    fn adjust_lod(&mut self) {
        // Don't adjust too frequently
        if self.last_adjustment.elapsed().as_secs() < 2 {
            return;
        }
        
        let mut target_lod = self.current_lod;
        
        // Analyze performance
        if !self.performance_history.is_empty() {
            let avg_fps = self.performance_history
                .iter()
                .map(|m| m.fps)
                .sum::<f64>() / self.performance_history.len() as f64;
            
            let avg_render_time = self.performance_history
                .iter()
                .map(|m| m.render_time)
                .sum::<f64>() / self.performance_history.len() as f64;
            
            // If FPS is too low, decrease detail
            if avg_fps < self.settings.target_fps * 0.8 {
                target_lod = (target_lod + 1).min(self.lods.len() - 1);
            }
            // If FPS is very high, increase detail
            else if avg_fps > self.settings.target_fps * 1.5 {
                target_lod = target_lod.saturating_sub(1);
            }
        }
        
        // Analyze user feedback
        if !self.user_feedback.is_empty() {
            let avg_satisfaction = self.user_feedback
                .iter()
                .map(|f| f.satisfaction)
                .sum::<f64>() / self.user_feedback.len() as f64;
            
            let has_performance_issues = self.user_feedback
                .iter()
                .any(|f| f.performance_issues);
            
            let has_quality_issues = self.user_feedback
                .iter()
                .any(|f| f.visual_quality_issues);
            
            // If user reports performance issues, decrease detail
            if has_performance_issues {
                target_lod = (target_lod + 1).min(self.lods.len() - 1);
            }
            // If user reports quality issues, increase detail
            else if has_quality_issues && avg_satisfaction < 0.5 {
                target_lod = target_lod.saturating_sub(1);
            }
        }
        
        // Apply adjustment
        if target_lod != self.current_lod {
            self.current_lod = target_lod;
            self.last_adjustment = Instant::now();
            println!("Adjusted LOD to level {}", self.current_lod);
        }
    }

    /// Get current LOD level
    pub fn get_current_lod(&self) -> usize {
        self.current_lod
    }

    /// Get all LOD levels
    pub fn get_lods(&self) -> &Vec<Mesh3D> {
        &self.lods
    }

    /// Manually set LOD level
    pub fn set_lod(&mut self, level: usize) {
        if level < self.lods.len() {
            self.current_lod = level;
            self.last_adjustment = Instant::now();
        }
    }

    /// Reset to original mesh
    pub fn reset_to_original(&mut self) {
        self.current_lod = 0;
        self.last_adjustment = Instant::now();
    }

    /// Get performance history
    pub fn get_performance_history(&self) -> &Vec<PerformanceMetrics> {
        &self.performance_history
    }

    /// Get user feedback history
    pub fn get_user_feedback(&self) -> &Vec<UserFeedback> {
        &self.user_feedback
    }
}

/// Adaptive Model Manager
pub struct AdaptiveModelManager {
    models: std::collections::HashMap<String, AdaptiveModel>,
}

impl AdaptiveModelManager {
    pub fn new() -> Self {
        Self {
            models: std::collections::HashMap::new(),
        }
    }

    /// Add an adaptive model
    pub fn add_model(&mut self, name: &str, model: AdaptiveModel) {
        self.models.insert(name.to_string(), model);
    }

    /// Get an adaptive model
    pub fn get_model(&self, name: &str) -> Option<&AdaptiveModel> {
        self.models.get(name)
    }

    /// Get a mutable reference to an adaptive model
    pub fn get_model_mut(&mut self, name: &str) -> Option<&mut AdaptiveModel> {
        self.models.get_mut(name)
    }

    /// Update performance for all models
    pub fn update_performance_for_all(&mut self, metrics: PerformanceMetrics) {
        for model in self.models.values_mut() {
            model.update_performance(metrics.clone());
        }
    }

    /// Add user feedback for a specific model
    pub fn add_user_feedback(&mut self, model_name: &str, feedback: UserFeedback) {
        if let Some(model) = self.models.get_mut(model_name) {
            model.add_user_feedback(feedback);
        }
    }

    /// Get current mesh for a model
    pub fn get_current_mesh(&self, model_name: &str) -> Option<&Mesh3D> {
        self.models.get(model_name).map(|m| m.get_current_mesh())
    }
}
