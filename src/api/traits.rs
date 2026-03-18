impl Mesh {
    pub fn from_mesh2d(mesh2d: crate::mesh::mesh_data::Mesh2D) -> Self {
        let vertices = mesh2d.vertices.iter().map(|v| v.point.clone()).collect();
        let triangles = mesh2d
            .faces
            .iter()
            .map(|f| {
                let v = &f.vertices;
                [v[0], v[1], v[2]]
            })
            .collect();
        Mesh {
            vertices,
            triangles,
            normals: Vec::new(),
            uvs: Vec::new(),
        }
    }
}
// Core traits for Rust Native API
//
// This module defines the fundamental traits that enable method chaining,
// generics with trait bounds, and type-safe shape operations.

use crate::foundation::handle::Handle;
use crate::geometry::{Axis, Direction, Point, Vector};
use crate::topology::{topods_shape::TopoDsShape, ShapeType};

/// Trait for types that can be transformed (moved, rotated, scaled)
pub trait Transformable {
    /// Translate the object by a vector
    fn translate(&mut self, vector: Vector) -> &mut Self;

    /// Rotate the object around an axis
    fn rotate(&mut self, axis: Axis, angle: f64) -> &mut Self;

    /// Scale the object uniformly
    fn scale(&mut self, factor: f64) -> Result<&mut Self, String>;

    /// Scale the object non-uniformly
    fn scale_xyz(&mut self, sx: f64, sy: f64, sz: f64) -> Result<&mut Self, String>;

    /// Mirror the object across a plane
    fn mirror(&mut self, point: Point, normal: Direction) -> &mut Self;

    /// Apply a transformation and return a new transformed object
    fn transformed(&self, vector: Vector) -> Self
    where
        Self: Sized + Clone;
}

/// Trait for types that support boolean operations
pub trait BooleanOps {
    /// Union (fuse) with another shape
    fn fuse(&self, other: &Self) -> Self
    where
        Self: Sized;

    /// Subtract (cut) another shape from this one
    fn cut(&self, other: &Self) -> Self
    where
        Self: Sized;

    /// Intersection (common) with another shape
    fn intersect(&self, other: &Self) -> Self
    where
        Self: Sized;

    /// Section with a plane
    fn section(&self, point: Point, normal: Direction) -> Self
    where
        Self: Sized;
}

/// Trait for types that support fillet and chamfer operations
pub trait FilletChamferOps {
    /// Apply fillet to all edges with the given radius
    fn fillet(&self, radius: f64) -> Result<Self, String>
    where
        Self: Sized;

    /// Apply fillet to specific edges
    fn fillet_edges(&self, edge_indices: &[usize], radius: f64) -> Result<Self, String>
    where
        Self: Sized;

    /// Apply chamfer to all edges with the given distance
    fn chamfer(&self, distance: f64) -> Result<Self, String>
    where
        Self: Sized;

    /// Apply chamfer to specific faces
    fn chamfer_faces(&self, face_indices: &[usize], distance: f64) -> Result<Self, String>
    where
        Self: Sized;
}

/// Trait for types that support offset operations
pub trait OffsetOps {
    /// Offset the shape by a distance (positive = outward, negative = inward)
    fn offset(&self, distance: f64) -> Self
    where
        Self: Sized;

    /// Create a thick solid from a shell
    fn thicken(&self, thickness: f64) -> Self
    where
        Self: Sized;

    /// Create a hollow shell with given wall thickness
    fn hollow(&self, thickness: f64) -> Self
    where
        Self: Sized;
}

/// Trait for types that can be measured
pub trait Measurable {
    /// Calculate the bounding box (min point, max point)
    fn bounding_box(&self) -> (Point, Point);

    /// Calculate the center of mass
    fn center_of_mass(&self) -> Point;

    /// Calculate the volume (for 3D shapes)
    fn volume(&self) -> f64;

    /// Calculate the surface area (for 2D/3D shapes)
    fn surface_area(&self) -> f64;

    /// Calculate the length (for 1D shapes)
    fn length(&self) -> f64;
}

/// Trait for types that can be validated
pub trait Validatable {
    /// Check if the shape is valid
    fn is_valid(&self) -> bool;

    /// Get validation errors if any
    fn validation_errors(&self) -> Vec<String>;

    /// Attempt to fix validation errors
    fn fix(&mut self) -> bool;
}

/// Trait for types that can be serialized/deserialized
pub trait Serializable {
    /// Serialize to JSON
    fn to_json(&self) -> Result<String, serde_json::Error>;

    /// Deserialize from JSON
    fn from_json(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized;

    /// Serialize to binary format
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    /// Deserialize from binary format
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

/// Trait for types that support mesh generation
pub trait Meshable {
    /// Generate a triangle mesh with specified quality
    fn triangulate(&self, linear_deflection: f64, angular_deflection: f64) -> Mesh;

    /// Generate a tetrahedral mesh (for solids)
    fn tetrahedralize(&self, max_edge_length: f64) -> TetMesh;

    /// Get mesh quality metrics
    fn mesh_quality(&self, mesh: &Mesh) -> MeshQuality;
}

/// Mesh structure for triangulation results
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Point>,
    pub triangles: Vec<[usize; 3]>,
    pub normals: Vec<Vector>,
    pub uvs: Vec<(f64, f64)>,
}

/// Tetrahedral mesh structure
#[derive(Debug, Clone)]
pub struct TetMesh {
    pub vertices: Vec<Point>,
    pub tetrahedra: Vec<[usize; 4]>,
}

/// Mesh quality metrics
#[derive(Debug, Clone)]
pub struct MeshQuality {
    pub min_angle: f64,
    pub max_angle: f64,
    pub min_edge_ratio: f64,
    pub max_edge_ratio: f64,
    pub num_bad_elements: usize,
}

/// Trait for builder pattern with type-safe state transitions
pub trait BuilderState {}

/// Empty state for builders
pub struct Empty;
impl BuilderState for Empty {}

/// Configured state for builders
pub struct Configured;
impl BuilderState for Configured {}

/// Built state for builders
pub struct Built;
impl BuilderState for Built {}

/// Trait for geometric primitives creation
pub trait PrimitiveCreator {
    /// Create a box primitive
    fn box_primitive(width: f64, height: f64, depth: f64) -> Self
    where
        Self: Sized;

    /// Create a sphere primitive
    fn sphere_primitive(radius: f64) -> Self
    where
        Self: Sized;

    /// Create a cylinder primitive
    fn cylinder_primitive(radius: f64, height: f64) -> Self
    where
        Self: Sized;

    /// Create a cone primitive
    fn cone_primitive(radius1: f64, radius2: f64, height: f64) -> Self
    where
        Self: Sized;

    /// Create a torus primitive
    fn torus_primitive(major_radius: f64, minor_radius: f64) -> Self
    where
        Self: Sized;
}

/// Trait for shape analysis operations
pub trait Analyzable {
    /// Get the shape type
    fn shape_type(&self) -> ShapeType;

    /// Check if the shape is closed
    fn is_closed(&self) -> bool;

    /// Check if the shape is infinite
    fn is_infinite(&self) -> bool;

    /// Get the number of sub-shapes of a specific type
    fn num_sub_shapes(&self, shape_type: ShapeType) -> usize;

    /// Get all sub-shapes of a specific type
    fn get_sub_shapes(&self, shape_type: ShapeType) -> Vec<Handle<TopoDsShape>>;
}

/// Trait for shape comparison operations
pub trait Comparable {
    /// Check if two shapes are geometrically equal
    fn is_congruent(&self, other: &Self, tolerance: f64) -> bool;

    /// Check if this shape contains another
    fn contains(&self, other: &Self) -> bool;

    /// Check if this shape intersects with another
    fn intersects(&self, other: &Self) -> bool;

    /// Calculate the distance to another shape
    fn distance_to(&self, other: &Self) -> f64;
}

/// Trait for shape modification operations
pub trait Modifiable {
    /// Reverse the orientation of the shape
    fn reverse(&mut self) -> &mut Self;

    /// Complement the shape (invert inside/outside)
    fn complement(&mut self) -> &mut Self;

    /// Limit the shape to a bounding box
    fn limit(&mut self, min: Point, max: Point) -> &mut Self;
}

/// Trait for shape export operations
pub trait Exportable {
    /// Export to STL format
    fn to_stl(&self, binary: bool) -> Result<String, Box<dyn std::error::Error>>;

    /// Export to STEP format
    fn to_step(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Export to IGES format
    fn to_iges(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Export to glTF format
    fn to_gltf(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Export to USD format
    fn to_usd(&self) -> Result<String, Box<dyn std::error::Error>>;
}

/// Trait for shape import operations
pub trait Importable {
    /// Import from STL format
    fn from_stl(stl: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// Import from STEP format
    fn from_step(step: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// Import from IGES format
    fn from_iges(iges: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

/// Trait for concurrent operations support
pub trait ParallelOps {
    /// Process sub-shapes in parallel
    fn par_map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&Self) -> R + Send + Sync,
        R: Send;

    /// Filter sub-shapes in parallel
    fn par_filter<F>(&self, f: F) -> Vec<Self>
    where
        F: Fn(&Self) -> bool + Send + Sync,
        Self: Sized + Clone;
}
