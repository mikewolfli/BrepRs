//! OpenCASCADE Geometry Compatibility Module
//!
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for geometric entities.

// Re-export geometry types with OpenCASCADE naming (gp_* prefix)
pub use crate::geometry::{
    ax2::Ax2 as gp_Ax2, axis::Axis as gp_Ax1, circle::Circle as gp_Circ,
    coordinate_system::CoordinateSystem as gp_Ax22d, cylinder::Cylinder as gp_Cylinder,
    direction::Direction as gp_Dir, ellipse::Ellipse as gp_Elips, line::Line as gp_Lin,
    plane::Plane as gp_Pln, point::Point as gp_Pnt, quaternion::Quaternion as gp_Quaternion,
    sphere::Sphere as gp_Sphere, transform::Transform as gp_Trsf, vector::Vector as gp_Vec,
};
