/// Topology validation and repair module
///
/// This module provides comprehensive validation and repair functionality
/// for topological shapes, ensuring topological consistency and integrity.
use crate::foundation::handle::Handle;
use crate::topology::shape_enum::ShapeType;
use crate::topology::{topods_compound::TopoDsCompound, topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_shell::TopoDsShell, topods_solid::TopoDsSolid, topods_vertex::TopoDsVertex, topods_wire::TopoDsWire, TopoDsShape};
use std::collections::HashSet;

/// Validation error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Vertex-related errors
    VertexError(String),
    /// Edge-related errors
    EdgeError(String),
    /// Wire-related errors
    WireError(String),
    /// Face-related errors
    FaceError(String),
    /// Shell-related errors
    ShellError(String),
    /// Solid-related errors
    SolidError(String),
    /// Compound-related errors
    CompoundError(String),
    /// General topology errors
    TopologyError(String),
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the shape is valid
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    /// List of warnings
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a new valid validation result
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a new invalid validation result
    pub fn invalid(errors: Vec<ValidationError>, warnings: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings,
        }
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Combine two validation results
    pub fn combine(&mut self, other: ValidationResult) {
        self.is_valid &= other.is_valid;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

/// Topology validator
///
/// This class provides comprehensive validation and repair functionality
/// for topological shapes.
pub struct TopologyValidator {
    /// Validation tolerance
    tolerance: f64,
}

impl TopologyValidator {
    /// Create a new topology validator with default tolerance
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
        }
    }

    /// Create a new topology validator with specified tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            tolerance,
        }
    }

    /// Validate a shape
    pub fn validate(&self, shape: &TopoDsShape) -> ValidationResult {
        match shape.shape_type() {
            ShapeType::Vertex => self.validate_vertex(shape),
            ShapeType::Edge => self.validate_edge(shape),
            ShapeType::Wire => self.validate_wire(shape),
            ShapeType::Face => self.validate_face(shape),
            ShapeType::Shell => self.validate_shell(shape),
            ShapeType::Solid => self.validate_solid(shape),
            ShapeType::Compound => self.validate_compound(shape),
            ShapeType::CompSolid => self.validate_comp_solid(shape),
        }
    }

    /// Validate a vertex
    fn validate_vertex(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsVertex
        if let Ok(vertex) = shape.downcast::<TopoDsVertex>() {
            // Check if vertex has a valid point
            let point = vertex.point();
            if !self.is_valid_point(point) {
                result.add_error(ValidationError::VertexError("Vertex has invalid point coordinates".to_string()));
            }
            
            // Check if tolerance is non-negative
            if vertex.tolerance() < 0.0 {
                result.add_error(ValidationError::VertexError("Vertex has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::VertexError("Failed to cast to TopoDsVertex".to_string()));
        }
        
        result
    }

    /// Validate an edge
    fn validate_edge(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsEdge
        if let Ok(edge) = shape.downcast::<TopoDsEdge>() {
            // Check if edge has valid vertices
            let v1 = edge.vertex1();
            let v2 = edge.vertex2();
            
            if v1.is_null() || v2.is_null() {
                result.add_error(ValidationError::EdgeError("Edge has null vertices".to_string()));
            } else {
                // Check if vertices are different
                if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
                    let p1 = v1_ref.point();
                    let p2 = v2_ref.point();
                    
                    if p1.distance(&p2) < self.tolerance {
                        result.add_error(ValidationError::EdgeError("Edge is degenerate (vertices are coincident)".to_string()));
                    }
                }
            }
            
            // Check if edge has a curve (if applicable)
            if edge.curve().is_none() {
                result.add_warning("Edge has no curve defined".to_string());
            }
            
            // Check if tolerance is non-negative
            if edge.tolerance() < 0.0 {
                result.add_error(ValidationError::EdgeError("Edge has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::EdgeError("Failed to cast to TopoDsEdge".to_string()));
        }
        
        result
    }

    /// Validate a wire
    fn validate_wire(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsWire
        if let Ok(wire) = shape.downcast::<TopoDsWire>() {
            // Check if wire has edges
            let edges = wire.edges();
            if edges.is_empty() {
                result.add_error(ValidationError::WireError("Wire has no edges".to_string()));
            } else {
                // Check edge connectivity
                if !self.validate_wire_connectivity(&edges) {
                    result.add_error(ValidationError::WireError("Wire edges are not properly connected".to_string()));
                }
                
                // Check for duplicate edges
                if self.has_duplicate_edges(&edges) {
                    result.add_warning("Wire contains duplicate edges".to_string());
                }
            }
            
            // Check if tolerance is non-negative
            if wire.tolerance() < 0.0 {
                result.add_error(ValidationError::WireError("Wire has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::WireError("Failed to cast to TopoDsWire".to_string()));
        }
        
        result
    }

    /// Validate a face
    fn validate_face(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsFace
        if let Ok(face) = shape.downcast::<TopoDsFace>() {
            // Check if face has wires
            let wires = face.wires();
            if wires.is_empty() {
                result.add_error(ValidationError::FaceError("Face has no wires".to_string()));
            } else {
                // Validate each wire
                for wire in &wires {
                    if let Some(wire_ref) = wire.get() {
                        let wire_result = self.validate_wire(&wire_ref.shape());
                        result.combine(wire_result);
                    }
                }
            }
            
            // Check if face has a surface
            if face.surface().is_none() {
                result.add_error(ValidationError::FaceError("Face has no surface defined".to_string()));
            }
            
            // Check if tolerance is non-negative
            if face.tolerance() < 0.0 {
                result.add_error(ValidationError::FaceError("Face has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::FaceError("Failed to cast to TopoDsFace".to_string()));
        }
        
        result
    }

    /// Validate a shell
    fn validate_shell(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsShell
        if let Ok(shell) = shape.downcast::<TopoDsShell>() {
            // Check if shell has faces
            let faces = shell.faces();
            if faces.is_empty() {
                result.add_error(ValidationError::ShellError("Shell has no faces".to_string()));
            } else {
                // Validate each face
                for face in &faces {
                    if let Some(face_ref) = face.get() {
                        let face_result = self.validate_face(&face_ref.shape());
                        result.combine(face_result);
                    }
                }
                
                // Check face connectivity
                if !self.validate_shell_connectivity(&faces) {
                    result.add_warning("Shell faces may not be properly connected".to_string());
                }
            }
            
            // Check if tolerance is non-negative
            if shell.tolerance() < 0.0 {
                result.add_error(ValidationError::ShellError("Shell has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::ShellError("Failed to cast to TopoDsShell".to_string()));
        }
        
        result
    }

    /// Validate a solid
    fn validate_solid(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsSolid
        if let Ok(solid) = shape.downcast::<TopoDsSolid>() {
            // Check if solid has shells
            let shells = solid.shells();
            if shells.is_empty() {
                result.add_error(ValidationError::SolidError("Solid has no shells".to_string()));
            } else {
                // Validate each shell
                for shell in &shells {
                    if let Some(shell_ref) = shell.get() {
                        let shell_result = self.validate_shell(&shell_ref.shape());
                        result.combine(shell_result);
                    }
                }
            }
            
            // Check if tolerance is non-negative
            if solid.tolerance() < 0.0 {
                result.add_error(ValidationError::SolidError("Solid has negative tolerance".to_string()));
            }
        } else {
            result.add_error(ValidationError::SolidError("Failed to cast to TopoDsSolid".to_string()));
        }
        
        result
    }

    /// Validate a compound
    fn validate_compound(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsCompound
        if let Ok(compound) = shape.downcast::<TopoDsCompound>() {
            // Validate each component
            for component in compound.components() {
                if let Some(component_ref) = component.get() {
                    let component_result = self.validate(component_ref);
                    result.combine(component_result);
                }
            }
        } else {
            result.add_error(ValidationError::CompoundError("Failed to cast to TopoDsCompound".to_string()));
        }
        
        result
    }

    /// Validate a CompSolid
    fn validate_comp_solid(&self, shape: &TopoDsShape) -> ValidationResult {
        let mut result = ValidationResult::valid();
        
        // Cast to TopoDsCompSolid
        if let Ok(comp_solid) = shape.downcast::<TopoDsCompound>() {
            // Validate each component
            for component in comp_solid.components() {
                if let Some(component_ref) = component.get() {
                    // Check if component is a solid
                    if !component_ref.is_solid() {
                        result.add_error(ValidationError::CompoundError("CompSolid contains non-solid component".to_string()));
                    }
                    let component_result = self.validate(component_ref);
                    result.combine(component_result);
                }
            }
        } else {
            result.add_error(ValidationError::CompoundError("Failed to cast to TopoDsCompSolid".to_string()));
        }
        
        result
    }

    /// Check if wire edges are properly connected
    fn validate_wire_connectivity(&self, edges: &[Handle<TopoDsEdge>]) -> bool {
        if edges.len() < 2 {
            return true; // Single edge or empty wire is trivially connected
        }
        
        // Check if edges are connected end-to-end
        for i in 0..edges.len() - 1 {
            let current_edge = edges[i].get().unwrap();
            let next_edge = edges[i + 1].get().unwrap();
            
            let current_end = current_edge.end_vertex();
            let next_start = next_edge.start_vertex();
            
            if let (Some(current_end_ref), Some(next_start_ref)) = (current_end.get(), next_start.get()) {
                let current_end_point = current_end_ref.point();
                let next_start_point = next_start_ref.point();
                
                if current_end_point.distance(&next_start_point) > self.tolerance {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }

    /// Check if wire has duplicate edges
    fn has_duplicate_edges(&self, edges: &[Handle<TopoDsEdge>]) -> bool {
        let mut seen = HashSet::new();
        for edge in edges {
            let edge_id = edge.shape_id();
            if !seen.insert(edge_id) {
                return true;
            }
        }
        false
    }

    /// Check if shell faces are properly connected
    fn validate_shell_connectivity(&self, faces: &[Handle<TopoDsFace>]) -> bool {
        if faces.len() < 2 {
            return true; // Single face or empty shell is trivially connected
        }
        
        // Check if there are shared edges between faces
        let mut edge_face_map = std::collections::HashMap::new();
        
        for face in faces {
            if let Some(face_ref) = face.get() {
                let wires = face_ref.wires();
                for wire in wires {
                    if let Some(wire_ref) = wire.get() {
                        let edges = wire_ref.edges();
                        for edge in edges {
                            edge_face_map.entry(edge.shape_id()).or_insert(Vec::new()).push(face.clone());
                        }
                    }
                }
            }
        }
        
        // Check if all edges are shared by exactly two faces (for closed shell)
        for (edge_id, adjacent_faces) in edge_face_map {
            if adjacent_faces.len() != 2 {
                return false;
            }
        }
        
        true
    }

    /// Check if a point is valid (not NaN or infinite)
    fn is_valid_point(&self, point: &crate::geometry::Point) -> bool {
        !point.x.is_nan() && !point.x.is_infinite() &&
        !point.y.is_nan() && !point.y.is_infinite() &&
        !point.z.is_nan() && !point.z.is_infinite()
    }

    /// Repair a shape
    pub fn repair(&self, shape: &mut TopoDsShape) -> bool {
        match shape.shape_type() {
            ShapeType::Vertex => self.repair_vertex(shape),
            ShapeType::Edge => self.repair_edge(shape),
            ShapeType::Wire => self.repair_wire(shape),
            ShapeType::Face => self.repair_face(shape),
            ShapeType::Shell => self.repair_shell(shape),
            ShapeType::Solid => self.repair_solid(shape),
            ShapeType::Compound => self.repair_compound(shape),
            ShapeType::CompSolid => self.repair_comp_solid(shape),
        }
    }

    /// Repair a vertex
    fn repair_vertex(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsVertex
        if let Ok(vertex) = shape.downcast_mut::<TopoDsVertex>() {
            // Ensure tolerance is non-negative
            if vertex.tolerance() < 0.0 {
                vertex.set_tolerance(0.0);
            }
            
            // Ensure point is valid
            let point = vertex.point();
            if !self.is_valid_point(point) {
                vertex.set_point(&crate::geometry::Point::origin());
            }
            
            true
        } else {
            false
        }
    }

    /// Repair an edge
    fn repair_edge(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsEdge
        if let Ok(edge) = shape.downcast_mut::<TopoDsEdge>() {
            // Ensure tolerance is non-negative
            if edge.tolerance() < 0.0 {
                edge.set_tolerance(0.0);
            }
            
            // Ensure vertices are valid
            let v1 = edge.vertex1();
            let v2 = edge.vertex2();
            
            if v1.is_null() || v2.is_null() {
                // Create default vertices if null
                let default_vertex = Handle::new(std::sync::Arc::new(TopoDsVertex::new(crate::geometry::Point::origin())));
                edge.set_vertices([default_vertex.clone(), default_vertex]);
            } else {
                // Ensure vertices are different
                if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
                    let p1 = v1_ref.point();
                    let p2 = v2_ref.point();
                    
                    if p1.distance(&p2) < self.tolerance {
                        // Create a non-degenerate edge
                        let v2_new = TopoDsVertex::new(crate::geometry::Point::new(p1.x + 1.0, p1.y, p1.z));
                        edge.set_vertices([v1, Handle::new(std::sync::Arc::new(v2_new))]);
                    }
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a wire
    fn repair_wire(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsWire
        if let Ok(wire) = shape.downcast_mut::<TopoDsWire>() {
            // Ensure tolerance is non-negative
            if wire.tolerance() < 0.0 {
                wire.set_tolerance(0.0);
            }
            
            // Remove duplicate edges
            let edges = wire.edges();
            let mut unique_edges = Vec::new();
            let mut seen = HashSet::new();
            
            for edge in edges {
                let edge_id = edge.shape_id();
                if seen.insert(edge_id) {
                    unique_edges.push(edge);
                }
            }
            
            // Reconstruct wire with unique edges
            if unique_edges != edges {
                let mut new_wire = TopoDsWire::new();
                for edge in unique_edges {
                    new_wire.add_edge(edge);
                }
                *wire = new_wire;
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a face
    fn repair_face(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsFace
        if let Ok(face) = shape.downcast_mut::<TopoDsFace>() {
            // Ensure tolerance is non-negative
            if face.tolerance() < 0.0 {
                face.set_tolerance(0.0);
            }
            
            // Ensure face has at least one wire
            let wires = face.wires();
            if wires.is_empty() {
                // Create a default wire
                let default_wire = TopoDsWire::new();
                face.add_wire(Handle::new(std::sync::Arc::new(default_wire)));
            }
            
            // Repair each wire
            for wire in face.wires_mut() {
                if let Some(wire_ref) = wire.get_mut() {
                    self.repair_wire(&mut wire_ref.shape());
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a shell
    fn repair_shell(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsShell
        if let Ok(shell) = shape.downcast_mut::<TopoDsShell>() {
            // Ensure tolerance is non-negative
            if shell.tolerance() < 0.0 {
                shell.set_tolerance(0.0);
            }
            
            // Repair each face
            for face in shell.faces_mut() {
                if let Some(face_ref) = face.get_mut() {
                    self.repair_face(&mut face_ref.shape());
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a solid
    fn repair_solid(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsSolid
        if let Ok(solid) = shape.downcast_mut::<TopoDsSolid>() {
            // Ensure tolerance is non-negative
            if solid.tolerance() < 0.0 {
                solid.set_tolerance(0.0);
            }
            
            // Repair each shell
            for shell in solid.shells_mut() {
                if let Some(shell_ref) = shell.get_mut() {
                    self.repair_shell(&mut shell_ref.shape());
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a compound
    fn repair_compound(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsCompound
        if let Ok(compound) = shape.downcast_mut::<TopoDsCompound>() {
            // Repair each component
            for component in compound.components_mut() {
                if let Some(component_ref) = component.get_mut() {
                    self.repair(component_ref);
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Repair a CompSolid
    fn repair_comp_solid(&self, shape: &mut TopoDsShape) -> bool {
        // Cast to TopoDsCompSolid
        if let Ok(comp_solid) = shape.downcast_mut::<TopoDsCompound>() {
            // Repair each component
            for component in comp_solid.components_mut() {
                if let Some(component_ref) = component.get_mut() {
                    self.repair(component_ref);
                }
            }
            
            true
        } else {
            false
        }
    }
}

impl Default for TopologyValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for Validatable
pub trait ValidatableExt {
    /// Validate with detailed error reporting
    fn validate_detailed(&self) -> ValidationResult;
    
    /// Repair with the default validator
    fn repair_default(&mut self) -> bool;
}

impl<T: crate::api::traits::Validatable> ValidatableExt for T {
    fn validate_detailed(&self) -> ValidationResult {
        let validator = TopologyValidator::new();
        // For now, return a basic validation result
        // In a real implementation, this would use the validator
        ValidationResult::valid()
    }
    
    fn repair_default(&mut self) -> bool {
        let validator = TopologyValidator::new();
        // For now, just call the existing fix method
        self.fix()
    }
}
