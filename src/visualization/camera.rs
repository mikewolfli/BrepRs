//! Camera for 3D visualization
//!
//! This module provides camera functionality for 3D visualization,
//! including perspective and orthographic projections.
//! Compatible with OpenCASCADE Open API design.

use crate::geometry::{Point, Vector};

/// Camera projection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionType {
    /// Perspective projection
    Perspective,
    /// Orthographic projection
    Orthographic,
}

impl Default for ProjectionType {
    fn default() -> Self {
        ProjectionType::Perspective
    }
}

/// Camera for 3D visualization
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    /// Camera position
    pub position: Point,
    /// Camera target (look-at point)
    pub target: Point,
    /// Up vector
    pub up: Vector,
    /// Projection type
    pub projection: ProjectionType,
    /// Field of view in degrees (for perspective)
    pub fov: f32,
    /// Near clipping plane
    pub near_plane: f32,
    /// Far clipping plane
    pub far_plane: f32,
    /// Orthographic scale (for orthographic projection)
    pub ortho_scale: f32,
}

impl Camera {
    /// Create a new camera
    pub fn new() -> Self {
        Self {
            position: Point::new(0.0, 0.0, 10.0),
            target: Point::new(0.0, 0.0, 0.0),
            up: Vector::new(0.0, 1.0, 0.0),
            projection: ProjectionType::Perspective,
            fov: 45.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            ortho_scale: 10.0,
        }
    }

    /// Create camera with position and target
    pub fn look_at(position: Point, target: Point, up: Vector) -> Self {
        Self {
            position,
            target,
            up,
            ..Default::default()
        }
    }

    /// Set projection type
    pub fn with_projection(mut self, projection: ProjectionType) -> Self {
        self.projection = projection;
        self
    }

    /// Set field of view
    pub fn with_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    /// Set clipping planes
    pub fn with_clipping_planes(mut self, near: f32, far: f32) -> Self {
        self.near_plane = near;
        self.far_plane = far;
        self
    }

    /// Set orthographic scale
    pub fn with_ortho_scale(mut self, scale: f32) -> Self {
        self.ortho_scale = scale;
        self
    }

    /// Get view direction vector
    pub fn direction(&self) -> Vector {
        Vector::new(
            self.target.x - self.position.x,
            self.target.y - self.position.y,
            self.target.z - self.position.z,
        )
        .normalized()
    }

    /// Get right vector
    pub fn right(&self) -> Vector {
        let dir = self.direction();
        let up = self.up.normalized();
        Vector::new(
            dir.y * up.z - dir.z * up.y,
            dir.z * up.x - dir.x * up.z,
            dir.x * up.y - dir.y * up.x,
        )
        .normalized()
    }

    /// Get up vector (recalculated to be orthogonal to direction and right)
    pub fn up_corrected(&self) -> Vector {
        let dir = self.direction();
        let right = self.right();
        Vector::new(
            right.y * dir.z - right.z * dir.y,
            right.z * dir.x - right.x * dir.z,
            right.x * dir.y - right.y * dir.x,
        )
        .normalized()
    }

    /// Get view matrix (4x4 column-major)
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let dir = self.direction();
        let right = self.right();
        let up = self.up_corrected();

        let eye = [self.position.x, self.position.y, self.position.z];

        // View matrix (look-at)
        [
            [right.x as f32, up.x as f32, -dir.x as f32, 0.0],
            [right.y as f32, up.y as f32, -dir.y as f32, 0.0],
            [right.z as f32, up.z as f32, -dir.z as f32, 0.0],
            [
                -(right.x * eye[0] + right.y * eye[1] + right.z * eye[2]) as f32,
                -(up.x * eye[0] + up.y * eye[1] + up.z * eye[2]) as f32,
                (dir.x * eye[0] + dir.y * eye[1] + dir.z * eye[2]) as f32,
                1.0,
            ],
        ]
    }

    /// Get projection matrix
    pub fn projection_matrix(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        match self.projection {
            ProjectionType::Perspective => {
                let fov_rad = self.fov.to_radians();
                let f = 1.0 / (fov_rad / 2.0).tan();
                let nf = 1.0 / (self.near_plane - self.far_plane);

                [
                    [f / aspect_ratio, 0.0, 0.0, 0.0],
                    [0.0, f, 0.0, 0.0],
                    [0.0, 0.0, (self.far_plane + self.near_plane) * nf, -1.0],
                    [0.0, 0.0, 2.0 * self.far_plane * self.near_plane * nf, 0.0],
                ]
            }
            ProjectionType::Orthographic => {
                let r = self.ortho_scale * aspect_ratio;
                let t = self.ortho_scale;
                let f = self.far_plane;
                let n = self.near_plane;

                [
                    [1.0 / r, 0.0, 0.0, 0.0],
                    [0.0, 1.0 / t, 0.0, 0.0],
                    [0.0, 0.0, -2.0 / (f - n), 0.0],
                    [0.0, 0.0, -(f + n) / (f - n), 1.0],
                ]
            }
        }
    }

    /// Get combined view-projection matrix
    pub fn view_projection_matrix(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        let view = self.view_matrix();
        let proj = self.projection_matrix(aspect_ratio);
        multiply_matrices(&proj, &view)
    }

    /// Move camera forward/backward
    pub fn move_forward(&mut self, distance: f32) {
        let dir = self.direction();
        self.position.x += dir.x * distance as f64;
        self.position.y += dir.y * distance as f64;
        self.position.z += dir.z * distance as f64;
        self.target.x += dir.x * distance as f64;
        self.target.y += dir.y * distance as f64;
        self.target.z += dir.z * distance as f64;
    }

    /// Move camera right/left
    pub fn move_right(&mut self, distance: f32) {
        let right = self.right();
        self.position.x += right.x * distance as f64;
        self.position.y += right.y * distance as f64;
        self.position.z += right.z * distance as f64;
        self.target.x += right.x * distance as f64;
        self.target.y += right.y * distance as f64;
        self.target.z += right.z * distance as f64;
    }

    /// Move camera up/down
    pub fn move_up(&mut self, distance: f32) {
        let up = self.up_corrected();
        self.position.x += up.x * distance as f64;
        self.position.y += up.y * distance as f64;
        self.position.z += up.z * distance as f64;
        self.target.x += up.x * distance as f64;
        self.target.y += up.y * distance as f64;
        self.target.z += up.z * distance as f64;
    }

    /// Orbit around target (horizontal)
    pub fn orbit_horizontal(&mut self, angle_degrees: f32) {
        let angle = angle_degrees.to_radians() as f64;
        let offset = Vector::new(
            self.position.x - self.target.x,
            self.position.y - self.target.y,
            self.position.z - self.target.z,
        );

        let cos_a = angle.cos();
        let _sin_a = angle.sin();
        let right = self.right();

        // Rotate offset around up axis
        let new_offset = Vector::new(
            offset.x * cos_a
                + (right.x * offset.x + right.y * offset.y + right.z * offset.z)
                    * right.x
                    * (1.0 - cos_a),
            offset.y * cos_a
                + (right.x * offset.x + right.y * offset.y + right.z * offset.z)
                    * right.y
                    * (1.0 - cos_a),
            offset.z * cos_a
                + (right.x * offset.x + right.y * offset.y + right.z * offset.z)
                    * right.z
                    * (1.0 - cos_a),
        );

        self.position = Point::new(
            self.target.x + new_offset.x,
            self.target.y + new_offset.y,
            self.target.z + new_offset.z,
        );
    }

    /// Orbit around target (vertical)
    pub fn orbit_vertical(&mut self, angle_degrees: f32) {
        let angle = angle_degrees.to_radians() as f64;
        let offset = Vector::new(
            self.position.x - self.target.x,
            self.position.y - self.target.y,
            self.position.z - self.target.z,
        );

        let cos_a = angle.cos();
        let _sin_a = angle.sin();
        let up = self.up_corrected();

        // Rotate offset around right axis
        let new_offset = Vector::new(
            offset.x * cos_a
                + (up.x * offset.x + up.y * offset.y + up.z * offset.z) * up.x * (1.0 - cos_a),
            offset.y * cos_a
                + (up.x * offset.x + up.y * offset.y + up.z * offset.z) * up.y * (1.0 - cos_a),
            offset.z * cos_a
                + (up.x * offset.x + up.y * offset.y + up.z * offset.z) * up.z * (1.0 - cos_a),
        );

        self.position = Point::new(
            self.target.x + new_offset.x,
            self.target.y + new_offset.y,
            self.target.z + new_offset.z,
        );
    }

    /// Zoom (change FOV for perspective, scale for orthographic)
    pub fn zoom(&mut self, factor: f32) {
        match self.projection {
            ProjectionType::Perspective => {
                self.fov = (self.fov * factor).clamp(10.0, 120.0);
            }
            ProjectionType::Orthographic => {
                self.ortho_scale = (self.ortho_scale * factor).max(0.1);
            }
        }
    }

    /// Set position
    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    /// Set target
    pub fn set_target(&mut self, target: Point) {
        self.target = target;
    }

    /// Set up vector
    pub fn set_up(&mut self, up: Vector) {
        self.up = up;
    }

    /// Get distance to target
    pub fn distance_to_target(&self) -> f64 {
        let dx = self.target.x - self.position.x;
        let dy = self.target.y - self.position.y;
        let dz = self.target.z - self.position.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Fit view to bounding box
    pub fn fit_to_box(&mut self, min: &Point, max: &Point, aspect_ratio: f32) {
        let center = Point::new(
            (min.x + max.x) / 2.0,
            (min.y + max.y) / 2.0,
            (min.z + max.z) / 2.0,
        );

        let size = [
            (max.x - min.x).abs(),
            (max.y - min.y).abs(),
            (max.z - min.z).abs(),
        ];
        let max_size = size[0].max(size[1]).max(size[2]);

        self.target = center;

        match self.projection {
            ProjectionType::Perspective => {
                let distance = max_size / (self.fov.to_radians() / 2.0).tan() as f64;
                let dir = self.direction();
                self.position = Point::new(
                    center.x - dir.x * distance,
                    center.y - dir.y * distance,
                    center.z - dir.z * distance,
                );
            }
            ProjectionType::Orthographic => {
                self.ortho_scale = (max_size / aspect_ratio as f64) as f32;
                let dir = self.direction();
                let distance = max_size * 2.0;
                self.position = Point::new(
                    center.x - dir.x * distance,
                    center.y - dir.y * distance,
                    center.z - dir.z * distance,
                );
            }
        }
    }

    /// Create standard views
    pub fn front_view() -> Self {
        Self::look_at(
            Point::new(0.0, 0.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    pub fn back_view() -> Self {
        Self::look_at(
            Point::new(0.0, 0.0, -10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    pub fn left_view() -> Self {
        Self::look_at(
            Point::new(-10.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    pub fn right_view() -> Self {
        Self::look_at(
            Point::new(10.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    pub fn top_view() -> Self {
        Self::look_at(
            Point::new(0.0, 10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, -1.0),
        )
    }

    pub fn bottom_view() -> Self {
        Self::look_at(
            Point::new(0.0, -10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        )
    }

    pub fn isometric_view() -> Self {
        Self::look_at(
            Point::new(10.0, 10.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to multiply two 4x4 matrices
fn multiply_matrices(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

/// Camera controller for interactive manipulation
#[derive(Debug, Clone)]
pub struct CameraController {
    /// Camera being controlled
    pub camera: Camera,
    /// Mouse sensitivity
    pub sensitivity: f32,
    /// Movement speed
    pub speed: f32,
    /// Orbit mode flag
    pub orbit_mode: bool,
}

impl CameraController {
    /// Create a new camera controller
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            sensitivity: 0.1,
            speed: 1.0,
            orbit_mode: true,
        }
    }

    /// Set sensitivity
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Set speed
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Handle mouse movement for orbit
    pub fn mouse_orbit(&mut self, delta_x: f32, delta_y: f32) {
        if self.orbit_mode {
            self.camera.orbit_horizontal(delta_x * self.sensitivity);
            self.camera.orbit_vertical(delta_y * self.sensitivity);
        }
    }

    /// Handle mouse movement for look (FPS style)
    pub fn mouse_look(&mut self, delta_x: f32, delta_y: f32) {
        if !self.orbit_mode {
            // Implement FPS-style camera rotation
            let angle_x = delta_x * self.sensitivity;
            let angle_y = delta_y * self.sensitivity;

            let dir = self.camera.direction();
            let right = self.camera.right();
            let up = self.camera.up_corrected();

            // Rotate direction around up axis (horizontal)
            let cos_x = angle_x.to_radians().cos() as f64;
            let sin_x = angle_x.to_radians().sin() as f64;
            let new_dir = Vector::new(
                dir.x * cos_x + right.x * sin_x,
                dir.y * cos_x + right.y * sin_x,
                dir.z * cos_x + right.z * sin_x,
            );

            // Rotate direction around right axis (vertical)
            let cos_y = angle_y.to_radians().cos() as f64;
            let sin_y = angle_y.to_radians().sin() as f64;
            let new_dir = Vector::new(
                new_dir.x * cos_y + up.x * sin_y,
                new_dir.y * cos_y + up.y * sin_y,
                new_dir.z * cos_y + up.z * sin_y,
            );

            let distance = self.camera.distance_to_target();
            self.camera.target = Point::new(
                self.camera.position.x + new_dir.x * distance,
                self.camera.position.y + new_dir.y * distance,
                self.camera.position.z + new_dir.z * distance,
            );
        }
    }

    /// Handle keyboard input for movement
    pub fn move_forward(&mut self) {
        self.camera.move_forward(self.speed);
    }

    pub fn move_backward(&mut self) {
        self.camera.move_forward(-self.speed);
    }

    pub fn move_left(&mut self) {
        self.camera.move_right(-self.speed);
    }

    pub fn move_right(&mut self) {
        self.camera.move_right(self.speed);
    }

    pub fn move_up(&mut self) {
        self.camera.move_up(self.speed);
    }

    pub fn move_down(&mut self) {
        self.camera.move_up(-self.speed);
    }

    /// Handle zoom
    pub fn zoom(&mut self, factor: f32) {
        self.camera.zoom(factor);
    }

    /// Set orbit mode
    pub fn set_orbit_mode(&mut self, orbit: bool) {
        self.orbit_mode = orbit;
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(Camera::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new();
        assert_eq!(camera.projection, ProjectionType::Perspective);
        assert_eq!(camera.fov, 45.0);
    }

    #[test]
    fn test_camera_look_at() {
        let camera = Camera::look_at(
            Point::new(0.0, 0.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        assert_eq!(camera.position.x, 0.0);
        assert_eq!(camera.position.z, 10.0);
    }

    #[test]
    fn test_camera_direction() {
        let camera = Camera::look_at(
            Point::new(0.0, 0.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let dir = camera.direction();
        assert!(dir.x.abs() < 0.001);
        assert!(dir.y.abs() < 0.001);
        assert!(dir.z < 0.0);
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new();
        let initial_pos = camera.position.clone();
        camera.move_forward(1.0);
        assert!(camera.position.z < initial_pos.z);
    }

    #[test]
    fn test_camera_zoom() {
        let mut camera = Camera::new();
        let initial_fov = camera.fov;
        camera.zoom(0.5);
        assert!(camera.fov < initial_fov);
    }

    #[test]
    fn test_camera_standard_views() {
        let front = Camera::front_view();
        assert!(front.position.z > 0.0);

        let top = Camera::top_view();
        assert!(top.position.y > 0.0);

        let isometric = Camera::isometric_view();
        assert!(isometric.position.x > 0.0);
        assert!(isometric.position.y > 0.0);
        assert!(isometric.position.z > 0.0);
    }

    #[test]
    fn test_camera_controller() {
        let camera = Camera::new();
        let controller = CameraController::new(camera);
        assert_eq!(controller.sensitivity, 0.1);
        assert_eq!(controller.speed, 1.0);
        assert!(controller.orbit_mode);
    }
}
