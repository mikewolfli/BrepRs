//! AI/ML Integration Module
//!
//! This module provides a comprehensive interface for AI and machine learning interactions,
//! including protocol layer, model management, data conversion, and framework integration.

pub mod adaptive;
pub mod advanced;
pub mod frameworks;
pub mod function_tools;
pub mod interactive;
pub mod material_texture;
pub mod model_integration;
pub mod model_optimization;
pub mod model_quality;
pub mod models;
pub mod multimodal;
pub mod protocol;
pub mod style_transfer;
pub mod text_to_3d;
pub mod utils;
pub mod visualization;
pub mod workflow;

pub use advanced::{
    FederatedLearningClient, FederatedLearningServer, ReinforcementLearningAgent,
    TransferLearningModel,
};
pub use frameworks::{onnx, pytorch, tensorflow};
pub use function_tools::{
    create_builtin_plugins, create_builtin_tools, FunctionCallTool, FunctionToolManager,
    SkillPlugin,
};
pub use interactive::{InteractiveExt, InteractiveGenerator, InteractiveSession, MultimodalPrompt};
pub use material_texture::{
    Material, MaterialGenerationResult, MaterialGenerationSettings, MaterialProperties,
    MaterialTextureExt, MaterialTextureGenerator, TextureProperties, TextureType,
};
pub use model_optimization::{
    LodResult, LodSettings, MeshOptimizationExt, ModelOptimizer, OptimizationSettings,
    SimplificationResult,
};
pub use model_quality::{
    MeshQualityExt, ModelQualityEvaluator, ModelQualityReport, ModelRepairTool,
};
pub use models::{
    AiModel, AiModelManager, FeatureRecognitionModel, MeshGenerationModel, ModelRepairModel,
};
/// Re-export common types and functions for easier access
pub use protocol::{
    AiDataType, AiMessage, AiProtocol, AiProtocolError, AiRequest, AiResponse, AiResult,
    DefaultAiProtocol,
};
pub use style_transfer::{
    StyleFeatures, StyleReference, StyleTransferExt, StyleTransferResult, StyleTransferSettings,
    StyleTransferTool,
};
pub use text_to_3d::{TextTo3DExt, TextTo3DGenerator, TextTo3DResult, TextTo3DSettings};
pub use utils::{AiMlUtils, MlDataset, MlModelFormat};
pub use visualization::{FeatureVisualization, MlVisualization, PerformanceMetrics};
pub use workflow::{MlPipeline, MlWorkflow, MlWorkflowBuilder};

#[cfg(test)]
mod tests;
