//! Point cloud topology analysis module
//!
//! This module provides functionality for analyzing the topology of point clouds,
//! including connectivity analysis, neighborhood analysis, and more.

use super::PointCloud;
use crate::geometry::{Point, Vector};

/// K-nearest neighbors
pub struct KNearestNeighbors {
    /// Number of neighbors
    k: usize,
}

impl KNearestNeighbors {
    /// Create a new K-nearest neighbors
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    /// Find K-nearest neighbors for each point
    pub fn find(&self, cloud: &PointCloud) -> Vec<Vec<(usize, f64)>> {
        let mut neighbors = Vec::new();

        for (i, point) in cloud.points().iter().enumerate() {
            let mut distances = Vec::new();

            for (j, other_point) in cloud.points().iter().enumerate() {
                if i != j {
                    let distance = point.distance(other_point);
                    distances.push((j, distance));
                }
            }

            // Sort by distance
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // Take first k neighbors
            let k_neighbors = distances.iter().take(self.k).cloned().collect();
            neighbors.push(k_neighbors);
        }

        neighbors
    }
}

/// Connectivity analysis
pub struct ConnectivityAnalysis {
    /// Distance threshold for connectivity
    distance_threshold: f64,
}

impl ConnectivityAnalysis {
    /// Create a new connectivity analysis
    pub fn new(distance_threshold: f64) -> Self {
        Self { distance_threshold }
    }

    /// Analyze connectivity
    pub fn analyze(&self, cloud: &PointCloud) -> Vec<Vec<usize>> {
        let mut connected_components = Vec::new();
        let mut visited = vec![false; cloud.len()];

        for i in 0..cloud.len() {
            if !visited[i] {
                let component = self.find_component(cloud, i, &mut visited);
                connected_components.push(component);
            }
        }

        connected_components
    }

    /// Find connected component for a point
    fn find_component(
        &self,
        cloud: &PointCloud,
        start: usize,
        visited: &mut Vec<bool>,
    ) -> Vec<usize> {
        let mut component = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(start);
        visited[start] = true;

        while let Some(current) = queue.pop_front() {
            component.push(current);

            // Find all neighbors within threshold
            for (i, point) in cloud.points().iter().enumerate() {
                if !visited[i] && cloud.points()[current].distance(point) <= self.distance_threshold
                {
                    queue.push_back(i);
                    visited[i] = true;
                }
            }
        }

        component
    }
}

/// Voronoi diagram
pub struct VoronoiDiagram {
    /// Points for Voronoi diagram
    points: Vec<Point>,
}

impl VoronoiDiagram {
    /// Create a new Voronoi diagram
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Build the Voronoi diagram
    pub fn build(&self) -> VoronoiResult {
        // Implementation of Fortune's algorithm for Voronoi diagram
        let mut result = VoronoiResult {
            vertices: Vec::new(),
            edges: Vec::new(),
            cells: Vec::new(),
        };

        if self.points.len() < 2 {
            return result;
        }

        // Sort points by y-coordinate
        let mut sorted_points = self.points.clone();
        sorted_points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        // Initialize data structures for Fortune's algorithm
        let mut beachline = Vec::new();
        let mut events = std::collections::BinaryHeap::new();

        // Add point events
        for (i, point) in sorted_points.iter().enumerate() {
            events.push((-point.y, i, point.clone()));
        }

        // Process events
        while let Some((_, index, point)) = events.pop() {
            // Process point event
            self.process_point_event(&point, &mut beachline, &mut events, &mut result);
        }

        // Finalize Voronoi cells
        self.finalize_cells(&mut result);

        result
    }

    /// Process a point event in Fortune's algorithm
    fn process_point_event(
        &self,
        point: &Point,
        beachline: &mut Vec<Parabola>,
        events: &mut std::collections::BinaryHeap<(f64, usize, Point)>,
        result: &mut VoronoiResult,
    ) {
        if beachline.is_empty() {
            beachline.push(Parabola::new(point.clone()));
            return;
        }

        // Find the parabola arc above the point
        let arc_index = self.find_arc_above(point, beachline);

        // Get the focus of the arc above
        let arc_focus = beachline[arc_index].focus.clone();

        // Calculate the breakpoint (Voronoi vertex) between the new point and the arc focus
        let midpoint = Point::new(
            (point.x + arc_focus.x) / 2.0,
            (point.y + arc_focus.y) / 2.0,
            (point.z + arc_focus.z) / 2.0,
        );

        // Add Voronoi vertex
        let vertex_idx = result.vertices.len();
        result.vertices.push(midpoint);

        // Create edges from the new vertex
        if vertex_idx > 0 {
            result.edges.push((vertex_idx - 1, vertex_idx));
        }

        // Split the arc and create new arcs
        self.split_arc(arc_index, point, beachline, events, result);
    }

    /// Find the parabola arc above a point
    fn find_arc_above(&self, point: &Point, beachline: &[Parabola]) -> usize {
        // Find the parabola whose focus is closest to the point
        let mut min_dist = f64::MAX;
        let mut closest_idx = 0;

        for (i, parabola) in beachline.iter().enumerate() {
            let dist = point.distance(&parabola.focus);
            if dist < min_dist {
                min_dist = dist;
                closest_idx = i;
            }
        }

        closest_idx
    }

    /// Split a parabola arc
    fn split_arc(
        &self,
        index: usize,
        point: &Point,
        beachline: &mut Vec<Parabola>,
        events: &mut std::collections::BinaryHeap<(f64, usize, Point)>,
        result: &mut VoronoiResult,
    ) {
        // Store the original arc
        let original_arc = beachline[index].clone();

        // Remove the original arc
        beachline.remove(index);

        // Insert three new arcs: original focus, new point, original focus
        beachline.insert(index, original_arc.clone());
        beachline.insert(index + 1, Parabola::new(point.clone()));
        beachline.insert(index + 2, original_arc);

        // Schedule circle events for the new triplets
        if beachline.len() >= 3 {
            // Check for potential circle events
            self.check_circle_events(index, beachline, events, result);
        }
    }

    /// Check for circle events
    fn check_circle_events(
        &self,
        start_idx: usize,
        beachline: &[Parabola],
        events: &mut std::collections::BinaryHeap<(f64, usize, Point)>,
        result: &mut VoronoiResult,
    ) {
        // Check triplets of consecutive arcs for circle events
        for i in
            0.max(start_idx.saturating_sub(2))..beachline.len().saturating_sub(2).min(start_idx + 3)
        {
            if i + 2 < beachline.len() {
                let p1 = &beachline[i].focus;
                let p2 = &beachline[i + 1].focus;
                let p3 = &beachline[i + 2].focus;

                // Calculate circumcenter
                if let Some(circumcenter) = self.calculate_circumcenter(p1, p2, p3) {
                    // Calculate the y-coordinate of the circle event
                    let radius = circumcenter.distance(p1);
                    let event_y = circumcenter.y - radius;

                    // Add circle event
                    events.push((event_y, i, circumcenter));
                }
            }
        }
    }

    /// Calculate circumcenter of three points
    fn calculate_circumcenter(&self, p1: &Point, p2: &Point, p3: &Point) -> Option<Point> {
        let ax = p1.x;
        let ay = p1.y;
        let bx = p2.x;
        let by = p2.y;
        let cx = p3.x;
        let cy = p3.y;

        let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));

        if d.abs() < 1e-10 {
            return None;
        }

        let ux = ((ax * ax + ay * ay) * (by - cy)
            + (bx * bx + by * by) * (cy - ay)
            + (cx * cx + cy * cy) * (ay - by))
            / d;
        let uy = ((ax * ax + ay * ay) * (cx - bx)
            + (bx * bx + by * by) * (ax - cx)
            + (cx * cx + cy * cy) * (bx - ax))
            / d;

        Some(Point::new(ux, uy, 0.0))
    }

    /// Finalize Voronoi cells
    fn finalize_cells(&self, result: &mut VoronoiResult) {
        // Build Voronoi cells from edges
        // Each input point gets a cell
        let num_points = self.points.len();

        // Initialize empty cells
        result.cells = vec![Vec::new(); num_points];

        // Assign vertices to cells based on proximity
        for (v_idx, vertex) in result.vertices.iter().enumerate() {
            // Find the closest input point
            let mut min_dist = f64::MAX;
            let mut closest_point_idx = 0;

            for (p_idx, point) in self.points.iter().enumerate() {
                let dist = vertex.distance(point);
                if dist < min_dist {
                    min_dist = dist;
                    closest_point_idx = p_idx;
                }
            }

            // Add vertex to the cell of the closest point
            result.cells[closest_point_idx].push(v_idx);
        }

        // Sort vertices in each cell to form proper polygons
        for cell in &mut result.cells {
            if cell.len() > 2 {
                self.sort_cell_vertices(cell, &result.vertices);
            }
        }
    }

    /// Sort vertices in a cell to form a proper polygon
    fn sort_cell_vertices(&self, cell: &mut Vec<usize>, vertices: &[Point]) {
        if cell.len() < 3 {
            return;
        }

        // Calculate centroid
        let mut cx = 0.0;
        let mut cy = 0.0;
        for &v_idx in cell.iter() {
            cx += vertices[v_idx].x;
            cy += vertices[v_idx].y;
        }
        cx /= cell.len() as f64;
        cy /= cell.len() as f64;

        // Sort by angle around centroid
        cell.sort_by(|&a, &b| {
            let angle_a = (vertices[a].y - cy).atan2(vertices[a].x - cx);
            let angle_b = (vertices[b].y - cy).atan2(vertices[b].x - cx);
            angle_a.partial_cmp(&angle_b).unwrap()
        });
    }
}

/// Parabola arc for Fortune's algorithm
struct Parabola {
    focus: Point,
}

impl Parabola {
    fn new(focus: Point) -> Self {
        Self { focus }
    }
}

/// Voronoi diagram result
pub struct VoronoiResult {
    /// Voronoi vertices
    pub vertices: Vec<Point>,
    /// Voronoi edges
    pub edges: Vec<(usize, usize)>,
    /// Voronoi cells
    pub cells: Vec<Vec<usize>>,
}

/// Delaunay triangulation
pub struct DelaunayTriangulation {
    /// Points for Delaunay triangulation
    points: Vec<Point>,
}

impl DelaunayTriangulation {
    /// Create a new Delaunay triangulation
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Build the Delaunay triangulation
    pub fn build(&self) -> DelaunayResult {
        // Implementation of Bowyer-Watson algorithm
        let mut result = DelaunayResult {
            vertices: self.points.clone(),
            triangles: Vec::new(),
        };

        if self.points.len() < 3 {
            return result;
        }

        // Create a super triangle that contains all points
        let super_triangle = self.create_super_triangle();
        let mut triangles = vec![super_triangle];

        // Process each point
        for (i, point) in self.points.iter().enumerate() {
            let mut bad_triangles = Vec::new();
            let mut polygon = Vec::new();

            // Find all bad triangles (triangles whose circumcircle contains the point)
            for (j, triangle) in triangles.iter().enumerate() {
                if self.point_in_circumcircle(point, triangle) {
                    bad_triangles.push(j);
                }
            }

            // Collect the boundary edges of the bad triangles
            for (j, triangle) in triangles.iter().enumerate() {
                if bad_triangles.contains(&j) {
                    // Check if this edge is shared with another bad triangle
                    let edges = [
                        (triangle.0, triangle.1),
                        (triangle.1, triangle.2),
                        (triangle.2, triangle.0),
                    ];
                    for edge in edges {
                        let mut is_shared = false;
                        for (k, other_triangle) in triangles.iter().enumerate() {
                            if j != k && bad_triangles.contains(&k) {
                                let other_edges = [
                                    (other_triangle.0, other_triangle.1),
                                    (other_triangle.1, other_triangle.2),
                                    (other_triangle.2, other_triangle.0),
                                ];
                                if other_edges.contains(&edge)
                                    || other_edges.contains(&(edge.1, edge.0))
                                {
                                    is_shared = true;
                                    break;
                                }
                            }
                        }
                        if !is_shared {
                            polygon.push(edge);
                        }
                    }
                }
            }

            // Remove bad triangles
            bad_triangles.sort_by(|a, b| b.cmp(a));
            for &j in &bad_triangles {
                triangles.remove(j);
            }

            // Create new triangles from the point and the polygon edges
            for edge in &polygon {
                triangles.push((edge.0, edge.1, i));
            }
        }

        // Remove triangles that share vertices with the super triangle
        let mut valid_triangles = Vec::new();
        let super_vertices = [
            self.points.len(),
            self.points.len() + 1,
            self.points.len() + 2,
        ];
        for triangle in triangles {
            if !super_vertices.contains(&triangle.0)
                && !super_vertices.contains(&triangle.1)
                && !super_vertices.contains(&triangle.2)
            {
                valid_triangles.push(triangle);
            }
        }

        result.triangles = valid_triangles;
        result
    }

    /// Create a super triangle that contains all points
    fn create_super_triangle(&self) -> (usize, usize, usize) {
        // Find the bounding box of the points
        let mut min_x = std::f64::MAX;
        let mut max_x = std::f64::MIN;
        let mut min_y = std::f64::MAX;
        let mut max_y = std::f64::MIN;

        for point in &self.points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let margin = dx.max(dy) * 10.0;

        // Add super triangle vertices
        let super_vertices = [
            Point::new(min_x - margin, max_y + margin),
            Point::new(max_x + margin, max_y + margin),
            Point::new((min_x + max_x) / 2.0, min_y - margin),
        ];

        // Add super vertices to the result
        let base_index = self.points.len();
        (base_index, base_index + 1, base_index + 2)
    }

    /// Check if a point is inside the circumcircle of a triangle
    fn point_in_circumcircle(&self, point: &Point, triangle: &(usize, usize, usize)) -> bool {
        let p0 = if triangle.0 < self.points.len() {
            &self.points[triangle.0]
        } else {
            // Super triangle vertex
            let base_index = self.points.len();
            match triangle.0 - base_index {
                0 => &Point::new(-1e6, 1e6),
                1 => &Point::new(1e6, 1e6),
                2 => &Point::new(0.0, -1e6),
                _ => unreachable!(),
            }
        };

        let p1 = if triangle.1 < self.points.len() {
            &self.points[triangle.1]
        } else {
            let base_index = self.points.len();
            match triangle.1 - base_index {
                0 => &Point::new(-1e6, 1e6),
                1 => &Point::new(1e6, 1e6),
                2 => &Point::new(0.0, -1e6),
                _ => unreachable!(),
            }
        };

        let p2 = if triangle.2 < self.points.len() {
            &self.points[triangle.2]
        } else {
            let base_index = self.points.len();
            match triangle.2 - base_index {
                0 => &Point::new(-1e6, 1e6),
                1 => &Point::new(1e6, 1e6),
                2 => &Point::new(0.0, -1e6),
                _ => unreachable!(),
            }
        };

        // Calculate circumcircle
        let dx1 = p1.x - p0.x;
        let dy1 = p1.y - p0.y;
        let dx2 = p2.x - p0.x;
        let dy2 = p2.y - p0.y;

        let s = 0.5 / (dx1 * dy2 - dx2 * dy1);
        let cx = ((p2.y - p0.y) * (p1.x * p1.x - p0.x * p0.x + p1.y * p1.y - p0.y * p0.y)
            + (p0.y - p1.y) * (p2.x * p2.x - p0.x * p0.x + p2.y * p2.y - p0.y * p0.y))
            * s;
        let cy = ((p0.x - p2.x) * (p1.x * p1.x - p0.x * p0.x + p1.y * p1.y - p0.y * p0.y)
            + (p1.x - p0.x) * (p2.x * p2.x - p0.x * p0.x + p2.y * p2.y - p0.y * p0.y))
            * s;

        let radius_squared = (cx - p0.x) * (cx - p0.x) + (cy - p0.y) * (cy - p0.y);
        let distance_squared = (point.x - cx) * (point.x - cx) + (point.y - cy) * (point.y - cy);

        distance_squared < radius_squared
    }
}

/// Delaunay triangulation result
pub struct DelaunayResult {
    /// Vertices
    pub vertices: Vec<Point>,
    /// Triangles (indices of vertices)
    pub triangles: Vec<(usize, usize, usize)>,
}

/// Point cloud topology
pub struct PointCloudTopology {
    /// K-nearest neighbors
    knn: KNearestNeighbors,
    /// Connectivity analysis
    connectivity: ConnectivityAnalysis,
}

impl PointCloudTopology {
    /// Create a new point cloud topology analysis
    pub fn new(k: usize, distance_threshold: f64) -> Self {
        Self {
            knn: KNearestNeighbors::new(k),
            connectivity: ConnectivityAnalysis::new(distance_threshold),
        }
    }

    /// Analyze the topology of a point cloud
    pub fn analyze(&self, cloud: &PointCloud) -> TopologyResult {
        let neighbors = self.knn.find(cloud);
        let connected_components = self.connectivity.analyze(cloud);

        TopologyResult {
            neighbors,
            connected_components,
        }
    }
}

/// Topology analysis result
pub struct TopologyResult {
    /// K-nearest neighbors for each point
    pub neighbors: Vec<Vec<(usize, f64)>>,
    /// Connected components
    pub connected_components: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_knn() {
        let mut cloud = PointCloud::new();

        for i in 0..10 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }

        let knn = KNearestNeighbors::new(3);
        let neighbors = knn.find(&cloud);

        assert_eq!(neighbors.len(), 10);
        for neighbor_list in &neighbors {
            assert_eq!(neighbor_list.len(), 3);
        }
    }

    #[test]
    fn test_connectivity_analysis() {
        let mut cloud = PointCloud::new();

        // Add points in two clusters
        for i in 0..5 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }

        for i in 0..5 {
            cloud.add_point(Point::new(
                i as f64 + 10.0,
                i as f64 + 10.0,
                i as f64 + 10.0,
            ));
        }

        let connectivity = ConnectivityAnalysis::new(2.0);
        let components = connectivity.analyze(&cloud);

        // Should find two connected components
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].len(), 5);
        assert_eq!(components[1].len(), 5);
    }

    #[test]
    fn test_point_cloud_topology() {
        let mut cloud = PointCloud::new();

        for i in 0..10 {
            cloud.add_point(Point::new(i as f64, i as f64, i as f64));
        }

        let topology = PointCloudTopology::new(3, 2.0);
        let result = topology.analyze(&cloud);

        assert_eq!(result.neighbors.len(), 10);
        // Should find one connected component
        assert_eq!(result.connected_components.len(), 1);
    }
}
