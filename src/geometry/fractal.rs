//! Fractal geometry module
//! 
//! This module provides generation and representation of fractal shapes,
//! including various fractal types and mesh generation.

use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, Vertex};

/// Fractal type
#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    /// Menger sponge
    MengerSponge { level: usize },
    /// Sierpinski tetrahedron
    SierpinskiTetrahedron { level: usize },
    /// Koch snowflake (3D version)
    KochSnowflake { level: usize },
    /// Dragon curve (3D version)
    DragonCurve { level: usize },
    /// Mandelbulb
    Mandelbulb { iterations: usize, power: f64 },
    /// Julia set (3D)
    JuliaSet { iterations: usize, constant: (f64, f64, f64) },
    /// L-system tree
    LSystemTree { iterations: usize, angle: f64, length: f64 },
    /// Terrain generation
    Terrain { size: usize, height: f64, roughness: f64 },
}

/// Fractal parameters
#[derive(Debug, Clone)]
pub struct FractalParams {
    /// Fractal type
    pub fractal_type: FractalType,
    /// Center position
    pub center: Point,
    /// Scale
    pub scale: f64,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            fractal_type: FractalType::MengerSponge { level: 2 },
            center: Point::origin(),
            scale: 1.0,
        }
    }
}

/// Fractal geometry generator
pub struct FractalGenerator {
    /// Fractal parameters
    params: FractalParams,
}

impl FractalGenerator {
    /// Create a new fractal generator
    pub fn new(params: FractalParams) -> Self {
        Self { params }
    }

    /// Generate mesh from fractal
    pub fn generate_mesh(&self) -> Mesh3D {
        match &self.params.fractal_type {
            FractalType::MengerSponge { level } => self.generate_menger_sponge(*level),
            FractalType::SierpinskiTetrahedron { level } => self.generate_sierpinski_tetrahedron(*level),
            FractalType::KochSnowflake { level } => self.generate_koch_snowflake(*level),
            FractalType::DragonCurve { level } => self.generate_dragon_curve(*level),
            FractalType::Mandelbulb { iterations, power } => self.generate_mandelbulb(*iterations, *power),
            FractalType::JuliaSet { iterations, constant } => self.generate_julia_set(*iterations, *constant),
            FractalType::LSystemTree { iterations, angle, length } => self.generate_l_system_tree(*iterations, *angle, *length),
            FractalType::Terrain { size, height, roughness } => self.generate_terrain(*size, *height, *roughness),
        }
    }

    /// Generate Menger sponge
    fn generate_menger_sponge(&self, level: usize) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        self.menger_sponge_recursive(&mut mesh, center, scale, level);
        
        mesh
    }

    /// Recursive Menger sponge generation
    fn menger_sponge_recursive(&self, mesh: &mut Mesh3D, center: &Point, size: f64, level: usize) {
        if level == 0 {
            // Create a cube
            let half_size = size / 2.0;
            let min = Point::new(center.x - half_size, center.y - half_size, center.z - half_size);
            let max = Point::new(center.x + half_size, center.y + half_size, center.z + half_size);
            self.create_cube(mesh, &min, &max);
            return;
        }
        
        let third = size / 3.0;
        let start = Point::new(center.x - size / 2.0, center.y - size / 2.0, center.z - size / 2.0);
        
        // Generate 27 sub-cubes, removing the center and face centers
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    // Skip center and face centers
                    let center_x = (i == 1) as usize;
                    let center_y = (j == 1) as usize;
                    let center_z = (k == 1) as usize;
                    let center_count = center_x + center_y + center_z;
                    
                    if center_count >= 2 {
                        continue;
                    }
                    
                    let new_center = Point::new(
                        start.x + (i as f64 + 0.5) * third,
                        start.y + (j as f64 + 0.5) * third,
                        start.z + (k as f64 + 0.5) * third,
                    );
                    
                    self.menger_sponge_recursive(mesh, &new_center, third, level - 1);
                }
            }
        }
    }

    /// Generate Sierpinski tetrahedron
    fn generate_sierpinski_tetrahedron(&self, level: usize) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        // Initial tetrahedron vertices
        let v0 = Point::new(center.x, center.y + scale, center.z + scale / std::f64::consts::SQRT_3);
        let v1 = Point::new(center.x - scale, center.y - scale, center.z + scale / std::f64::consts::SQRT_3);
        let v2 = Point::new(center.x + scale, center.y - scale, center.z + scale / std::f64::consts::SQRT_3);
        let v3 = Point::new(center.x, center.y, center.z - scale / std::f64::consts::SQRT_3);
        
        self.sierpinski_recursive(&mut mesh, &v0, &v1, &v2, &v3, level);
        
        mesh
    }

    /// Recursive Sierpinski tetrahedron generation
    fn sierpinski_recursive(&self, mesh: &mut Mesh3D, v0: &Point, v1: &Point, v2: &Point, v3: &Point, level: usize) {
        if level == 0 {
            // Create tetrahedron
            let idx0 = mesh.add_vertex(*v0, Vector::zero());
            let idx1 = mesh.add_vertex(*v1, Vector::zero());
            let idx2 = mesh.add_vertex(*v2, Vector::zero());
            let idx3 = mesh.add_vertex(*v3, Vector::zero());
            mesh.add_tetrahedron(idx0, idx1, idx2, idx3);
            return;
        }
        
        // Calculate midpoints
        let m01 = Point::new((v0.x + v1.x) / 2.0, (v0.y + v1.y) / 2.0, (v0.z + v1.z) / 2.0);
        let m02 = Point::new((v0.x + v2.x) / 2.0, (v0.y + v2.y) / 2.0, (v0.z + v2.z) / 2.0);
        let m03 = Point::new((v0.x + v3.x) / 2.0, (v0.y + v3.y) / 2.0, (v0.z + v3.z) / 2.0);
        let m12 = Point::new((v1.x + v2.x) / 2.0, (v1.y + v2.y) / 2.0, (v1.z + v2.z) / 2.0);
        let m13 = Point::new((v1.x + v3.x) / 2.0, (v1.y + v3.y) / 2.0, (v1.z + v3.z) / 2.0);
        let m23 = Point::new((v2.x + v3.x) / 2.0, (v2.y + v3.y) / 2.0, (v2.z + v3.z) / 2.0);
        
        // Recursively generate 4 smaller tetrahedra
        self.sierpinski_recursive(mesh, v0, &m01, &m02, &m03, level - 1);
        self.sierpinski_recursive(mesh, &m01, v1, &m12, &m13, level - 1);
        self.sierpinski_recursive(mesh, &m02, &m12, v2, &m23, level - 1);
        self.sierpinski_recursive(mesh, &m03, &m13, &m23, v3, level - 1);
    }

    /// Generate Koch snowflake (3D version)
    fn generate_koch_snowflake(&self, level: usize) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        // Start with a regular tetrahedron
        let v0 = Point::new(center.x, center.y + scale, center.z + scale / std::f64::consts::SQRT_3);
        let v1 = Point::new(center.x - scale, center.y - scale, center.z + scale / std::f64::consts::SQRT_3);
        let v2 = Point::new(center.x + scale, center.y - scale, center.z + scale / std::f64::consts::SQRT_3);
        let v3 = Point::new(center.x, center.y, center.z - scale / std::f64::consts::SQRT_3);
        
        // Apply Koch subdivision to each edge
        let edges = vec![
            (v0, v1), (v1, v2), (v2, v0),
            (v0, v3), (v1, v3), (v2, v3),
        ];
        
        let subdivided_edges: Vec<(Point, Point)> = edges.iter()
            .flat_map(|(a, b)| self.koch_subdivide_edge(a, b, level))
            .collect();
        
        // Create faces from subdivided edges
        self.create_faces_from_edges(&mut mesh, &subdivided_edges);
        
        mesh
    }

    /// Koch subdivision for an edge
    fn koch_subdivide_edge(&self, a: &Point, b: &Point, level: usize) -> Vec<(Point, Point)> {
        if level == 0 {
            return vec![(*a, *b)];
        }
        
        // Divide edge into 4 segments
        let p1 = Point::new(
            a.x + (b.x - a.x) / 3.0,
            a.y + (b.y - a.y) / 3.0,
            a.z + (b.z - a.z) / 3.0,
        );
        let p2 = Point::new(
            a.x + 2.0 * (b.x - a.x) / 3.0,
            a.y + 2.0 * (b.y - a.y) / 3.0,
            a.z + 2.0 * (b.z - a.z) / 3.0,
        );
        
        // Create a peak point
        let mid = Point::new(
            (a.x + b.x) / 2.0,
            (a.y + b.y) / 2.0,
            (a.z + b.z) / 2.0,
        );
        let dir = Vector::new(
            (b.y - a.y) * (a.z - b.z),
            (b.z - a.z) * (a.x - b.x),
            (b.x - a.x) * (a.y - b.y),
        ).normalize();
        let peak = Point::new(
            mid.x + dir.x * (b.distance(a) / 3.0),
            mid.y + dir.y * (b.distance(a) / 3.0),
            mid.z + dir.z * (b.distance(a) / 3.0),
        );
        
        // Recursively subdivide
        let mut result = Vec::new();
        result.extend(self.koch_subdivide_edge(a, &p1, level - 1));
        result.extend(self.koch_subdivide_edge(&p1, &peak, level - 1));
        result.extend(self.koch_subdivide_edge(&peak, &p2, level - 1));
        result.extend(self.koch_subdivide_edge(&p2, b, level - 1));
        
        result
    }

    /// Create faces from edges
    fn create_faces_from_edges(&self, mesh: &mut Mesh3D, edges: &[(Point, Point)]) {
        // Simplified face creation
        for (i, (a, b)) in edges.iter().enumerate() {
            let idx0 = mesh.add_vertex(*a, Vector::zero());
            let idx1 = mesh.add_vertex(*b, Vector::zero());
            
            if i + 1 < edges.len() {
                let (c, _) = &edges[i + 1];
                let idx2 = mesh.add_vertex(*c, Vector::zero());
                mesh.add_tetrahedron(idx0, idx1, idx2, idx0);
            }
        }
    }

    /// Generate Dragon curve (3D version)
    fn generate_dragon_curve(&self, level: usize) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        // Start with a line segment
        let mut points = vec![
            Point::new(center.x - scale / 2.0, center.y, center.z),
            Point::new(center.x + scale / 2.0, center.y, center.z),
        ];
        
        // Apply dragon curve iterations
        for _ in 0..level {
            points = self.dragon_curve_iteration(&points);
        }
        
        // Create tube around the curve
        self.create_tube_from_points(&mut mesh, &points, scale / 20.0);
        
        mesh
    }

    /// Dragon curve iteration
    fn dragon_curve_iteration(&self, points: &[Point]) -> Vec<Point> {
        let mut new_points = Vec::new();
        
        for i in 0..points.len() - 1 {
            let a = &points[i];
            let b = &points[i + 1];
            
            new_points.push(*a);
            
            // Calculate midpoint and rotate
            let mid = Point::new(
                (a.x + b.x) / 2.0,
                (a.y + b.y) / 2.0,
                (a.z + b.z) / 2.0,
            );
            
            let dir = Vector::new(b.x - a.x, b.y - a.y, b.z - a.z).normalize();
            let up = Vector::new(0.0, 0.0, 1.0);
            let right = dir.cross(&up).normalize();
            
            let rotated = Point::new(
                mid.x + right.x * (a.distance(b) / 2.0),
                mid.y + right.y * (a.distance(b) / 2.0),
                mid.z + right.z * (a.distance(b) / 2.0),
            );
            
            new_points.push(rotated);
        }
        
        new_points.push(points[points.len() - 1]);
        new_points
    }

    /// Create tube from points
    fn create_tube_from_points(&self, mesh: &mut Mesh3D, points: &[Point], radius: f64) {
        let segments = 8;
        
        for i in 0..points.len() - 1 {
            let a = &points[i];
            let b = &points[i + 1];
            
            let dir = Vector::new(b.x - a.x, b.y - a.y, b.z - a.z).normalize();
            let up = if dir.z.abs() > 0.9 {
                Vector::new(1.0, 0.0, 0.0)
            } else {
                Vector::new(0.0, 0.0, 1.0)
            };
            let right = dir.cross(&up).normalize();
            let forward = right.cross(&dir).normalize();
            
            let mut ring_a = Vec::new();
            let mut ring_b = Vec::new();
            
            for j in 0..segments {
                let angle = 2.0 * std::f64::consts::PI * j as f64 / segments as f64;
                let offset = right * (radius * angle.cos()) + forward * (radius * angle.sin());
                
                ring_a.push(Point::new(a.x + offset.x, a.y + offset.y, a.z + offset.z));
                ring_b.push(Point::new(b.x + offset.x, b.y + offset.y, b.z + offset.z));
            }
            
            // Create tube segments
            for j in 0..segments {
                let j_next = (j + 1) % segments;
                let v0 = mesh.add_vertex(ring_a[j], Vector::zero());
                let v1 = mesh.add_vertex(ring_a[j_next], Vector::zero());
                let v2 = mesh.add_vertex(ring_b[j_next], Vector::zero());
                let v3 = mesh.add_vertex(ring_b[j], Vector::zero());
                mesh.add_tetrahedron(v0, v1, v2, v3);
            }
        }
    }

    /// Generate Mandelbulb
    fn generate_mandelbulb(&self, iterations: usize, power: f64) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        let resolution = 50;
        let bounds = scale * 2.0;
        let step = bounds / resolution as f64;
        
        // Marching cubes for Mandelbulb
        for i in 0..resolution {
            for j in 0..resolution {
                for k in 0..resolution {
                    let x = center.x - bounds / 2.0 + i as f64 * step;
                    let y = center.y - bounds / 2.0 + j as f64 * step;
                    let z = center.z - bounds / 2.0 + k as f64 * step;
                    
                    let point = Point::new(x, y, z);
                    let value = self.mandelbulb_evaluate(&point, iterations, power);
                    
                    if value.abs() < 0.1 {
                        let normal = self.mandelbulb_gradient(&point, iterations, power);
                        mesh.add_vertex(point, normal);
                    }
                }
            }
        }
        
        mesh
    }

    /// Evaluate Mandelbulb at a point
    fn mandelbulb_evaluate(&self, point: &Point, iterations: usize, power: f64) -> f64 {
        let mut x = point.x;
        let mut y = point.y;
        let mut z = point.z;
        
        for _ in 0..iterations {
            let r = (x * x + y * y + z * z).sqrt();
            if r > 2.0 {
                return r - 2.0;
            }
            
            let theta = (z / r).acos();
            let phi = y.atan2(x);
            
            let r_new = r.powf(power);
            let theta_new = theta * power;
            let phi_new = phi * power;
            
            x = r_new * theta_new.sin() * phi_new.cos();
            y = r_new * theta_new.sin() * phi_new.sin();
            z = r_new * theta_new.cos();
            
            x += point.x;
            y += point.y;
            z += point.z;
        }
        
        (x * x + y * y + z * z).sqrt() - 2.0
    }

    /// Calculate Mandelbulb gradient
    fn mandelbulb_gradient(&self, point: &Point, iterations: usize, power: f64) -> Vector {
        let epsilon = 1e-6;
        let f = self.mandelbulb_evaluate(point, iterations, power);
        
        let fx = self.mandelbulb_evaluate(&Point::new(point.x + epsilon, point.y, point.z), iterations, power);
        let fy = self.mandelbulb_evaluate(&Point::new(point.x, point.y + epsilon, point.z), iterations, power);
        let fz = self.mandelbulb_evaluate(&Point::new(point.x, point.y, point.z + epsilon), iterations, power);
        
        Vector::new(
            (fx - f) / epsilon,
            (fy - f) / epsilon,
            (fz - f) / epsilon,
        ).normalize()
    }

    /// Generate Julia set (3D)
    fn generate_julia_set(&self, iterations: usize, constant: (f64, f64, f64)) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        let resolution = 50;
        let bounds = scale * 2.0;
        let step = bounds / resolution as f64;
        
        // Marching cubes for Julia set
        for i in 0..resolution {
            for j in 0..resolution {
                for k in 0..resolution {
                    let x = center.x - bounds / 2.0 + i as f64 * step;
                    let y = center.y - bounds / 2.0 + j as f64 * step;
                    let z = center.z - bounds / 2.0 + k as f64 * step;
                    
                    let point = Point::new(x, y, z);
                    let value = self.julia_evaluate(&point, iterations, constant);
                    
                    if value.abs() < 0.1 {
                        let normal = self.julia_gradient(&point, iterations, constant);
                        mesh.add_vertex(point, normal);
                    }
                }
            }
        }
        
        mesh
    }

    /// Evaluate Julia set at a point
    fn julia_evaluate(&self, point: &Point, iterations: usize, constant: (f64, f64, f64)) -> f64 {
        let mut x = point.x;
        let mut y = point.y;
        let mut z = point.z;
        
        for _ in 0..iterations {
            let r = (x * x + y * y + z * z).sqrt();
            if r > 2.0 {
                return r - 2.0;
            }
            
            let x_new = x * x - y * y - z * z + constant.0;
            let y_new = 2.0 * x * y + constant.1;
            let z_new = 2.0 * x * z + constant.2;
            
            x = x_new;
            y = y_new;
            z = z_new;
        }
        
        (x * x + y * y + z * z).sqrt() - 2.0
    }

    /// Calculate Julia set gradient
    fn julia_gradient(&self, point: &Point, iterations: usize, constant: (f64, f64, f64)) -> Vector {
        let epsilon = 1e-6;
        let f = self.julia_evaluate(point, iterations, constant);
        
        let fx = self.julia_evaluate(&Point::new(point.x + epsilon, point.y, point.z), iterations, constant);
        let fy = self.julia_evaluate(&Point::new(point.x, point.y + epsilon, point.z), iterations, constant);
        let fz = self.julia_evaluate(&Point::new(point.x, point.y, point.z + epsilon), iterations, constant);
        
        Vector::new(
            (fx - f) / epsilon,
            (fy - f) / epsilon,
            (fz - f) / epsilon,
        ).normalize()
    }

    /// Generate L-system tree
    fn generate_l_system_tree(&self, iterations: usize, angle: f64, length: f64) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        // L-system rules
        let axiom = "F";
        let rules = [('F', "FF+[+F-F-F]-[-F+F+F]")];
        
        // Generate string
        let mut current = axiom.to_string();
        for _ in 0..iterations {
            let mut next = String::new();
            for c in current.chars() {
                let mut replaced = false;
                for &(from, to) in &rules {
                    if c == from {
                        next.push_str(to);
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    next.push(c);
                }
            }
            current = next;
        }
        
        // Interpret string and create geometry
        let mut position = Point::new(center.x, center.y - scale, center.z);
        let mut direction = Vector::new(0.0, 1.0, 0.0);
        let mut stack: Vec<(Point, Vector)> = Vec::new();
        
        for c in current.chars() {
            match c {
                'F' => {
                    let end = Point::new(
                        position.x + direction.x * length * scale,
                        position.y + direction.y * length * scale,
                        position.z + direction.z * length * scale,
                    );
                    self.create_cylinder(&mut mesh, &position, &end, length * scale / 10.0);
                    position = end;
                }
                '+' => {
                    direction = self.rotate_vector(&direction, angle, 0.0, 0.0);
                }
                '-' => {
                    direction = self.rotate_vector(&direction, -angle, 0.0, 0.0);
                }
                '[' => {
                    stack.push((position, direction));
                }
                ']' => {
                    if let Some((pos, dir)) = stack.pop() {
                        position = pos;
                        direction = dir;
                    }
                }
                _ => {}
            }
        }
        
        mesh
    }

    /// Rotate a vector
    fn rotate_vector(&self, vec: &Vector, rx: f64, ry: f64, rz: f64) -> Vector {
        let cos_x = rx.cos();
        let sin_x = rx.sin();
        let cos_y = ry.cos();
        let sin_y = ry.sin();
        let cos_z = rz.cos();
        let sin_z = rz.sin();
        
        // Rotate around X
        let mut y = vec.y * cos_x - vec.z * sin_x;
        let mut z = vec.y * sin_x + vec.z * cos_x;
        let mut x = vec.x;
        
        // Rotate around Y
        let temp_x = x * cos_y + z * sin_y;
        z = -x * sin_y + z * cos_y;
        x = temp_x;
        
        // Rotate around Z
        let temp_x = x * cos_z - y * sin_z;
        y = x * sin_z + y * cos_z;
        x = temp_x;
        
        Vector::new(x, y, z)
    }

    /// Generate terrain
    fn generate_terrain(&self, size: usize, height: f64, roughness: f64) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let center = &self.params.center;
        let scale = self.params.scale;
        
        // Generate heightmap using diamond-square algorithm
        let mut heightmap = vec![vec![0.0f64; size + 1]; size + 1];
        
        // Initialize corners
        heightmap[0][0] = (rand::random::<f64>() - 0.5) * height;
        heightmap[0][size] = (rand::random::<f64>() - 0.5) * height;
        heightmap[size][0] = (rand::random::<f64>() - 0.5) * height;
        heightmap[size][size] = (rand::random::<f64>() - 0.5) * height;
        
        // Diamond-square algorithm
        let mut step = size;
        let mut scale_factor = roughness;
        
        while step > 1 {
            let half = step / 2;
            
            // Diamond step
            for y in (0..size).step_by(step) {
                for x in (0..size).step_by(step) {
                    let avg = (heightmap[y][x] + 
                              heightmap[y + step][x] + 
                              heightmap[y][x + step] + 
                              heightmap[y + step][x + step]) / 4.0;
                    heightmap[y + half][x + half] = avg + (rand::random::<f64>() - 0.5) * scale_factor * height;
                }
            }
            
            // Square step
            for y in (0..=size).step_by(half) {
                for x in ((y + half) % step..=size).step_by(step) {
                    let mut sum = 0.0;
                    let mut count = 0;
                    
                    if y >= half {
                        sum += heightmap[y - half][x];
                        count += 1;
                    }
                    if y + half <= size {
                        sum += heightmap[y + half][x];
                        count += 1;
                    }
                    if x >= half {
                        sum += heightmap[y][x - half];
                        count += 1;
                    }
                    if x + half <= size {
                        sum += heightmap[y][x + half];
                        count += 1;
                    }
                    
                    heightmap[y][x] = sum / count + (rand::random::<f64>() - 0.5) * scale_factor * height;
                }
            }
            
            step = half;
            scale_factor *= 0.5;
        }
        
        // Create mesh from heightmap
        let offset = size as f64 / 2.0;
        for y in 0..size {
            for x in 0..size {
                let x0 = center.x + (x as f64 - offset) * scale / size as f64;
                let y0 = center.y + (y as f64 - offset) * scale / size as f64;
                let z0 = center.z + heightmap[y][x];
                
                let x1 = center.x + ((x + 1) as f64 - offset) * scale / size as f64;
                let y1 = center.y + (y as f64 - offset) * scale / size as f64;
                let z1 = center.z + heightmap[y][x + 1];
                
                let x2 = center.x + ((x + 1) as f64 - offset) * scale / size as f64;
                let y2 = center.y + ((y + 1) as f64 - offset) * scale / size as f64;
                let z2 = center.z + heightmap[y + 1][x + 1];
                
                let x3 = center.x + (x as f64 - offset) * scale / size as f64;
                let y3 = center.y + ((y + 1) as f64 - offset) * scale / size as f64;
                let z3 = center.z + heightmap[y + 1][x];
                
                let v0 = mesh.add_vertex(Point::new(x0, y0, z0), Vector::zero());
                let v1 = mesh.add_vertex(Point::new(x1, y1, z1), Vector::zero());
                let v2 = mesh.add_vertex(Point::new(x2, y2, z2), Vector::zero());
                let v3 = mesh.add_vertex(Point::new(x3, y3, z3), Vector::zero());
                
                mesh.add_tetrahedron(v0, v1, v2, v3);
            }
        }
        
        mesh
    }

    /// Create a cube
    fn create_cube(&self, mesh: &mut Mesh3D, min: &Point, max: &Point) {
        let v0 = mesh.add_vertex(Point::new(min.x, min.y, min.z), Vector::new(-1.0, -1.0, -1.0));
        let v1 = mesh.add_vertex(Point::new(max.x, min.y, min.z), Vector::new(1.0, -1.0, -1.0));
        let v2 = mesh.add_vertex(Point::new(max.x, max.y, min.z), Vector::new(1.0, 1.0, -1.0));
        let v3 = mesh.add_vertex(Point::new(min.x, max.y, min.z), Vector::new(-1.0, 1.0, -1.0));
        let v4 = mesh.add_vertex(Point::new(min.x, min.y, max.z), Vector::new(-1.0, -1.0, 1.0));
        let v5 = mesh.add_vertex(Point::new(max.x, min.y, max.z), Vector::new(1.0, -1.0, 1.0));
        let v6 = mesh.add_vertex(Point::new(max.x, max.y, max.z), Vector::new(1.0, 1.0, 1.0));
        let v7 = mesh.add_vertex(Point::new(min.x, max.y, max.z), Vector::new(-1.0, 1.0, 1.0));
        
        mesh.add_tetrahedron(v0, v1, v2, v4);
        mesh.add_tetrahedron(v2, v4, v6, v7);
        mesh.add_tetrahedron(v0, v2, v3, v7);
        mesh.add_tetrahedron(v0, v4, v5, v7);
        mesh.add_tetrahedron(v1, v2, v5, v6);
    }

    /// Create a cylinder
    fn create_cylinder(&self, mesh: &mut Mesh3D, start: &Point, end: &Point, radius: f64) {
        let segments = 8;
        let dir = Vector::new(end.x - start.x, end.y - start.y, end.z - start.z).normalize();
        let up = if dir.z.abs() > 0.9 {
            Vector::new(1.0, 0.0, 0.0)
        } else {
            Vector::new(0.0, 0.0, 1.0)
        };
        let right = dir.cross(&up).normalize();
        let forward = right.cross(&dir).normalize();
        
        let mut ring_start = Vec::new();
        let mut ring_end = Vec::new();
        
        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let offset = right * (radius * angle.cos()) + forward * (radius * angle.sin());
            
            ring_start.push(Point::new(start.x + offset.x, start.y + offset.y, start.z + offset.z));
            ring_end.push(Point::new(end.x + offset.x, end.y + offset.y, end.z + offset.z));
        }
        
        for i in 0..segments {
            let i_next = (i + 1) % segments;
            let v0 = mesh.add_vertex(ring_start[i], Vector::zero());
            let v1 = mesh.add_vertex(ring_start[i_next], Vector::zero());
            let v2 = mesh.add_vertex(ring_end[i_next], Vector::zero());
            let v3 = mesh.add_vertex(ring_end[i], Vector::zero());
            mesh.add_tetrahedron(v0, v1, v2, v3);
        }
    }
}

impl Default for FractalGenerator {
    fn default() -> Self {
        Self::new(FractalParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menger_sponge() {
        let params = FractalParams {
            fractal_type: FractalType::MengerSponge { level: 1 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_sierpinski_tetrahedron() {
        let params = FractalParams {
            fractal_type: FractalType::SierpinskiTetrahedron { level: 2 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_koch_snowflake() {
        let params = FractalParams {
            fractal_type: FractalType::KochSnowflake { level: 1 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_dragon_curve() {
        let params = FractalParams {
            fractal_type: FractalType::DragonCurve { level: 3 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_mandelbulb() {
        let params = FractalParams {
            fractal_type: FractalType::Mandelbulb { iterations: 5, power: 8.0 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_julia_set() {
        let params = FractalParams {
            fractal_type: FractalType::JuliaSet { iterations: 10, constant: (-0.8, 0.156, 0.0) },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_l_system_tree() {
        let params = FractalParams {
            fractal_type: FractalType::LSystemTree { iterations: 2, angle: 0.5, length: 0.2 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_terrain() {
        let params = FractalParams {
            fractal_type: FractalType::Terrain { size: 16, height: 0.5, roughness: 0.5 },
            center: Point::origin(),
            scale: 1.0,
        };
        
        let generator = FractalGenerator::new(params);
        let mesh = generator.generate_mesh();
        
        assert!(mesh.vertices.len() > 0);
    }
}
