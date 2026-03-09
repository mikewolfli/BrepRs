//! Interactive objects for 3D visualization
//!
//! This module provides interactive object functionality for 3D visualization,
//! including selection, highlighting, and manipulation handles.
//! Compatible with OpenCASCADE Open API design.

use crate::geometry::{Direction, Point, Transform, Vector};
use crate::visualization::primitives::*;

/// Interactive object state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractiveState {
    /// Normal state
    Normal,
    /// Hovered (mouse over)
    Hovered,
    /// Selected
    Selected,
    /// Preselected (highlighted)
    Preselected,
    /// Active (being manipulated)
    Active,
    /// Disabled
    Disabled,
}

impl Default for InteractiveState {
    fn default() -> Self {
        InteractiveState::Normal
    }
}

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single selection
    Single,
    /// Multiple selection (toggle)
    Multiple,
    /// Rectangle selection
    Rectangle,
    /// Lasso selection
    Lasso,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Single
    }
}

/// Interactive object trait
pub trait InteractiveObject {
    /// Get current state
    fn state(&self) -> InteractiveState;
    /// Set state
    fn set_state(&mut self, state: InteractiveState);
    /// Check if selectable
    fn is_selectable(&self) -> bool;
    /// Set selectable
    fn set_selectable(&mut self, selectable: bool);
    /// Check if visible
    fn is_visible(&self) -> bool;
    /// Set visible
    fn set_visible(&mut self, visible: bool);
    /// Get bounding box
    fn bounding_box(&self) -> ([f32; 3], [f32; 3]);
    /// Check if point is inside (for picking)
    fn contains_point(&self, point: &Point) -> bool;
    /// Get pick priority (higher = picked first)
    fn pick_priority(&self) -> i32;
}

/// Selection set for managing selected objects
#[derive(Debug, Clone, Default)]
pub struct SelectionSet {
    /// Selected object IDs
    selected: Vec<u64>,
    /// Primary selection (last selected)
    primary: Option<u64>,
    /// Selection mode
    mode: SelectionMode,
}

impl SelectionSet {
    /// Create a new selection set
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
            primary: None,
            mode: SelectionMode::Single,
        }
    }

    /// Set selection mode
    pub fn with_mode(mut self, mode: SelectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Select an object
    pub fn select(&mut self, id: u64) {
        match self.mode {
            SelectionMode::Single => {
                self.selected.clear();
                self.selected.push(id);
                self.primary = Some(id);
            }
            SelectionMode::Multiple => {
                if !self.selected.contains(&id) {
                    self.selected.push(id);
                    self.primary = Some(id);
                }
            }
            _ => {
                // Rectangle and lasso modes handled separately
            }
        }
    }

    /// Deselect an object
    pub fn deselect(&mut self, id: u64) {
        if let Some(pos) = self.selected.iter().position(|&x| x == id) {
            self.selected.remove(pos);
            if self.primary == Some(id) {
                self.primary = self.selected.last().copied();
            }
        }
    }

    /// Toggle selection
    pub fn toggle(&mut self, id: u64) {
        if self.is_selected(id) {
            self.deselect(id);
        } else {
            self.select(id);
        }
    }

    /// Check if object is selected
    pub fn is_selected(&self, id: u64) -> bool {
        self.selected.contains(&id)
    }

    /// Clear selection
    pub fn clear(&mut self) {
        self.selected.clear();
        self.primary = None;
    }

    /// Get selected count
    pub fn count(&self) -> usize {
        self.selected.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// Get selected IDs
    pub fn selected(&self) -> &[u64] {
        &self.selected
    }

    /// Get primary selection
    pub fn primary(&self) -> Option<u64> {
        self.primary
    }

    /// Set primary selection
    pub fn set_primary(&mut self, id: u64) {
        if self.selected.contains(&id) {
            self.primary = Some(id);
        }
    }

    /// Select all from list
    pub fn select_all(&mut self, ids: &[u64]) {
        for &id in ids {
            if !self.selected.contains(&id) {
                self.selected.push(id);
            }
        }
        if !self.selected.is_empty() && self.primary.is_none() {
            self.primary = Some(self.selected[0]);
        }
    }

    /// Invert selection
    pub fn invert_selection(&mut self, all_ids: &[u64]) {
        let new_selection: Vec<u64> = all_ids
            .iter()
            .copied()
            .filter(|id| !self.selected.contains(id))
            .collect();
        self.selected = new_selection;
        self.primary = self.selected.first().copied();
    }
}

/// Pick result from ray casting
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PickResult {
    /// Object ID
    pub object_id: u64,
    /// Distance from ray origin
    pub distance: f64,
    /// Hit point
    pub point: Point,
    /// Normal at hit point
    pub normal: Vector,
    /// UV coordinates
    pub uv: [f32; 2],
}

impl PickResult {
    /// Create a new pick result
    pub fn new(object_id: u64, distance: f64, point: Point, normal: Vector) -> Self {
        Self {
            object_id,
            distance,
            point,
            normal,
            uv: [0.0, 0.0],
        }
    }

    /// Set UV coordinates
    pub fn with_uv(mut self, u: f32, v: f32) -> Self {
        self.uv = [u, v];
        self
    }
}

/// Ray for picking
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PickRay {
    /// Ray origin
    pub origin: Point,
    /// Ray direction (normalized)
    pub direction: Vector,
}

impl PickRay {
    /// Create a new pick ray
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    /// Get point at distance
    pub fn point_at(&self, distance: f64) -> Point {
        Point::new(
            self.origin.x + self.direction.x * distance,
            self.origin.y + self.direction.y * distance,
            self.origin.z + self.direction.z * distance,
        )
    }

    /// Transform ray by transformation
    pub fn transform(&self, transform: &Transform) -> Self {
        let new_origin = transform.transforms(&self.origin);
        let new_direction = transform.transforms_vec(&self.direction);
        Self::new(new_origin, new_direction)
    }
}

/// Manipulation mode for interactive handles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManipulationMode {
    /// No manipulation
    None,
    /// Translation
    Translate,
    /// Rotation
    Rotate,
    /// Scale
    Scale,
    /// Universal (combined)
    Universal,
}

impl Default for ManipulationMode {
    fn default() -> Self {
        ManipulationMode::None
    }
}

/// Manipulation space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManipulationSpace {
    /// World space
    World,
    /// Local/object space
    Local,
    /// View/camera space
    View,
}

impl Default for ManipulationSpace {
    fn default() -> Self {
        ManipulationSpace::Local
    }
}

/// Interactive manipulation handle
#[derive(Debug, Clone)]
pub struct Manipulator {
    /// Manipulation mode
    pub mode: ManipulationMode,
    /// Manipulation space
    pub space: ManipulationSpace,
    /// Handle position
    pub position: Point,
    /// Handle orientation
    pub orientation: Transform,
    /// Handle size
    pub size: f64,
    /// Active axis (0=X, 1=Y, 2=Z, -1=none)
    pub active_axis: i32,
    /// Active plane (0=YZ, 1=XZ, 2=XY, -1=none)
    pub active_plane: i32,
    /// Visible flag
    pub visible: bool,
    /// Enabled flag
    pub enabled: bool,
}

impl Manipulator {
    /// Create a new manipulator
    pub fn new(mode: ManipulationMode) -> Self {
        Self {
            mode,
            space: ManipulationSpace::Local,
            position: Point::new(0.0, 0.0, 0.0),
            orientation: Transform::identity(),
            size: 1.0,
            active_axis: -1,
            active_plane: -1,
            visible: true,
            enabled: true,
        }
    }

    /// Set position
    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    /// Set orientation
    pub fn with_orientation(mut self, orientation: Transform) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set size
    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Set space
    pub fn with_space(mut self, space: ManipulationSpace) -> Self {
        self.space = space;
        self
    }

    /// Set visible
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get X axis direction
    pub fn x_axis(&self) -> Vector {
        match self.space {
            ManipulationSpace::Local => {
                let dir = Direction::new(1.0, 0.0, 0.0);
                let transformed = self.orientation.transforms_dir(&dir);
                Vector::new(transformed.x, transformed.y, transformed.z)
            }
            ManipulationSpace::World => Vector::new(1.0, 0.0, 0.0),
            ManipulationSpace::View => Vector::new(1.0, 0.0, 0.0), // Simplified
        }
    }

    /// Get Y axis direction
    pub fn y_axis(&self) -> Vector {
        match self.space {
            ManipulationSpace::Local => {
                let dir = Direction::new(0.0, 1.0, 0.0);
                let transformed = self.orientation.transforms_dir(&dir);
                Vector::new(transformed.x, transformed.y, transformed.z)
            }
            ManipulationSpace::World => Vector::new(0.0, 1.0, 0.0),
            ManipulationSpace::View => Vector::new(0.0, 1.0, 0.0),
        }
    }

    /// Get Z axis direction
    pub fn z_axis(&self) -> Vector {
        match self.space {
            ManipulationSpace::Local => {
                let dir = Direction::new(0.0, 0.0, 1.0);
                let transformed = self.orientation.transforms_dir(&dir);
                Vector::new(transformed.x, transformed.y, transformed.z)
            }
            ManipulationSpace::World => Vector::new(0.0, 0.0, 1.0),
            ManipulationSpace::View => Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// Set active axis
    pub fn set_active_axis(&mut self, axis: i32) {
        self.active_axis = axis;
        if axis >= 0 {
            self.active_plane = -1;
        }
    }

    /// Set active plane
    pub fn set_active_plane(&mut self, plane: i32) {
        self.active_plane = plane;
        if plane >= 0 {
            self.active_axis = -1;
        }
    }

    /// Clear active
    pub fn clear_active(&mut self) {
        self.active_axis = -1;
        self.active_plane = -1;
    }

    /// Check if any axis is active
    pub fn has_active_axis(&self) -> bool {
        self.active_axis >= 0
    }

    /// Check if any plane is active
    pub fn has_active_plane(&self) -> bool {
        self.active_plane >= 0
    }

    /// Generate geometry for the manipulator
    pub fn generate_geometry(&self) -> Vec<Line> {
        let mut lines = Vec::new();

        if !self.visible || !self.enabled {
            return lines;
        }

        let pos = [
            self.position.x as f32,
            self.position.y as f32,
            self.position.z as f32,
        ];
        let size = self.size as f32;

        match self.mode {
            ManipulationMode::Translate => {
                // X axis (red)
                let x_color = if self.active_axis == 0 {
                    Color::yellow()
                } else {
                    Color::red()
                };
                let x_dir = self.x_axis();
                lines.push(Line::new(
                    pos,
                    [
                        pos[0] + x_dir.x as f32 * size,
                        pos[1] + x_dir.y as f32 * size,
                        pos[2] + x_dir.z as f32 * size,
                    ],
                    x_color,
                    2.0,
                ));

                // Y axis (green)
                let y_color = if self.active_axis == 1 {
                    Color::yellow()
                } else {
                    Color::green()
                };
                let y_dir = self.y_axis();
                lines.push(Line::new(
                    pos,
                    [
                        pos[0] + y_dir.x as f32 * size,
                        pos[1] + y_dir.y as f32 * size,
                        pos[2] + y_dir.z as f32 * size,
                    ],
                    y_color,
                    2.0,
                ));

                // Z axis (blue)
                let z_color = if self.active_axis == 2 {
                    Color::yellow()
                } else {
                    Color::blue()
                };
                let z_dir = self.z_axis();
                lines.push(Line::new(
                    pos,
                    [
                        pos[0] + z_dir.x as f32 * size,
                        pos[1] + z_dir.y as f32 * size,
                        pos[2] + z_dir.z as f32 * size,
                    ],
                    z_color,
                    2.0,
                ));
            }
            ManipulationMode::Rotate => {
                // Simplified rotation circles
                let segments = 32;
                let radius = size * 0.5;

                // X axis circle (red)
                let x_color = if self.active_axis == 0 {
                    Color::yellow()
                } else {
                    Color::red()
                };
                lines.extend(self.generate_circle(
                    &self.y_axis(),
                    &self.z_axis(),
                    radius,
                    x_color,
                    segments,
                ));

                // Y axis circle (green)
                let y_color = if self.active_axis == 1 {
                    Color::yellow()
                } else {
                    Color::green()
                };
                lines.extend(self.generate_circle(
                    &self.x_axis(),
                    &self.z_axis(),
                    radius,
                    y_color,
                    segments,
                ));

                // Z axis circle (blue)
                let z_color = if self.active_axis == 2 {
                    Color::yellow()
                } else {
                    Color::blue()
                };
                lines.extend(self.generate_circle(
                    &self.x_axis(),
                    &self.y_axis(),
                    radius,
                    z_color,
                    segments,
                ));
            }
            ManipulationMode::Scale => {
                // Similar to translate but with box at ends
                let box_size = size * 0.1;

                // X axis
                let x_color = if self.active_axis == 0 {
                    Color::yellow()
                } else {
                    Color::red()
                };
                let x_dir = self.x_axis();
                let x_end = [
                    pos[0] + x_dir.x as f32 * size,
                    pos[1] + x_dir.y as f32 * size,
                    pos[2] + x_dir.z as f32 * size,
                ];
                lines.push(Line::new(pos, x_end, x_color, 2.0));
                lines.extend(self.generate_box(x_end, box_size, x_color));

                // Y axis
                let y_color = if self.active_axis == 1 {
                    Color::yellow()
                } else {
                    Color::green()
                };
                let y_dir = self.y_axis();
                let y_end = [
                    pos[0] + y_dir.x as f32 * size,
                    pos[1] + y_dir.y as f32 * size,
                    pos[2] + y_dir.z as f32 * size,
                ];
                lines.push(Line::new(pos, y_end, y_color, 2.0));
                lines.extend(self.generate_box(y_end, box_size, y_color));

                // Z axis
                let z_color = if self.active_axis == 2 {
                    Color::yellow()
                } else {
                    Color::blue()
                };
                let z_dir = self.z_axis();
                let z_end = [
                    pos[0] + z_dir.x as f32 * size,
                    pos[1] + z_dir.y as f32 * size,
                    pos[2] + z_dir.z as f32 * size,
                ];
                lines.push(Line::new(pos, z_end, z_color, 2.0));
                lines.extend(self.generate_box(z_end, box_size, z_color));
            }
            _ => {}
        }

        lines
    }

    /// Generate circle lines
    fn generate_circle(
        &self,
        axis1: &Vector,
        axis2: &Vector,
        radius: f32,
        color: Color,
        segments: u32,
    ) -> Vec<Line> {
        let mut lines = Vec::new();
        let pos = [
            self.position.x as f32,
            self.position.y as f32,
            self.position.z as f32,
        ];

        let mut prev_point = [
            pos[0] + axis1.x as f32 * radius,
            pos[1] + axis1.y as f32 * radius,
            pos[2] + axis1.z as f32 * radius,
        ];

        for i in 1..=segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let point = [
                pos[0] + (axis1.x as f32 * cos_a + axis2.x as f32 * sin_a) * radius,
                pos[1] + (axis1.y as f32 * cos_a + axis2.y as f32 * sin_a) * radius,
                pos[2] + (axis1.z as f32 * cos_a + axis2.z as f32 * sin_a) * radius,
            ];

            lines.push(Line::new(prev_point, point, color, 1.5));
            prev_point = point;
        }

        lines
    }

    /// Generate box wireframe
    fn generate_box(&self, center: [f32; 3], size: f32, color: Color) -> Vec<Line> {
        let mut lines = Vec::new();
        let hs = size * 0.5;

        // Box corners
        let corners = [
            [center[0] - hs, center[1] - hs, center[2] - hs],
            [center[0] + hs, center[1] - hs, center[2] - hs],
            [center[0] + hs, center[1] + hs, center[2] - hs],
            [center[0] - hs, center[1] + hs, center[2] - hs],
            [center[0] - hs, center[1] - hs, center[2] + hs],
            [center[0] + hs, center[1] - hs, center[2] + hs],
            [center[0] + hs, center[1] + hs, center[2] + hs],
            [center[0] - hs, center[1] + hs, center[2] + hs],
        ];

        // Bottom face
        lines.push(Line::new(corners[0], corners[1], color, 1.0));
        lines.push(Line::new(corners[1], corners[2], color, 1.0));
        lines.push(Line::new(corners[2], corners[3], color, 1.0));
        lines.push(Line::new(corners[3], corners[0], color, 1.0));

        // Top face
        lines.push(Line::new(corners[4], corners[5], color, 1.0));
        lines.push(Line::new(corners[5], corners[6], color, 1.0));
        lines.push(Line::new(corners[6], corners[7], color, 1.0));
        lines.push(Line::new(corners[7], corners[4], color, 1.0));

        // Vertical edges
        lines.push(Line::new(corners[0], corners[4], color, 1.0));
        lines.push(Line::new(corners[1], corners[5], color, 1.0));
        lines.push(Line::new(corners[2], corners[6], color, 1.0));
        lines.push(Line::new(corners[3], corners[7], color, 1.0));

        lines
    }
}

impl Default for Manipulator {
    fn default() -> Self {
        Self::new(ManipulationMode::Translate)
    }
}

/// Interactive context for managing interactions
#[derive(Debug, Clone, Default)]
pub struct InteractiveContext {
    /// Current selection
    pub selection: SelectionSet,
    /// Current manipulator
    pub manipulator: Option<Manipulator>,
    /// Hover state
    pub hover_id: Option<u64>,
    /// Preselection state
    pub preselect_id: Option<u64>,
    /// Picking enabled
    pub picking_enabled: bool,
    /// Manipulation enabled
    pub manipulation_enabled: bool,
}

impl InteractiveContext {
    /// Create a new interactive context
    pub fn new() -> Self {
        Self {
            selection: SelectionSet::new(),
            manipulator: None,
            hover_id: None,
            preselect_id: None,
            picking_enabled: true,
            manipulation_enabled: true,
        }
    }

    /// Set hover
    pub fn set_hover(&mut self, id: Option<u64>) {
        self.hover_id = id;
    }

    /// Set preselection
    pub fn set_preselect(&mut self, id: Option<u64>) {
        self.preselect_id = id;
    }

    /// Show manipulator
    pub fn show_manipulator(&mut self, mode: ManipulationMode, position: Point) {
        self.manipulator = Some(Manipulator::new(mode).with_position(position));
    }

    /// Hide manipulator
    pub fn hide_manipulator(&mut self) {
        self.manipulator = None;
    }

    /// Check if manipulator is visible
    pub fn is_manipulator_visible(&self) -> bool {
        self.manipulator.as_ref().map_or(false, |m| m.visible)
    }

    /// Update manipulator position
    pub fn update_manipulator_position(&mut self, position: Point) {
        if let Some(ref mut manip) = self.manipulator {
            manip.position = position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_set() {
        let mut selection = SelectionSet::new();
        selection.select(1);
        selection.select(2);
        assert_eq!(selection.count(), 1); // Single mode replaces

        let mut multi = SelectionSet::new().with_mode(SelectionMode::Multiple);
        multi.select(1);
        multi.select(2);
        assert_eq!(multi.count(), 2);
        assert!(multi.is_selected(1));
        assert!(multi.is_selected(2));
    }

    #[test]
    fn test_pick_ray() {
        let ray = PickRay::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let point = ray.point_at(5.0);
        assert_eq!(point.z, 5.0);
    }

    #[test]
    fn test_manipulator() {
        let mut manip = Manipulator::new(ManipulationMode::Translate);
        assert_eq!(manip.mode, ManipulationMode::Translate);

        manip.set_active_axis(0);
        assert!(manip.has_active_axis());
        assert_eq!(manip.active_axis, 0);

        manip.clear_active();
        assert!(!manip.has_active_axis());
    }

    #[test]
    fn test_manipulator_geometry() {
        let manip = Manipulator::new(ManipulationMode::Translate)
            .with_position(Point::new(0.0, 0.0, 0.0))
            .with_size(1.0);

        let lines = manip.generate_geometry();
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_interactive_context() {
        let mut ctx = InteractiveContext::new();
        assert!(ctx.manipulator.is_none());

        ctx.show_manipulator(ManipulationMode::Rotate, Point::new(1.0, 2.0, 3.0));
        assert!(ctx.manipulator.is_some());
        assert!(ctx.is_manipulator_visible());

        ctx.hide_manipulator();
        assert!(!ctx.is_manipulator_visible());
    }
}
