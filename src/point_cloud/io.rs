//! Point cloud I/O module
//! 
//! This module provides functionality for loading and saving point clouds
//! in various file formats.

use super::PointCloud;
use crate::geometry::{Point, Vector};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Point cloud I/O
pub struct PointCloudIO;

impl PointCloudIO {
    /// Load a point cloud from a PCD file
    pub fn load_pcd(path: &str) -> Result<PointCloud, String> {
        let file = File::open(path).map_err(|e| crate::foundation::exception::Failure::io_error(
            format!("Failed to open file: {}", e),
            Some(format!("load_pcd: path={:?}", path)),
            Some(Box::new(e)),
        ))?;
        let reader = BufReader::new(file);
        
        let mut cloud = PointCloud::new();
        let mut in_data = false;
        let mut has_normals = false;
        let mut has_colors = false;
        
        for line in reader.lines() {
            let line = line.map_err(|e| crate::foundation::exception::Failure::io_error(
                format!("Failed to read line: {}", e),
                Some(format!("load_pcd: path={:?}", path)),
                Some(Box::new(e)),
            ))?;
            let line = line.trim();
            
            if line.starts_with("DATA") {
                in_data = true;
                continue;
            }
            
            if in_data {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let x = parts[0].parse::<f64>().map_err(|e| format!("Failed to parse x coordinate: {}", e))?;
                    let y = parts[1].parse::<f64>().map_err(|e| format!("Failed to parse y coordinate: {}", e))?;
                    let z = parts[2].parse::<f64>().map_err(|e| format!("Failed to parse z coordinate: {}", e))?;
                    
                    let point = Point::new(x, y, z);
                    
                    if parts.len() >= 6 && has_normals {
                        let nx = parts[3].parse::<f64>().map_err(|e| format!("Failed to parse nx coordinate: {}", e))?;
                        let ny = parts[4].parse::<f64>().map_err(|e| format!("Failed to parse ny coordinate: {}", e))?;
                        let nz = parts[5].parse::<f64>().map_err(|e| format!("Failed to parse nz coordinate: {}", e))?;
                        
                        let normal = Vector::new(nx, ny, nz);
                        
                        if parts.len() >= 9 && has_colors {
                            let r = parts[6].parse::<u8>().map_err(|e| format!("Failed to parse r color: {}", e))?;
                            let g = parts[7].parse::<u8>().map_err(|e| format!("Failed to parse g color: {}", e))?;
                            let b = parts[8].parse::<u8>().map_err(|e| format!("Failed to parse b color: {}", e))?;
                            
                            cloud.add_point_with_normal_and_color(point, normal, (r, g, b));
                        } else {
                            cloud.add_point_with_normal(point, normal);
                        }
                    } else if parts.len() >= 6 && has_colors {
                        let r = parts[3].parse::<u8>().map_err(|e| format!("Failed to parse r color: {}", e))?;
                        let g = parts[4].parse::<u8>().map_err(|e| format!("Failed to parse g color: {}", e))?;
                        let b = parts[5].parse::<u8>().map_err(|e| format!("Failed to parse b color: {}", e))?;
                        
                        cloud.add_point_with_color(point, (r, g, b));
                    } else {
                        cloud.add_point(point);
                    }
                }
            } else {
                if line.starts_with("FIELDS") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    has_normals = fields.contains(&"nx") && fields.contains(&"ny") && fields.contains(&"nz");
                    has_colors = fields.contains(&"r") && fields.contains(&"g") && fields.contains(&"b");
                }
            }
        }
        
        Ok(cloud)
    }

    /// Save a point cloud to a PCD file
    pub fn save_pcd(cloud: &PointCloud, path: &str) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| crate::foundation::exception::Failure::io_error(
            format!("Failed to create file: {}", e),
            Some(format!("save_pcd: path={:?}", path)),
            Some(Box::new(e)),
        ))?;
        
        // Write header
        writeln!(file, "# .PCD v0.7 - Point Cloud Data file format").map_err(|e| format!("Failed to write header: {}", e))?;
        writeln!(file, "VERSION 0.7").map_err(|e| format!("Failed to write version: {}", e))?;
        writeln!(file, "FIELDS x y z").map_err(|e| format!("Failed to write fields: {}", e))?;
        writeln!(file, "SIZE 4 4 4").map_err(|e| format!("Failed to write size: {}", e))?;
        writeln!(file, "TYPE F F F").map_err(|e| format!("Failed to write type: {}", e))?;
        writeln!(file, "COUNT 1 1 1").map_err(|e| format!("Failed to write count: {}", e))?;
        writeln!(file, "WIDTH {}", cloud.len()).map_err(|e| format!("Failed to write width: {}", e))?;
        writeln!(file, "HEIGHT 1").map_err(|e| format!("Failed to write height: {}", e))?;
        writeln!(file, "VIEWPOINT 0 0 0 1 0 0 0").map_err(|e| format!("Failed to write viewpoint: {}", e))?;
        writeln!(file, "POINTS {}", cloud.len()).map_err(|e| format!("Failed to write points: {}", e))?;
        writeln!(file, "DATA ascii").map_err(|e| format!("Failed to write data: {}", e))?;
        
        // Write points
        for (i, point) in cloud.points().iter().enumerate() {
            writeln!(file, "{} {} {}", point.x, point.y, point.z).map_err(|e| format!("Failed to write point: {}", e))?;
        }
        
        Ok(())
    }

    /// Load a point cloud from a PLY file
    pub fn load_ply(path: &str) -> Result<PointCloud, String> {
        let file = File::open(path).map_err(|e| crate::foundation::exception::Failure::io_error(
            format!("Failed to open file: {}", e),
            Some(format!("load_ply: path={:?}", path)),
            Some(Box::new(e)),
        ))?;
        let reader = BufReader::new(file);
        
        let mut cloud = PointCloud::new();
        let mut in_data = false;
        let mut vertex_count = 0;
        let mut has_normals = false;
        let mut has_colors = false;
        
        for line in reader.lines() {
            let line = line.map_err(|e| crate::foundation::exception::Failure::io_error(
                format!("Failed to read line: {}", e),
                Some(format!("load_ply: path={:?}", path)),
                Some(Box::new(e)),
            ))?;
            let line = line.trim();
            
            if line == "end_header" {
                in_data = true;
                continue;
            }
            
            if in_data {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let x = parts[0].parse::<f64>().map_err(|e| format!("Failed to parse x coordinate: {}", e))?;
                    let y = parts[1].parse::<f64>().map_err(|e| format!("Failed to parse y coordinate: {}", e))?;
                    let z = parts[2].parse::<f64>().map_err(|e| format!("Failed to parse z coordinate: {}", e))?;
                    
                    let point = Point::new(x, y, z);
                    
                    if parts.len() >= 6 && has_normals {
                        let nx = parts[3].parse::<f64>().map_err(|e| format!("Failed to parse nx coordinate: {}", e))?;
                        let ny = parts[4].parse::<f64>().map_err(|e| format!("Failed to parse ny coordinate: {}", e))?;
                        let nz = parts[5].parse::<f64>().map_err(|e| format!("Failed to parse nz coordinate: {}", e))?;
                        
                        let normal = Vector::new(nx, ny, nz);
                        
                        if parts.len() >= 9 && has_colors {
                            let r = parts[6].parse::<u8>().map_err(|e| format!("Failed to parse r color: {}", e))?;
                            let g = parts[7].parse::<u8>().map_err(|e| format!("Failed to parse g color: {}", e))?;
                            let b = parts[8].parse::<u8>().map_err(|e| format!("Failed to parse b color: {}", e))?;
                            
                            cloud.add_point_with_normal_and_color(point, normal, (r, g, b));
                        } else {
                            cloud.add_point_with_normal(point, normal);
                        }
                    } else if parts.len() >= 6 && has_colors {
                        let r = parts[3].parse::<u8>().map_err(|e| format!("Failed to parse r color: {}", e))?;
                        let g = parts[4].parse::<u8>().map_err(|e| format!("Failed to parse g color: {}", e))?;
                        let b = parts[5].parse::<u8>().map_err(|e| format!("Failed to parse b color: {}", e))?;
                        
                        cloud.add_point_with_color(point, (r, g, b));
                    } else {
                        cloud.add_point(point);
                    }
                }
            } else {
                if line.starts_with("element vertex") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    vertex_count = parts[2].parse::<usize>().map_err(|e| format!("Failed to parse vertex count: {}", e))?;
                } else if line.starts_with("property float nx") || line.starts_with("property float32 nx") {
                    has_normals = true;
                } else if line.starts_with("property uchar red") || line.starts_with("property uchar r") {
                    has_colors = true;
                }
            }
        }
        
        Ok(cloud)
    }

    /// Save a point cloud to a PLY file
    pub fn save_ply(cloud: &PointCloud, path: &str) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write header
        writeln!(file, "ply").map_err(|e| format!("Failed to write header: {}", e))?;
        writeln!(file, "format ascii 1.0").map_err(|e| format!("Failed to write format: {}", e))?;
        writeln!(file, "element vertex {}", cloud.len()).map_err(|e| format!("Failed to write vertex count: {}", e))?;
        writeln!(file, "property float x").map_err(|e| format!("Failed to write x property: {}", e))?;
        writeln!(file, "property float y").map_err(|e| format!("Failed to write y property: {}", e))?;
        writeln!(file, "property float z").map_err(|e| format!("Failed to write z property: {}", e))?;
        writeln!(file, "end_header").map_err(|e| format!("Failed to write end header: {}", e))?;
        
        // Write points
        for (i, point) in cloud.points().iter().enumerate() {
            writeln!(file, "{} {} {}", point.x, point.y, point.z).map_err(|e| format!("Failed to write point: {}", e))?;
        }
        
        Ok(())
    }

    /// Load a point cloud from an OBJ file
    pub fn load_obj(path: &str) -> Result<PointCloud, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        
        let mut cloud = PointCloud::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let line = line.trim();
            
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let x = parts[1].parse::<f64>().map_err(|e| format!("Failed to parse x coordinate: {}", e))?;
                    let y = parts[2].parse::<f64>().map_err(|e| format!("Failed to parse y coordinate: {}", e))?;
                    let z = parts[3].parse::<f64>().map_err(|e| format!("Failed to parse z coordinate: {}", e))?;
                    
                    let point = Point::new(x, y, z);
                    cloud.add_point(point);
                }
            }
        }
        
        Ok(cloud)
    }

    /// Save a point cloud to an OBJ file
    pub fn save_obj(cloud: &PointCloud, path: &str) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write points
        for (i, point) in cloud.points().iter().enumerate() {
            writeln!(file, "v {} {} {}", point.x, point.y, point.z).map_err(|e| format!("Failed to write point: {}", e))?;
        }
        
        Ok(())
    }

    /// Load a point cloud from a simple text file
    pub fn load_txt(path: &str) -> Result<PointCloud, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        
        let mut cloud = PointCloud::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let line = line.trim();
            
            if !line.is_empty() && !line.starts_with('#') {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let x = parts[0].parse::<f64>().map_err(|e| format!("Failed to parse x coordinate: {}", e))?;
                    let y = parts[1].parse::<f64>().map_err(|e| format!("Failed to parse y coordinate: {}", e))?;
                    let z = parts[2].parse::<f64>().map_err(|e| format!("Failed to parse z coordinate: {}", e))?;
                    
                    let point = Point::new(x, y, z);
                    cloud.add_point(point);
                }
            }
        }
        
        Ok(cloud)
    }

    /// Save a point cloud to a simple text file
    pub fn save_txt(cloud: &PointCloud, path: &str) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write header
        writeln!(file, "# Point cloud data").map_err(|e| format!("Failed to write header: {}", e))?;
        writeln!(file, "# {} points", cloud.len()).map_err(|e| format!("Failed to write point count: {}", e))?;
        
        // Write points
        for (i, point) in cloud.points().iter().enumerate() {
            writeln!(file, "{} {} {}", point.x, point.y, point.z).map_err(|e| format!("Failed to write point: {}", e))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_txt() {
        let mut cloud = PointCloud::new();
        
        cloud.add_point(Point::new(0.0, 0.0, 0.0));
        cloud.add_point(Point::new(1.0, 1.0, 1.0));
        cloud.add_point(Point::new(2.0, 2.0, 2.0));
        
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Save the point cloud
        PointCloudIO::save_txt(&cloud, file_path.to_str().unwrap()).unwrap();
        
        // Load the point cloud
        let loaded_cloud = PointCloudIO::load_txt(file_path.to_str().unwrap()).unwrap();
        
        // Verify the loaded cloud
        assert_eq!(loaded_cloud.len(), 3);
        assert_eq!(loaded_cloud.points()[0], Point::new(0.0, 0.0, 0.0));
        assert_eq!(loaded_cloud.points()[1], Point::new(1.0, 1.0, 1.0));
        assert_eq!(loaded_cloud.points()[2], Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_save_and_load_ply() {
        let mut cloud = PointCloud::new();
        
        cloud.add_point(Point::new(0.0, 0.0, 0.0));
        cloud.add_point(Point::new(1.0, 1.0, 1.0));
        cloud.add_point(Point::new(2.0, 2.0, 2.0));
        
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.ply");
        
        // Save the point cloud
        PointCloudIO::save_ply(&cloud, file_path.to_str().unwrap()).unwrap();
        
        // Load the point cloud
        let loaded_cloud = PointCloudIO::load_ply(file_path.to_str().unwrap()).unwrap();
        
        // Verify the loaded cloud
        assert_eq!(loaded_cloud.len(), 3);
    }

    #[test]
    fn test_save_and_load_obj() {
        let mut cloud = PointCloud::new();
        
        cloud.add_point(Point::new(0.0, 0.0, 0.0));
        cloud.add_point(Point::new(1.0, 1.0, 1.0));
        cloud.add_point(Point::new(2.0, 2.0, 2.0));
        
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.obj");
        
        // Save the point cloud
        PointCloudIO::save_obj(&cloud, file_path.to_str().unwrap()).unwrap();
        
        // Load the point cloud
        let loaded_cloud = PointCloudIO::load_obj(file_path.to_str().unwrap()).unwrap();
        
        // Verify the loaded cloud
        assert_eq!(loaded_cloud.len(), 3);
    }
}
