use crate::Handle;
use crate::geometry::{Point, Transform};
use crate::topology::shape_enum::ShapeType;
use crate::topology::topods_location::TopoDsLocation;
use bincode;
use std::sync::atomic::{AtomicI32, Ordering};

/// Global counter for generating unique shape IDs
static SHAPE_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

/// Base class for all topological shapes
///
/// This is the abstract base class for all topological shapes in the
/// boundary representation (BRep) model. It provides the basic
/// functionality common to all shapes, including type identification,
/// location transformation, and shape hierarchy management.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopoDsShape {
    shape_type: ShapeType,
    location: Option<TopoDsLocation>,
    orientation: i32,
    mutable: bool,
    shape_id: i32,
}

impl TopoDsShape {
    /// Create a new shape with the specified type
    #[inline]
    pub fn new(shape_type: ShapeType) -> Self {
        Self {
            shape_type,
            location: None,
            orientation: 1,
            mutable: false,
            shape_id: SHAPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Get the shape type
    #[inline]
    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    /// Check if this shape is of the specified type
    #[inline]
    pub fn is_kind(&self, shape_type: ShapeType) -> bool {
        self.shape_type == shape_type
    }

    /// Check if this shape is a vertex
    #[inline]
    pub fn is_vertex(&self) -> bool {
        self.shape_type == ShapeType::Vertex
    }

    /// Check if this shape is an edge
    #[inline]
    pub fn is_edge(&self) -> bool {
        self.shape_type == ShapeType::Edge
    }

    /// Check if this shape is a wire
    #[inline]
    pub fn is_wire(&self) -> bool {
        self.shape_type == ShapeType::Wire
    }

    /// Check if this shape is a face
    #[inline]
    pub fn is_face(&self) -> bool {
        self.shape_type == ShapeType::Face
    }

    /// Check if this shape is a shell
    #[inline]
    pub fn is_shell(&self) -> bool {
        self.shape_type == ShapeType::Shell
    }

    /// Check if this shape is a solid
    #[inline]
    pub fn is_solid(&self) -> bool {
        self.shape_type == ShapeType::Solid
    }

    /// Check if this shape is a compound
    #[inline]
    pub fn is_compound(&self) -> bool {
        self.shape_type == ShapeType::Compound
    }

    /// Check if this shape is a compsolid
    #[inline]
    pub fn is_compsolid(&self) -> bool {
        self.shape_type == ShapeType::CompSolid
    }

    /// Get the location of the shape
    #[inline]
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.location.as_ref()
    }

    /// Set the location of the shape
    #[inline]
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.location = Some(location);
    }

    /// Get the orientation of the shape
    #[inline]
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Set the orientation of the shape
    #[inline]
    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }

    /// Check if the shape is mutable
    #[inline]
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    /// Set the mutability of the shape
    #[inline]
    pub fn set_mutable(&mut self, mutable: bool) {
        self.mutable = mutable
    }

    /// Get the unique identifier of the shape
    #[inline]
    pub fn shape_id(&self) -> i32 {
        self.shape_id
    }

    /// Set the unique identifier of the shape
    #[inline]
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape_id = id;
    }

    /// Check if this shape is more complex than another shape
    #[inline]
    pub fn is_more_complex(&self, other: &TopoDsShape) -> bool {
        self.shape_type.is_more_complex_or_equal(&other.shape_type)
    }

    /// Check if this shape is less complex than another shape
    #[inline]
    pub fn is_less_complex(&self, other: &TopoDsShape) -> bool {
        self.shape_type.is_less_complex(&other.shape_type)
    }

    /// Get the name of the shape type
    #[inline]
    pub fn type_name(&self) -> &'static str {
        self.shape_type.name()
    }

    /// Apply a transformation to the shape
    #[inline]
    pub fn transform(&mut self, transformation: &Transform) {
        if let Some(loc) = self.location.as_mut() {
            loc.transform(transformation);
        }
    }

    /// Get the transformed location of the shape
    #[inline]
    pub fn transformed_location(&self) -> Option<Point> {
        self.location.as_ref().map(|loc| {
            let transform = loc.to_transform();
            transform.transforms(&Point::origin())
        })
    }

    /// Create a copy of this shape with a new orientation
    #[inline]
    pub fn reversed(&self) -> Self {
        let mut result = self.clone();
        result.set_orientation(-result.orientation());
        result
    }

    /// Check if this shape is reversed
    #[inline]
    pub fn is_reversed(&self) -> bool {
        self.orientation() < 0
    }

    /// Clear the location of the shape
    #[inline]
    pub fn clear_location(&mut self) {
        self.location = None;
    }

    /// Check if the shape has a location
    #[inline]
    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }
}

impl Default for TopoDsShape {
    fn default() -> Self {
        Self::new(ShapeType::Compound)
    }
}

impl PartialEq for TopoDsShape {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id == other.shape_id
    }
}

impl Eq for TopoDsShape {}

impl TopoDsShape {
    /// Compute the bounding box of the shape
    ///
    /// Returns (min_point, max_point) representing the bounding box
    pub fn bounding_box(&self) -> (Point, Point) {
        // Default implementation returns origin points
        // Subclasses should override this with proper implementation
        (Point::origin(), Point::origin())
    }

    /// Get faces of the shape
    ///
    /// Default implementation returns empty vector
    /// Subclasses should override this with proper implementation
    pub fn faces(&self) -> Vec<crate::topology::topods_face::TopoDsFace> {
        Vec::new()
    }

    /// Try to cast to face reference
    ///
    /// Returns None if this shape is not a face
    pub fn as_face(&self) -> Option<&crate::topology::topods_face::TopoDsFace> {
        None
    }
    
    /// Try to cast to edge reference
    ///
    /// Returns None if this shape is not an edge
    pub fn as_edge(&self) -> Option<&crate::topology::topods_edge::TopoDsEdge> {
        None
    }
    
    /// Try to cast to vertex reference
    ///
    /// Returns None if this shape is not a vertex
    pub fn as_vertex(&self) -> Option<&crate::topology::topods_vertex::TopoDsVertex> {
        None
    }
    
    /// Try to cast to wire reference
    ///
    /// Returns None if this shape is not a wire
    pub fn as_wire(&self) -> Option<&crate::topology::topods_wire::TopoDsWire> {
        None
    }
    
    /// Try to cast to shell reference
    ///
    /// Returns None if this shape is not a shell
    pub fn as_shell(&self) -> Option<&crate::topology::topods_shell::TopoDsShell> {
        None
    }
    
    /// Try to cast to solid reference
    ///
    /// Returns None if this shape is not a solid
    pub fn as_solid(&self) -> Option<&crate::topology::topods_solid::TopoDsSolid> {
        None
    }
    
    /// Try to cast to compound reference
    ///
    /// Returns None if this shape is not a compound
    pub fn as_compound(&self) -> Option<&crate::topology::topods_compound::TopoDsCompound> {
        None
    }

    /// Try to cast to compsolid reference
    ///
    /// Returns None if this shape is not a compsolid
    pub fn as_compsolid(&self) -> Option<&crate::topology::topods_compsolid::TopoDsCompSolid> {
        None
    }
}

// Implement Transformable trait for TopoDsShape
impl crate::api::traits::Transformable for TopoDsShape {
    fn translate(&mut self, vector: crate::geometry::Vector) -> &mut Self {
        // Implement translation
        // For now, we'll just update the location if it exists
        if let Some(loc) = self.location.as_mut() {
            loc.translate(vector);
        } else {
            // Create a new location with the translation
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.translate(vector);
            self.location = Some(loc);
        }
        self
    }

    fn rotate(&mut self, axis: crate::geometry::Axis, angle: f64) -> &mut Self {
        // Implement rotation
        if let Some(loc) = self.location.as_mut() {
            loc.rotate(axis, angle);
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.rotate(axis, angle);
            self.location = Some(loc);
        }
        self
    }

    fn scale(&mut self, factor: f64) -> &mut Self {
        // Implement uniform scaling
        if factor <= 0.0 {
            panic!("Scale factor must be positive");
        }
        if let Some(loc) = self.location.as_mut() {
            loc.scale(factor);
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.scale(factor);
            self.location = Some(loc);
        }
        self
    }

    fn scale_xyz(&mut self, sx: f64, sy: f64, sz: f64) -> &mut Self {
        // Implement non-uniform scaling
        if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
            panic!("Scale factors must be positive");
        }
        if let Some(loc) = self.location.as_mut() {
            loc.scale_xyz(sx, sy, sz);
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.scale_xyz(sx, sy, sz);
            self.location = Some(loc);
        }
        self
    }

    fn mirror(
        &mut self,
        point: crate::geometry::Point,
        normal: crate::geometry::Direction,
    ) -> &mut Self {
        // Implement mirroring
        if let Some(loc) = self.location.as_mut() {
            loc.mirror(point, normal);
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.mirror(point, normal);
            self.location = Some(loc);
        }
        self
    }

    fn transformed(&self, vector: crate::geometry::Vector) -> Self
    where
        Self: Sized + Clone,
    {
        // Implement transformed
        let mut result = self.clone();
        result.translate(vector);
        result
    }
}

// Implement BooleanOps trait for TopoDsShape
impl crate::api::traits::BooleanOps for TopoDsShape {
    /// Perform fuse operation on two shapes
    /// 
    /// Default implementation: create a compound shape containing both shapes
    /// Subclasses should override this with proper implementation for specific shape types
    fn fuse(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();
        compound.add_component(Handle::new(std::sync::Arc::new(self.clone())));
        compound.add_component(Handle::new(std::sync::Arc::new(other.clone())));
        compound.shape().clone()
    }

    /// Perform cut operation on two shapes
    /// 
    /// Default implementation: return self (no actual cutting)
    /// Subclasses should override this with proper implementation for specific shape types
    fn cut(&self, _other: &Self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    /// Perform intersect operation on two shapes
    /// 
    /// Default implementation: return an empty compound
    /// Subclasses should override this with proper implementation for specific shape types
    fn intersect(&self, _other: &Self) -> Self
    where
        Self: Sized,
    {
        crate::topology::topods_compound::TopoDsCompound::new()
            .shape()
            .clone()
    }

    /// Perform section operation on a shape with a plane
    /// 
    /// Default implementation: return an empty compound
    /// Subclasses should override this with proper implementation for specific shape types
    fn section(&self, _point: crate::geometry::Point, _normal: crate::geometry::Direction) -> Self
    where
        Self: Sized,
    {
        crate::topology::topods_compound::TopoDsCompound::new()
            .shape()
            .clone()
    }
}

// Implement FilletChamferOps trait for TopoDsShape
impl crate::api::traits::FilletChamferOps for TopoDsShape {
    /// Apply fillet to all edges of the shape
    /// 
    /// Default implementation: return self (no actual filleting)
    /// Subclasses should override this with proper implementation for specific shape types
    fn fillet(&self, radius: f64) -> Self
    where
        Self: Sized,
    {
        if radius <= 0.0 {
            panic!("Fillet radius must be positive");
        }
        self.clone()
    }

    /// Apply fillet to specific edges of the shape
    /// 
    /// Default implementation: return self (no actual filleting)
    /// Subclasses should override this with proper implementation for specific shape types
    fn fillet_edges(&self, edge_indices: &[usize], radius: f64) -> Self
    where
        Self: Sized,
    {
        if radius <= 0.0 {
            panic!("Fillet radius must be positive");
        }
        if edge_indices.is_empty() {
            return self.clone();
        }
        self.clone()
    }

    /// Apply chamfer to all edges of the shape
    /// 
    /// Default implementation: return self (no actual chamfering)
    /// Subclasses should override this with proper implementation for specific shape types
    fn chamfer(&self, distance: f64) -> Self
    where
        Self: Sized,
    {
        if distance <= 0.0 {
            panic!("Chamfer distance must be positive");
        }
        self.clone()
    }

    /// Apply chamfer to edges between specific faces
    /// 
    /// Default implementation: return self (no actual chamfering)
    /// Subclasses should override this with proper implementation for specific shape types
    fn chamfer_faces(&self, face_indices: &[usize], distance: f64) -> Self
    where
        Self: Sized,
    {
        if distance <= 0.0 {
            panic!("Chamfer distance must be positive");
        }
        if face_indices.is_empty() {
            return self.clone();
        }
        self.clone()
    }
}

// Implement OffsetOps trait for TopoDsShape
impl crate::api::traits::OffsetOps for TopoDsShape {
    /// Apply offset to the shape
    /// 
    /// Default implementation: return self (no actual offsetting)
    /// Subclasses should override this with proper implementation for specific shape types
    fn offset(&self, _distance: f64) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    /// Apply thickening to the shape
    /// 
    /// Default implementation: return self (no actual thickening)
    /// Subclasses should override this with proper implementation for specific shape types
    fn thicken(&self, _thickness: f64) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    /// Create a hollow version of the shape with specified thickness
    /// 
    /// Default implementation: return self (no actual hollowing)
    /// Subclasses should override this with proper implementation for specific shape types
    fn hollow(&self, _thickness: f64) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }
}

// Implement Measurable trait for TopoDsShape
impl crate::api::traits::Measurable for TopoDsShape {
    fn bounding_box(&self) -> (crate::geometry::Point, crate::geometry::Point) {
        // Use the existing bounding_box method
        self.bounding_box()
    }

    fn center_of_mass(&self) -> crate::geometry::Point {
        // Calculate center of mass based on shape type
        match self.shape_type {
            ShapeType::Vertex => {
                // For vertex, return the point itself (need unsafe downcast)
                // SAFETY: Safe because we know the shape type
                if self.is_vertex() {
                    unsafe {
                        let vertex = &*(self as *const _
                            as *const crate::topology::topods_vertex::TopoDsVertex);
                        return vertex.point().clone();
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Edge => {
                // For edge, return the midpoint between vertices
                if self.is_edge() {
                    unsafe {
                        let edge =
                            &*(self as *const _ as *const crate::topology::topods_edge::TopoDsEdge);
                        let v1 = edge.vertex1();
                        let v2 = edge.vertex2();
                        if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
                            let p1 = v1_ref.point();
                            let p2 = v2_ref.point();
                            return crate::geometry::Point::new(
                                (p1.x + p2.x) / 2.0,
                                (p1.y + p2.y) / 2.0,
                                (p1.z + p2.z) / 2.0,
                            );
                        }
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Wire => {
                // For wire, return average of all vertex positions
                if self.is_wire() {
                    unsafe {
                        let wire =
                            &*(self as *const _ as *const crate::topology::topods_wire::TopoDsWire);
                        return wire.centroid().unwrap_or(crate::geometry::Point::origin());
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Face => {
                // For face, return face centroid
                if self.is_face() {
                    unsafe {
                        let face =
                            &*(self as *const _ as *const crate::topology::topods_face::TopoDsFace);
                        return face.centroid().unwrap_or(crate::geometry::Point::origin());
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Shell => {
                // For shell, return average of face centroids
                if self.is_shell() {
                    unsafe {
                        let shell = &*(self as *const _
                            as *const crate::topology::topods_shell::TopoDsShell);
                        let faces = shell.faces();
                        if faces.is_empty() {
                            return crate::geometry::Point::origin();
                        }
                        let mut sum_x = 0.0;
                        let mut sum_y = 0.0;
                        let mut sum_z = 0.0;
                        for face in faces {
                            if let Some(face_ref) = face.get() {
                                if let Some(centroid) = face_ref.centroid() {
                                    sum_x += centroid.x;
                                    sum_y += centroid.y;
                                    sum_z += centroid.z;
                                }
                            }
                        }
                        let n = faces.len() as f64;
                        return crate::geometry::Point::new(sum_x / n, sum_y / n, sum_z / n);
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Solid => {
                // For solid, return volume centroid
                if self.is_solid() {
                    unsafe {
                        let solid = &*(self as *const _
                            as *const crate::topology::topods_solid::TopoDsSolid);
                        return solid.centroid().unwrap_or(crate::geometry::Point::origin());
                    }
                }
                crate::geometry::Point::origin()
            }
            ShapeType::Compound | ShapeType::CompSolid => {
                // For compound, return average of component centroids
                if self.is_compound() {
                    unsafe {
                        let compound = &*(self as *const _ as *const crate::topology::topods_compound::TopoDsCompound);
                        let components = compound.components();
                        if components.is_empty() {
                            return crate::geometry::Point::origin();
                        }
                        let mut sum_x = 0.0;
                        let mut sum_y = 0.0;
                        let mut sum_z = 0.0;
                        for component in components {
                            if let Some(comp_ref) = component.get() {
                                let centroid = comp_ref.center_of_mass();
                                sum_x += centroid.x;
                                sum_y += centroid.y;
                                sum_z += centroid.z;
                            }
                        }
                        let n = components.len() as f64;
                        return crate::geometry::Point::new(sum_x / n, sum_y / n, sum_z / n);
                    }
                }
                crate::geometry::Point::origin()
            }
        }
    }

    fn volume(&self) -> f64 {
        // Calculate volume based on shape type
        match self.shape_type {
            ShapeType::Solid => {
                // For solid, return the volume
                if self.is_solid() {
                    unsafe {
                        let solid = &*(self as *const _
                            as *const crate::topology::topods_solid::TopoDsSolid);
                        return solid.volume();
                    }
                }
                0.0
            }
            ShapeType::CompSolid => {
                // For compsolid, return sum of solid volumes
                if self.is_compsolid() {
                    unsafe {
                        let compsolid = &*(self as *const _
                            as *const crate::topology::topods_compound::TopoDsCompSolid);
                        return compsolid.volume();
                    }
                }
                0.0
            }
            ShapeType::Compound => {
                // For compound, return sum of component volumes
                if self.is_compound() {
                    unsafe {
                        let compound = &*(self as *const _
                            as *const crate::topology::topods_compound::TopoDsCompound);
                        let mut total_volume = 0.0;
                        for component in compound.components() {
                            if let Some(comp_ref) = component.get() {
                                total_volume += comp_ref.volume();
                            }
                        }
                        return total_volume;
                    }
                }
                0.0
            }
            _ => 0.0,
        }
    }

    fn surface_area(&self) -> f64 {
        // Calculate surface area based on shape type
        match self.shape_type {
            ShapeType::Face => {
                // For face, return the face area
                if self.is_face() {
                    unsafe {
                        let face =
                            &*(self as *const _ as *const crate::topology::topods_face::TopoDsFace);
                        return face.area();
                    }
                }
                0.0
            }
            ShapeType::Shell => {
                // For shell, return sum of face areas
                if self.is_shell() {
                    unsafe {
                        let shell = &*(self as *const _
                            as *const crate::topology::topods_shell::TopoDsShell);
                        return shell.area();
                    }
                }
                0.0
            }
            ShapeType::Solid => {
                // For solid, return surface area
                if self.is_solid() {
                    unsafe {
                        let solid = &*(self as *const _
                            as *const crate::topology::topods_solid::TopoDsSolid);
                        return solid.area();
                    }
                }
                0.0
            }
            ShapeType::CompSolid => {
                // For compsolid, return sum of solid surface areas
                if self.is_compsolid() {
                    unsafe {
                        let compsolid = &*(self as *const _
                            as *const crate::topology::topods_compound::TopoDsCompSolid);
                        return compsolid.area();
                    }
                }
                0.0
            }
            ShapeType::Compound => {
                // For compound, return sum of component surface areas
                if self.is_compound() {
                    unsafe {
                        let compound = &*(self as *const _
                            as *const crate::topology::topods_compound::TopoDsCompound);
                        let mut total_area = 0.0;
                        for component in compound.components() {
                            if let Some(comp_ref) = component.get() {
                                total_area += comp_ref.surface_area();
                            }
                        }
                        return total_area;
                    }
                }
                0.0
            }
            _ => 0.0,
        }
    }

    fn length(&self) -> f64 {
        // Calculate length based on shape type
        match self.shape_type {
            ShapeType::Edge => {
                // For edge, return the edge length
                if self.is_edge() {
                    unsafe {
                        let edge =
                            &*(self as *const _ as *const crate::topology::topods_edge::TopoDsEdge);
                        return edge.length();
                    }
                }
                0.0
            }
            ShapeType::Wire => {
                // For wire, return sum of edge lengths
                if self.is_wire() {
                    unsafe {
                        let wire =
                            &*(self as *const _ as *const crate::topology::topods_wire::TopoDsWire);
                        return wire.length();
                    }
                }
                0.0
            }
            _ => 0.0,
        }
    }
}

// Implement Validatable trait for TopoDsShape
impl crate::api::traits::Validatable for TopoDsShape {
    fn is_valid(&self) -> bool {
        let validator = crate::topology::validation::TopologyValidator::new();
        let result = validator.validate(self);
        result.is_valid
    }

    fn validation_errors(&self) -> Vec<String> {
        let validator = crate::topology::validation::TopologyValidator::new();
        let result = validator.validate(self);
        result.errors.into_iter().map(|e| format!("{:?}", e)).collect()
    }

    fn fix(&mut self) -> bool {
        let validator = crate::topology::validation::TopologyValidator::new();
        validator.repair(self)
    }
}

// Implement Serializable trait for TopoDsShape
impl crate::api::traits::Serializable for TopoDsShape {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        // Implement JSON serialization
        serde_json::to_string(self)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized,
    {
        // Implement JSON deserialization
        serde_json::from_str(json)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implement binary serialization
        Ok(bincode::serialize(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Implement binary deserialization
        Ok(bincode::deserialize(bytes)?)
    }
}

// Implement Meshable trait for TopoDsShape
impl crate::api::traits::Meshable for TopoDsShape {
    fn triangulate(
        &self,
        _linear_deflection: f64,
        _angular_deflection: f64,
    ) -> crate::api::traits::Mesh {
        // Default implementation returns empty mesh
        // Subclasses should override this with proper implementation
        crate::api::traits::Mesh {
            vertices: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
        }
    }

    fn tetrahedralize(&self, _max_edge_length: f64) -> crate::api::traits::TetMesh {
        // Default implementation returns empty tet mesh
        // Subclasses should override this with proper implementation
        crate::api::traits::TetMesh {
            vertices: Vec::new(),
            tetrahedra: Vec::new(),
        }
    }

    fn mesh_quality(&self, _mesh: &crate::api::traits::Mesh) -> crate::api::traits::MeshQuality {
        // Default implementation returns default values
        // Subclasses should override this with proper implementation
        crate::api::traits::MeshQuality {
            min_angle: 0.0,
            max_angle: 0.0,
            min_edge_ratio: 0.0,
            max_edge_ratio: 0.0,
            num_bad_elements: 0,
        }
    }
}

// Implement Analyzable trait for TopoDsShape
impl crate::api::traits::Analyzable for TopoDsShape {
    fn shape_type(&self) -> crate::topology::shape_enum::ShapeType {
        // Use the existing shape_type method
        self.shape_type()
    }

    fn is_closed(&self) -> bool {
        // Default implementation returns false
        // Subclasses should override this with proper implementation
        false
    }

    fn is_infinite(&self) -> bool {
        // Default implementation returns false
        // Subclasses should override this with proper implementation
        false
    }

    fn num_sub_shapes(&self, _shape_type: crate::topology::shape_enum::ShapeType) -> usize {
        // Default implementation returns 0
        // Subclasses should override this with proper implementation
        0
    }

    fn get_sub_shapes(
        &self,
        _shape_type: crate::topology::shape_enum::ShapeType,
    ) -> Vec<crate::foundation::handle::Handle<TopoDsShape>> {
        // Default implementation returns empty vector
        // Subclasses should override this with proper implementation
        Vec::new()
    }
}

// Implement Comparable trait for TopoDsShape
impl crate::api::traits::Comparable for TopoDsShape {
    fn is_congruent(&self, _other: &Self, _tolerance: f64) -> bool {
        // Default implementation returns false
        // Subclasses should override this with proper implementation
        false
    }

    fn contains(&self, _other: &Self) -> bool {
        // Default implementation returns false
        // Subclasses should override this with proper implementation
        false
    }

    fn intersects(&self, _other: &Self) -> bool {
        // Default implementation returns false
        // Subclasses should override this with proper implementation
        false
    }

    fn distance_to(&self, _other: &Self) -> f64 {
        // Default implementation returns 0.0
        // Subclasses should override this with proper implementation
        0.0
    }
}

// Implement Modifiable trait for TopoDsShape
impl crate::api::traits::Modifiable for TopoDsShape {
    fn reverse(&mut self) -> &mut Self {
        // Reverse the orientation of the shape
        self.set_orientation(-self.orientation());
        self
    }

    fn complement(&mut self) -> &mut Self {
        // Default implementation returns self
        // Subclasses should override this with proper implementation
        self
    }

    fn limit(&mut self, _min: crate::geometry::Point, _max: crate::geometry::Point) -> &mut Self {
        // Default implementation returns self
        // Subclasses should override this with proper implementation
        self
    }
}

// Implement Exportable trait for TopoDsShape
impl crate::api::traits::Exportable for TopoDsShape {
    fn to_stl(&self, _binary: bool) -> Result<String, Box<dyn std::error::Error>> {
        // Default implementation returns empty string
        // Subclasses should override this with proper implementation
        Ok(String::new())
    }

    fn to_step(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Default implementation returns empty string
        // Subclasses should override this with proper implementation
        Ok(String::new())
    }

    fn to_iges(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Default implementation returns empty string
        // Subclasses should override this with proper implementation
        Ok(String::new())
    }

    fn to_gltf(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Default implementation returns empty string
        // Subclasses should override this with proper implementation
        Ok(String::new())
    }

    fn to_usd(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Default implementation returns empty string
        // Subclasses should override this with proper implementation
        Ok(String::new())
    }
}

// Implement Importable trait for TopoDsShape
impl crate::api::traits::Importable for TopoDsShape {
    fn from_stl(_stl: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Default implementation returns default shape
        // Subclasses should override this with proper implementation
        Ok(Self::default())
    }

    fn from_step(_step: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Default implementation returns default shape
        // Subclasses should override this with proper implementation
        Ok(Self::default())
    }

    fn from_iges(_iges: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Default implementation returns default shape
        // Subclasses should override this with proper implementation
        Ok(Self::default())
    }
}

// Implement ParallelOps trait for TopoDsShape
impl crate::api::traits::ParallelOps for TopoDsShape {
    fn par_map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&Self) -> R + Send + Sync,
        R: Send,
    {
        // Default implementation processes single element
        // Subclasses should override this with proper parallel implementation
        vec![f(self)]
    }

    fn par_filter<F>(&self, f: F) -> Vec<Self>
    where
        F: Fn(&Self) -> bool + Send + Sync,
        Self: Sized + Clone,
    {
        // Default implementation filters single element
        // Subclasses should override this with proper parallel implementation
        if f(self) {
            vec![self.clone()]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        assert!(shape.is_vertex());
        assert!(!shape.is_edge());
        assert_eq!(shape.type_name(), "Vertex");
    }

    #[test]
    fn test_shape_type_checks() {
        let vertex = TopoDsShape::new(ShapeType::Vertex);
        let edge = TopoDsShape::new(ShapeType::Edge);
        let face = TopoDsShape::new(ShapeType::Face);
        let solid = TopoDsShape::new(ShapeType::Solid);

        assert!(vertex.is_vertex());
        assert!(edge.is_edge());
        assert!(face.is_face());
        assert!(solid.is_solid());
    }

    #[test]
    fn test_shape_complexity() {
        let vertex = TopoDsShape::new(ShapeType::Vertex);
        let edge = TopoDsShape::new(ShapeType::Edge);
        let face = TopoDsShape::new(ShapeType::Face);

        assert!(face.is_more_complex(&vertex));
        assert!(vertex.is_less_complex(&face));
        assert!(!vertex.is_more_complex(&edge));
    }

    #[test]
    fn test_shape_orientation() {
        let mut shape = TopoDsShape::new(ShapeType::Edge);
        assert_eq!(shape.orientation(), 1);

        shape.set_orientation(-1);
        assert_eq!(shape.orientation(), -1);
    }

    #[test]
    fn test_shape_mutable() {
        let mut shape = TopoDsShape::new(ShapeType::Face);
        assert!(!shape.is_mutable());

        shape.set_mutable(true);
        assert!(shape.is_mutable());
    }

    #[test]
    fn test_shape_id() {
        let mut shape = TopoDsShape::new(ShapeType::Solid);
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = shape.shape_id();
        assert!(initial_id > 0);

        shape.set_shape_id(42);
        assert_eq!(shape.shape_id(), 42);
    }

    #[test]
    fn test_shape_clone() {
        let mut shape1 = TopoDsShape::new(ShapeType::Edge);
        shape1.set_shape_id(10);
        shape1.set_orientation(-1);

        let shape2 = shape1.clone();
        assert_eq!(shape2.shape_id(), 10);
        assert_eq!(shape2.orientation(), -1);
        assert_eq!(shape1, shape2);
    }
}
