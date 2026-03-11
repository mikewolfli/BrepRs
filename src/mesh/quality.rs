impl MeshQuality {
    /// Returns a simple overall quality metric (average of relevant fields)
    pub fn overall_quality(&self) -> f64 {
        // Example: average of avg_angle, avg_size, aspect_ratio, skewness, condition_number
        let values = [
            self.avg_angle,
            self.avg_size,
            self.aspect_ratio,
            self.skewness,
            self.condition_number,
        ];
        let sum: f64 = values.iter().sum();
        sum / values.len() as f64
    }
}
// Mesh quality optimization
//
// This module provides functionality for mesh quality assessment and optimization.

use super::mesh_data::{Mesh2D, Mesh3D};
use crate::geometry::{Point, Vector};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Mesh quality metrics
#[derive(Debug, Clone)]
pub struct MeshQuality {
    /// Minimum angle
    pub min_angle: f64,
    /// Maximum angle
    pub max_angle: f64,
    /// Average angle
    pub avg_angle: f64,
    /// Minimum area/volume
    pub min_size: f64,
    /// Maximum area/volume
    pub max_size: f64,
    /// Average area/volume
    pub avg_size: f64,
    /// Aspect ratio
    pub aspect_ratio: f64,
    /// Skewness
    pub skewness: f64,
    /// Condition number
    pub condition_number: f64,
}

impl Default for MeshQuality {
    fn default() -> Self {
        Self {
            min_angle: 0.0,
            max_angle: 0.0,
            avg_angle: 0.0,
            min_size: 0.0,
            max_size: 0.0,
            avg_size: 0.0,
            aspect_ratio: 0.0,
            skewness: 0.0,
            condition_number: 0.0,
        }
    }
}

/// Mesh quality analyzer
pub struct MeshQualityAnalyzer {
    /// Quality thresholds
    pub thresholds: QualityThresholds,
}

/// Quality thresholds
#[derive(Debug, Clone)]
pub struct QualityThresholds {
    /// Minimum acceptable angle (degrees)
    pub min_angle: f64,
    /// Maximum acceptable angle (degrees)
    pub max_angle: f64,
    /// Maximum acceptable aspect ratio
    pub max_aspect_ratio: f64,
    /// Maximum acceptable skewness
    pub max_skewness: f64,
    /// Maximum acceptable condition number
    pub max_condition_number: f64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_angle: 20.0,
            max_angle: 120.0,
            max_aspect_ratio: 5.0,
            max_skewness: 0.8,
            max_condition_number: 10.0,
        }
    }
}

impl MeshQualityAnalyzer {
    /// Create a new mesh quality analyzer
    pub fn new(thresholds: QualityThresholds) -> Self {
        Self { thresholds }
    }

    /// Analyze 2D mesh quality
    pub fn analyze_2d(&self, mesh: &Mesh2D) -> MeshQuality {
        let mut quality = MeshQuality::default();

        if mesh.faces.is_empty() {
            return quality;
        }

        #[cfg(feature = "rayon")]
        {
            let results: Vec<_> = mesh.faces.par_iter().filter_map(|face| {
                if face.vertices.len() != 3 {
                    return None;
                }

                let v0 = &mesh.vertices[face.vertices[0]];
                let v1 = &mesh.vertices[face.vertices[1]];
                let v2 = &mesh.vertices[face.vertices[2]];

                // Calculate angles
                let angle1 = self.calculate_angle(&v1.point, &v0.point, &v2.point);
                let angle2 = self.calculate_angle(&v0.point, &v1.point, &v2.point);
                let angle3 = self.calculate_angle(&v0.point, &v2.point, &v1.point);

                // Calculate area
                let area = self.calculate_triangle_area(&v0.point, &v1.point, &v2.point);

                // Calculate aspect ratio
                let aspect_ratio = self.calculate_aspect_ratio(&v0.point, &v1.point, &v2.point);

                // Calculate skewness
                let skewness = self.calculate_skewness(&v0.point, &v1.point, &v2.point);

                // Calculate condition number
                let condition_number = self.calculate_condition_number(&v0.point, &v1.point, &v2.point);

                Some((angle1, angle2, angle3, area, aspect_ratio, skewness, condition_number))
            }).collect();

            let mut angles = Vec::with_capacity(results.len() * 3);
            let mut areas = Vec::with_capacity(results.len());
            let mut aspect_ratios = Vec::with_capacity(results.len());
            let mut skewnesses = Vec::with_capacity(results.len());
            let mut condition_numbers = Vec::with_capacity(results.len());

            for (angle1, angle2, angle3, area, aspect_ratio, skewness, condition_number) in results {
                angles.push(angle1);
                angles.push(angle2);
                angles.push(angle3);
                areas.push(area);
                aspect_ratios.push(aspect_ratio);
                skewnesses.push(skewness);
                condition_numbers.push(condition_number);
            }

            if !angles.is_empty() {
                quality.min_angle = angles
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_angle = angles
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_angle = angles.iter().sum::<f64>() / angles.len() as f64;
            }

            if !areas.is_empty() {
                quality.min_size = areas
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_size = areas
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_size = areas.iter().sum::<f64>() / areas.len() as f64;
            }

            if !aspect_ratios.is_empty() {
                quality.aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
            }

            if !skewnesses.is_empty() {
                quality.skewness = skewnesses.iter().sum::<f64>() / skewnesses.len() as f64;
            }

            if !condition_numbers.is_empty() {
                quality.condition_number = 
                    condition_numbers.iter().sum::<f64>() / condition_numbers.len() as f64;
            }
        }

        #[cfg(not(feature = "rayon"))]
        {
            let mut angles = Vec::new();
            let mut areas = Vec::new();
            let mut aspect_ratios = Vec::new();
            let mut skewnesses = Vec::new();
            let mut condition_numbers = Vec::new();

            for face in &mesh.faces {
                if face.vertices.len() != 3 {
                    continue;
                }

                let v0 = &mesh.vertices[face.vertices[0]];
                let v1 = &mesh.vertices[face.vertices[1]];
                let v2 = &mesh.vertices[face.vertices[2]];

                // Calculate angles
                let angle1 = self.calculate_angle(&v1.point, &v0.point, &v2.point);
                let angle2 = self.calculate_angle(&v0.point, &v1.point, &v2.point);
                let angle3 = self.calculate_angle(&v0.point, &v2.point, &v1.point);

                angles.push(angle1);
                angles.push(angle2);
                angles.push(angle3);

                // Calculate area
                let area = self.calculate_triangle_area(&v0.point, &v1.point, &v2.point);
                areas.push(area);

                // Calculate aspect ratio
                let aspect_ratio = self.calculate_aspect_ratio(&v0.point, &v1.point, &v2.point);
                aspect_ratios.push(aspect_ratio);

                // Calculate skewness
                let skewness = self.calculate_skewness(&v0.point, &v1.point, &v2.point);
                skewnesses.push(skewness);

                // Calculate condition number
                let condition_number = self.calculate_condition_number(&v0.point, &v1.point, &v2.point);
                condition_numbers.push(condition_number);
            }

            if !angles.is_empty() {
                quality.min_angle = angles
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_angle = angles
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_angle = angles.iter().sum::<f64>() / angles.len() as f64;
            }

            if !areas.is_empty() {
                quality.min_size = areas
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_size = areas
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_size = areas.iter().sum::<f64>() / areas.len() as f64;
            }

            if !aspect_ratios.is_empty() {
                quality.aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
            }

            if !skewnesses.is_empty() {
                quality.skewness = skewnesses.iter().sum::<f64>() / skewnesses.len() as f64;
            }

            if !condition_numbers.is_empty() {
                quality.condition_number = 
                    condition_numbers.iter().sum::<f64>() / condition_numbers.len() as f64;
            }
        }

        quality
    }

    /// Analyze 3D mesh quality
    pub fn analyze_3d(&self, mesh: &Mesh3D) -> MeshQuality {
        let mut quality = MeshQuality::default();

        if mesh.tetrahedrons.is_empty() {
            return quality;
        }

        #[cfg(feature = "rayon")]
        {
            let results: Vec<_> = mesh.tetrahedrons.par_iter().map(|tetra| {
                let v0 = &mesh.vertices[tetra.vertices[0]];
                let v1 = &mesh.vertices[tetra.vertices[1]];
                let v2 = &mesh.vertices[tetra.vertices[2]];
                let v3 = &mesh.vertices[tetra.vertices[3]];

                // Calculate dihedral angles
                let angle1 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v0.point, &v1.point, &v3.point,
                );
                let angle2 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v0.point, &v2.point, &v3.point,
                );
                let angle3 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v1.point, &v2.point, &v3.point,
                );
                let angle4 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v3.point, &v0.point, &v2.point, &v3.point,
                );
                let angle5 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v3.point, &v1.point, &v2.point, &v3.point,
                );
                let angle6 = self.calculate_dihedral_angle(
                    &v0.point, &v2.point, &v3.point, &v1.point, &v2.point, &v3.point,
                );

                // Calculate volume
                let volume = 
                    self.calculate_tetrahedron_volume(&v0.point, &v1.point, &v2.point, &v3.point);

                // Calculate aspect ratio
                let aspect_ratio = 
                    self.calculate_tetrahedron_aspect_ratio(&v0.point, &v1.point, &v2.point, &v3.point);

                // Calculate skewness
                let skewness = 
                    self.calculate_tetrahedron_skewness(&v0.point, &v1.point, &v2.point, &v3.point);

                // Calculate condition number
                let condition_number = self
                    .calculate_tetrahedron_condition_number(&v0.point, &v1.point, &v2.point, &v3.point);

                (angle1, angle2, angle3, angle4, angle5, angle6, volume, aspect_ratio, skewness, condition_number)
            }).collect();

            let mut dihedral_angles = Vec::with_capacity(results.len() * 6);
            let mut volumes = Vec::with_capacity(results.len());
            let mut aspect_ratios = Vec::with_capacity(results.len());
            let mut skewnesses = Vec::with_capacity(results.len());
            let mut condition_numbers = Vec::with_capacity(results.len());

            for (angle1, angle2, angle3, angle4, angle5, angle6, volume, aspect_ratio, skewness, condition_number) in results {
                dihedral_angles.push(angle1);
                dihedral_angles.push(angle2);
                dihedral_angles.push(angle3);
                dihedral_angles.push(angle4);
                dihedral_angles.push(angle5);
                dihedral_angles.push(angle6);
                volumes.push(volume);
                aspect_ratios.push(aspect_ratio);
                skewnesses.push(skewness);
                condition_numbers.push(condition_number);
            }

            if !dihedral_angles.is_empty() {
                quality.min_angle = dihedral_angles
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_angle = dihedral_angles
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_angle = dihedral_angles.iter().sum::<f64>() / dihedral_angles.len() as f64;
            }

            if !volumes.is_empty() {
                quality.min_size = volumes
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_size = volumes
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_size = volumes.iter().sum::<f64>() / volumes.len() as f64;
            }

            if !aspect_ratios.is_empty() {
                quality.aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
            }

            if !skewnesses.is_empty() {
                quality.skewness = skewnesses.iter().sum::<f64>() / skewnesses.len() as f64;
            }

            if !condition_numbers.is_empty() {
                quality.condition_number = 
                    condition_numbers.iter().sum::<f64>() / condition_numbers.len() as f64;
            }
        }

        #[cfg(not(feature = "rayon"))]
        {
            let mut dihedral_angles = Vec::new();
            let mut volumes = Vec::new();
            let mut aspect_ratios = Vec::new();
            let mut skewnesses = Vec::new();
            let mut condition_numbers = Vec::new();

            for tetra in &mesh.tetrahedrons {
                let v0 = &mesh.vertices[tetra.vertices[0]];
                let v1 = &mesh.vertices[tetra.vertices[1]];
                let v2 = &mesh.vertices[tetra.vertices[2]];
                let v3 = &mesh.vertices[tetra.vertices[3]];

                // Calculate dihedral angles
                let angle1 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v0.point, &v1.point, &v3.point,
                );
                let angle2 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v0.point, &v2.point, &v3.point,
                );
                let angle3 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v2.point, &v1.point, &v2.point, &v3.point,
                );
                let angle4 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v3.point, &v0.point, &v2.point, &v3.point,
                );
                let angle5 = self.calculate_dihedral_angle(
                    &v0.point, &v1.point, &v3.point, &v1.point, &v2.point, &v3.point,
                );
                let angle6 = self.calculate_dihedral_angle(
                    &v0.point, &v2.point, &v3.point, &v1.point, &v2.point, &v3.point,
                );

                dihedral_angles.push(angle1);
                dihedral_angles.push(angle2);
                dihedral_angles.push(angle3);
                dihedral_angles.push(angle4);
                dihedral_angles.push(angle5);
                dihedral_angles.push(angle6);

                // Calculate volume
                let volume = 
                    self.calculate_tetrahedron_volume(&v0.point, &v1.point, &v2.point, &v3.point);
                volumes.push(volume);

                // Calculate aspect ratio
                let aspect_ratio = 
                    self.calculate_tetrahedron_aspect_ratio(&v0.point, &v1.point, &v2.point, &v3.point);
                aspect_ratios.push(aspect_ratio);

                // Calculate skewness
                let skewness = 
                    self.calculate_tetrahedron_skewness(&v0.point, &v1.point, &v2.point, &v3.point);
                skewnesses.push(skewness);

                // Calculate condition number
                let condition_number = self
                    .calculate_tetrahedron_condition_number(&v0.point, &v1.point, &v2.point, &v3.point);
                condition_numbers.push(condition_number);
            }

            if !dihedral_angles.is_empty() {
                quality.min_angle = dihedral_angles
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_angle = dihedral_angles
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_angle = dihedral_angles.iter().sum::<f64>() / dihedral_angles.len() as f64;
            }

            if !volumes.is_empty() {
                quality.min_size = volumes
                    .iter()
                    .copied()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.max_size = volumes
                    .iter()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                quality.avg_size = volumes.iter().sum::<f64>() / volumes.len() as f64;
            }

            if !aspect_ratios.is_empty() {
                quality.aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
            }

            if !skewnesses.is_empty() {
                quality.skewness = skewnesses.iter().sum::<f64>() / skewnesses.len() as f64;
            }

            if !condition_numbers.is_empty() {
                quality.condition_number = 
                    condition_numbers.iter().sum::<f64>() / condition_numbers.len() as f64;
            }
        }

        quality
    }

    /// Calculate angle between three points
    fn calculate_angle(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let v1 = Vector::new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z);
        let v2 = Vector::new(p3.x - p2.x, p3.y - p2.y, p3.z - p2.z);
        let dot = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
        let mag1 = (v1.x * v1.x + v1.y * v1.y + v1.z * v1.z).sqrt();
        let mag2 = (v2.x * v2.x + v2.y * v2.y + v2.z * v2.z).sqrt();

        if mag1 < 1e-6 || mag2 < 1e-6 {
            return 0.0;
        }

        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        cos_angle.acos() * 180.0 / std::f64::consts::PI
    }

    /// Calculate triangle area
    fn calculate_triangle_area(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let v1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let v2 = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        let cross = Vector::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        );
        0.5 * (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt()
    }

    /// Calculate aspect ratio of a triangle
    fn calculate_aspect_ratio(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let a = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt();
        let b = ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt();
        let c = ((p1.x - p3.x).powi(2) + (p1.y - p3.y).powi(2) + (p1.z - p3.z).powi(2)).sqrt();
        let max_side = a.max(b).max(c);
        let min_side = a.min(b).min(c);
        max_side / min_side
    }

    /// Calculate skewness of a triangle
    fn calculate_skewness(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let angles = vec![
            self.calculate_angle(&p2, &p1, &p3),
            self.calculate_angle(&p1, &p2, &p3),
            self.calculate_angle(&p1, &p3, &p2),
        ];
        let max_angle = angles
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        (max_angle - 60.0) / 120.0
    }

    /// Calculate condition number of a triangle
    fn calculate_condition_number(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let a = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt();
        let b = ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt();
        let c = ((p1.x - p3.x).powi(2) + (p1.y - p3.y).powi(2) + (p1.z - p3.z).powi(2)).sqrt();
        let min_side = a.min(b).min(c);
        let max_side = a.max(b).max(c);
        max_side / min_side
    }

    /// Calculate dihedral angle between two planes
    fn calculate_dihedral_angle(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
        p5: &Point,
        p6: &Point,
    ) -> f64 {
        let v1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let v2 = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        let normal1 = Vector::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        );

        let v3 = Vector::new(p5.x - p4.x, p5.y - p4.y, p5.z - p4.z);
        let v4 = Vector::new(p6.x - p4.x, p6.y - p4.y, p6.z - p4.z);
        let normal2 = Vector::new(
            v3.y * v4.z - v3.z * v4.y,
            v3.z * v4.x - v3.x * v4.z,
            v3.x * v4.y - v3.y * v4.x,
        );

        let dot = normal1.x * normal2.x + normal1.y * normal2.y + normal1.z * normal2.z;
        let mag1 = (normal1.x * normal1.x + normal1.y * normal1.y + normal1.z * normal1.z).sqrt();
        let mag2 = (normal2.x * normal2.x + normal2.y * normal2.y + normal2.z * normal2.z).sqrt();

        if mag1 < 1e-6 || mag2 < 1e-6 {
            return 0.0;
        }

        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        cos_angle.acos() * 180.0 / std::f64::consts::PI
    }

    /// Calculate tetrahedron volume
    fn calculate_tetrahedron_volume(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
        let v1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let v2 = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        let v3 = Vector::new(p4.x - p1.x, p4.y - p1.y, p4.z - p1.z);

        let cross = Vector::new(
            v2.y * v3.z - v2.z * v3.y,
            v2.z * v3.x - v2.x * v3.z,
            v2.x * v3.y - v2.y * v3.x,
        );

        (v1.x * cross.x + v1.y * cross.y + v1.z * cross.z).abs() / 6.0
    }

    /// Calculate tetrahedron aspect ratio
    fn calculate_tetrahedron_aspect_ratio(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> f64 {
        let edges = vec![
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2) + (p3.z - p1.z).powi(2)).sqrt(),
            ((p4.x - p1.x).powi(2) + (p4.y - p1.y).powi(2) + (p4.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p2.x).powi(2) + (p4.y - p2.y).powi(2) + (p4.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p3.x).powi(2) + (p4.y - p3.y).powi(2) + (p4.z - p3.z).powi(2)).sqrt(),
        ];

        let min_edge = edges
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_edge = edges
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        max_edge / min_edge
    }

    /// Calculate tetrahedron skewness
    fn calculate_tetrahedron_skewness(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> f64 {
        let volume = self.calculate_tetrahedron_volume(p1, p2, p3, p4);
        let edges = vec![
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2) + (p3.z - p1.z).powi(2)).sqrt(),
            ((p4.x - p1.x).powi(2) + (p4.y - p1.y).powi(2) + (p4.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p2.x).powi(2) + (p4.y - p2.y).powi(2) + (p4.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p3.x).powi(2) + (p4.y - p3.y).powi(2) + (p4.z - p3.z).powi(2)).sqrt(),
        ];

        let product = edges.iter().product::<f64>();
        let ideal_volume = product / (6.0 * 2.0_f64.sqrt());
        1.0 - (volume / ideal_volume)
    }

    /// Calculate tetrahedron condition number
    fn calculate_tetrahedron_condition_number(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> f64 {
        let edges = vec![
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2) + (p3.z - p1.z).powi(2)).sqrt(),
            ((p4.x - p1.x).powi(2) + (p4.y - p1.y).powi(2) + (p4.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p2.x).powi(2) + (p4.y - p2.y).powi(2) + (p4.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p3.x).powi(2) + (p4.y - p3.y).powi(2) + (p4.z - p3.z).powi(2)).sqrt(),
        ];

        let min_edge = edges
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_edge = edges
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        max_edge / min_edge
    }

    /// Check if mesh quality is acceptable
    pub fn is_quality_acceptable(&self, quality: &MeshQuality) -> bool {
        quality.min_angle >= self.thresholds.min_angle
            && quality.max_angle <= self.thresholds.max_angle
            && quality.aspect_ratio <= self.thresholds.max_aspect_ratio
            && quality.skewness <= self.thresholds.max_skewness
            && quality.condition_number <= self.thresholds.max_condition_number
    }
}

/// Mesh quality repairer
pub struct MeshQualityRepairer {
    /// Repair parameters
    pub params: RepairParams,
}

/// Repair parameters
#[derive(Debug, Clone)]
pub struct RepairParams {
    /// Number of smoothing iterations
    pub smoothing_iterations: usize,
    /// Smoothing factor
    pub smoothing_factor: f64,
    /// Taubin smoothing lambda
    pub taubin_lambda: f64,
    /// Taubin smoothing mu
    pub taubin_mu: f64,
    /// Maximum edge length for hole filling
    pub max_hole_edge_length: f64,
    /// Minimum hole area to fill
    pub min_hole_area: f64,
    /// Maximum hole area to fill
    pub max_hole_area: f64,
}

impl Default for RepairParams {
    fn default() -> Self {
        Self {
            smoothing_iterations: 5,
            smoothing_factor: 0.5,
            taubin_lambda: 0.5,
            taubin_mu: -0.53,
            max_hole_edge_length: 1.0,
            min_hole_area: 0.001,
            max_hole_area: 10.0,
        }
    }
}

impl MeshQualityRepairer {
    /// Create a new mesh quality repairer
    pub fn new(params: RepairParams) -> Self {
        Self { params }
    }

    /// Repair 2D mesh
    pub fn repair_2d(&self, mesh: &mut Mesh2D) -> bool {
        // Fix flipped faces
        self.fix_flipped_faces_2d(mesh);

        // Remove overlapping faces
        self.remove_overlapping_faces_2d(mesh);

        // Fill holes
        self.fill_holes_2d(mesh);

        // Smooth mesh
        self.smooth_mesh_2d(mesh);

        // Check normal consistency
        self.fix_normal_consistency_2d(mesh);

        true
    }

    /// Repair 3D mesh
    pub fn repair_3d(&self, mesh: &mut Mesh3D) -> bool {
        // Fix flipped tetrahedrons
        self.fix_flipped_tetrahedrons_3d(mesh);

        // Remove overlapping elements
        self.remove_overlapping_elements_3d(mesh);

        // Smooth mesh
        self.smooth_mesh_3d(mesh);

        // Check normal consistency
        self.fix_normal_consistency_3d(mesh);

        true
    }

    /// Fix flipped faces in 2D mesh
    fn fix_flipped_faces_2d(&self, mesh: &mut Mesh2D) {
        for face in &mut mesh.faces {
            if face.vertices.len() == 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;

                // Calculate face normal
                let v1v0 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                let v2v0 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                let normal = Vector::new(
                    v1v0.y * v2v0.z - v1v0.z * v2v0.y,
                    v1v0.z * v2v0.x - v1v0.x * v2v0.z,
                    v1v0.x * v2v0.y - v1v0.y * v2v0.x,
                );

                // Check if face is flipped (negative z-component for 2D)
                if normal.z < 0.0 {
                    // Reverse face vertices
                    face.vertices.reverse();
                }
            }
        }
    }

    /// Remove overlapping faces in 2D mesh
    fn remove_overlapping_faces_2d(&self, mesh: &mut Mesh2D) {
        let mut to_remove = Vec::new();

        for i in 0..mesh.faces.len() {
            for j in i + 1..mesh.faces.len() {
                if self.are_faces_overlapping_2d(mesh, i, j) {
                    to_remove.push(j);
                }
            }
        }

        // Remove overlapping faces (in reverse order to avoid index shifting)
        to_remove.sort_by(|a, b| b.cmp(a));
        for idx in to_remove {
            mesh.faces.remove(idx);
        }
    }

    /// Check if two faces are overlapping in 2D
    fn are_faces_overlapping_2d(&self, mesh: &Mesh2D, face1_idx: usize, face2_idx: usize) -> bool {
        let face1 = &mesh.faces[face1_idx];
        let face2 = &mesh.faces[face2_idx];

        if face1.vertices.len() != 3 || face2.vertices.len() != 3 {
            return false;
        }

        let v1_0 = &mesh.vertices[face1.vertices[0]].point;
        let v1_1 = &mesh.vertices[face1.vertices[1]].point;
        let v1_2 = &mesh.vertices[face1.vertices[2]].point;

        let v2_0 = &mesh.vertices[face2.vertices[0]].point;
        let v2_1 = &mesh.vertices[face2.vertices[1]].point;
        let v2_2 = &mesh.vertices[face2.vertices[2]].point;

        // Check if any vertex of face1 is inside face2
        if self.is_point_in_triangle(v2_0, v1_0, v1_1, v1_2)
            || self.is_point_in_triangle(v2_1, v1_0, v1_1, v1_2)
            || self.is_point_in_triangle(v2_2, v1_0, v1_1, v1_2)
        {
            return true;
        }

        // Check if any vertex of face2 is inside face1
        if self.is_point_in_triangle(v1_0, v2_0, v2_1, v2_2)
            || self.is_point_in_triangle(v1_1, v2_0, v2_1, v2_2)
            || self.is_point_in_triangle(v1_2, v2_0, v2_1, v2_2)
        {
            return true;
        }

        false
    }

    /// Check if a point is inside a triangle
    fn is_point_in_triangle(&self, p: &Point, v0: &Point, v1: &Point, v2: &Point) -> bool {
        let v0v1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let v0v2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        let v0p = Vector::new(p.x - v0.x, p.y - v0.y, p.z - v0.z);

        let dot00 = v0v2.dot(&v0v2);
        let dot01 = v0v2.dot(&v0v1);
        let dot02 = v0v2.dot(&v0p);
        let dot11 = v0v1.dot(&v0v1);
        let dot12 = v0v1.dot(&v0p);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
    }

    /// Fill holes in 2D mesh
    fn fill_holes_2d(&self, mesh: &mut Mesh2D) {
        // Identify holes (boundary edges with only one face)
        let mut edge_face_count = std::collections::HashMap::new();

        for (face_id, face) in mesh.faces.iter().enumerate() {
            for i in 0..face.vertices.len() {
                let v0 = face.vertices[i];
                let v1 = face.vertices[(i + 1) % face.vertices.len()];
                let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                edge_face_count
                    .entry(edge)
                    .or_insert(Vec::new())
                    .push(face_id);
            }
        }

        // Find boundary edges
        let boundary_edges: Vec<_> = edge_face_count
            .iter()
            .filter(|(_, faces)| faces.len() == 1)
            .map(|(edge, _)| *edge)
            .collect();

        // Group boundary edges into holes
        let holes = self.group_boundary_edges_into_holes(&boundary_edges);

        // Fill each hole
        for hole in holes {
            if self.should_fill_hole(mesh, &hole) {
                self.fill_hole_2d(mesh, &hole);
            }
        }
    }

    /// Group boundary edges into holes
    fn group_boundary_edges_into_holes(
        &self,
        boundary_edges: &[(usize, usize)],
    ) -> Vec<Vec<(usize, usize)>> {
        let mut holes = Vec::new();
        let mut used_edges = std::collections::HashSet::new();

        for edge in boundary_edges {
            if !used_edges.contains(edge) {
                let mut hole = Vec::new();
                let mut current_edge = *edge;

                loop {
                    hole.push(current_edge);
                    used_edges.insert(current_edge);

                    // Find next edge
                    let mut next_edge = None;
                    for e in boundary_edges {
                        if !used_edges.contains(e) {
                            if e.0 == current_edge.1 || e.1 == current_edge.1 {
                                next_edge = Some(*e);
                                break;
                            }
                        }
                    }

                    if let Some(e) = next_edge {
                        current_edge = e;
                    } else {
                        break;
                    }
                }

                if hole.len() >= 3 {
                    holes.push(hole);
                }
            }
        }

        holes
    }

    /// Check if a hole should be filled
    fn should_fill_hole(&self, mesh: &Mesh2D, hole: &[(usize, usize)]) -> bool {
        if hole.len() < 3 {
            return false;
        }

        // Calculate hole area
        let area = self.calculate_hole_area(mesh, hole);
        if area < self.params.min_hole_area || area > self.params.max_hole_area {
            return false;
        }

        // Check edge lengths
        for &(v0, v1) in hole {
            let p0 = &mesh.vertices[v0].point;
            let p1 = &mesh.vertices[v1].point;
            let length = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt();
            if length > self.params.max_hole_edge_length {
                return false;
            }
        }

        true
    }

    /// Calculate hole area
    fn calculate_hole_area(&self, mesh: &Mesh2D, hole: &[(usize, usize)]) -> f64 {
        if hole.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        let mut vertices = Vec::new();

        for &(v0, _v1) in hole {
            vertices.push(&mesh.vertices[v0].point);
        }

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            area += vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
        }

        area.abs() / 2.0
    }

    /// Fill a hole in 2D mesh
    fn fill_hole_2d(&self, mesh: &mut Mesh2D, hole: &[(usize, usize)]) {
        if hole.len() < 3 {
            return;
        }

        // Collect hole vertices
        let mut hole_vertices = Vec::new();
        for &(v0, _v1) in hole {
            if !hole_vertices.contains(&v0) {
                hole_vertices.push(v0);
            }
        }

        // Simple triangulation: fan from first vertex
        let first_vertex = hole_vertices[0];
        for i in 2..hole_vertices.len() {
            mesh.add_face(first_vertex, hole_vertices[i - 1], hole_vertices[i]);
        }
    }

    /// Smooth 2D mesh using Laplacian smoothing
    fn smooth_mesh_2d(&self, mesh: &mut Mesh2D) {
        for _ in 0..self.params.smoothing_iterations {
            let mut new_positions = Vec::new();

            for (i, vertex) in mesh.vertices.iter().enumerate() {
                // Find adjacent vertices
                let mut adjacent_vertices = std::collections::HashSet::new();

                for face in &mesh.faces {
                    if face.vertices.contains(&i) {
                        for &v in &face.vertices {
                            if v != i {
                                adjacent_vertices.insert(v);
                            }
                        }
                    }
                }

                if !adjacent_vertices.is_empty() {
                    // Calculate average position
                    let mut avg_x = 0.0;
                    let mut avg_y = 0.0;
                    let mut avg_z = 0.0;

                    for &v in &adjacent_vertices {
                        let adj_vertex = &mesh.vertices[v];
                        avg_x += adj_vertex.point.x;
                        avg_y += adj_vertex.point.y;
                        avg_z += adj_vertex.point.z;
                    }

                    let count = adjacent_vertices.len() as f64;
                    avg_x /= count;
                    avg_y /= count;
                    avg_z /= count;

                    // Move vertex towards average position
                    let new_x = vertex.point.x * (1.0 - self.params.smoothing_factor)
                        + avg_x * self.params.smoothing_factor;
                    let new_y = vertex.point.y * (1.0 - self.params.smoothing_factor)
                        + avg_y * self.params.smoothing_factor;
                    let new_z = vertex.point.z * (1.0 - self.params.smoothing_factor)
                        + avg_z * self.params.smoothing_factor;

                    new_positions.push(Point::new(new_x, new_y, new_z));
                } else {
                    new_positions.push(vertex.point.clone());
                }
            }

            // Update vertex positions
            for (i, new_pos) in new_positions.iter().enumerate() {
                mesh.vertices[i].point = new_pos.clone();
            }
        }
    }

    /// Fix normal consistency in 2D mesh
    fn fix_normal_consistency_2d(&self, mesh: &mut Mesh2D) {
        // Simple approach: ensure all faces have the same winding order
        if !mesh.faces.is_empty() {
            let first_face = &mesh.faces[0];
            if first_face.vertices.len() == 3 {
                let v0 = &mesh.vertices[first_face.vertices[0]].point;
                let v1 = &mesh.vertices[first_face.vertices[1]].point;
                let v2 = &mesh.vertices[first_face.vertices[2]].point;

                // Calculate reference normal
                let v1v0 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                let v2v0 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                let ref_normal = Vector::new(
                    v1v0.y * v2v0.z - v1v0.z * v2v0.y,
                    v1v0.z * v2v0.x - v1v0.x * v2v0.z,
                    v1v0.x * v2v0.y - v1v0.y * v2v0.x,
                );

                // Adjust all faces to match reference normal direction
                for face in &mut mesh.faces {
                    if face.vertices.len() == 3 {
                        let v0 = &mesh.vertices[face.vertices[0]].point;
                        let v1 = &mesh.vertices[face.vertices[1]].point;
                        let v2 = &mesh.vertices[face.vertices[2]].point;

                        let v1v0 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                        let v2v0 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                        let normal = Vector::new(
                            v1v0.y * v2v0.z - v1v0.z * v2v0.y,
                            v1v0.z * v2v0.x - v1v0.x * v2v0.z,
                            v1v0.x * v2v0.y - v1v0.y * v2v0.x,
                        );

                        if normal.dot(&ref_normal) < 0.0 {
                            face.vertices.reverse();
                        }
                    }
                }
            }
        }
    }

    /// Fix flipped tetrahedrons in 3D mesh
    fn fix_flipped_tetrahedrons_3d(&self, mesh: &mut Mesh3D) {
        let mut to_remove = Vec::new();

        for (tetra_id, tetra) in mesh.tetrahedrons.iter().enumerate() {
            let v0 = &mesh.vertices[tetra.vertices[0]].point;
            let v1 = &mesh.vertices[tetra.vertices[1]].point;
            let v2 = &mesh.vertices[tetra.vertices[2]].point;
            let v3 = &mesh.vertices[tetra.vertices[3]].point;

            // Calculate tetrahedron volume
            let v1v0 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
            let v2v0 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
            let v3v0 = Vector::new(v3.x - v0.x, v3.y - v0.y, v3.z - v0.z);

            let cross = Vector::new(
                v2v0.y * v3v0.z - v2v0.z * v3v0.y,
                v2v0.z * v3v0.x - v2v0.x * v3v0.z,
                v2v0.x * v3v0.y - v2v0.y * v3v0.x,
            );

            let volume = (v1v0.x * cross.x + v1v0.y * cross.y + v1v0.z * cross.z) / 6.0;

            // Remove tetrahedrons with negative volume
            if volume < 0.0 {
                to_remove.push(tetra_id);
            }
        }

        // Remove flipped tetrahedrons
        to_remove.sort_by(|a, b| b.cmp(a));
        for idx in to_remove {
            mesh.tetrahedrons.remove(idx);
        }
    }

    /// Remove overlapping elements in 3D mesh
    fn remove_overlapping_elements_3d(&self, _mesh: &mut Mesh3D) {
        // This is a simplified implementation
        // In a real implementation, we would check for overlapping tetrahedrons
    }

    /// Smooth 3D mesh using Taubin smoothing
    fn smooth_mesh_3d(&self, mesh: &mut Mesh3D) {
        for _ in 0..self.params.smoothing_iterations {
            // Taubin smoothing: two steps
            self.taubin_smooth_step(mesh, self.params.taubin_lambda);
            self.taubin_smooth_step(mesh, self.params.taubin_mu);
        }
    }

    /// Taubin smoothing step
    fn taubin_smooth_step(&self, mesh: &mut Mesh3D, factor: f64) {
        let mut new_positions = Vec::new();

        for (i, vertex) in mesh.vertices.iter().enumerate() {
            // Find adjacent vertices
            let mut adjacent_vertices = std::collections::HashSet::new();

            for tetra in &mesh.tetrahedrons {
                if tetra.vertices.contains(&i) {
                    for &v in &tetra.vertices {
                        if v != i {
                            adjacent_vertices.insert(v);
                        }
                    }
                }
            }

            if !adjacent_vertices.is_empty() {
                // Calculate average position
                let mut avg_x = 0.0;
                let mut avg_y = 0.0;
                let mut avg_z = 0.0;

                for &v in &adjacent_vertices {
                    let adj_vertex = &mesh.vertices[v];
                    avg_x += adj_vertex.point.x;
                    avg_y += adj_vertex.point.y;
                    avg_z += adj_vertex.point.z;
                }

                let count = adjacent_vertices.len() as f64;
                avg_x /= count;
                avg_y /= count;
                avg_z /= count;

                // Move vertex
                let new_x = vertex.point.x + factor * (avg_x - vertex.point.x);
                let new_y = vertex.point.y + factor * (avg_y - vertex.point.y);
                let new_z = vertex.point.z + factor * (avg_z - vertex.point.z);

                new_positions.push(Point::new(new_x, new_y, new_z));
            } else {
                new_positions.push(vertex.point.clone());
            }
        }

        // Update vertex positions
        for (i, new_pos) in new_positions.iter().enumerate() {
            mesh.vertices[i].point = new_pos.clone();
        }
    }

    /// Fix normal consistency in 3D mesh
    fn fix_normal_consistency_3d(&self, _mesh: &mut Mesh3D) {
        // This is a simplified implementation
        // In a real implementation, we would check and fix normal consistency for faces
    }
}

#[cfg(test)]
mod repair_tests {
    use super::*;

    #[test]
    fn test_mesh_quality_repairer_creation() {
        let params = RepairParams::default();
        let repairer = MeshQualityRepairer::new(params);
        assert_eq!(repairer.params.smoothing_iterations, 5);
    }

    #[test]
    fn test_fix_flipped_faces_2d() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        // Add a flipped face (clockwise winding)
        mesh.faces
            .push(crate::mesh::mesh_data::MeshFace::new(0, vec![v0, v2, v1]));

        let repairer = MeshQualityRepairer::new(RepairParams::default());
        repairer.fix_flipped_faces_2d(&mut mesh);

        // Check if face was fixed
        let face = &mesh.faces[0];
        assert_eq!(face.vertices, vec![v0, v1, v2]);
    }

    #[test]
    fn test_smooth_mesh_2d() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        mesh.add_face(v0, v1, v2);
        mesh.add_face(v0, v2, v3);

        let repairer = MeshQualityRepairer::new(RepairParams::default());
        repairer.smooth_mesh_2d(&mut mesh);

        // Check if vertices were moved
        assert!(mesh.vertices[v0].point.x != 0.0 || mesh.vertices[v0].point.y != 0.0);
    }

    #[test]
    fn test_repair_2d() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        mesh.add_face(v0, v1, v2);
        mesh.add_face(v0, v2, v3);

        let repairer = MeshQualityRepairer::new(RepairParams::default());
        let result = repairer.repair_2d(&mut mesh);
        assert!(result);
    }

    #[test]
    fn test_repair_3d() {
        let mut mesh = Mesh3D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v4 = mesh.add_vertex(Point::new(0.5, 0.5, 1.0));

        mesh.add_tetrahedron(v0, v1, v2, v4);
        mesh.add_tetrahedron(v0, v2, v3, v4);

        let repairer = MeshQualityRepairer::new(RepairParams::default());
        let result = repairer.repair_3d(&mut mesh);
        assert!(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_quality_analyzer_creation() {
        let thresholds = QualityThresholds::default();
        let analyzer = MeshQualityAnalyzer::new(thresholds);
        assert_eq!(analyzer.thresholds.min_angle, 20.0);
    }

    #[test]
    fn test_calculate_angle() {
        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let angle = analyzer.calculate_angle(&p2, &p1, &p3);
        assert!((angle - 90.0).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_triangle_area() {
        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let area = analyzer.calculate_triangle_area(&p1, &p2, &p3);
        assert!((area - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_aspect_ratio() {
        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let aspect_ratio = analyzer.calculate_aspect_ratio(&p1, &p2, &p3);
        assert!((aspect_ratio - 1.41421356237).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_tetrahedron_volume() {
        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let p4 = Point::new(0.0, 0.0, 1.0);
        let volume = analyzer.calculate_tetrahedron_volume(&p1, &p2, &p3, &p4);
        assert!((volume - 1.0 / 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_analyze_2d() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let quality = analyzer.analyze_2d(&mesh);
        assert!(quality.avg_angle > 0.0);
        assert!(quality.avg_size > 0.0);
    }

    #[test]
    fn test_analyze_3d() {
        let mut mesh = Mesh3D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0));
        mesh.add_tetrahedron(v0, v1, v2, v3);

        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let quality = analyzer.analyze_3d(&mesh);
        assert!(quality.avg_angle > 0.0);
        assert!(quality.avg_size > 0.0);
    }

    #[test]
    fn test_is_quality_acceptable() {
        let analyzer = MeshQualityAnalyzer::new(QualityThresholds::default());
        let mut quality = MeshQuality::default();
        quality.min_angle = 30.0;
        quality.max_angle = 100.0;
        quality.aspect_ratio = 2.0;
        quality.skewness = 0.5;
        quality.condition_number = 5.0;

        assert!(analyzer.is_quality_acceptable(&quality));
    }
}
