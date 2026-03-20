/// Edge classification for BSP tree
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeClassification {
    Front,
    Back,
    Coplanar,
    Spanning,
}
use crate::foundation::handle::Handle;
use crate::geometry::{Plane, Point};
use crate::topology::{TopoDsFace, TopoDsShape};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BspNode {
    pub plane: Plane,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub faces: Vec<Arc<TopoDsFace>>,
}

impl BspNode {
    pub fn new(plane: Plane) -> Self {
        Self {
            plane,
            front: None,
            back: None,
            faces: Vec::new(),
        }
    }

    pub fn insert(&mut self, face: &TopoDsFace, tolerance: f64) {
        // Classify face relative to node plane
        let classification = self.classify_face(face, tolerance);

        match classification {
            FaceClassification::Front => {
                if let Some(ref mut front) = self.front {
                    front.insert(face, tolerance);
                } else {
                    let plane = self.create_splitting_plane(face);
                    let mut new_node = BspNode::new(plane);
                    new_node.faces.push(Arc::new(face.clone()));
                    self.front = Some(Box::new(new_node));
                }
            }
            FaceClassification::Back => {
                if let Some(ref mut back) = self.back {
                    back.insert(face, tolerance);
                } else {
                    let plane = self.create_splitting_plane(face);
                    let mut new_node = BspNode::new(plane);
                    new_node.faces.push(Arc::new(face.clone()));
                    self.back = Some(Box::new(new_node));
                }
            }
            FaceClassification::Coplanar => {
                self.faces.push(Arc::new(face.clone()));
            }
            FaceClassification::Spanning => {
                // Split face and insert both parts
                if let Some((front_face, back_face)) = self.split_face(face, tolerance) {
                    self.insert(&front_face, tolerance);
                    self.insert(&back_face, tolerance);
                }
            }
        }
    }

    fn classify_face(&self, face: &TopoDsFace, tolerance: f64) -> FaceClassification {
        // Get face vertices
        let vertices = face.vertices();
        if vertices.is_empty() {
            return FaceClassification::Coplanar;
        }

        let mut front_count = 0;
        let mut back_count = 0;

        for vertex in vertices {
            if let Some(vertex_ref) = vertex.as_ref() {
                let point = vertex_ref.point();
                let distance = self.plane.distance(&point);

                if distance > tolerance {
                    front_count += 1;
                } else if distance < -tolerance {
                    back_count += 1;
                }
            }
        }

        if front_count > 0 && back_count > 0 {
            FaceClassification::Spanning
        } else if front_count > 0 {
            FaceClassification::Front
        } else if back_count > 0 {
            FaceClassification::Back
        } else {
            FaceClassification::Coplanar
        }
    }

    fn split_face(&self, face: &TopoDsFace, tolerance: f64) -> Option<(TopoDsFace, TopoDsFace)> {
        // Get face wires and vertices
        let wires = face.wires();
        if wires.is_empty() {
            return None;
        }

        // Get the outer wire
        let outer_wire = wires[0].get()?;
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();
        if edges_slice.len() < 3 {
            return None;
        }

        // Find intersection points between face edges and the splitting plane
        let mut intersection_points = Vec::new();
        let mut edge_classifications = Vec::new();

        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) = (edge.start_vertex().get(), edge.end_vertex().get()) {
                    let start_point = start.point();
                    let end_point = end.point();
                    let start_dist = self.plane.distance(&start_point);
                    let end_dist = self.plane.distance(&end_point);

                    // Classify edge
                    let classification = if start_dist > tolerance && end_dist > tolerance {
                        EdgeClassification::Front
                    } else if start_dist < -tolerance && end_dist < -tolerance {
                        EdgeClassification::Back
                    } else if start_dist.abs() <= tolerance && end_dist.abs() <= tolerance {
                        EdgeClassification::Coplanar
                    } else {
                        // Edge crosses the plane, find intersection
                        if let Some(intersection) = self.edge_plane_intersection(edge, tolerance) {
                            intersection_points.push(intersection);
                        }
                        EdgeClassification::Spanning
                    };
                    edge_classifications.push(classification);
                }
            }
        }

        // If no intersection points, no splitting needed
        if intersection_points.len() < 2 {
            return None;
        }

        // Create front and back faces using BRepBuilder
        use crate::modeling::brep_builder::BrepBuilder;
        let builder = BrepBuilder::new();

        // Create vertices for intersection points
        let mut intersection_vertices = Vec::new();
        for point in &intersection_points {
            let vertex = builder.make_vertex(*point);
            intersection_vertices.push(vertex);
        }

        // Reconstruct front face
        let front_face = self.reconstruct_face(face, &self.plane, true, tolerance);
        // Reconstruct back face
        let back_face = self.reconstruct_face(face, &self.plane, false, tolerance);

        if let (Some(front), Some(back)) = (front_face, back_face) {
            Some((front, back))
        } else {
            None
        }
    }

    /// Find intersection between edge and plane
    fn edge_plane_intersection(&self, edge: &crate::topology::TopoDsEdge, tolerance: f64) -> Option<crate::geometry::Point> {
        if let (Some(start), Some(end)) = (edge.start_vertex().get(), edge.end_vertex().get()) {
            let p1 = start.point();
            let p2 = end.point();
            
            let d1 = self.plane.distance(&p1);
            let d2 = self.plane.distance(&p2);
            
            // Check if edge crosses the plane
            if d1 * d2 < -tolerance * tolerance {
                // Calculate intersection point
                let t = d1 / (d1 - d2);
                let x = p1.x + t * (p2.x - p1.x);
                let y = p1.y + t * (p2.y - p1.y);
                let z = p1.z + t * (p2.z - p1.z);
                
                Some(crate::geometry::Point::new(x, y, z))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Reconstruct face on one side of the plane
    fn reconstruct_face(&self, face: &TopoDsFace, plane: &crate::geometry::Plane, front: bool, tolerance: f64) -> Option<TopoDsFace> {
        use crate::modeling::brep_builder::BrepBuilder;
        use crate::topology::TopoDsWire;
        
        let builder = BrepBuilder::new();
        let wires = face.wires();
        if wires.is_empty() {
            return None;
        }
        
        let outer_wire = wires[0].get()?;
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();
        
        // Collect vertices on the specified side
        let mut vertices = Vec::new();
        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) = (edge.start_vertex().get(), edge.end_vertex().get()) {
                    let start_point = *start.point();
                    let end_point = *end.point();
                    let start_dist = plane.distance(&start_point);
                    let end_dist = plane.distance(&end_point);
                    
                    // Add start vertex if on the specified side
                    if (front && start_dist > -tolerance) || (!front && start_dist < tolerance) {
                        let vertex = builder.make_vertex(start_point);
                        vertices.push(vertex);
                    }
                    
                    // Add intersection point if edge crosses the plane
                    if (start_dist > tolerance && end_dist < -tolerance) || (start_dist < -tolerance && end_dist > tolerance) {
                        if let Some(intersection) = self.edge_plane_intersection(edge, tolerance) {
                            let vertex = builder.make_vertex(intersection);
                            vertices.push(vertex);
                        }
                    }
                    
                    // Add end vertex if on the specified side
                    if (front && end_dist > -tolerance) || (!front && end_dist < tolerance) {
                        let vertex = builder.make_vertex(end_point);
                        vertices.push(vertex);
                    }
                }
            }
        }
        
        // Create wire from vertices
        if vertices.len() >= 3 {
            let mut wire = TopoDsWire::new();
            for i in 0..vertices.len() {
                let start = vertices[i].clone();
                let end = vertices[(i + 1) % vertices.len()].clone();
                let edge = builder.make_edge(start, end);
                wire.add_edge(edge);
            }
            
            // Create face from wire
            let wire_handle = crate::foundation::handle::Handle::new(std::sync::Arc::new(wire));
            builder.make_face_with_wire(wire_handle).as_ref().map(|f| f.clone())
        } else {
            None
        }
    }


    fn create_splitting_plane(&self, face: &TopoDsFace) -> Plane {
        // Create a plane from the face's surface
        if let Some(surface) = face.surface() {
            if let Some(surface_ref) = surface.as_ref() {
                // Get surface parameters and create a plane
                let (u_range, v_range) = surface_ref.parameter_range();
                let u_mid = (u_range.0 + u_range.1) / 2.0;
                let v_mid = (v_range.0 + v_range.1) / 2.0;
                let point = surface_ref.value(u_mid, v_mid);
                let normal = surface_ref.normal(u_mid, v_mid);
                return Plane::new(point, crate::geometry::Direction::from_vector(&normal), crate::geometry::Direction::x_axis());
            }
        }

        // Fallback plane
        Plane::new(
            Point::origin(),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        )
    }

    pub fn collect_faces(&self) -> Vec<TopoDsFace> {
        let mut faces = Vec::new();

        // Add current node's faces
        for face in &self.faces {
            faces.push(face.as_ref().clone());
        }

        // Recursively collect from children
        if let Some(ref front) = self.front {
            faces.extend(front.collect_faces());
        }
        if let Some(ref back) = self.back {
            faces.extend(back.collect_faces());
        }

        faces
    }

    pub fn collect_faces_parallel(&self) -> Vec<TopoDsFace> {
        let mut faces: Vec<TopoDsFace> =
            self.faces.par_iter().map(|f| f.as_ref().clone()).collect();

        // Collect from front child
        if let Some(ref front) = self.front {
            let front_faces = front.collect_faces_parallel();
            faces.extend(front_faces);
        }

        // Collect from back child
        if let Some(ref back) = self.back {
            let back_faces = back.collect_faces_parallel();
            faces.extend(back_faces);
        }

        faces
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FaceClassification {
    Front,
    Back,
    Coplanar,
    Spanning,
}

#[derive(Debug, Clone)]
pub struct BspTree {
    pub root: Option<Box<BspNode>>,
    pub tolerance: f64,
}

impl BspTree {
    pub fn new(tolerance: f64) -> Self {
        Self {
            root: None,
            tolerance,
        }
    }

    pub fn insert_face(&mut self, face: TopoDsFace) {
        if let Some(ref mut root) = self.root {
            root.insert(&face, self.tolerance);
        } else {
            // Create initial plane from face
            let plane = self.create_initial_plane(&face);
            let mut node = BspNode::new(plane);
            node.faces.push(Arc::new(face));
            self.root = Some(Box::new(node));
        }
    }

    fn create_initial_plane(&self, face: &TopoDsFace) -> Plane {
        if let Some(surface) = face.surface() {
            if let Some(surface_ref) = surface.as_ref() {
                let (u_range, v_range) = surface_ref.parameter_range();
                let u_mid = (u_range.0 + u_range.1) / 2.0;
                let v_mid = (v_range.0 + v_range.1) / 2.0;
                let point = surface_ref.value(u_mid, v_mid);
                let normal = surface_ref.normal(u_mid, v_mid);
                return Plane::new(point, crate::geometry::Direction::from_vector(&normal), crate::geometry::Direction::x_axis());
            }
        }

        // Fallback plane
        Plane::new(
            Point::origin(),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        )
    }

    pub fn build(&mut self, faces: &[TopoDsFace]) {
        for face in faces {
            self.insert_face(face.clone());
        }
    }

    pub fn build_parallel(&mut self, faces: &[TopoDsFace]) {
        // For small number of faces, build sequentially
        if faces.len() < 10 {
            for face in faces {
                self.insert_face(face.clone());
            }
            return;
        }

        // Build initial tree with first face
        if self.root.is_none() && !faces.is_empty() {
            let plane = self.create_initial_plane(&faces[0]);
            let mut node = BspNode::new(plane);
            node.faces.push(Arc::new(faces[0].clone()));
            self.root = Some(Box::new(node));
        }

        // Insert remaining faces in parallel
        if self.root.is_some() {
            // Create a thread-safe vector to hold results from parallel processing
            use std::sync::{Arc, Mutex};
            let subtrees = Arc::new(Mutex::new(Vec::new()));
            
            // Process faces in chunks in parallel
            faces[1..].par_chunks(100).for_each(|chunk| {
                let mut subtree = BspTree::new(self.tolerance);
                // Use the same initial plane as the main tree
                if let Some(ref root_node) = self.root {
                    let plane = root_node.plane.clone();
                    let node = BspNode::new(plane);
                    subtree.root = Some(Box::new(node));
                }
                
                // Insert faces into subtree
                for face in chunk {
                    subtree.insert_face(face.clone());
                }
                
                // Add subtree to results
                subtrees.lock().unwrap().push(subtree);
            });
            
            // Merge all subtrees back into the main tree
            for subtree in Arc::try_unwrap(subtrees).unwrap().into_inner().unwrap() {
                if let Some(sub_root) = subtree.root {
                    self.merge_tree(&sub_root);
                }
            }
        }
    }

    pub fn union(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Add all faces from self
        let self_faces = self.collect_all_faces();
        for face in self_faces {
            result.insert_face(face);
        }

        // Add all faces from other
        let other_faces = other.collect_all_faces();
        for face in other_faces {
            result.insert_face(face);
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }

    pub fn difference(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Get all faces from self
        let self_faces = self.collect_all_faces();
        let _other_faces = other.collect_all_faces();

        // For each face in self, check if it's inside other
        for face in self_faces {
            if !self.face_inside_tree(&face, other) {
                result.insert_face(face);
            }
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }

    pub fn intersection(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Get all faces from both trees
        let self_faces = self.collect_all_faces();
        let other_faces = other.collect_all_faces();

        // For each face in self, check if it's inside other
        for face in self_faces {
            if self.face_inside_tree(&face, other) {
                result.insert_face(face);
            }
        }

        // For each face in other, check if it's inside self
        for face in other_faces {
            if self.point_inside_tree(&self.face_center(&face), self) {
                result.insert_face(face);
            }
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }
    
    /// Check if a face is inside a BSP tree
    fn face_inside_tree(&self, face: &TopoDsFace, tree: &BspTree) -> bool {
        // Get face center
        let center = self.face_center(face);
        
        // Check if center is inside the tree
        self.point_inside_tree(&center, tree)
    }
    
    /// Get face center
    fn face_center(&self, face: &TopoDsFace) -> crate::geometry::Point {
        let wires = face.wires();
        if wires.is_empty() {
            return crate::geometry::Point::origin();
        }
        
        let outer_wire = wires[0].get().unwrap();
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();
        
        let mut center = crate::geometry::Point::origin();
        let mut count = 0;
        
        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) = (edge.start_vertex().get(), edge.end_vertex().get()) {
                    let start_point = start.point();
                    let end_point = end.point();
                    
                    center.x += (start_point.x + end_point.x) / 2.0;
                    center.y += (start_point.y + end_point.y) / 2.0;
                    center.z += (start_point.z + end_point.z) / 2.0;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            center.x /= count as f64;
            center.y /= count as f64;
            center.z /= count as f64;
        }
        
        center
    }
    
    /// Check if a point is inside a BSP tree
    fn point_inside_tree(&self, point: &crate::geometry::Point, tree: &BspTree) -> bool {
        if let Some(ref root) = tree.root {
            self.point_inside_node(point, root)
        } else {
            false
        }
    }
    
    /// Check if a point is inside a BSP node
    fn point_inside_node(&self, point: &crate::geometry::Point, node: &BspNode) -> bool {
        let distance = node.plane.distance(point);
        
        if distance > self.tolerance {
            // Point is in front of the plane
            if let Some(ref front) = node.front {
                return self.point_inside_node(point, front);
            }
            return false;
        } else if distance < -self.tolerance {
            // Point is behind the plane
            if let Some(ref back) = node.back {
                return self.point_inside_node(point, back);
            }
            return false;
        } else {
            // Point is on the plane
            // Check if point is inside any of the faces on this node
            for face in &node.faces {
                if self.point_inside_face(point, face.as_ref()) {
                    return true;
                }
            }
            
            // Check child nodes
            if let Some(ref front) = node.front {
                if self.point_inside_node(point, front) {
                    return true;
                }
            }
            if let Some(ref back) = node.back {
                if self.point_inside_node(point, back) {
                    return true;
                }
            }
            
            return false;
        }
    }
    
    /// Check if a point is inside a face
    fn point_inside_face(&self, point: &crate::geometry::Point, face: &TopoDsFace) -> bool {
        let wires = face.wires();
        if wires.is_empty() {
            return false;
        }
        
        let outer_wire = wires[0].get().unwrap();
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();
        
        // Ray casting algorithm
        let mut intersection_count = 0;
        let ray_dir = crate::geometry::Vector::new(1.0, 0.0, 0.0);
        
        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) = (edge.start_vertex().get(), edge.end_vertex().get()) {
                    let start_point = start.point();
                    let end_point = end.point();
                    
                    // Check if ray intersects this edge
                    if self.ray_intersects_edge(point, &ray_dir, &start_point, &end_point) {
                        intersection_count += 1;
                    }
                }
            }
        }
        
        // If odd, point is inside
        intersection_count % 2 == 1
    }
    
    /// Check if a ray intersects an edge
    fn ray_intersects_edge(&self, origin: &crate::geometry::Point, dir: &crate::geometry::Vector, p1: &crate::geometry::Point, p2: &crate::geometry::Point) -> bool {
        // Implementation of ray-edge intersection
        let edge_vec = crate::geometry::Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let ray_vec = dir;
        
        let cross = edge_vec.cross(&ray_vec);
        let denom = cross.magnitude();
        
        if denom < 1e-6 {
            return false; // Ray and edge are parallel
        }
        
        let origin_to_p1 = crate::geometry::Vector::new(p1.x - origin.x, p1.y - origin.y, p1.z - origin.z);
        let t = origin_to_p1.cross(&edge_vec).dot(&cross) / (denom * denom);
        
        if t < 0.0 {
            return false; // Intersection is behind ray origin
        }
        
        let u = origin_to_p1.cross(&ray_vec).dot(&cross) / (denom * denom);
        
        u >= 0.0 && u <= 1.0
    }
    
    /// Optimize the BSP tree
    pub fn optimize(&mut self) {
        // Implement tree optimization here
        // This could include balancing the tree, removing redundant nodes, etc.
    }

    pub fn collect_all_faces(&self) -> Vec<TopoDsFace> {
        if let Some(ref root) = self.root {
            root.collect_faces()
        } else {
            Vec::new()
        }
    }

    pub fn collect_all_faces_parallel(&self) -> Vec<TopoDsFace> {
        if let Some(ref root) = self.root {
            root.collect_faces_parallel()
        } else {
            Vec::new()
        }
    }

    /// Merge another BSP tree into this tree
    pub fn merge_tree(&mut self, other: &BspNode) {
        // Collect all faces from the other tree
        let faces = other.collect_faces();
        for face in faces {
            self.insert_face(face);
        }
        
        // Recursively merge child nodes
        if let Some(ref front) = other.front {
            if self.root.as_mut().unwrap().front.is_some() {
                self.merge_tree(front);
            } else {
                // If we don't have a front node, create one
                if let Some(ref mut root) = self.root {
                    root.front = Some(front.clone());
                }
            }
        }
        
        if let Some(ref back) = other.back {
            if self.root.as_mut().unwrap().back.is_some() {
                self.merge_tree(back);
            } else {
                // If we don't have a back node, create one
                if let Some(ref mut root) = self.root {
                    root.back = Some(back.clone());
                }
            }
        }
    }

    pub fn difference_with_tolerance(&self, other: &BspTree, tolerance: f64) -> BspTree {
        let mut result = BspTree::new(tolerance);

        // Get all faces from self
        let self_faces = self.collect_all_faces();
        let other_faces = other.collect_all_faces();

        // Add faces from self that are not within tolerance of other faces
        for face in self_faces {
            let mut found = false;
            for other_face in &other_faces {
                if self.faces_are_close(&face, other_face, tolerance) {
                    found = true;
                    break;
                }
            }
            if !found {
                result.insert_face(face);
            }
        }

        result
    }

    fn faces_are_close(&self, face1: &TopoDsFace, face2: &TopoDsFace, tolerance: f64) -> bool {
        // Check if faces are close based on bounding boxes
        if let (Some((min1, _max1)), Some((min2, _max2))) = (face1.bounding_box(), face2.bounding_box()) {
            // Check if bounding boxes overlap within tolerance
            let dx = (min1.x - min2.x).abs();
            let dy = (min1.y - min2.y).abs();
            let dz = (min1.z - min2.z).abs();

            return dx <= tolerance && dy <= tolerance && dz <= tolerance;
        }
        
        false
    }
}

/// BSP tree builder
#[derive(Debug, Clone)]
pub struct BspTreeBuilder {
    tolerance: f64,
}

impl BspTreeBuilder {
    /// Create a new BSP tree builder
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Build BSP tree from a shape
    pub fn build_from_shape(&self, shape: &Handle<TopoDsShape>) -> BspTree {
        let mut tree = BspTree::new(self.tolerance);

        // Extract faces from the shape based on its type
        match shape.shape_type() {
            crate::topology::ShapeType::Solid => {
                // For solids, get faces from the solid's shells
                if let Some(solid) = shape.as_solid() {
                    let shells = solid.shells();
                    if shells.len() > 10 {
                        // Parallel processing for large number of shells
                        let faces: Vec<TopoDsFace> = shells
                            .par_iter()
                            .filter_map(|shell_handle| shell_handle.get())
                            .flat_map(|shell| {
                                shell.faces()
                                    .iter()
                                    .filter_map(|face_handle| face_handle.get())
                                    .map(|face| face.as_ref().clone())
                                    .collect::<Vec<TopoDsFace>>()
                            })
                            .collect();
                        tree.build_parallel(&faces);
                    } else {
                        // Sequential processing for small number of shells
                        for shell_handle in shells {
                            if let Some(shell) = shell_handle.get() {
                                for face_handle in shell.faces() {
                                    if let Some(face) = face_handle.get() {
                                        tree.insert_face(face.as_ref().clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            crate::topology::ShapeType::Shell => {
                // For shells, extract faces directly
                if let Some(shell) = shape.as_shell() {
                    let faces = shell.faces();
                    if faces.len() > 100 {
                        // Parallel processing for large number of faces
                        let face_vec: Vec<TopoDsFace> = faces
                            .par_iter()
                            .filter_map(|face_handle| face_handle.get())
                            .map(|face| face.as_ref().clone())
                            .collect();
                        tree.build_parallel(&face_vec);
                    } else {
                        // Sequential processing for small number of faces
                        for face_handle in faces {
                            if let Some(face) = face_handle.get() {
                                tree.insert_face(face.as_ref().clone());
                            }
                        }
                    }
                }
            }
            crate::topology::ShapeType::Face => {
                // For faces, add the face directly
                if let Some(face) = shape.as_face() {
                    tree.insert_face(face.clone());
                }
            }
            crate::topology::ShapeType::Compound => {
                // For compounds, recursively extract faces from components
                if let Some(compound) = shape.as_compound() {
                    let components = compound.components();
                    if components.len() > 10 {
                        // Parallel processing for large number of components
                        let component_trees: Vec<BspTree> = components
                            .par_iter()
                            .map(|component| {
                                let component_handle = Handle::new(std::sync::Arc::new(component.clone()));
                                self.build_from_shape(&component_handle)
                            })
                            .collect();
                        
                        // Merge all component trees
                        for component_tree in component_trees {
                            if let Some(root) = component_tree.root {
                                tree.merge_tree(&root);
                            }
                        }
                    } else {
                        // Sequential processing for small number of components
                        for component in components {
                            let component_handle = Handle::new(std::sync::Arc::new(component.clone()));
                            let component_tree = self.build_from_shape(&component_handle);
                            if let Some(root) = component_tree.root {
                                tree.merge_tree(&root);
                            }
                        }
                    }
                }
            }
            _ => {
                // For other types, no faces to extract
            }
        }

        tree
    }
}

pub trait BspCommand {
    fn apply(&mut self, tree: &mut BspTree);
    fn undo(&mut self, tree: &mut BspTree);
}
