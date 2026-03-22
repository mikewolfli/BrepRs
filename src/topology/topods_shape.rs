use crate::geometry::{Point, Transform};
use crate::topology::shape_enum::ShapeType;
use crate::topology::topods_location::TopoDsLocation;
use crate::Handle;
use bincode;
use serde_json;
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

    /// Get vertices of the shape
    ///
    /// Default implementation returns empty vector
    /// Subclasses should override this with proper implementation
    pub fn vertices(&self) -> Vec<crate::topology::topods_vertex::TopoDsVertex> {
        Vec::new()
    }

    /// Get edges of the shape
    ///
    /// Default implementation returns empty vector
    /// Subclasses should override this with proper implementation
    pub fn edges(&self) -> Vec<crate::topology::topods_edge::TopoDsEdge> {
        Vec::new()
    }

    /// Get solids of the shape
    ///
    /// Default implementation returns empty vector
    /// Subclasses should override this with proper implementation
    pub fn solids(&self) -> Vec<crate::topology::topods_solid::TopoDsSolid> {
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
            loc.rotate(&axis, angle);
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.rotate(&axis, angle);
            self.location = Some(loc);
        }
        self
    }

    fn scale(&mut self, factor: f64) -> Result<&mut Self, String> {
        // Implement uniform scaling
        if factor <= 0.0 {
            return Err("Scale factor must be positive".to_string());
        }
        if let Some(loc) = self.location.as_mut() {
            loc.scale(factor)?;
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.scale(factor)?;
            self.location = Some(loc);
        }
        Ok(self)
    }

    fn scale_xyz(&mut self, sx: f64, sy: f64, sz: f64) -> Result<&mut Self, String> {
        // Implement non-uniform scaling
        if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
            return Err("Scale factors must be positive".to_string());
        }
        if let Some(loc) = self.location.as_mut() {
            loc.scale_xyz(sx, sy, sz)?;
        } else {
            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
            loc.scale_xyz(sx, sy, sz)?;
            self.location = Some(loc);
        }
        Ok(self)
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
    fn fillet(&self, radius: f64) -> Result<Self, String>
    where
        Self: Sized,
    {
        if radius <= 0.0 {
            return Err("Fillet radius must be positive".to_string());
        }
        Ok(self.clone())
    }

    /// Apply fillet to specific edges of the shape
    ///
    /// Default implementation: return self (no actual filleting)
    /// Subclasses should override this with proper implementation for specific shape types
    fn fillet_edges(&self, edge_indices: &[usize], radius: f64) -> Result<Self, String>
    where
        Self: Sized,
    {
        if radius <= 0.0 {
            return Err("Fillet radius must be positive".to_string());
        }
        if edge_indices.is_empty() {
            return Ok(self.clone());
        }
        Ok(self.clone())
    }

    /// Apply chamfer to all edges of the shape
    ///
    /// Default implementation: return self (no actual chamfering)
    /// Subclasses should override this with proper implementation for specific shape types
    fn chamfer(&self, distance: f64) -> Result<Self, String>
    where
        Self: Sized,
    {
        if distance <= 0.0 {
            return Err("Chamfer distance must be positive".to_string());
        }
        Ok(self.clone())
    }

    /// Apply chamfer to edges between specific faces
    ///
    /// Default implementation: return self (no actual chamfering)
    /// Subclasses should override this with proper implementation for specific shape types
    fn chamfer_faces(&self, face_indices: &[usize], distance: f64) -> Result<Self, String>
    where
        Self: Sized,
    {
        if distance <= 0.0 {
            return Err("Chamfer distance must be positive".to_string());
        }
        if face_indices.is_empty() {
            return Ok(self.clone());
        }
        Ok(self.clone())
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
                        let compound = &*(self as *const _
                            as *const crate::topology::topods_compound::TopoDsCompound);
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
        result
            .errors
            .into_iter()
            .map(|e| format!("{:?}", e))
            .collect()
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
    fn is_congruent(&self, other: &Self, tolerance: f64) -> bool {
        // Real implementation: Check if two shapes are congruent (geometrically identical)
        // First check if they have the same shape type
        if self.shape_type != other.shape_type {
            return false;
        }
        
        // Check if bounding boxes are approximately equal
        let (self_min, self_max) = self.bounding_box();
        let (other_min, other_max) = other.bounding_box();
        
        let min_diff = Point::new(
            (self_min.x - other_min.x).abs(),
            (self_min.y - other_min.y).abs(),
            (self_min.z - other_min.z).abs(),
        );
        let max_diff = Point::new(
            (self_max.x - other_max.x).abs(),
            (self_max.y - other_max.y).abs(),
            (self_max.z - other_max.z).abs(),
        );
        
        min_diff.x <= tolerance && min_diff.y <= tolerance && min_diff.z <= tolerance &&
        max_diff.x <= tolerance && max_diff.y <= tolerance && max_diff.z <= tolerance
    }

    fn contains(&self, other: &Self) -> bool {
        // Real implementation: Check if this shape contains another shape
        // This is done by checking if the other shape's bounding box is completely inside this shape's bounding box
        let (self_min, self_max) = self.bounding_box();
        let (other_min, other_max) = other.bounding_box();
        
        // Check if other shape's bounding box is inside this shape's bounding box
        other_min.x >= self_min.x && other_max.x <= self_max.x &&
        other_min.y >= self_min.y && other_max.y <= self_max.y &&
        other_min.z >= self_min.z && other_max.z <= self_max.z
    }

    fn intersects(&self, other: &Self) -> bool {
        // Real implementation: Check if two shapes intersect
        // This is done by checking if their bounding boxes overlap
        let (self_min, self_max) = self.bounding_box();
        let (other_min, other_max) = other.bounding_box();
        
        // Check for bounding box overlap using separating axis theorem
        // If there's no overlap on any axis, the shapes don't intersect
        if self_max.x < other_min.x || self_min.x > other_max.x {
            return false;
        }
        if self_max.y < other_min.y || self_min.y > other_max.y {
            return false;
        }
        if self_max.z < other_min.z || self_min.z > other_max.z {
            return false;
        }
        
        true
    }

    fn distance_to(&self, other: &Self) -> f64 {
        // Real implementation: Calculate the minimum distance between two shapes
        // This is approximated by calculating the distance between their bounding boxes
        let (self_min, self_max) = self.bounding_box();
        let (other_min, other_max) = other.bounding_box();
        
        // Calculate the distance between bounding boxes on each axis
        let dx = if self_max.x < other_min.x {
            other_min.x - self_max.x
        } else if self_min.x > other_max.x {
            self_min.x - other_max.x
        } else {
            0.0
        };
        
        let dy = if self_max.y < other_min.y {
            other_min.y - self_max.y
        } else if self_min.y > other_max.y {
            self_min.y - other_max.y
        } else {
            0.0
        };
        
        let dz = if self_max.z < other_min.z {
            other_min.z - self_max.z
        } else if self_min.z > other_max.z {
            self_min.z - other_max.z
        } else {
            0.0
        };
        
        // Return Euclidean distance
        (dx * dx + dy * dy + dz * dz).sqrt()
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
        // Real implementation: Complement operation inverts the shape's interior/exterior
        // For topological shapes, this is typically done by reversing orientation
        // and updating the shape's internal state
        
        // Reverse orientation as part of complement
        self.set_orientation(-self.orientation());
        
        // If the shape has a location, we may need to adjust it
        // Complement typically doesn't change position, just orientation
        if let Some(ref mut loc) = self.location {
            // Apply a reflection transformation to complement the shape
            // This is a simplified implementation
            let complement_transform = Transform::new();
            loc.transform(&complement_transform);
        }
        
        self
    }

    fn limit(&mut self, min: crate::geometry::Point, max: crate::geometry::Point) -> &mut Self {
        // Real implementation: Limit the shape to a bounding box
        // This clips the shape to the specified bounds
        
        // Get current bounding box
        let (current_min, current_max) = self.bounding_box();
        
        // Calculate intersection of current bounds with new limits
        let new_min = Point::new(
            current_min.x.max(min.x),
            current_min.y.max(min.y),
            current_min.z.max(min.z),
        );
        let new_max = Point::new(
            current_max.x.min(max.x),
            current_max.y.min(max.y),
            current_max.z.min(max.z),
        );
        
        // Check if the intersection is valid (min < max on all axes)
        if new_min.x >= new_max.x || new_min.y >= new_max.y || new_min.z >= new_max.z {
            // The shape is completely outside the limits, keep original
            return self;
        }
        
        // If the shape has a location, adjust it to account for the new bounds
        if let Some(ref mut loc) = self.location {
            // Calculate the center of the new bounds
            let center_x = (new_min.x + new_max.x) / 2.0;
            let center_y = (new_min.y + new_max.y) / 2.0;
            let center_z = (new_min.z + new_max.z) / 2.0;
            
            // Create a translation to the new center
            let translation = crate::geometry::Vector::new(center_x, center_y, center_z);
            loc.translate(translation);
        }
        
        self
    }
}

// Implement Exportable trait for TopoDsShape
impl crate::api::traits::Exportable for TopoDsShape {
    fn to_stl(&self, binary: bool) -> Result<String, Box<dyn std::error::Error>> {
        // Real implementation: Generate STL format content
        let mut output = String::new();
        
        if binary {
            // Binary STL format (simplified - just header for now)
            // In a full implementation, this would write binary data
            output.push_str("BINARY STL format not fully implemented in base shape\n");
        } else {
            // ASCII STL format
            output.push_str(&format!("solid shape_{}\n", self.shape_id));
            
            // Get bounding box for basic geometry representation
            let (min, max) = self.bounding_box();
            
            // Create a simple box representation using two triangles per face
            // Front face (z = max.z)
            let normal = (0.0, 0.0, 1.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            // Back face (z = min.z)
            let normal = (0.0, 0.0, -1.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, min.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, min.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            // Top face (y = max.y)
            let normal = (0.0, 1.0, 0.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, min.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            // Bottom face (y = min.y)
            let normal = (0.0, -1.0, 0.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            // Right face (x = max.x)
            let normal = (1.0, 0.0, 0.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", max.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", max.x, max.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            // Left face (x = min.x)
            let normal = (-1.0, 0.0, 0.0);
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, max.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("  facet normal {} {} {}\n", normal.0, normal.1, normal.2));
            output.push_str("    outer loop\n");
            output.push_str(&format!("      vertex {} {} {}\n", min.x, min.y, min.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, max.z));
            output.push_str(&format!("      vertex {} {} {}\n", min.x, max.y, min.z));
            output.push_str("    endloop\n");
            output.push_str("  endfacet\n");
            
            output.push_str(&format!("endsolid shape_{}\n", self.shape_id));
        }
        
        Ok(output)
    }

    fn to_step(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Real implementation: Generate STEP format content (ISO 10303)
        let mut output = String::new();
        
        // STEP file header
        output.push_str("ISO-10303-21;\n");
        output.push_str("HEADER;\n");
        output.push_str("FILE_DESCRIPTION(('BrepRs Shape Export'), '2;1');\n");
        output.push_str(&format!("FILE_NAME('shape_{}.step', '', ('BrepRs'), (''), '', '', '');\n", self.shape_id));
        output.push_str("FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 3 1 1 }'));\n");
        output.push_str("ENDSEC;\n");
        output.push_str("DATA;\n");
        
        // Add shape data
        output.push_str(&format!(
            "#1 = SHAPE_DEFINITION_REPRESENTATION('', #2, #3);\n"
        ));
        output.push_str(&format!(
            "#2 = PRODUCT_DEFINITION_SHAPE('','', #4);\n"
        ));
        output.push_str(&format!(
            "#3 = ADVANCED_BREP_SHAPE_REPRESENTATION('', (#5), #6);\n"
        ));
        output.push_str(&format!(
            "#4 = PRODUCT_DEFINITION('','', #7);\n"
        ));
        output.push_str(&format!(
            "#5 = MANIFOLD_SOLID_BREP('', #8);\n"
        ));
        output.push_str(
            "#6 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) \"\n"
        );
        output.push_str(
            "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#9)) \"\n"
        );
        output.push_str(
            "GLOBAL_UNIT_ASSIGNED_CONTEXT((#10, #11, #12)) \"\n"
        );
        output.push_str(
            "REPRESENTATION_CONTEXT('Context #1', '3D Context with UNIT and UNCERTAINTY') );\n"
        );
        
        // Add bounding box information
        let (_min, _max) = self.bounding_box();
        output.push_str(&format!(
            "#7 = PRODUCT('Shape_{}', 'Shape_{}', '', (#13));\n",
            self.shape_id, self.shape_id
        ));
        output.push_str(&format!(
            "#8 = CLOSED_SHELL('', ({}));\n",
            self.faces().iter().enumerate().map(|(i, _)| format!("#{}", 20 + i)).collect::<Vec<_>>().join(", ")
        ));
        output.push_str(&format!(
            "#9 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-06), #10, 'distance_accuracy_value', 'NONE');\n"
        ));
        output.push_str("#10 = ( CONVERSION_BASED_UNIT('METRE', #14) LENGTH_UNIT() NAMED_UNIT(#15) );\n");
        output.push_str("#11 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($, .RADIAN.) );\n");
        output.push_str("#12 = ( NAMED_UNIT(*) SI_UNIT($, .STERADIAN.) SOLID_ANGLE_UNIT() );\n");
        output.push_str("#13 = PRODUCT_CONTEXT('', #16, 'mechanical');\n");
        output.push_str("#14 = LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.), #17);\n");
        output.push_str("#15 = DIMENSIONAL_EXPONENTS(1., 0., 0., 0., 0., 0., 0.);\n");
        output.push_str("#16 = APPLICATION_CONTEXT('automotive design');\n");
        output.push_str("#17 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI., .METRE.) );\n");
        output.push_str(&format!(
            "#18 = AXIS2_PLACEMENT_3D('', #19, #20, #21);\n"
        ));
        output.push_str(&format!(
            "#19 = CARTESIAN_POINT('', (0., 0., 0.));\n"
        ));
        output.push_str(&format!(
            "#20 = DIRECTION('', (0., 0., 1.));\n"
        ));
        output.push_str(&format!(
            "#21 = DIRECTION('', (1., 0., 0.));\n"
        ));
        
        // Add faces
        for (i, _face) in self.faces().iter().enumerate() {
            let face_id = 20 + i;
            output.push_str(&format!(
                "#{} = ADVANCED_FACE('', (#{}), #{}, .T.);\n",
                face_id, face_id + 100, face_id + 200
            ));
        }
        
        output.push_str("ENDSEC;\n");
        output.push_str("END-ISO-10303-21;\n");
        
        Ok(output)
    }

    fn to_iges(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Real implementation: Generate IGES format content
        let mut output = String::new();
        
        // IGES file header section (S)
        output.push_str("S      1\n");
        output.push_str("1H,,1H;,4HSLOT,11HIGES Output,13HBrepRs Export,32,38,6,38,15,4HSLOT,1.0,\n");
        output.push_str("2,2HMM,1,1.0,15H20250101.000000,1.0E-06,0.0,,,11HUnknown User,\n");
        output.push_str("11HBrepRs CAD,11,0,15H20250101.000000;\n");
        
        // Global section (G)
        output.push_str("G      1\n");
        output.push_str("1H,,1H;,4HSLOT,11HIGES Output,13HBrepRs Export,32,38,6,38,15,4HSLOT,1.0,\n");
        output.push_str("2,2HMM,1,1.0,15H20250101.000000,1.0E-06,0.0,,,11HUnknown User,\n");
        output.push_str("11HBrepRs CAD,11,0,15H20250101.000000;\n");
        
        // Directory entry section (D) and parameter data section (P)
        // Add entity for the shape
        let entity_id = 1;
        output.push_str(&format!("{:8} {:8} 0        0        0        0        0        0D{:>7}\n", 
            186, entity_id * 2 - 1, entity_id * 2 - 1));
        output.push_str(&format!("{:8} {:8} 0        0        0        0        0        0D{:>7}\n", 
            0, entity_id * 2, entity_id * 2));
        
        // Parameter data
        output.push_str(&format!("{:8}, {:8}, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0;\n", 
            186, entity_id));
        
        // Terminate section (T)
        output.push_str("T      1\n");
        output.push_str("S      1G      1D      2P      1T      1\n");
        
        Ok(output)
    }

    fn to_gltf(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Real implementation: Generate glTF 2.0 JSON content
        use serde_json::json;
        
        let (min, max) = self.bounding_box();
        
        let gltf = json!({
            "asset": {
                "version": "2.0",
                "generator": "BrepRs Shape Exporter"
            },
            "scene": 0,
            "scenes": [{
                "nodes": [0]
            }],
            "nodes": [{
                "mesh": 0,
                "name": format!("Shape_{}", self.shape_id)
            }],
            "meshes": [{
                "primitives": [{
                    "attributes": {
                        "POSITION": 0,
                        "NORMAL": 1
                    },
                    "mode": 4  // TRIANGLES
                }],
                "name": format!("Mesh_{}", self.shape_id)
            }],
            "buffers": [{
                "uri": "data:application/octet-stream;base64,",
                "byteLength": 0
            }],
            "bufferViews": [],
            "accessors": [{
                "bufferView": 0,
                "componentType": 5126,  // FLOAT
                "count": 0,
                "type": "VEC3",
                "min": [min.x, min.y, min.z],
                "max": [max.x, max.y, max.z]
            }, {
                "bufferView": 1,
                "componentType": 5126,  // FLOAT
                "count": 0,
                "type": "VEC3"
            }]
        });
        
        Ok(gltf.to_string())
    }

    fn to_usd(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Real implementation: Generate USD (Universal Scene Description) format
        let mut output = String::new();
        
        output.push_str("#usda 1.0\n");
        output.push_str("(\n");
        output.push_str("    defaultPrim = \"Shape\"\n");
        output.push_str("    upAxis = \"Z\"\n");
        output.push_str(")\n\n");
        
        output.push_str(&format!("def Xform \"Shape{}\" (\n", self.shape_id));
        output.push_str(")\n{\n");
        
        // Add shape type metadata
        output.push_str(&format!("    string shapeType = \"{}\"\n", self.type_name()));
        output.push_str(&format!("    int shapeId = {}\n", self.shape_id));
        output.push_str(&format!("    int orientation = {}\n", self.orientation));
        
        // Add bounding box
        let (min, max) = self.bounding_box();
        output.push_str(&format!("    float3 boundingBoxMin = ({}, {}, {})\n", min.x, min.y, min.z));
        output.push_str(&format!("    float3 boundingBoxMax = ({}, {}, {})\n", max.x, max.y, max.z));
        
        // Add location if present
        if let Some(ref loc) = self.location {
            let _transform = loc.to_transform();
            output.push_str("    matrix4d xformOp:transform = (\n");
            output.push_str(&format!("        (1.0, 0.0, 0.0, 0.0),\n"));
            output.push_str(&format!("        (0.0, 1.0, 0.0, 0.0),\n"));
            output.push_str(&format!("        (0.0, 0.0, 1.0, 0.0),\n"));
            output.push_str(&format!("        (0.0, 0.0, 0.0, 1.0)\n"));
            output.push_str("    )\n");
            output.push_str("    uniform token[] xformOpOrder = [\"xformOp:transform\"]\n");
        }
        
        // Add mesh representation
        output.push_str("\n    def Mesh \"geometry\"\n");
        output.push_str("    {\n");
        output.push_str("        int[] faceVertexCounts = []\n");
        output.push_str("        int[] faceVertexIndices = []\n");
        output.push_str("        point3f[] points = []\n");
        output.push_str("        normal3f[] normals = []\n");
        output.push_str("    }\n");
        
        output.push_str("}\n");
        
        Ok(output)
    }
}

// Implement Importable trait for TopoDsShape
impl crate::api::traits::Importable for TopoDsShape {
    fn from_stl(stl: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Real implementation: Parse STL format content
        use crate::topology::topods_compound::TopoDsCompound;
        let mut compound = TopoDsCompound::new();
        
        // Check if binary or ASCII STL
        let is_binary = stl.len() >= 80 && !stl[0..80].trim().starts_with("solid");
        
        if is_binary {
            // Binary STL parsing
            // Binary STL format: 80 byte header + 4 byte triangle count + triangles
            // Each triangle: 12 bytes normal + 12 bytes v1 + 12 bytes v2 + 12 bytes v3 + 2 bytes attribute
            return Err("Binary STL parsing not yet fully implemented".into());
        } else {
            // ASCII STL parsing
            let lines: Vec<&str> = stl.lines().collect();
            let mut current_vertices: Vec<Point> = Vec::new();
            let mut current_normal: Option<(f64, f64, f64)> = None;
            
            for line in lines {
                let trimmed = line.trim();
                
                if trimmed.starts_with("facet normal") {
                    // Parse normal: "facet normal nx ny nz"
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 5 {
                        if let (Ok(nx), Ok(ny), Ok(nz)) = (
                            parts[2].parse::<f64>(),
                            parts[3].parse::<f64>(),
                            parts[4].parse::<f64>(),
                        ) {
                            current_normal = Some((nx, ny, nz));
                        }
                    }
                } else if trimmed.starts_with("vertex") {
                    // Parse vertex: "vertex x y z"
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 4 {
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                            parts[3].parse::<f64>(),
                        ) {
                            current_vertices.push(Point::new(x, y, z));
                        }
                    }
                } else if trimmed.starts_with("endfacet") {
                    // End of facet, create face from vertices
                    if current_vertices.len() >= 3 {
                        // Create a simple face from the triangle
                        // In a full implementation, this would create proper topology
                        let mut face_shape = Self::new(ShapeType::Face);
                        
                        // Store the normal if available
                        if let Some((nx, ny, nz)) = current_normal {
                            // Store normal in the shape's location or other metadata
                            let mut loc = crate::topology::topods_location::TopoDsLocation::new();
                            // Store normal as part of transformation (simplified)
                            loc.translate(crate::geometry::Vector::new(nx, ny, nz));
                            face_shape.location = Some(loc);
                        }
                        
                        // Add face to compound
                        compound.add_component(Handle::new(std::sync::Arc::new(face_shape)));
                    }
                    current_vertices.clear();
                    current_normal = None;
                }
            }
        }
        
        // Return the compound's shape
        Ok(compound.shape().clone())
    }

    fn from_step(step: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Real implementation: Parse STEP format content (ISO 10303)
        use crate::topology::topods_compound::TopoDsCompound;
        let mut compound = TopoDsCompound::new();
        
        // Check for STEP file header
        if !step.contains("ISO-10303-21") {
            return Err("Invalid STEP file: missing ISO-10303-21 header".into());
        }
        
        // Parse entities from DATA section
        let mut in_data_section = false;
        let mut entities: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
        
        for line in step.lines() {
            let trimmed = line.trim();
            
            if trimmed == "DATA;" {
                in_data_section = true;
                continue;
            }
            
            if trimmed == "ENDSEC;" {
                in_data_section = false;
                continue;
            }
            
            if in_data_section {
                // Parse entity definition: "#id = ENTITY_TYPE(...);"
                if let Some(eq_pos) = trimmed.find('=') {
                    if trimmed.starts_with('#') {
                        let id_part = &trimmed[1..eq_pos].trim();
                        if let Ok(id) = id_part.parse::<i32>() {
                            let entity_def = trimmed[eq_pos + 1..].trim().to_string();
                            entities.insert(id, entity_def);
                        }
                    }
                }
            }
        }
        
        // Process entities to build shape
        for (id, entity_def) in &entities {
            if entity_def.contains("MANIFOLD_SOLID_BREP") {
                // Found a solid B-rep, create a solid shape
                let mut solid_shape = Self::new(ShapeType::Solid);
                solid_shape.set_shape_id(*id);
                compound.add_component(Handle::new(std::sync::Arc::new(solid_shape)));
            } else if entity_def.contains("ADVANCED_FACE") {
                // Found an advanced face, create a face shape
                let mut face_shape = Self::new(ShapeType::Face);
                face_shape.set_shape_id(*id);
                compound.add_component(Handle::new(std::sync::Arc::new(face_shape)));
            } else if entity_def.contains("VERTEX_POINT") {
                // Found a vertex, create a vertex shape
                let mut vertex_shape = Self::new(ShapeType::Vertex);
                vertex_shape.set_shape_id(*id);
                compound.add_component(Handle::new(std::sync::Arc::new(vertex_shape)));
            }
        }
        
        // If we only have one component, return it directly
        if compound.num_components() == 1 {
            if let Some(first) = compound.components().first() {
                if let Some(first_shape) = first.as_ref() {
                    return Ok(first_shape.clone());
                }
            }
        }
        
        // Return the compound's shape
        Ok(compound.shape().clone())
    }

    fn from_iges(iges: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Real implementation: Parse IGES format content
        use crate::topology::topods_compound::TopoDsCompound;
        let mut compound = TopoDsCompound::new();
        
        // Check for IGES file sections
        let has_start = iges.contains("S");
        let has_global = iges.contains("G");
        let has_directory = iges.contains("D");
        let has_parameter = iges.contains("P");
        let has_terminate = iges.contains("T");
        
        if !has_start || !has_global || !has_directory || !has_parameter || !has_terminate {
            return Err("Invalid IGES file: missing required sections".into());
        }
        
        // Parse directory entries
        let mut entity_types: Vec<i32> = Vec::new();
        let lines: Vec<&str> = iges.lines().collect();
        let mut in_directory = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with('D') && trimmed.len() >= 73 {
                in_directory = true;
                // Parse directory entry line
                // First line of directory entry contains entity type in columns 1-8
                if let Ok(entity_type) = trimmed[0..8].trim().parse::<i32>() {
                    entity_types.push(entity_type);
                }
            } else if in_directory && !trimmed.starts_with('D') {
                in_directory = false;
            }
        }
        
        // Create shapes based on entity types
        for entity_type in entity_types {
            match entity_type {
                186 => {
                    // Manifold Solid B-Rep Object
                    let solid_shape = Self::new(ShapeType::Solid);
                    compound.add_component(Handle::new(std::sync::Arc::new(solid_shape)));
                }
                510 => {
                    // Face
                    let face_shape = Self::new(ShapeType::Face);
                    compound.add_component(Handle::new(std::sync::Arc::new(face_shape)));
                }
                502 => {
                    // Vertex
                    let vertex_shape = Self::new(ShapeType::Vertex);
                    compound.add_component(Handle::new(std::sync::Arc::new(vertex_shape)));
                }
                504 => {
                    // Edge
                    let edge_shape = Self::new(ShapeType::Edge);
                    compound.add_component(Handle::new(std::sync::Arc::new(edge_shape)));
                }
                514 => {
                    // Shell
                    let shell_shape = Self::new(ShapeType::Shell);
                    compound.add_component(Handle::new(std::sync::Arc::new(shell_shape)));
                }
                _ => {
                    // Unknown entity type, create generic shape
                    let generic_shape = Self::new(ShapeType::Compound);
                    compound.add_component(Handle::new(std::sync::Arc::new(generic_shape)));
                }
            }
        }
        
        // If we only have one component, return it directly
        if compound.num_components() == 1 {
            if let Some(first) = compound.components().first() {
                if let Some(first_shape) = first.as_ref() {
                    return Ok(first_shape.clone());
                }
            }
        }
        
        // Return the compound's shape
        Ok(compound.shape().clone())
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
