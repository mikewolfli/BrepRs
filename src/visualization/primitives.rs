//! Graphics primitives for visualization
//!
//! This module provides basic graphics primitives for 3D rendering,
//! including points, lines, triangles, and meshes.
//! Compatible with OpenCASCADE Open API design.

 use crate::geometry::{Point};

/// Color representation with RGBA components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 - 1.0)
    pub r: f32,
    /// Green component (0.0 - 1.0)
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    pub b: f32,
    /// Alpha component (0.0 - 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create color from RGB (alpha = 1.0)
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create color from 8-bit RGB values
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }

    /// Create color from 8-bit RGBA values
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Predefined colors
    pub fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }
    pub fn red() -> Self {
        Self::from_rgb(1.0, 0.0, 0.0)
    }
    pub fn green() -> Self {
        Self::from_rgb(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Self {
        Self::from_rgb(0.0, 0.0, 1.0)
    }
    pub fn yellow() -> Self {
        Self::from_rgb(1.0, 1.0, 0.0)
    }
    pub fn cyan() -> Self {
        Self::from_rgb(0.0, 1.0, 1.0)
    }
    pub fn magenta() -> Self {
        Self::from_rgb(1.0, 0.0, 1.0)
    }
    pub fn gray() -> Self {
        Self::from_rgb(0.5, 0.5, 0.5)
    }
    pub fn dark_gray() -> Self {
        Self::from_rgb(0.25, 0.25, 0.25)
    }
    pub fn light_gray() -> Self {
        Self::from_rgb(0.75, 0.75, 0.75)
    }
    pub fn orange() -> Self {
        Self::from_rgb(1.0, 0.5, 0.0)
    }
    pub fn pink() -> Self {
        Self::from_rgb(1.0, 0.75, 0.8)
    }
    pub fn purple() -> Self {
        Self::from_rgb(0.6, 0.2, 0.8)
    }
    pub fn brown() -> Self {
        Self::from_rgb(0.6, 0.4, 0.2)
    }
    pub fn teal() -> Self {
        Self::from_rgb(0.0, 0.5, 0.5)
    }
    pub fn lime() -> Self {
        Self::from_rgb(0.5, 1.0, 0.0)
    }
    pub fn navy() -> Self {
        Self::from_rgb(0.0, 0.2, 0.4)
    }
    pub fn maroon() -> Self {
        Self::from_rgb(0.5, 0.0, 0.0)
    }
    pub fn olive() -> Self {
        Self::from_rgb(0.5, 0.5, 0.0)
    }
    pub fn silver() -> Self {
        Self::from_rgb(0.75, 0.75, 0.75)
    }
    pub fn gold() -> Self {
        Self::from_rgb(1.0, 0.78, 0.34)
    }
    pub fn bronze() -> Self {
        Self::from_rgb(0.8, 0.5, 0.2)
    }
    pub fn emerald() -> Self {
        Self::from_rgb(0.0, 0.6, 0.3)
    }
    pub fn sky_blue() -> Self {
        Self::from_rgb(0.5, 0.7, 1.0)
    }
    pub fn coral() -> Self {
        Self::from_rgb(1.0, 0.5, 0.3)
    }
    pub fn salmon() -> Self {
        Self::from_rgb(0.9, 0.5, 0.5)
    }
    pub fn lavender() -> Self {
        Self::from_rgb(0.8, 0.8, 1.0)
    }
    pub fn mint() -> Self {
        Self::from_rgb(0.7, 1.0, 0.8)
    }
    pub fn peach() -> Self {
        Self::from_rgb(1.0, 0.7, 0.5)
    }
    pub fn plum() -> Self {
        Self::from_rgb(0.6, 0.4, 0.6)
    }
    pub fn khaki() -> Self {
        Self::from_rgb(0.8, 0.8, 0.3)
    }
    pub fn turquoise() -> Self {
        Self::from_rgb(0.3, 0.8, 0.8)
    }
    pub fn magenta_dark() -> Self {
        Self::from_rgb(0.6, 0.0, 0.6)
    }
    pub fn yellow_light() -> Self {
        Self::from_rgb(1.0, 1.0, 0.7)
    }
    pub fn red_dark() -> Self {
        Self::from_rgb(0.6, 0.0, 0.0)
    }
    pub fn green_dark() -> Self {
        Self::from_rgb(0.0, 0.5, 0.0)
    }
    pub fn blue_dark() -> Self {
        Self::from_rgb(0.0, 0.0, 0.5)
    }
    pub fn gray_light() -> Self {
        Self::from_rgb(0.85, 0.85, 0.85)
    }
    pub fn gray_dark() -> Self {
        Self::from_rgb(0.15, 0.15, 0.15)
    }

    /// Linear interpolation between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Convert to RGB array
    pub fn to_rgb_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    /// Convert to RGBA array
    pub fn to_rgba_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

/// Vertex with position, normal, color, and texture coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// Position
    pub position: [f32; 3],
    /// Normal vector
    pub normal: [f32; 3],
    /// Color
    pub color: [f32; 4],
    /// Texture coordinates
    pub tex_coords: [f32; 2],
}

impl Vertex {
    /// Create a new vertex
    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        color: [f32; 4],
        tex_coords: [f32; 2],
    ) -> Self {
        Self {
            position,
            normal,
            color,
            tex_coords,
        }
    }

    /// Create vertex with position only
    pub fn from_position(position: [f32; 3]) -> Self {
        Self {
            position,
            normal: [0.0, 0.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0],
        }
    }

    /// Create vertex with position and color
    pub fn from_position_color(position: [f32; 3], color: Color) -> Self {
        Self {
            position,
            normal: [0.0, 0.0, 1.0],
            color: color.to_rgba_array(),
            tex_coords: [0.0, 0.0],
        }
    }

    /// Set normal
    pub fn with_normal(mut self, normal: [f32; 3]) -> Self {
        self.normal = normal;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color.to_rgba_array();
        self
    }

    /// Set texture coordinates
    pub fn with_tex_coords(mut self, tex_coords: [f32; 2]) -> Self {
        self.tex_coords = tex_coords;
        self
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self::from_position([0.0, 0.0, 0.0])
    }
}

/// 3D point with color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GraphicPoint {
    /// Position
    pub position: [f32; 3],
    /// Color
    pub color: Color,
    /// Size
    pub size: f32,
}

impl GraphicPoint {
    /// Create a new graphic point
    pub fn new(position: [f32; 3], color: Color, size: f32) -> Self {
        Self {
            position,
            color,
            size,
        }
    }

    /// Create from Point
    pub fn from_point(point: &Point, color: Color, size: f32) -> Self {
        Self::new(
            [point.x as f32, point.y as f32, point.z as f32],
            color,
            size,
        )
    }
}

/// 3D line segment
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    /// Start point
    pub start: [f32; 3],
    /// End point
    pub end: [f32; 3],
    /// Color
    pub color: Color,
    /// Line width
    pub width: f32,
}

impl Line {
    /// Create a new line
    pub fn new(start: [f32; 3], end: [f32; 3], color: Color, width: f32) -> Self {
        Self {
            start,
            end,
            color,
            width,
        }
    }

    /// Create line from points
    pub fn from_points(start: &Point, end: &Point, color: Color, width: f32) -> Self {
        Self::new(
            [start.x as f32, start.y as f32, start.z as f32],
            [end.x as f32, end.y as f32, end.z as f32],
            color,
            width,
        )
    }

    /// Get line length
    pub fn length(&self) -> f32 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get direction vector
    pub fn direction(&self) -> [f32; 3] {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        let len = self.length();
        if len > 0.0 {
            [dx / len, dy / len, dz / len]
        } else {
            [0.0, 0.0, 0.0]
        }
    }
}

/// Triangle primitive
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// Vertices
    pub vertices: [Vertex; 3],
}

impl Triangle {
    /// Create a new triangle
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self {
            vertices: [v0, v1, v2],
        }
    }

    /// Create triangle from positions
    pub fn from_positions(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3], color: Color) -> Self {
        let normal = Self::calculate_normal(p0, p1, p2);
        Self::new(
            Vertex::from_position_color(p0, color).with_normal(normal),
            Vertex::from_position_color(p1, color).with_normal(normal),
            Vertex::from_position_color(p2, color).with_normal(normal),
        )
    }

    /// Calculate normal from positions
    fn calculate_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
        let v0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
        let v1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

        let normal = [
            v0[1] * v1[2] - v0[2] * v1[1],
            v0[2] * v1[0] - v0[0] * v1[2],
            v0[0] * v1[1] - v0[1] * v1[0],
        ];

        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if len > 0.0 {
            [normal[0] / len, normal[1] / len, normal[2] / len]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    /// Calculate area
    pub fn area(&self) -> f32 {
        let p0 = self.vertices[0].position;
        let p1 = self.vertices[1].position;
        let p2 = self.vertices[2].position;

        let v0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
        let v1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

        let cross = [
            v0[1] * v1[2] - v0[2] * v1[1],
            v0[2] * v1[0] - v0[0] * v1[2],
            v0[0] * v1[1] - v0[1] * v1[0],
        ];

        0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
    }
}

/// Quad primitive (two triangles)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    /// Vertices
    pub vertices: [Vertex; 4],
}

impl Quad {
    /// Create a new quad
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Self {
            vertices: [v0, v1, v2, v3],
        }
    }

    /// Create quad from positions
    pub fn from_positions(
        p0: [f32; 3],
        p1: [f32; 3],
        p2: [f32; 3],
        p3: [f32; 3],
        color: Color,
    ) -> Self {
        let normal = Self::calculate_normal(p0, p1, p2);
        Self::new(
            Vertex::from_position_color(p0, color).with_normal(normal),
            Vertex::from_position_color(p1, color).with_normal(normal),
            Vertex::from_position_color(p2, color).with_normal(normal),
            Vertex::from_position_color(p3, color).with_normal(normal),
        )
    }

    /// Calculate normal from positions
    fn calculate_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
        let v0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
        let v1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

        let normal = [
            v0[1] * v1[2] - v0[2] * v1[1],
            v0[2] * v1[0] - v0[0] * v1[2],
            v0[0] * v1[1] - v0[1] * v1[0],
        ];

        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if len > 0.0 {
            [normal[0] / len, normal[1] / len, normal[2] / len]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    /// Convert to two triangles
    pub fn to_triangles(&self) -> (Triangle, Triangle) {
        (
            Triangle::new(self.vertices[0], self.vertices[1], self.vertices[2]),
            Triangle::new(self.vertices[0], self.vertices[2], self.vertices[3]),
        )
    }
}

/// Mesh primitive containing vertices and indices
#[derive(Debug, Clone, PartialEq)]
pub struct MeshPrimitive {
    /// Vertices
    pub vertices: Vec<Vertex>,
    /// Indices (triangles)
    pub indices: Vec<u32>,
    /// Bounding box
    pub bbox: ([f32; 3], [f32; 3]),
}

impl MeshPrimitive {
    /// Create a new mesh primitive
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            bbox: ([0.0; 3], [0.0; 3]),
        }
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(vertex);
        self.update_bbox();
        index
    }

    /// Add a triangle
    pub fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    /// Add a quad (as two triangles)
    pub fn add_quad(&mut self, i0: u32, i1: u32, i2: u32, i3: u32) {
        self.add_triangle(i0, i1, i2);
        self.add_triangle(i0, i2, i3);
    }

    /// Update bounding box
    fn update_bbox(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min = self.vertices[0].position;
        let mut max = self.vertices[0].position;

        for vertex in &self.vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        self.bbox = (min, max);
    }

    /// Calculate normals for the mesh
    pub fn calculate_normals(&mut self) {
        // Reset normals
        for vertex in &mut self.vertices {
            vertex.normal = [0.0; 3];
        }

        // Accumulate face normals
        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i] as usize;
            let i1 = self.indices[i + 1] as usize;
            let i2 = self.indices[i + 2] as usize;

            let p0 = self.vertices[i0].position;
            let p1 = self.vertices[i1].position;
            let p2 = self.vertices[i2].position;

            let v0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

            let normal = [
                v0[1] * v1[2] - v0[2] * v1[1],
                v0[2] * v1[0] - v0[0] * v1[2],
                v0[0] * v1[1] - v0[1] * v1[0],
            ];

            for i in [i0, i1, i2] {
                self.vertices[i].normal[0] += normal[0];
                self.vertices[i].normal[1] += normal[1];
                self.vertices[i].normal[2] += normal[2];
            }
        }

        // Normalize
        for vertex in &mut self.vertices {
            let len = (vertex.normal[0] * vertex.normal[0]
                + vertex.normal[1] * vertex.normal[1]
                + vertex.normal[2] * vertex.normal[2])
                .sqrt();
            if len > 0.0 {
                vertex.normal[0] /= len;
                vertex.normal[1] /= len;
                vertex.normal[2] /= len;
            }
        }
    }

    /// Clear the mesh
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.bbox = ([0.0; 3], [0.0; 3]);
    }

    /// Check if mesh is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

impl Default for MeshPrimitive {
    fn default() -> Self {
        Self::new()
    }
}

/// Polyline - connected line segments
#[derive(Debug, Clone, PartialEq)]
pub struct Polyline {
    /// Points
    pub points: Vec<[f32; 3]>,
    /// Color
    pub color: Color,
    /// Line width
    pub width: f32,
    /// Closed flag
    pub closed: bool,
}

impl Polyline {
    /// Create a new polyline
    pub fn new(color: Color, width: f32) -> Self {
        Self {
            points: Vec::new(),
            color,
            width,
            closed: false,
        }
    }

    /// Add a point
    pub fn add_point(&mut self, point: [f32; 3]) {
        self.points.push(point);
    }

    /// Set closed
    pub fn set_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    /// Get line segments
    pub fn segments(&self) -> Vec<Line> {
        let mut segments = Vec::new();
        if self.points.len() < 2 {
            return segments;
        }

        for i in 0..self.points.len() - 1 {
            segments.push(Line::new(
                self.points[i],
                self.points[i + 1],
                self.color,
                self.width,
            ));
        }

        if self.closed && self.points.len() > 2 {
            segments.push(Line::new(
                self.points[self.points.len() - 1],
                self.points[0],
                self.color,
                self.width,
            ));
        }

        segments
    }

    /// Calculate length
    pub fn length(&self) -> f32 {
        let mut length = 0.0;
        for i in 0..self.points.len().saturating_sub(1) {
            let dx = self.points[i + 1][0] - self.points[i][0];
            let dy = self.points[i + 1][1] - self.points[i][1];
            let dz = self.points[i + 1][2] - self.points[i][2];
            length += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        length
    }
}

/// Text label for 3D annotation
#[derive(Debug, Clone, PartialEq)]
pub struct TextLabel {
    /// Text content
    pub text: String,
    /// Position
    pub position: [f32; 3],
    /// Color
    pub color: Color,
    /// Font size
    pub font_size: f32,
    /// Screen space flag (always face camera)
    pub screen_space: bool,
}

impl TextLabel {
    /// Create a new text label
    pub fn new(text: &str, position: [f32; 3], color: Color, font_size: f32) -> Self {
        Self {
            text: text.to_string(),
            position,
            color,
            font_size,
            screen_space: true,
        }
    }

    /// Set screen space
    pub fn with_screen_space(mut self, screen_space: bool) -> Self {
        self.screen_space = screen_space;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::from_rgb(1.0, 0.5, 0.0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_from_rgb8() {
        let color = Color::from_rgb8(255, 128, 0);
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.5).abs() < 0.01);
        assert!(color.b < 0.01);
    }

    #[test]
    fn test_color_lerp() {
        let c1 = Color::red();
        let c2 = Color::blue();
        let c3 = c1.lerp(&c2, 0.5);
        assert!((c3.r - 0.5).abs() < 0.01);
        assert!(c3.g < 0.01);
        assert!((c3.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::from_position([1.0, 2.0, 3.0]);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_line_creation() {
        let line = Line::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], Color::white(), 1.0);
        assert_eq!(line.length(), 1.0);
        assert_eq!(line.direction(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle::from_positions(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            Color::white(),
        );
        assert!((triangle.area() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_mesh_primitive() {
        let mut mesh = MeshPrimitive::new();
        let v0 = mesh.add_vertex(Vertex::from_position([0.0, 0.0, 0.0]));
        let v1 = mesh.add_vertex(Vertex::from_position([1.0, 0.0, 0.0]));
        let v2 = mesh.add_vertex(Vertex::from_position([0.0, 1.0, 0.0]));
        mesh.add_triangle(v0, v1, v2);

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
        assert!(!mesh.is_empty());
    }

    #[test]
    fn test_polyline() {
        let mut polyline = Polyline::new(Color::white(), 1.0);
        polyline.add_point([0.0, 0.0, 0.0]);
        polyline.add_point([1.0, 0.0, 0.0]);
        polyline.add_point([1.0, 1.0, 0.0]);

        assert_eq!(polyline.points.len(), 3);
        assert_eq!(polyline.segments().len(), 2);
    }

    #[test]
    fn test_text_label() {
        let label = TextLabel::new("Test", [0.0, 0.0, 0.0], Color::white(), 12.0);
        assert_eq!(label.text, "Test");
        assert_eq!(label.font_size, 12.0);
        assert!(label.screen_space);
    }
}
