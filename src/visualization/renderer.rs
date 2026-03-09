//! Renderer for 3D visualization
//!
//! This module provides rendering capabilities for 3D visualization,
//! supporting both OpenGL and WGPU backends.
//! Compatible with OpenCASCADE Open API design.

 use crate::geometry::{Transform};
use crate::visualization::camera::Camera;
use crate::visualization::light::Light;
use crate::visualization::material::Material;
use crate::visualization::primitives::*;
use crate::visualization::gpu_memory::{GpuMemoryManager, GpuMemoryStats};
use crate::visualization::gpu_buffer::{GpuBufferManager, GpuBufferUsage};
use crate::visualization::texture_stream::{TextureStreamingSystem, TextureDescriptor, TextureFormat};
use crate::visualization::gpu_compression::{GpuMemoryCompressor, CompressionAlgorithm, CompressionQuality};
use std::sync::Arc;

/// Render mode for visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// Wireframe rendering
    Wireframe,
    /// Shaded rendering
    Shaded,
    /// Wireframe with shaded overlay
    WireframeShaded,
    /// Point cloud rendering
    Points,
    /// Hidden line removal
    HiddenLine,
}

impl Default for RenderMode {
    fn default() -> Self {
        RenderMode::Shaded
    }
}

/// Rendering statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats {
    /// Frame count
    pub frame_count: u64,
    /// Frames per second
    pub fps: f32,
    /// Draw call count
    pub draw_calls: u32,
    /// Triangle count
    pub triangle_count: u32,
    /// Vertex count
    pub vertex_count: u32,
    /// Render time in milliseconds
    pub render_time_ms: f32,
}

/// Render target configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderTarget {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Color format
    pub color_format: ColorFormat,
    /// Depth format
    pub depth_format: DepthFormat,
    /// Multi-sample count
    pub sample_count: u32,
}

impl RenderTarget {
    /// Create a new render target
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_format: ColorFormat::RGBA8,
            depth_format: DepthFormat::Depth24Plus,
            sample_count: 1,
        }
    }

    /// Set color format
    pub fn with_color_format(mut self, format: ColorFormat) -> Self {
        self.color_format = format;
        self
    }

    /// Set depth format
    pub fn with_depth_format(mut self, format: DepthFormat) -> Self {
        self.depth_format = format;
        self
    }

    /// Set sample count
    pub fn with_sample_count(mut self, count: u32) -> Self {
        self.sample_count = count;
        self
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            1.0
        } else {
            self.width as f32 / self.height as f32
        }
    }
}

/// Color format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    /// 8-bit RGBA
    RGBA8,
    /// 8-bit BGRA
    BGRA8,
    /// 16-bit floating point RGBA
    RGBA16F,
    /// 32-bit floating point RGBA
    RGBA32F,
}

/// Depth format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthFormat {
    /// No depth buffer
    None,
    /// 16-bit depth
    Depth16,
    /// 24-bit depth
    Depth24,
    /// 24-bit depth + 8-bit stencil
    Depth24Stencil8,
    /// 32-bit floating point depth
    Depth32F,
    /// 24-bit depth (plus)
    Depth24Plus,
}

/// Cull mode for face culling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    /// No culling
    None,
    /// Cull front faces
    Front,
    /// Cull back faces
    Back,
}

impl Default for CullMode {
    fn default() -> Self {
        CullMode::Back
    }
}

/// Blend mode for transparency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// No blending
    None,
    /// Alpha blending
    Alpha,
    /// Additive blending
    Additive,
    /// Multiplicative blending
    Multiply,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::None
    }
}

/// Render state configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderState {
    /// Render mode
    pub render_mode: RenderMode,
    /// Cull mode
    pub cull_mode: CullMode,
    /// Blend mode
    pub blend_mode: BlendMode,
    /// Depth testing enabled
    pub depth_test: bool,
    /// Depth writing enabled
    pub depth_write: bool,
    /// Wireframe line width
    pub wireframe_width: f32,
    /// Point size
    pub point_size: f32,
}

impl RenderState {
    /// Create default render state
    pub fn new() -> Self {
        Self {
            render_mode: RenderMode::Shaded,
            cull_mode: CullMode::Back,
            blend_mode: BlendMode::None,
            depth_test: true,
            depth_write: true,
            wireframe_width: 1.0,
            point_size: 1.0,
        }
    }

    /// Set render mode
    pub fn with_render_mode(mut self, mode: RenderMode) -> Self {
        self.render_mode = mode;
        self
    }

    /// Set cull mode
    pub fn with_cull_mode(mut self, mode: CullMode) -> Self {
        self.cull_mode = mode;
        self
    }

    /// Set blend mode
    pub fn with_blend_mode(mut self, mode: BlendMode) -> Self {
        self.blend_mode = mode;
        self
    }

    /// Set depth test
    pub fn with_depth_test(mut self, enabled: bool) -> Self {
        self.depth_test = enabled;
        self
    }

    /// Set depth write
    pub fn with_depth_write(mut self, enabled: bool) -> Self {
        self.depth_write = enabled;
        self
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}

/// Renderable object trait
pub trait Renderable {
    /// Render the object
    fn render(&self, renderer: &mut dyn Renderer);
    /// Get bounding box
    fn bounding_box(&self) -> ([f32; 3], [f32; 3]);
    /// Check if visible
    fn is_visible(&self) -> bool;
    /// Set visibility
    fn set_visible(&mut self, visible: bool);
}

/// Renderer trait for backend abstraction
pub trait Renderer {
    /// Initialize the renderer
    fn initialize(&mut self, target: &RenderTarget) -> Result<(), RenderError>;
    /// Resize the render target
    fn resize(&mut self, width: u32, height: u32);
    /// Begin frame
    fn begin_frame(&mut self, clear_color: Color);
    /// End frame
    fn end_frame(&mut self);
    /// Set camera
    fn set_camera(&mut self, camera: &Camera);
    /// Set lights
    fn set_lights(&mut self, lights: &[Light]);
    /// Set render state
    fn set_render_state(&mut self, state: &RenderState);
    /// Render mesh
    fn render_mesh(&mut self, mesh: &MeshPrimitive, material: &Material, transform: &Transform);
    /// Render lines
    fn render_lines(&mut self, lines: &[Line], transform: &Transform);
    /// Render points
    fn render_points(&mut self, points: &[GraphicPoint], transform: &Transform);
    /// Render text
    fn render_text(&mut self, text: &TextLabel);
    /// Get render stats
    fn stats(&self) -> RenderStats;
    /// Check if initialized
    fn is_initialized(&self) -> bool;
}

/// Render error types
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    /// Initialization failed
    InitializationFailed(String),
    /// Shader compilation failed
    ShaderCompilationFailed(String),
    /// Buffer creation failed
    BufferCreationFailed(String),
    /// Texture creation failed
    TextureCreationFailed(String),
    /// Invalid parameter
    InvalidParameter(String),
    /// Out of memory
    OutOfMemory,
    /// Backend not available
    BackendNotAvailable,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::InitializationFailed(msg) => {
                write!(f, "Initialization failed: {}", msg)
            }
            RenderError::ShaderCompilationFailed(msg) => {
                write!(f, "Shader compilation failed: {}", msg)
            }
            RenderError::BufferCreationFailed(msg) => {
                write!(f, "Buffer creation failed: {}", msg)
            }
            RenderError::TextureCreationFailed(msg) => {
                write!(f, "Texture creation failed: {}", msg)
            }
            RenderError::InvalidParameter(msg) => {
                write!(f, "Invalid parameter: {}", msg)
            }
            RenderError::OutOfMemory => write!(f, "Out of memory"),
            RenderError::BackendNotAvailable => write!(f, "Backend not available"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Software renderer for CPU-based rendering
pub struct SoftwareRenderer {
    /// Render target
    target: RenderTarget,
    /// Render state
    state: RenderState,
    /// Camera
    camera: Option<Camera>,
    /// Lights
    lights: Vec<Light>,
    /// Frame buffer (RGBA)
    frame_buffer: Vec<u8>,
    /// Depth buffer
    depth_buffer: Vec<f32>,
    /// Stats
    stats: RenderStats,
    /// Initialized flag
    initialized: bool,
    /// GPU memory manager
    gpu_memory_manager: Arc<GpuMemoryManager>,
    /// GPU buffer manager
    gpu_buffer_manager: Arc<GpuBufferManager>,
    /// Texture streaming system
    texture_streaming: Arc<TextureStreamingSystem>,
    /// GPU memory compressor
    gpu_compressor: Arc<GpuMemoryCompressor>,
}

impl SoftwareRenderer {
    /// Create a new software renderer
    pub fn new() -> Self {
        let gpu_memory_manager = Arc::new(GpuMemoryManager::new());
        let gpu_buffer_manager = Arc::new(GpuBufferManager::new(gpu_memory_manager.clone()));
        let texture_streaming = Arc::new(TextureStreamingSystem::new(gpu_memory_manager.clone(), 100));
        let gpu_compressor = Arc::new(GpuMemoryCompressor::new(
            CompressionAlgorithm::Bc3,
            CompressionQuality::Balanced,
        ));

        Self {
            target: RenderTarget::new(800, 600),
            state: RenderState::default(),
            camera: None,
            lights: Vec::new(),
            frame_buffer: Vec::new(),
            depth_buffer: Vec::new(),
            stats: RenderStats::default(),
            initialized: false,
            gpu_memory_manager,
            gpu_buffer_manager,
            texture_streaming,
            gpu_compressor,
        }
    }

    /// Get frame buffer
    pub fn frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    /// Get GPU memory statistics
    #[inline]
    pub fn gpu_memory_stats(&self) -> GpuMemoryStats {
        self.gpu_memory_manager.global_stats()
    }

    /// Get GPU buffer manager
    #[inline]
    pub fn gpu_buffer_manager(&self) -> &Arc<GpuBufferManager> {
        &self.gpu_buffer_manager
    }

    /// Get texture streaming system
    #[inline]
    pub fn texture_streaming(&self) -> &Arc<TextureStreamingSystem> {
        &self.texture_streaming
    }

    /// Get GPU compressor
    #[inline]
    pub fn gpu_compressor(&self) -> &Arc<GpuMemoryCompressor> {
        &self.gpu_compressor
    }

    /// Clear buffers
    fn clear_buffers(&mut self, color: Color) {
        let pixel_count = (self.target.width * self.target.height) as usize;

        // Clear color buffer
        for i in 0..pixel_count {
            let idx = i * 4;
            if idx + 3 < self.frame_buffer.len() {
                self.frame_buffer[idx] = (color.r * 255.0) as u8;
                self.frame_buffer[idx + 1] = (color.g * 255.0) as u8;
                self.frame_buffer[idx + 2] = (color.b * 255.0) as u8;
                self.frame_buffer[idx + 3] = (color.a * 255.0) as u8;
            }
        }

        // Clear depth buffer
        for depth in &mut self.depth_buffer {
            *depth = 1.0;
        }
    }

    /// Project 3D point to screen space
    fn project_point(&self, point: &[f32; 3]) -> Option<[f32; 3]> {
        if let Some(camera) = &self.camera {
            // Simple perspective projection
            let view_matrix = camera.view_matrix();
            let proj_matrix = camera.projection_matrix(self.target.aspect_ratio());

            // Transform to view space
            let view_pos = transform_point(point, &view_matrix);

            // Transform to clip space
            let clip_pos = transform_point(&[view_pos[0], view_pos[1], view_pos[2]], &proj_matrix);

            // Perspective divide
            if clip_pos[3] != 0.0 {
                let ndc_x = clip_pos[0] / clip_pos[3];
                let ndc_y = clip_pos[1] / clip_pos[3];
                let ndc_z = clip_pos[2] / clip_pos[3];

                // NDC to screen space
                let screen_x = (ndc_x + 1.0) * 0.5 * self.target.width as f32;
                let screen_y = (1.0 - ndc_y) * 0.5 * self.target.height as f32;

                Some([screen_x, screen_y, ndc_z])
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Draw a line using Bresenham's algorithm
    fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: Color) {
        let x0 = start[0] as i32;
        let y0 = start[1] as i32;
        let x1 = end[0] as i32;
        let y1 = end[1] as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < self.target.width as i32 && y >= 0 && y < self.target.height as i32 {
                self.set_pixel(x as u32, y as u32, color);
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Set a pixel
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.target.width && y < self.target.height {
            let idx = ((y * self.target.width + x) * 4) as usize;
            if idx + 3 < self.frame_buffer.len() {
                self.frame_buffer[idx] = (color.r * 255.0) as u8;
                self.frame_buffer[idx + 1] = (color.g * 255.0) as u8;
                self.frame_buffer[idx + 2] = (color.b * 255.0) as u8;
                self.frame_buffer[idx + 3] = (color.a * 255.0) as u8;
            }
        }
    }
}

impl Renderer for SoftwareRenderer {
    fn initialize(&mut self, target: &RenderTarget) -> Result<(), RenderError> {
        self.target = *target;
        let pixel_count = (target.width * target.height) as usize;
        self.frame_buffer.resize(pixel_count * 4, 0);
        self.depth_buffer.resize(pixel_count, 1.0);
        self.initialized = true;
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.target.width = width;
        self.target.height = height;
        let pixel_count = (width * height) as usize;
        self.frame_buffer.resize(pixel_count * 4, 0);
        self.depth_buffer.resize(pixel_count, 1.0);
    }

    fn begin_frame(&mut self, clear_color: Color) {
        self.clear_buffers(clear_color);
        self.stats.draw_calls = 0;
        self.stats.triangle_count = 0;
        self.stats.vertex_count = 0;
    }

    fn end_frame(&mut self) {
        self.stats.frame_count += 1;
    }

    fn set_camera(&mut self, camera: &Camera) {
        self.camera = Some(camera.clone());
    }

    fn set_lights(&mut self, lights: &[Light]) {
        self.lights = lights.to_vec();
    }

    fn set_render_state(&mut self, state: &RenderState) {
        self.state = *state;
    }

    fn render_mesh(&mut self, mesh: &MeshPrimitive, _material: &Material, _transform: &Transform) {
        if self.state.render_mode == RenderMode::Wireframe
            || self.state.render_mode == RenderMode::WireframeShaded
        {
            // Render wireframe
            for i in (0..mesh.indices.len()).step_by(3) {
                if i + 2 < mesh.indices.len() {
                    let i0 = mesh.indices[i] as usize;
                    let i1 = mesh.indices[i + 1] as usize;
                    let i2 = mesh.indices[i + 2] as usize;

                    if let (Some(p0), Some(p1), Some(p2)) = (
                        self.project_point(&mesh.vertices[i0].position),
                        self.project_point(&mesh.vertices[i1].position),
                        self.project_point(&mesh.vertices[i2].position),
                    ) {
                        let color = Color::white();
                        self.draw_line(p0, p1, color);
                        self.draw_line(p1, p2, color);
                        self.draw_line(p2, p0, color);
                    }
                }
            }
            self.stats.draw_calls += 1;
            self.stats.triangle_count += mesh.triangle_count() as u32;
        }
    }

    fn render_lines(&mut self, lines: &[Line], _transform: &Transform) {
        for line in lines {
            if let (Some(start), Some(end)) = (
                self.project_point(&line.start),
                self.project_point(&line.end),
            ) {
                self.draw_line(start, end, line.color);
            }
        }
        self.stats.draw_calls += 1;
    }

    fn render_points(&mut self, points: &[GraphicPoint], _transform: &Transform) {
        for point in points {
            if let Some(pos) = self.project_point(&point.position) {
                let radius = point.size as i32;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        if dx * dx + dy * dy <= radius * radius {
                            self.set_pixel(
                                (pos[0] as i32 + dx) as u32,
                                (pos[1] as i32 + dy) as u32,
                                point.color,
                            );
                        }
                    }
                }
            }
        }
        self.stats.draw_calls += 1;
    }

    fn render_text(&mut self, _text: &TextLabel) {
        // Text rendering not implemented in software renderer
    }

    fn stats(&self) -> RenderStats {
        self.stats
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Helper function to transform a point by a 4x4 matrix
fn transform_point(point: &[f32; 3], matrix: &[[f32; 4]; 4]) -> [f32; 4] {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    let w = 1.0;

    [
        matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z + matrix[0][3] * w,
        matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z + matrix[1][3] * w,
        matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z + matrix[2][3] * w,
        matrix[3][0] * x + matrix[3][1] * y + matrix[3][2] * z + matrix[3][3] * w,
    ]
}

impl Default for SoftwareRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Scene renderer that manages multiple renderable objects
pub struct SceneRenderer {
    /// Renderer backend
    renderer: Box<dyn Renderer>,
    /// Camera
    camera: Camera,
    /// Lights
    lights: Vec<Light>,
    /// Render state
    state: RenderState,
    /// Render target
    target: RenderTarget,
}

impl SceneRenderer {
    /// Create a new scene renderer
    pub fn new(renderer: Box<dyn Renderer>, target: RenderTarget) -> Self {
        Self {
            renderer,
            camera: Camera::default(),
            lights: Vec::new(),
            state: RenderState::default(),
            target,
        }
    }

    /// Initialize the renderer
    pub fn initialize(&mut self) -> Result<(), RenderError> {
        self.renderer.initialize(&self.target)
    }

    /// Resize the render target
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.width = width;
        self.target.height = height;
        self.renderer.resize(width, height);
    }

    /// Set camera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
        self.renderer.set_camera(&self.camera);
    }

    /// Add light
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
        self.renderer.set_lights(&self.lights);
    }

    /// Clear lights
    pub fn clear_lights(&mut self) {
        self.lights.clear();
        self.renderer.set_lights(&[]);
    }

    /// Set render state
    pub fn set_render_state(&mut self, state: RenderState) {
        self.state = state;
        self.renderer.set_render_state(&self.state);
    }

    /// Begin rendering frame
    pub fn begin_frame(&mut self, clear_color: Color) {
        self.renderer.begin_frame(clear_color);
    }

    /// End rendering frame
    pub fn end_frame(&mut self) {
        self.renderer.end_frame();
    }

    /// Render a mesh
    pub fn render_mesh(
        &mut self,
        mesh: &MeshPrimitive,
        material: &Material,
        transform: &Transform,
    ) {
        self.renderer.render_mesh(mesh, material, transform);
    }

    /// Render lines
    pub fn render_lines(&mut self, lines: &[Line], transform: &Transform) {
        self.renderer.render_lines(lines, transform);
    }

    /// Render points
    pub fn render_points(&mut self, points: &[GraphicPoint], transform: &Transform) {
        self.renderer.render_points(points, transform);
    }

    /// Render text
    pub fn render_text(&mut self, text: &TextLabel) {
        self.renderer.render_text(text);
    }

    /// Get render stats
    pub fn stats(&self) -> RenderStats {
        self.renderer.stats()
    }

    /// Get camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get mutable camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_target() {
        let target = RenderTarget::new(800, 600);
        assert_eq!(target.width, 800);
        assert_eq!(target.height, 600);
        assert!((target.aspect_ratio() - 800.0 / 600.0).abs() < 0.001);
    }

    #[test]
    fn test_render_state() {
        let state = RenderState::new()
            .with_render_mode(RenderMode::Wireframe)
            .with_cull_mode(CullMode::Front);
        assert_eq!(state.render_mode, RenderMode::Wireframe);
        assert_eq!(state.cull_mode, CullMode::Front);
    }

    #[test]
    fn test_software_renderer() {
        let mut renderer = SoftwareRenderer::new();
        let target = RenderTarget::new(100, 100);
        assert!(renderer.initialize(&target).is_ok());
        assert!(renderer.is_initialized());

        renderer.begin_frame(Color::black());
        renderer.end_frame();

        let stats = renderer.stats();
        assert_eq!(stats.frame_count, 1);
    }

    #[test]
    fn test_scene_renderer() {
        let software_renderer = SoftwareRenderer::new();
        let target = RenderTarget::new(100, 100);
        let mut scene = SceneRenderer::new(Box::new(software_renderer), target);

        assert!(scene.initialize().is_ok());

        scene.begin_frame(Color::black());
        scene.end_frame();

        let stats = scene.stats();
        assert_eq!(stats.frame_count, 1);
    }
}
