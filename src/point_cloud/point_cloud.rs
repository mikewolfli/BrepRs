//! Point cloud structure
//! 
//! This module defines the basic point cloud data structure and operations.

use crate::geometry::{Point, Vector};

/// Point cloud
#[derive(Debug, Clone)]
pub struct PointCloud {
    /// Points in the point cloud
    points: Vec<Point>,
    /// Optional normals for each point
    normals: Option<Vec<Vector>>,
    /// Optional colors for each point
    colors: Option<Vec<(u8, u8, u8)>>,
}

impl PointCloud {
    /// Create a new empty point cloud
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            normals: None,
            colors: None,
        }
    }

    /// Create a point cloud from a list of points
    pub fn from_points(points: Vec<Point>) -> Self {
        Self {
            points,
            normals: None,
            colors: None,
        }
    }

    /// Create a point cloud with normals
    pub fn from_points_with_normals(points: Vec<Point>, normals: Vec<Vector>) -> Result<Self, String> {
        if points.len() != normals.len() {
            return Err(crate::foundation::exception::Failure::range_error(
                "Number of points and normals must match",
                Some(format!("from_points_with_normals: points.len()={}, normals.len()={}", points.len(), normals.len())),
                None,
            ));
        }
        
        Ok(Self {
            points,
            normals: Some(normals),
            colors: None,
        })
    }

    /// Create a point cloud with colors
    pub fn from_points_with_colors(points: Vec<Point>, colors: Vec<(u8, u8, u8)>) -> Result<Self, String> {
        if points.len() != colors.len() {
            return Err(crate::foundation::exception::Failure::range_error(
                "Number of points and colors must match",
                Some(format!("from_points_with_colors: points.len()={}, colors.len()={}", points.len(), colors.len())),
                None,
            ));
        }
        
        Ok(Self {
            points,
            normals: None,
            colors: Some(colors),
        })
    }

    /// Create a point cloud with both normals and colors
    pub fn from_points_with_normals_and_colors(
        points: Vec<Point>,
        normals: Vec<Vector>,
        colors: Vec<(u8, u8, u8)>,
    ) -> Result<Self, String> {
        if points.len() != normals.len() || points.len() != colors.len() {
            return Err(crate::foundation::exception::Failure::range_error(
                "Number of points, normals, and colors must match",
                Some(format!("from_points_with_normals_and_colors: points.len()={}, normals.len()={}, colors.len()={}", points.len(), normals.len(), colors.len())),
                None,
            ));
        }
        
        Ok(Self {
            points,
            normals: Some(normals),
            colors: Some(colors),
        })
    }

    /// Add a point to the point cloud
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        
        // If normals or colors are present, we need to handle them
        if self.normals.is_some() {
            self.normals.as_mut().unwrap().push(Vector::zero());
        }
        
        if self.colors.is_some() {
            self.colors.as_mut().unwrap().push((0, 0, 0));
        }
    }

    /// Add a point with normal
    pub fn add_point_with_normal(&mut self, point: Point, normal: Vector) {
        self.points.push(point);
        
        if let Some(normals) = &mut self.normals {
            normals.push(normal);
        } else {
            self.normals = Some(vec![normal]);
        }
        
        if self.colors.is_some() {
            self.colors.as_mut().unwrap().push((0, 0, 0));
        }
    }

    /// Add a point with color
    pub fn add_point_with_color(&mut self, point: Point, color: (u8, u8, u8)) {
        self.points.push(point);
        
        if self.normals.is_some() {
            self.normals.as_mut().unwrap().push(Vector::zero());
        }
        
        if let Some(colors) = &mut self.colors {
            colors.push(color);
        } else {
            self.colors = Some(vec![color]);
        }
    }

    /// Add a point with both normal and color
    pub fn add_point_with_normal_and_color(&mut self, point: Point, normal: Vector, color: (u8, u8, u8)) {
        self.points.push(point);
        
        if let Some(normals) = &mut self.normals {
            normals.push(normal);
        } else {
            self.normals = Some(vec![normal]);
        }
        
        if let Some(colors) = &mut self.colors {
            colors.push(color);
        } else {
            self.colors = Some(vec![color]);
        }
    }

    /// Get the number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the point cloud is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get the points
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Get mutable access to the points
    pub fn points_mut(&mut self) -> &mut Vec<Point> {
        &mut self.points
    }

    /// Get the normals
    pub fn normals(&self) -> Option<&[Vector]> {
        self.normals.as_deref()
    }

    /// Get mutable access to the normals
    pub fn normals_mut(&mut self) -> Option<&mut Vec<Vector>> {
        self.normals.as_mut()
    }

    /// Get the colors
    pub fn colors(&self) -> Option<&[(u8, u8, u8)]> {
        self.colors.as_deref()
    }

    /// Get mutable access to the colors
    pub fn colors_mut(&mut self) -> Option<&mut Vec<(u8, u8, u8)>> {
        self.colors.as_mut()
    }

    /// Calculate the bounding box of the point cloud
    pub fn bounding_box(&self) -> (Point, Point) {
        if self.points.is_empty() {
            return (Point::origin(), Point::origin());
        }
        
        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut min_z = self.points[0].z;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;
        let mut max_z = self.points[0].z;
        
        for point in &self.points[1..] {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }
        
        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z)
        )
    }

    /// Calculate the centroid of the point cloud
    pub fn centroid(&self) -> Point {
        if self.points.is_empty() {
            return Point::origin();
        }
        
        let sum = self.points.iter().fold(Point::origin(), |sum, p| sum + *p);
        sum / self.points.len() as f64
    }

    /// Transform the point cloud
    pub fn transform(&mut self, transformation: &impl Fn(&Point) -> Point) {
        for point in &mut self.points {
            *point = transformation(point);
        }
        
        // If normals are present, we need to transform them too
        if let Some(normals) = &mut self.normals {
            for normal in normals {
                // For normals, we should use the inverse transpose of the transformation
                // This is a simplified version - in a real implementation, we would need
                // to handle the transformation matrix properly
                *normal = (transformation(&(*normal + Point::origin())) - Point::origin()).normalize();
            }
        }
    }

    /// Filter the point cloud based on a predicate
    pub fn filter<F>(&mut self, predicate: F) where F: Fn(&Point) -> bool {
        let mut filtered_points = Vec::new();
        let mut filtered_normals = if self.normals.is_some() { Some(Vec::new()) } else { None };
        let mut filtered_colors = if self.colors.is_some() { Some(Vec::new()) } else { None };
        
        for (i, point) in self.points.iter().enumerate() {
            if predicate(point) {
                filtered_points.push(*point);
                
                if let Some(normals) = &self.normals {
                    filtered_normals.as_mut().unwrap().push(normals[i]);
                }
                
                if let Some(colors) = &self.colors {
                    filtered_colors.as_mut().unwrap().push(colors[i]);
                }
            }
        }
        
        self.points = filtered_points;
        self.normals = filtered_normals;
        self.colors = filtered_colors;
    }

    /// Downsample the point cloud by keeping every nth point
    pub fn downsample(&mut self, factor: usize) {
        if factor <= 1 {
            return;
        }
        
        let mut downsampled_points = Vec::new();
        let mut downsampled_normals = if self.normals.is_some() { Some(Vec::new()) } else { None };
        let mut downsampled_colors = if self.colors.is_some() { Some(Vec::new()) } else { None };
        
        for (i, point) in self.points.iter().enumerate() {
            if i % factor == 0 {
                downsampled_points.push(*point);
                
                if let Some(normals) = &self.normals {
                    downsampled_normals.as_mut().unwrap().push(normals[i]);
                }
                
                if let Some(colors) = &self.colors {
                    downsampled_colors.as_mut().unwrap().push(colors[i]);
                }
            }
        }
        
        self.points = downsampled_points;
        self.normals = downsampled_normals;
        self.colors = downsampled_colors;
    }
}

impl Default for PointCloud {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_point_cloud_creation() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let cloud = PointCloud::from_points(points);
        assert_eq!(cloud.len(), 3);
        assert_eq!(cloud.points().len(), 3);
        assert!(cloud.normals().is_none());
        assert!(cloud.colors().is_none());
    }

    #[test]
    fn test_point_cloud_bounding_box() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 3.0),
            Point::new(-1.0, -2.0, -3.0),
        ];
        
        let cloud = PointCloud::from_points(points);
        let (min, max) = cloud.bounding_box();
        
        assert!((min.x - (-1.0)).abs() < 1e-10);
        assert!((min.y - (-2.0)).abs() < 1e-10);
        assert!((min.z - (-3.0)).abs() < 1e-10);
        assert!((max.x - 1.0).abs() < 1e-10);
        assert!((max.y - 2.0).abs() < 1e-10);
        assert!((max.z - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_cloud_centroid() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(2.0, 2.0, 2.0),
            Point::new(4.0, 4.0, 4.0),
        ];
        
        let cloud = PointCloud::from_points(points);
        let centroid = cloud.centroid();
        
        assert!((centroid.x - 2.0).abs() < 1e-10);
        assert!((centroid.y - 2.0).abs() < 1e-10);
        assert!((centroid.z - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_cloud_filter() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
        ];
        
        let mut cloud = PointCloud::from_points(points);
        cloud.filter(|p| p.x < 2.0);
        
        assert_eq!(cloud.len(), 2);
        assert_eq!(cloud.points()[0], Point::new(0.0, 0.0, 0.0));
        assert_eq!(cloud.points()[1], Point::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_point_cloud_downsample() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
            Point::new(4.0, 4.0, 4.0),
        ];
        
        let mut cloud = PointCloud::from_points(points);
        cloud.downsample(2);
        
        assert_eq!(cloud.len(), 3); // Should keep points at indices 0, 2, 4
        assert_eq!(cloud.points()[0], Point::new(0.0, 0.0, 0.0));
        assert_eq!(cloud.points()[1], Point::new(2.0, 2.0, 2.0));
        assert_eq!(cloud.points()[2], Point::new(4.0, 4.0, 4.0));
    }
}
