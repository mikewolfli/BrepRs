use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimplificationStrategy {
    EdgeCollapse,
    VertexClustering,
    QuadricError,
    ProgressiveMesh,
}

#[derive(Debug, Clone)]
pub struct SimplificationConfig {
    pub target_ratio: StandardReal,
    pub preserve_boundaries: bool,
    pub preserve_sharp_edges: bool,
    pub sharp_angle_threshold: StandardReal,
    pub max_iterations: usize,
    pub aggressiveness: StandardReal,
}

impl Default for SimplificationConfig {
    fn default() -> Self {
        Self {
            target_ratio: 0.5,
            preserve_boundaries: true,
            preserve_sharp_edges: true,
            sharp_angle_threshold: 30.0_f64.to_radians(),
            max_iterations: 1000,
            aggressiveness: 1.0,
        }
    }
}

impl SimplificationConfig {
    pub fn new(target_ratio: StandardReal) -> Self {
        Self {
            target_ratio,
            ..Default::default()
        }
    }

    pub fn with_boundary_preservation(mut self, preserve: bool) -> Self {
        self.preserve_boundaries = preserve;
        self
    }

    pub fn with_sharp_edge_preservation(mut self, preserve: bool, threshold: StandardReal) -> Self {
        self.preserve_sharp_edges = preserve;
        self.sharp_angle_threshold = threshold;
        self
    }

    pub fn with_aggressiveness(mut self, aggressiveness: StandardReal) -> Self {
        self.aggressiveness = aggressiveness;
        self
    }
}

#[derive(Debug, Clone)]
pub struct MeshVertex {
    pub id: usize,
    pub position: Point,
    pub normal: Option<Vector>,
    pub neighbors: HashSet<usize>,
    pub is_boundary: bool,
    pub is_sharp: bool,
}

impl MeshVertex {
    pub fn new(id: usize, position: Point) -> Self {
        Self {
            id,
            position,
            normal: None,
            neighbors: HashSet::new(),
            is_boundary: false,
            is_sharp: false,
        }
    }

    pub fn with_normal(mut self, normal: Vector) -> Self {
        self.normal = Some(normal);
        self
    }

    pub fn quadric_error(&self, quadric: &QuadricErrorMetric) -> StandardReal {
        quadric.evaluate(&self.position)
    }
}

#[derive(Debug, Clone)]
pub struct MeshFace {
    pub id: usize,
    pub vertices: [usize; 3],
    pub normal: Option<Vector>,
    pub area: StandardReal,
}

impl MeshFace {
    pub fn new(id: usize, v0: usize, v1: usize, v2: usize) -> Self {
        Self {
            id,
            vertices: [v0, v1, v2],
            normal: None,
            area: 0.0,
        }
    }

    pub fn compute_normal(&mut self, positions: &[Point]) {
        if self.vertices.iter().all(|&v| v < positions.len()) {
            let v0 = &positions[self.vertices[0]];
            let v1 = &positions[self.vertices[1]];
            let v2 = &positions[self.vertices[2]];

            let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
            let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);

            let normal = edge1.cross(&edge2);
            let length = (normal.x.powi(2) + normal.y.powi(2) + normal.z.powi(2)).sqrt();
            
            if length > 1e-10 {
                self.normal = Some(Vector::new(
                    normal.x / length,
                    normal.y / length,
                    normal.z / length,
                ));
            }

            self.area = length / 2.0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadricErrorMetric {
    pub a00: StandardReal, pub a01: StandardReal, pub a02: StandardReal, pub a03: StandardReal,
    pub a11: StandardReal, pub a12: StandardReal, pub a13: StandardReal,
    pub a22: StandardReal, pub a23: StandardReal,
    pub a33: StandardReal,
}

impl QuadricErrorMetric {
    pub fn zero() -> Self {
        Self {
            a00: 0.0, a01: 0.0, a02: 0.0, a03: 0.0,
            a11: 0.0, a12: 0.0, a13: 0.0,
            a22: 0.0, a23: 0.0,
            a33: 0.0,
        }
    }

    pub fn from_plane(normal: &Vector, point: &Point) -> Self {
        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let d = -(a * point.x + b * point.y + c * point.z);

        Self {
            a00: a * a, a01: a * b, a02: a * c, a03: a * d,
            a11: b * b, a12: b * c, a13: b * d,
            a22: c * c, a23: c * d,
            a33: d * d,
        }
    }

    pub fn add(&self, other: &QuadricErrorMetric) -> QuadricErrorMetric {
        QuadricErrorMetric {
            a00: self.a00 + other.a00,
            a01: self.a01 + other.a01,
            a02: self.a02 + other.a02,
            a03: self.a03 + other.a03,
            a11: self.a11 + other.a11,
            a12: self.a12 + other.a12,
            a13: self.a13 + other.a13,
            a22: self.a22 + other.a22,
            a23: self.a23 + other.a23,
            a33: self.a33 + other.a33,
        }
    }

    pub fn evaluate(&self, point: &Point) -> StandardReal {
        let x = point.x;
        let y = point.y;
        let z = point.z;

        self.a00 * x * x + 2.0 * self.a01 * x * y + 2.0 * self.a02 * x * z + 2.0 * self.a03 * x
            + self.a11 * y * y + 2.0 * self.a12 * y * z + 2.0 * self.a13 * y
            + self.a22 * z * z + 2.0 * self.a23 * z
            + self.a33
    }

    pub fn optimal_position(&self) -> Option<Point> {
        let det = self.a00 * (self.a11 * self.a22 - self.a12 * self.a12)
            - self.a01 * (self.a01 * self.a22 - self.a12 * self.a02)
            + self.a02 * (self.a01 * self.a12 - self.a11 * self.a02);

        if det.abs() < 1e-10 {
            return None;
        }

        let x = (self.a01 * (self.a12 * self.a23 - self.a13 * self.a22)
            - self.a02 * (self.a01 * self.a23 - self.a13 * self.a12)
            + self.a03 * (self.a01 * self.a12 - self.a11 * self.a02))
            / det;

        let y = (self.a00 * (self.a12 * self.a23 - self.a13 * self.a22)
            - self.a02 * (self.a02 * self.a23 - self.a03 * self.a22)
            + self.a03 * (self.a02 * self.a12 - self.a03 * self.a22))
            / det;

        let z = (self.a00 * (self.a11 * self.a23 - self.a13 * self.a12)
            - self.a01 * (self.a01 * self.a23 - self.a13 * self.a02)
            + self.a03 * (self.a01 * self.a02 - self.a03 * self.a11))
            / det;

        Some(Point::new(x, y, z))
    }
}

#[derive(Debug, Clone)]
pub struct EdgeCollapse {
    pub v1: usize,
    pub v2: usize,
    pub new_position: Point,
    pub error: StandardReal,
}

impl PartialEq for EdgeCollapse {
    fn eq(&self, other: &Self) -> bool {
        self.error == other.error
    }
}

impl Eq for EdgeCollapse {}

impl PartialOrd for EdgeCollapse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EdgeCollapse {
    fn cmp(&self, other: &Self) -> Ordering {
        other.error.partial_cmp(&self.error).unwrap_or(Ordering::Equal)
    }
}

pub struct MeshSimplifier {
    config: SimplificationConfig,
    vertices: Vec<MeshVertex>,
    faces: Vec<MeshFace>,
    quadrics: HashMap<usize, QuadricErrorMetric>,
    edge_heap: BinaryHeap<EdgeCollapse>,
}

impl MeshSimplifier {
    pub fn new(config: SimplificationConfig) -> Self {
        Self {
            config,
            vertices: Vec::new(),
            faces: Vec::new(),
            quadrics: HashMap::new(),
            edge_heap: BinaryHeap::new(),
        }
    }

    pub fn load_mesh(&mut self, positions: &[Point], indices: &[usize]) {
        self.vertices.clear();
        self.faces.clear();
        self.quadrics.clear();
        self.edge_heap.clear();

        for (id, pos) in positions.iter().enumerate() {
            self.vertices.push(MeshVertex::new(id, pos.clone()));
        }

        for (id, chunk) in indices.chunks(3).enumerate() {
            if chunk.len() == 3 {
                let face = MeshFace::new(id, chunk[0], chunk[1], chunk[2]);
                self.faces.push(face);
            }
        }

        self.compute_face_normals();
        self.identify_boundaries();
        self.identify_sharp_edges();
        self.compute_quadrics();
        self.build_edge_heap();
    }

    fn compute_face_normals(&mut self) {
        let positions: Vec<Point> = self.vertices.iter().map(|v| v.position.clone()).collect();
        for face in &mut self.faces {
            face.compute_normal(&positions);
        }
    }

    fn identify_boundaries(&mut self) {
        let mut edge_count: HashMap<(usize, usize), i32> = HashMap::new();

        for face in &self.faces {
            for i in 0..3 {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % 3];
                let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                *edge_count.entry(key).or_insert(0) += 1;
            }
        }

        for ((v1, v2), count) in &edge_count {
            if *count == 1 {
                self.vertices[*v1].is_boundary = true;
                self.vertices[*v2].is_boundary = true;
            }
        }

        for face in &self.faces {
            for i in 0..3 {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % 3];
                self.vertices[v1].neighbors.insert(v2);
                self.vertices[v2].neighbors.insert(v1);
            }
        }
    }

    fn identify_sharp_edges(&mut self) {
        if !self.config.preserve_sharp_edges {
            return;
        }

        let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (face_id, face) in self.faces.iter().enumerate() {
            for i in 0..3 {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % 3];
                let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                edge_faces.entry(key).or_default().push(face_id);
            }
        }

        for (_, face_ids) in &edge_faces {
            if face_ids.len() == 2 {
                let n1 = self.faces[face_ids[0]].normal.clone();
                let n2 = self.faces[face_ids[1]].normal.clone();

                if let (Some(n1), Some(n2)) = (n1, n2) {
                    let dot = n1.x * n2.x + n1.y * n2.y + n1.z * n2.z;
                    let angle = dot.acos();

                    if angle > self.config.sharp_angle_threshold {
                        for &fid in face_ids {
                            for &vid in &self.faces[fid].vertices {
                                self.vertices[vid].is_sharp = true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn compute_quadrics(&mut self) {
        for vertex in &self.vertices {
            self.quadrics.insert(vertex.id, QuadricErrorMetric::zero());
        }

        for face in &self.faces {
            if let Some(normal) = &face.normal {
                let center = Point::new(
                    (self.vertices[face.vertices[0]].position.x
                        + self.vertices[face.vertices[1]].position.x
                        + self.vertices[face.vertices[2]].position.x)
                        / 3.0,
                    (self.vertices[face.vertices[0]].position.y
                        + self.vertices[face.vertices[1]].position.y
                        + self.vertices[face.vertices[2]].position.y)
                        / 3.0,
                    (self.vertices[face.vertices[0]].position.z
                        + self.vertices[face.vertices[1]].position.z
                        + self.vertices[face.vertices[2]].position.z)
                        / 3.0,
                );

                let face_quadric = QuadricErrorMetric::from_plane(normal, &center);

                for &vid in &face.vertices {
                    let current = self.quadrics.get(&vid).cloned().unwrap_or_else(QuadricErrorMetric::zero);
                    self.quadrics.insert(vid, current.add(&face_quadric));
                }
            }
        }
    }

    fn build_edge_heap(&mut self) {
        self.edge_heap.clear();

        let mut processed_edges: HashSet<(usize, usize)> = HashSet::new();

        for vertex in &self.vertices {
            for &neighbor in &vertex.neighbors {
                let key = if vertex.id < neighbor {
                    (vertex.id, neighbor)
                } else {
                    (neighbor, vertex.id)
                };

                if processed_edges.contains(&key) {
                    continue;
                }
                processed_edges.insert(key);

                if self.can_collapse_edge(vertex.id, neighbor) {
                    if let Some(collapse) = self.compute_edge_collapse(vertex.id, neighbor) {
                        self.edge_heap.push(collapse);
                    }
                }
            }
        }
    }

    fn can_collapse_edge(&self, v1: usize, v2: usize) -> bool {
        if self.config.preserve_boundaries
            && (self.vertices[v1].is_boundary || self.vertices[v2].is_boundary)
        {
            if !(self.vertices[v1].is_boundary && self.vertices[v2].is_boundary) {
                return false;
            }
        }

        if self.config.preserve_sharp_edges
            && (self.vertices[v1].is_sharp || self.vertices[v2].is_sharp)
        {
            return false;
        }

        true
    }

    fn compute_edge_collapse(&self, v1: usize, v2: usize) -> Option<EdgeCollapse> {
        let q1 = self.quadrics.get(&v1)?;
        let q2 = self.quadrics.get(&v2)?;
        let combined = q1.add(q2);

        let new_position = combined.optimal_position().unwrap_or_else(|| {
            Point::new(
                (self.vertices[v1].position.x + self.vertices[v2].position.x) / 2.0,
                (self.vertices[v1].position.y + self.vertices[v2].position.y) / 2.0,
                (self.vertices[v1].position.z + self.vertices[v2].position.z) / 2.0,
            )
        });

        let error = combined.evaluate(&new_position);

        Some(EdgeCollapse {
            v1,
            v2,
            new_position,
            error,
        })
    }

    pub fn simplify(&mut self) -> (Vec<Point>, Vec<usize>) {
        let target_face_count = (self.faces.len() as StandardReal * self.config.target_ratio) as usize;
        let mut iterations = 0;

        while self.faces.len() > target_face_count && iterations < self.config.max_iterations {
            if let Some(collapse) = self.edge_heap.pop() {
                if self.apply_collapse(&collapse) {
                    self.update_edge_heap(&collapse);
                }
            } else {
                break;
            }
            iterations += 1;
        }

        self.extract_mesh()
    }

    fn apply_collapse(&mut self, collapse: &EdgeCollapse) -> bool {
        if collapse.v1 >= self.vertices.len() || collapse.v2 >= self.vertices.len() {
            return false;
        }

        self.vertices[collapse.v1].position = collapse.new_position.clone();

        let neighbors: Vec<usize> = self.vertices[collapse.v2].neighbors.iter().cloned().collect();
        for neighbor in neighbors {
            if neighbor != collapse.v1 {
                self.vertices[collapse.v1].neighbors.insert(neighbor);
                self.vertices[neighbor].neighbors.insert(collapse.v1);
                self.vertices[neighbor].neighbors.remove(&collapse.v2);
            }
        }

        let v2 = collapse.v2;
        self.vertices[v2].neighbors.clear();

        self.faces.retain(|face| {
            let has_v2 = face.vertices.contains(&v2);
            if has_v2 {
                let has_v1 = face.vertices.contains(&collapse.v1);
                !has_v1
            } else {
                true
            }
        });

        for face in &mut self.faces {
            for v in face.vertices.iter_mut() {
                if *v == v2 {
                    *v = collapse.v1;
                }
            }
        }

        if let Some(q1) = self.quadrics.get(&collapse.v1).cloned() {
            if let Some(q2) = self.quadrics.get(&v2).cloned() {
                self.quadrics.insert(collapse.v1, q1.add(&q2));
            }
        }

        true
    }

    fn update_edge_heap(&mut self, collapse: &EdgeCollapse) {
        let mut new_heap = BinaryHeap::new();

        while let Some(edge) = self.edge_heap.pop() {
            if edge.v1 != collapse.v2 && edge.v2 != collapse.v2 {
                if edge.v1 == collapse.v1 || edge.v2 == collapse.v1 {
                    if self.can_collapse_edge(edge.v1, edge.v2) {
                        if let Some(new_collapse) = self.compute_edge_collapse(edge.v1, edge.v2) {
                            new_heap.push(new_collapse);
                        }
                    }
                } else {
                    new_heap.push(edge);
                }
            }
        }

        self.edge_heap = new_heap;
    }

    fn extract_mesh(&self) -> (Vec<Point>, Vec<usize>) {
        let mut vertex_map: HashMap<usize, usize> = HashMap::new();
        let mut positions = Vec::new();
        let mut indices = Vec::new();

        for face in &self.faces {
            for &vid in &face.vertices {
                if !vertex_map.contains_key(&vid) {
                    let new_id = positions.len();
                    vertex_map.insert(vid, new_id);
                    positions.push(self.vertices[vid].position.clone());
                }
            }
        }

        for face in &self.faces {
            indices.push(vertex_map[&face.vertices[0]]);
            indices.push(vertex_map[&face.vertices[1]]);
            indices.push(vertex_map[&face.vertices[2]]);
        }

        (positions, indices)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.iter().filter(|v| !v.neighbors.is_empty()).count()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplification_config() {
        let config = SimplificationConfig::new(0.5)
            .with_boundary_preservation(true)
            .with_aggressiveness(1.5);

        assert_eq!(config.target_ratio, 0.5);
        assert!(config.preserve_boundaries);
        assert_eq!(config.aggressiveness, 1.5);
    }

    #[test]
    fn test_quadric_error_metric() {
        let normal = Vector::new(0.0, 0.0, 1.0);
        let point = Point::new(0.0, 0.0, 0.0);

        let q = QuadricErrorMetric::from_plane(&normal, &point);
        
        let test_point = Point::new(0.0, 0.0, 1.0);
        let error = q.evaluate(&test_point);
        assert!(error > 0.0);
    }

    #[test]
    fn test_mesh_simplifier() {
        let config = SimplificationConfig::new(0.5);
        let mut simplifier = MeshSimplifier::new(config);

        let positions = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        simplifier.load_mesh(&positions, &indices);
        assert_eq!(simplifier.face_count(), 2);

        let (new_positions, new_indices) = simplifier.simplify();
        assert!(!new_positions.is_empty());
        assert!(!new_indices.is_empty());
    }

    #[test]
    fn test_edge_collapse_ordering() {
        let c1 = EdgeCollapse {
            v1: 0,
            v2: 1,
            new_position: Point::origin(),
            error: 0.1,
        };

        let c2 = EdgeCollapse {
            v1: 0,
            v2: 2,
            new_position: Point::origin(),
            error: 0.5,
        };

        assert!(c1 > c2);
    }
}
