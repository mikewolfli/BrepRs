//! Surface deformation operations
//!
//! This module provides functionality for deforming surfaces through various
//! techniques including push, twist, and bend deformations.
//!
//! # Features
//! - Interactive deformation with region selection
//! - Push deformation in a specific direction
//! - Twist deformation around an axis
//! - Bend deformation with angle control
//! - Smooth falloff for natural-looking deformations

pub use crate::surface::surface_editing::{
    DeformationRegion, InteractiveDeformation,
};

use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::geometry::nurbs_surface::NurbsSurface;

/// Deform a surface by moving control points in a specific direction
///
/// # Parameters
/// - `surface`: The surface to deform
/// - `center`: The center point of the deformation
/// - `direction`: The direction of the deformation
/// - `strength`: The strength of the deformation
/// - `radius`: The radius of influence
///
/// # Returns
/// A result indicating success or failure
pub fn deform_surface_push(
    surface: &mut NurbsSurface,
    center: Point,
    direction: Vector,
    strength: StandardReal,
    radius: StandardReal,
) -> Result<(), &'static str> {
    let normalized_direction = direction.normalized();
    
    let poles = surface.poles().to_vec();
    let mut new_poles = poles.clone();
    
    for (u_idx, row) in poles.iter().enumerate() {
        for (v_idx, point) in row.iter().enumerate() {
            let distance = point.distance_to(&center);
            
            if distance < radius {
                let falloff = 1.0 - (distance / radius);
                let displacement = normalized_direction * (strength * falloff);
                new_poles[u_idx][v_idx] = *point + displacement;
            }
        }
    }
    
    let weights = surface.weights().to_vec();
    let u_knots = surface.u_knots().to_vec();
    let v_knots = surface.v_knots().to_vec();
    let u_multiplicities = surface.u_multiplicities().to_vec();
    let v_multiplicities = surface.v_multiplicities().to_vec();
    
    *surface = NurbsSurface::new(
        surface.u_degree(),
        surface.v_degree(),
        new_poles,
        weights,
        u_knots,
        v_knots,
        u_multiplicities,
        v_multiplicities,
    );
    
    Ok(())
}

/// Deform a surface by twisting around an axis
///
/// # Parameters
/// - `surface`: The surface to deform
/// - `axis_u`: If true, twist around the U axis; otherwise around V axis
/// - `angle`: The twist angle in radians
/// - `center_u`: The center U coordinate for the twist
/// - `center_v`: The center V coordinate for the twist
///
/// # Returns
/// A result indicating success or failure
pub fn deform_surface_twist(
    surface: &mut NurbsSurface,
    axis_u: bool,
    angle: StandardReal,
    center_u: usize,
    center_v: usize,
) -> Result<(), &'static str> {
    let poles = surface.poles().to_vec();
    let mut new_poles = poles.clone();
    
    let num_u = surface.nb_u_poles() as usize;
    let num_v = surface.nb_v_poles() as usize;
    
    let center_point = if center_u < num_u && center_v < num_v {
        poles[center_u][center_v]
    } else {
        Point::origin()
    };
    
    for (u_idx, row) in poles.iter().enumerate() {
        for (v_idx, point) in row.iter().enumerate() {
            let distance = if axis_u {
                (u_idx as StandardReal - center_u as StandardReal).abs()
            } else {
                (v_idx as StandardReal - center_v as StandardReal).abs()
            };
            
            let max_distance = if axis_u {
                num_u as StandardReal
            } else {
                num_v as StandardReal
            };
            
            let falloff = 1.0 - (distance / max_distance).min(1.0);
            let twist_angle = angle * falloff;
            
            let relative = *point - center_point;
            let cos_a = twist_angle.cos();
            let sin_a = twist_angle.sin();
            
            let rotated = if axis_u {
                Point::new(
                    center_point.x + relative.x * cos_a - relative.y * sin_a,
                    center_point.y + relative.x * sin_a + relative.y * cos_a,
                    point.z,
                )
            } else {
                Point::new(
                    center_point.x + relative.x * cos_a - relative.z * sin_a,
                    point.y,
                    center_point.z + relative.x * sin_a + relative.z * cos_a,
                )
            };
            
            new_poles[u_idx][v_idx] = rotated;
        }
    }
    
    let weights = surface.weights().to_vec();
    let u_knots = surface.u_knots().to_vec();
    let v_knots = surface.v_knots().to_vec();
    let u_multiplicities = surface.u_multiplicities().to_vec();
    let v_multiplicities = surface.v_multiplicities().to_vec();
    
    *surface = NurbsSurface::new(
        surface.u_degree(),
        surface.v_degree(),
        new_poles,
        weights,
        u_knots,
        v_knots,
        u_multiplicities,
        v_multiplicities,
    );
    
    Ok(())
}

/// Deform a surface by bending
///
/// # Parameters
/// - `surface`: The surface to deform
/// - `bend_angle`: The bend angle in radians
/// - `axis_u`: If true, bend around the U axis; otherwise around V axis
///
/// # Returns
/// A result indicating success or failure
pub fn deform_surface_bend(
    surface: &mut NurbsSurface,
    bend_angle: StandardReal,
    axis_u: bool,
) -> Result<(), &'static str> {
    let poles = surface.poles().to_vec();
    let mut new_poles = poles.clone();
    
    let num_u = surface.nb_u_poles() as usize;
    let num_v = surface.nb_v_poles() as usize;
    
    for (u_idx, row) in poles.iter().enumerate() {
        for (v_idx, point) in row.iter().enumerate() {
            let normalized_coord = if axis_u {
                if num_u > 1 {
                    u_idx as StandardReal / (num_u as StandardReal - 1.0)
                } else {
                    0.0
                }
            } else {
                if num_v > 1 {
                    v_idx as StandardReal / (num_v as StandardReal - 1.0)
                } else {
                    0.0
                }
            };
            
            let angle = bend_angle * (normalized_coord - 0.5);
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            let bent = if axis_u {
                Point::new(
                    point.x * cos_a - point.z * sin_a,
                    point.y,
                    point.x * sin_a + point.z * cos_a,
                )
            } else {
                Point::new(
                    point.x,
                    point.y * cos_a - point.z * sin_a,
                    point.y * sin_a + point.z * cos_a,
                )
            };
            
            new_poles[u_idx][v_idx] = bent;
        }
    }
    
    let weights = surface.weights().to_vec();
    let u_knots = surface.u_knots().to_vec();
    let v_knots = surface.v_knots().to_vec();
    let u_multiplicities = surface.u_multiplicities().to_vec();
    let v_multiplicities = surface.v_multiplicities().to_vec();
    
    *surface = NurbsSurface::new(
        surface.u_degree(),
        surface.v_degree(),
        new_poles,
        weights,
        u_knots,
        v_knots,
        u_multiplicities,
        v_multiplicities,
    );
    
    Ok(())
}

/// Trait for surfaces that can be deformed
pub trait DeformableSurface {
    /// Apply a push deformation to the surface
    fn push_deform(&mut self, center: Point, direction: Vector, strength: StandardReal, radius: StandardReal);
    
    /// Apply a twist deformation to the surface
    fn twist_deform(&mut self, axis_u: bool, angle: StandardReal, center_u: usize, center_v: usize);
    
    /// Apply a bend deformation to the surface
    fn bend_deform(&mut self, bend_angle: StandardReal, axis_u: bool);
}

impl DeformableSurface for NurbsSurface {
    fn push_deform(&mut self, center: Point, direction: Vector, strength: StandardReal, radius: StandardReal) {
        let _ = deform_surface_push(self, center, direction, strength, radius);
    }
    
    fn twist_deform(&mut self, axis_u: bool, angle: StandardReal, center_u: usize, center_v: usize) {
        let _ = deform_surface_twist(self, axis_u, angle, center_u, center_v);
    }
    
    fn bend_deform(&mut self, bend_angle: StandardReal, axis_u: bool) {
        let _ = deform_surface_bend(self, bend_angle, axis_u);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_deformation_module() {
        let _region = DeformationRegion::new(0, 10, 0, 10);
    }
    
    #[test]
    fn test_deform_surface_push() {
        let poles = vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 1.0, 0.0)],
            vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0), Point::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let u_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let v_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let u_multiplicities = vec![3, 3];
        let v_multiplicities = vec![3, 3];
        
        let mut nurbs = NurbsSurface::new(
            2, 2,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        );
        
        let center = Point::new(1.0, 1.0, 0.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        let result = deform_surface_push(&mut nurbs, center, direction, 0.1, 1.0);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_deform_surface_twist() {
        let poles = vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 1.0, 0.0)],
            vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0), Point::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let u_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let v_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let u_multiplicities = vec![3, 3];
        let v_multiplicities = vec![3, 3];
        
        let mut nurbs = NurbsSurface::new(
            2, 2,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        );
        
        let result = deform_surface_twist(&mut nurbs, true, 0.1, 1, 1);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_deform_surface_bend() {
        let poles = vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 1.0, 0.0)],
            vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0), Point::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let u_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let v_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let u_multiplicities = vec![3, 3];
        let v_multiplicities = vec![3, 3];
        
        let mut nurbs = NurbsSurface::new(
            2, 2,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        );
        
        let result = deform_surface_bend(&mut nurbs, 0.1, true);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_deformable_surface_trait() {
        let poles = vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 1.0, 0.0)],
            vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0), Point::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let u_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let v_knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let u_multiplicities = vec![3, 3];
        let v_multiplicities = vec![3, 3];
        
        let mut nurbs = NurbsSurface::new(
            2, 2,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
        );
        
        let center = Point::new(1.0, 1.0, 0.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        
        nurbs.push_deform(center, direction, 0.1, 1.0);
        nurbs.twist_deform(true, 0.1, 1, 1);
        nurbs.bend_deform(0.1, true);
    }
}
