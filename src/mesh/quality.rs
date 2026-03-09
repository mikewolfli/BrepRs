//! Mesh quality optimization
//!
//! This module provides functionality for mesh quality assessment and optimization.

use super::mesh_data::{Mesh2D, Mesh3D};
use crate::geometry::{Point, Vector};

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

        quality
    }

    /// Analyze 3D mesh quality
    pub fn analyze_3d(&self, mesh: &Mesh3D) -> MeshQuality {
        let mut quality = MeshQuality::default();

        if mesh.tetrahedrons.is_empty() {
            return quality;
        }

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
