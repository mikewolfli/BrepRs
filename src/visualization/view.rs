//! View control and manipulation for 3D visualization
//!
//! This module provides view control functionality for 3D visualization,
//! including viewports, view manipulation, and display modes.
//! Compatible with OpenCASCADE Open API design.

use crate::geometry::Point;
use crate::visualization::camera::{Camera, CameraController};
use crate::visualization::primitives::Color;
use crate::visualization::renderer::RenderMode;

/// View type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    /// Perspective view
    Perspective,
    /// Orthographic front view
    Front,
    /// Orthographic back view
    Back,
    /// Orthographic top view
    Top,
    /// Orthographic bottom view
    Bottom,
    /// Orthographic left view
    Left,
    /// Orthographic right view
    Right,
    /// Isometric view
    Isometric,
    /// Custom view
    Custom,
}

impl Default for ViewType {
    fn default() -> Self {
        ViewType::Perspective
    }
}

impl ViewType {
    /// Get view name
    pub fn name(&self) -> &'static str {
        match self {
            ViewType::Perspective => "Perspective",
            ViewType::Front => "Front",
            ViewType::Back => "Back",
            ViewType::Top => "Top",
            ViewType::Bottom => "Bottom",
            ViewType::Left => "Left",
            ViewType::Right => "Right",
            ViewType::Isometric => "Isometric",
            ViewType::Custom => "Custom",
        }
    }

    /// Check if orthographic
    pub fn is_orthographic(&self) -> bool {
        matches!(
            self,
            ViewType::Front
                | ViewType::Back
                | ViewType::Top
                | ViewType::Bottom
                | ViewType::Left
                | ViewType::Right
                | ViewType::Isometric
        )
    }

    /// Get standard camera for view type
    pub fn camera(&self) -> Camera {
        match self {
            ViewType::Perspective => Camera::new(),
            ViewType::Front => Camera::front_view(),
            ViewType::Back => Camera::back_view(),
            ViewType::Top => Camera::top_view(),
            ViewType::Bottom => Camera::bottom_view(),
            ViewType::Left => Camera::left_view(),
            ViewType::Right => Camera::right_view(),
            ViewType::Isometric => Camera::isometric_view(),
            ViewType::Custom => Camera::new(),
        }
    }
}

/// Viewport configuration
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Viewport ID
    pub id: u32,
    /// X position (0-1 normalized)
    pub x: f32,
    /// Y position (0-1 normalized)
    pub y: f32,
    /// Width (0-1 normalized)
    pub width: f32,
    /// Height (0-1 normalized)
    pub height: f32,
    /// Pixel X position
    pub pixel_x: i32,
    /// Pixel Y position
    pub pixel_y: i32,
    /// Pixel width
    pub pixel_width: u32,
    /// Pixel height
    pub pixel_height: u32,
    /// View type
    pub view_type: ViewType,
    /// Camera
    pub camera: Camera,
    /// Camera controller
    pub controller: CameraController,
    /// Render mode
    pub render_mode: RenderMode,
    /// Background color
    pub background_color: Color,
    /// Gradient background
    pub gradient_background: bool,
    /// Background color top (for gradient)
    pub background_color_top: Color,
    /// Background color bottom (for gradient)
    pub background_color_bottom: Color,
    /// Show grid
    pub show_grid: bool,
    /// Grid color
    pub grid_color: Color,
    /// Grid size
    pub grid_size: f32,
    /// Grid spacing
    pub grid_spacing: f32,
    /// Show axes
    pub show_axes: bool,
    /// Axes size
    pub axes_size: f32,
    /// Active flag
    pub active: bool,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(id: u32) -> Self {
        Self {
            id,
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            pixel_x: 0,
            pixel_y: 0,
            pixel_width: 800,
            pixel_height: 600,
            view_type: ViewType::Perspective,
            camera: Camera::new(),
            controller: CameraController::new(Camera::new()),
            render_mode: RenderMode::Shaded,
            background_color: Color::from_rgb(0.2, 0.2, 0.2),
            gradient_background: true,
            background_color_top: Color::from_rgb(0.3, 0.3, 0.35),
            background_color_bottom: Color::from_rgb(0.15, 0.15, 0.15),
            show_grid: true,
            grid_color: Color::from_rgb(0.4, 0.4, 0.4),
            grid_size: 10.0,
            grid_spacing: 1.0,
            show_axes: true,
            axes_size: 1.0,
            active: false,
        }
    }

    /// Set normalized position and size
    pub fn with_normalized_rect(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.x = x.clamp(0.0, 1.0);
        self.y = y.clamp(0.0, 1.0);
        self.width = width.clamp(0.0, 1.0);
        self.height = height.clamp(0.0, 1.0);
        self
    }

    /// Set pixel position and size
    pub fn with_pixel_rect(mut self, x: i32, y: i32, width: u32, height: u32) -> Self {
        self.pixel_x = x;
        self.pixel_y = y;
        self.pixel_width = width;
        self.pixel_height = height;
        self
    }

    /// Set view type
    pub fn with_view_type(mut self, view_type: ViewType) -> Self {
        self.view_type = view_type;
        self.camera = view_type.camera();
        self.controller = CameraController::new(self.camera.clone());
        self
    }

    /// Set render mode
    pub fn with_render_mode(mut self, mode: RenderMode) -> Self {
        self.render_mode = mode;
        self
    }

    /// Set background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self.gradient_background = false;
        self
    }

    /// Set gradient background
    pub fn with_gradient_background(mut self, top: Color, bottom: Color) -> Self {
        self.gradient_background = true;
        self.background_color_top = top;
        self.background_color_bottom = bottom;
        self
    }

    /// Set grid
    pub fn with_grid(mut self, show: bool, color: Color, size: f32, spacing: f32) -> Self {
        self.show_grid = show;
        self.grid_color = color;
        self.grid_size = size;
        self.grid_spacing = spacing;
        self
    }

    /// Set axes
    pub fn with_axes(mut self, show: bool, size: f32) -> Self {
        self.show_axes = show;
        self.axes_size = size;
        self
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.pixel_height == 0 {
            1.0
        } else {
            self.pixel_width as f32 / self.pixel_height as f32
        }
    }

    /// Update pixel coordinates from normalized coordinates
    pub fn update_pixel_coords(&mut self, parent_width: u32, parent_height: u32) {
        self.pixel_x = (self.x * parent_width as f32) as i32;
        self.pixel_y = (self.y * parent_height as f32) as i32;
        self.pixel_width = (self.width * parent_width as f32) as u32;
        self.pixel_height = (self.height * parent_height as f32) as u32;
    }

    /// Update normalized coordinates from pixel coordinates
    pub fn update_normalized_coords(&mut self, parent_width: u32, parent_height: u32) {
        if parent_width > 0 {
            self.x = self.pixel_x as f32 / parent_width as f32;
            self.width = self.pixel_width as f32 / parent_width as f32;
        }
        if parent_height > 0 {
            self.y = self.pixel_y as f32 / parent_height as f32;
            self.height = self.pixel_height as f32 / parent_height as f32;
        }
    }

    /// Check if point is inside viewport
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.pixel_x
            && x < self.pixel_x + self.pixel_width as i32
            && y >= self.pixel_y
            && y < self.pixel_y + self.pixel_height as i32
    }

    /// Convert screen coordinates to normalized device coordinates (-1 to 1)
    pub fn screen_to_ndc(&self, x: i32, y: i32) -> [f32; 2] {
        let ndc_x = 2.0 * (x - self.pixel_x) as f32 / self.pixel_width as f32 - 1.0;
        let ndc_y = 1.0 - 2.0 * (y - self.pixel_y) as f32 / self.pixel_height as f32;
        [ndc_x, ndc_y]
    }

    /// Fit view to bounding box
    pub fn fit_to_box(&mut self, min: &Point, max: &Point) {
        self.camera.fit_to_box(min, max, self.aspect_ratio());
        self.controller.camera = self.camera.clone();
    }

    /// Reset view to standard position
    pub fn reset_view(&mut self) {
        self.camera = self.view_type.camera();
        self.controller = CameraController::new(self.camera.clone());
    }

    /// Zoom to fit all
    pub fn zoom_fit(&mut self, min: &Point, max: &Point) {
        self.fit_to_box(min, max);
    }

    /// Zoom in
    pub fn zoom_in(&mut self, factor: f32) {
        self.camera.zoom(1.0 / factor);
    }

    /// Zoom out
    pub fn zoom_out(&mut self, factor: f32) {
        self.camera.zoom(factor);
    }

    /// Pan view
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let _right = self.camera.right();
        let _up = self.camera.up_corrected();

        let distance = self.camera.distance_to_target() as f32;
        let scale = distance * 0.001;

        self.camera.move_right(-delta_x * scale);
        self.camera.move_up(delta_y * scale);
    }

    /// Rotate view (orbit)
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.controller.mouse_orbit(delta_x, delta_y);
        self.camera = self.controller.camera.clone();
    }

    /// Set active
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0)
    }
}

/// View manager for handling multiple viewports
#[derive(Debug, Clone, Default)]
pub struct ViewManager {
    /// Viewports
    viewports: Vec<Viewport>,
    /// Active viewport ID
    active_viewport: Option<u32>,
    /// Layout mode
    layout: ViewLayout,
    /// Parent window width
    parent_width: u32,
    /// Parent window height
    parent_height: u32,
}

/// View layout mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewLayout {
    /// Single viewport
    Single,
    /// Two viewports (horizontal split)
    SplitHorizontal,
    /// Two viewports (vertical split)
    SplitVertical,
    /// Four viewports (quad)
    Quad,
    /// Custom layout
    Custom,
}

impl Default for ViewLayout {
    fn default() -> Self {
        ViewLayout::Single
    }
}

impl ViewManager {
    /// Create a new view manager
    pub fn new() -> Self {
        let mut manager = Self {
            viewports: Vec::new(),
            active_viewport: None,
            layout: ViewLayout::Single,
            parent_width: 800,
            parent_height: 600,
        };
        manager.setup_single_view();
        manager
    }

    /// Set parent size
    pub fn set_parent_size(&mut self, width: u32, height: u32) {
        self.parent_width = width;
        self.parent_height = height;
        self.update_layout();
    }

    /// Setup single view
    pub fn setup_single_view(&mut self) {
        self.layout = ViewLayout::Single;
        self.viewports.clear();
        let mut viewport = Viewport::new(0).with_view_type(ViewType::Perspective);
        viewport.update_pixel_coords(self.parent_width, self.parent_height);
        self.viewports.push(viewport);
        self.active_viewport = Some(0);
    }

    /// Setup horizontal split
    pub fn setup_horizontal_split(&mut self) {
        self.layout = ViewLayout::SplitHorizontal;
        self.viewports.clear();

        let mut viewport1 = Viewport::new(0)
            .with_view_type(ViewType::Perspective)
            .with_normalized_rect(0.0, 0.0, 0.5, 1.0);
        viewport1.update_pixel_coords(self.parent_width, self.parent_height);

        let mut viewport2 = Viewport::new(1)
            .with_view_type(ViewType::Front)
            .with_normalized_rect(0.5, 0.0, 0.5, 1.0);
        viewport2.update_pixel_coords(self.parent_width, self.parent_height);

        self.viewports.push(viewport1);
        self.viewports.push(viewport2);
        self.active_viewport = Some(0);
    }

    /// Setup vertical split
    pub fn setup_vertical_split(&mut self) {
        self.layout = ViewLayout::SplitVertical;
        self.viewports.clear();

        let mut viewport1 = Viewport::new(0)
            .with_view_type(ViewType::Perspective)
            .with_normalized_rect(0.0, 0.0, 1.0, 0.5);
        viewport1.update_pixel_coords(self.parent_width, self.parent_height);

        let mut viewport2 = Viewport::new(1)
            .with_view_type(ViewType::Top)
            .with_normalized_rect(0.0, 0.5, 1.0, 0.5);
        viewport2.update_pixel_coords(self.parent_width, self.parent_height);

        self.viewports.push(viewport1);
        self.viewports.push(viewport2);
        self.active_viewport = Some(0);
    }

    /// Setup quad view
    pub fn setup_quad_view(&mut self) {
        self.layout = ViewLayout::Quad;
        self.viewports.clear();

        let view_types = [
            ViewType::Top,
            ViewType::Perspective,
            ViewType::Front,
            ViewType::Right,
        ];

        for i in 0..4 {
            let x = if i % 2 == 0 { 0.0 } else { 0.5 };
            let y = if i < 2 { 0.0 } else { 0.5 };

            let mut viewport = Viewport::new(i as u32)
                .with_view_type(view_types[i])
                .with_normalized_rect(x, y, 0.5, 0.5);
            viewport.update_pixel_coords(self.parent_width, self.parent_height);
            self.viewports.push(viewport);
        }

        self.active_viewport = Some(1); // Perspective is default
    }

    /// Update layout after resize
    fn update_layout(&mut self) {
        for viewport in &mut self.viewports {
            viewport.update_pixel_coords(self.parent_width, self.parent_height);
        }
    }

    /// Get viewport by ID
    pub fn get_viewport(&self, id: u32) -> Option<&Viewport> {
        self.viewports.iter().find(|v| v.id == id)
    }

    /// Get mutable viewport by ID
    pub fn get_viewport_mut(&mut self, id: u32) -> Option<&mut Viewport> {
        self.viewports.iter_mut().find(|v| v.id == id)
    }

    /// Get active viewport
    pub fn active_viewport(&self) -> Option<&Viewport> {
        self.active_viewport.and_then(|id| self.get_viewport(id))
    }

    /// Get mutable active viewport
    pub fn active_viewport_mut(&mut self) -> Option<&mut Viewport> {
        self.active_viewport
            .and_then(move |id| self.get_viewport_mut(id))
    }

    /// Set active viewport
    pub fn set_active_viewport(&mut self, id: u32) {
        if self.get_viewport(id).is_some() {
            // Deactivate previous
            if let Some(prev_id) = self.active_viewport {
                if let Some(vp) = self.get_viewport_mut(prev_id) {
                    vp.set_active(false);
                }
            }
            // Activate new
            self.active_viewport = Some(id);
            if let Some(vp) = self.get_viewport_mut(id) {
                vp.set_active(true);
            }
        }
    }

    /// Get viewport at screen position
    pub fn viewport_at(&self, x: i32, y: i32) -> Option<u32> {
        self.viewports
            .iter()
            .find(|v| v.contains_point(x, y))
            .map(|v| v.id)
    }

    /// Set active viewport at position
    pub fn set_active_at_position(&mut self, x: i32, y: i32) {
        if let Some(id) = self.viewport_at(x, y) {
            self.set_active_viewport(id);
        }
    }

    /// Get all viewports
    pub fn viewports(&self) -> &[Viewport] {
        &self.viewports
    }

    /// Get mutable viewports
    pub fn viewports_mut(&mut self) -> &mut [Viewport] {
        &mut self.viewports
    }

    /// Viewport count
    pub fn viewport_count(&self) -> usize {
        self.viewports.len()
    }

    /// Reset all views
    pub fn reset_all_views(&mut self) {
        for viewport in &mut self.viewports {
            viewport.reset_view();
        }
    }

    /// Zoom fit all
    pub fn zoom_fit_all(&mut self, min: &Point, max: &Point) {
        for viewport in &mut self.viewports {
            viewport.fit_to_box(min, max);
        }
    }
}

/// Display mode configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMode {
    /// Mode name
    pub name: String,
    /// Render mode
    pub render_mode: RenderMode,
    /// Show edges
    pub show_edges: bool,
    /// Edge color
    pub edge_color: Color,
    /// Edge width
    pub edge_width: f32,
    /// Show vertices
    pub show_vertices: bool,
    /// Vertex color
    pub vertex_color: Color,
    /// Vertex size
    pub vertex_size: f32,
    /// Show faces
    pub show_faces: bool,
    /// Lighting enabled
    pub lighting: bool,
    /// Shadows enabled
    pub shadows: bool,
    /// Anti-aliasing
    pub anti_aliasing: bool,
    /// Culling enabled
    pub culling: bool,
}

impl DisplayMode {
    /// Create shaded display mode
    pub fn shaded() -> Self {
        Self {
            name: "Shaded".to_string(),
            render_mode: RenderMode::Shaded,
            show_edges: false,
            edge_color: Color::black(),
            edge_width: 1.0,
            show_vertices: false,
            vertex_color: Color::black(),
            vertex_size: 3.0,
            show_faces: true,
            lighting: true,
            shadows: true,
            anti_aliasing: true,
            culling: true,
        }
    }

    /// Create wireframe display mode
    pub fn wireframe() -> Self {
        Self {
            name: "Wireframe".to_string(),
            render_mode: RenderMode::Wireframe,
            show_edges: true,
            edge_color: Color::white(),
            edge_width: 1.0,
            show_vertices: true,
            vertex_color: Color::white(),
            vertex_size: 3.0,
            show_faces: false,
            lighting: false,
            shadows: false,
            anti_aliasing: true,
            culling: false,
        }
    }

    /// Create hidden line display mode
    pub fn hidden_line() -> Self {
        Self {
            name: "Hidden Line".to_string(),
            render_mode: RenderMode::HiddenLine,
            show_edges: true,
            edge_color: Color::black(),
            edge_width: 1.0,
            show_vertices: false,
            vertex_color: Color::black(),
            vertex_size: 3.0,
            show_faces: true,
            lighting: false,
            shadows: false,
            anti_aliasing: true,
            culling: true,
        }
    }

    /// Create points display mode
    pub fn points() -> Self {
        Self {
            name: "Points".to_string(),
            render_mode: RenderMode::Points,
            show_edges: false,
            edge_color: Color::white(),
            edge_width: 1.0,
            show_vertices: true,
            vertex_color: Color::white(),
            vertex_size: 3.0,
            show_faces: false,
            lighting: false,
            shadows: false,
            anti_aliasing: true,
            culling: false,
        }
    }

    /// Create wireframe with shaded display mode
    pub fn wireframe_shaded() -> Self {
        Self {
            name: "Wireframe + Shaded".to_string(),
            render_mode: RenderMode::WireframeShaded,
            show_edges: true,
            edge_color: Color::black(),
            edge_width: 1.0,
            show_vertices: false,
            vertex_color: Color::black(),
            vertex_size: 3.0,
            show_faces: true,
            lighting: true,
            shadows: true,
            anti_aliasing: true,
            culling: true,
        }
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::shaded()
    }
}

/// View manipulation tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewTool {
    /// No tool
    None,
    /// Pan tool
    Pan,
    /// Zoom tool
    Zoom,
    /// Rotate/Orbit tool
    Rotate,
    /// Window zoom
    WindowZoom,
    /// Fit all
    FitAll,
}

impl Default for ViewTool {
    fn default() -> Self {
        ViewTool::None
    }
}

/// View controller for handling view operations
#[derive(Debug, Clone, Default)]
pub struct ViewController {
    /// Current tool
    pub current_tool: ViewTool,
    /// Previous tool (for toggle)
    pub previous_tool: ViewTool,
    /// Display mode
    pub display_mode: DisplayMode,
    /// Standard views available
    pub standard_views: Vec<ViewType>,
}

impl ViewController {
    /// Create a new view controller
    pub fn new() -> Self {
        Self {
            current_tool: ViewTool::None,
            previous_tool: ViewTool::None,
            display_mode: DisplayMode::shaded(),
            standard_views: vec![
                ViewType::Perspective,
                ViewType::Front,
                ViewType::Back,
                ViewType::Top,
                ViewType::Bottom,
                ViewType::Left,
                ViewType::Right,
                ViewType::Isometric,
            ],
        }
    }

    /// Set current tool
    pub fn set_tool(&mut self, tool: ViewTool) {
        self.previous_tool = self.current_tool;
        self.current_tool = tool;
    }

    /// Toggle between current and previous tool
    pub fn toggle_tool(&mut self) {
        std::mem::swap(&mut self.current_tool, &mut self.previous_tool);
    }

    /// Set display mode
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        self.display_mode = mode;
    }

    /// Cycle display modes
    pub fn cycle_display_mode(&mut self) {
        let modes = [
            DisplayMode::shaded(),
            DisplayMode::wireframe(),
            DisplayMode::wireframe_shaded(),
            DisplayMode::hidden_line(),
            DisplayMode::points(),
        ];

        let current_idx = modes.iter().position(|m| m.name == self.display_mode.name);
        let next_idx = current_idx.map_or(0, |i| (i + 1) % modes.len());
        self.display_mode = modes[next_idx].clone();
    }

    /// Get standard view by index
    pub fn get_standard_view(&self, index: usize) -> Option<ViewType> {
        self.standard_views.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_creation() {
        let viewport = Viewport::new(0)
            .with_view_type(ViewType::Perspective)
            .with_pixel_rect(0, 0, 800, 600);

        assert_eq!(viewport.id, 0);
        assert_eq!(viewport.view_type, ViewType::Perspective);
        assert_eq!(viewport.pixel_width, 800);
        assert_eq!(viewport.pixel_height, 600);
    }

    #[test]
    fn test_viewport_contains_point() {
        let viewport = Viewport::new(0).with_pixel_rect(100, 100, 200, 200);

        assert!(viewport.contains_point(150, 150));
        assert!(!viewport.contains_point(50, 50));
        assert!(!viewport.contains_point(350, 350));
    }

    #[test]
    fn test_viewport_screen_to_ndc() {
        let viewport = Viewport::new(0).with_pixel_rect(0, 0, 100, 100);

        let ndc = viewport.screen_to_ndc(50, 50);
        assert!((ndc[0]).abs() < 0.01);
        assert!((ndc[1]).abs() < 0.01);
    }

    #[test]
    fn test_view_type() {
        assert!(ViewType::Front.is_orthographic());
        assert!(!ViewType::Perspective.is_orthographic());
        assert_eq!(ViewType::Top.name(), "Top");
    }

    #[test]
    fn test_view_manager() {
        let mut manager = ViewManager::new();
        manager.set_parent_size(800, 600);

        assert_eq!(manager.viewport_count(), 1);

        manager.setup_quad_view();
        assert_eq!(manager.viewport_count(), 4);

        assert!(manager.active_viewport().is_some());
    }

    #[test]
    fn test_display_mode() {
        let shaded = DisplayMode::shaded();
        assert_eq!(shaded.name, "Shaded");
        assert!(shaded.show_faces);

        let wireframe = DisplayMode::wireframe();
        assert_eq!(wireframe.name, "Wireframe");
        assert!(!wireframe.show_faces);
    }

    #[test]
    fn test_view_controller() {
        let mut controller = ViewController::new();
        assert_eq!(controller.current_tool, ViewTool::None);

        controller.set_tool(ViewTool::Pan);
        assert_eq!(controller.current_tool, ViewTool::Pan);

        controller.set_tool(ViewTool::Zoom);
        controller.toggle_tool();
        assert_eq!(controller.current_tool, ViewTool::Pan);
    }
}
