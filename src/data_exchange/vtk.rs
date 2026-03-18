//! VTK (Visualization Toolkit) support
//!
//! This module provides functionality to import and export VTK files.

use crate::geometry::Point;
use crate::mesh::mesh_data::Mesh3D;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

/// VTK file format writer
pub struct VtkWriter {
    file: File,
    binary: bool,
}

impl VtkWriter {
    /// Create a new VTK writer
    pub fn new(file_path: &str, binary: bool) -> Result<Self, std::io::Error> {
        let file = File::create(file_path)?;
        Ok(Self { file, binary })
    }

    /// Write mesh to VTK file
    pub fn write_mesh(&mut self, mesh: &Mesh3D) -> Result<(), std::io::Error> {
        // Write VTK header
        self.write_header()?;

        // Write points
        self.write_points(mesh)?;

        // Write cells
        self.write_cells(mesh)?;

        // Write cell data
        self.write_cell_data(mesh)?;

        // Write point data
        self.write_point_data(mesh)?;

        Ok(())
    }

    /// Write VTK header
    fn write_header(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.file, "# vtk DataFile Version 3.0")?;
        writeln!(self.file, "BrepRs Mesh")?;
        writeln!(
            self.file,
            "{}",
            if self.binary { "BINARY" } else { "ASCII" }
        )?;
        writeln!(self.file, "DATASET UNSTRUCTURED_GRID")?;
        Ok(())
    }

    /// Write points
    fn write_points(&mut self, mesh: &Mesh3D) -> Result<(), std::io::Error> {
        writeln!(self.file, "POINTS {} float", mesh.vertices.len())?;

        if self.binary {
            // Write binary header
            self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
        }

        for vertex in &mesh.vertices {
            if self.binary {
                // Write binary format
                let x = vertex.point.x as f32;
                let y = vertex.point.y as f32;
                let z = vertex.point.z as f32;
                self.file.write_all(&x.to_le_bytes())?;
                self.file.write_all(&y.to_le_bytes())?;
                self.file.write_all(&z.to_le_bytes())?;
            } else {
                writeln!(
                    self.file,
                    "{} {} {}",
                    vertex.point.x, vertex.point.y, vertex.point.z
                )?;
            }
        }

        Ok(())
    }

    /// Write cells
    fn write_cells(&mut self, mesh: &Mesh3D) -> Result<(), std::io::Error> {
        // Count total cells
        let total_cells =
            mesh.faces.len() + mesh.tetrahedrons.len() + mesh.hexahedrons.len() + mesh.prisms.len();

        // Count total cell points
        let mut total_cell_points = 0;
        for face in &mesh.faces {
            total_cell_points += face.vertices.len();
        }
        total_cell_points += mesh.tetrahedrons.len() * 4;
        total_cell_points += mesh.hexahedrons.len() * 8;
        total_cell_points += mesh.prisms.len() * 6;

        writeln!(
            self.file,
            "CELLS {} {}",
            total_cells,
            total_cells + total_cell_points
        )?;

        if self.binary {
            // Write binary header
            self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
        }

        // Write faces as polygons
        for face in &mesh.faces {
            if self.binary {
                // Write binary format
                let count = face.vertices.len() as i32;
                self.file.write_all(&count.to_le_bytes())?;
                for &v in &face.vertices {
                    let vertex_index = v as i32;
                    self.file.write_all(&vertex_index.to_le_bytes())?;
                }
            } else {
                write!(self.file, "{}", face.vertices.len())?;
                for &v in &face.vertices {
                    write!(self.file, " {}", v)?;
                }
                writeln!(self.file)?;
            }
        }

        // Write tetrahedrons
        for tetra in &mesh.tetrahedrons {
            if self.binary {
                // Write binary format
                self.file.write_all(&4i32.to_le_bytes())?;
                for &v in &tetra.vertices {
                    let vertex_index = v as i32;
                    self.file.write_all(&vertex_index.to_le_bytes())?;
                }
            } else {
                writeln!(
                    self.file,
                    "4 {} {} {} {}",
                    tetra.vertices[0], tetra.vertices[1], tetra.vertices[2], tetra.vertices[3]
                )?;
            }
        }

        // Write hexahedrons
        for hex in &mesh.hexahedrons {
            if self.binary {
                // Write binary format
                self.file.write_all(&8i32.to_le_bytes())?;
                for &v in &hex.vertices {
                    let vertex_index = v as i32;
                    self.file.write_all(&vertex_index.to_le_bytes())?;
                }
            } else {
                writeln!(
                    self.file,
                    "8 {} {} {} {} {} {} {} {}",
                    hex.vertices[0],
                    hex.vertices[1],
                    hex.vertices[2],
                    hex.vertices[3],
                    hex.vertices[4],
                    hex.vertices[5],
                    hex.vertices[6],
                    hex.vertices[7]
                )?;
            }
        }

        // Write prisms
        for prism in &mesh.prisms {
            if self.binary {
                // Write binary format
                self.file.write_all(&6i32.to_le_bytes())?;
                for &v in &prism.vertices {
                    let vertex_index = v as i32;
                    self.file.write_all(&vertex_index.to_le_bytes())?;
                }
            } else {
                writeln!(
                    self.file,
                    "6 {} {} {} {} {} {}",
                    prism.vertices[0],
                    prism.vertices[1],
                    prism.vertices[2],
                    prism.vertices[3],
                    prism.vertices[4],
                    prism.vertices[5]
                )?;
            }
        }

        // Write cell types
        writeln!(self.file, "CELL_TYPES {}", total_cells)?;

        if self.binary {
            // Write binary header
            self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
        }

        // Write face types (polygon = 7)
        for _ in &mesh.faces {
            if self.binary {
                self.file.write_all(&7i32.to_le_bytes())?;
            } else {
                writeln!(self.file, "7")?;
            }
        }

        // Write tetrahedron types (tetrahedron = 10)
        for _ in &mesh.tetrahedrons {
            if self.binary {
                self.file.write_all(&10i32.to_le_bytes())?;
            } else {
                writeln!(self.file, "10")?;
            }
        }

        // Write hexahedron types (hexahedron = 12)
        for _ in &mesh.hexahedrons {
            if self.binary {
                self.file.write_all(&12i32.to_le_bytes())?;
            } else {
                writeln!(self.file, "12")?;
            }
        }

        // Write prism types (wedge = 13)
        for _ in &mesh.prisms {
            if self.binary {
                self.file.write_all(&13i32.to_le_bytes())?;
            } else {
                writeln!(self.file, "13")?;
            }
        }

        Ok(())
    }

    /// Write cell data
    fn write_cell_data(&mut self, mesh: &Mesh3D) -> Result<(), std::io::Error> {
        // Count total cells
        let total_cells =
            mesh.faces.len() + mesh.tetrahedrons.len() + mesh.hexahedrons.len() + mesh.prisms.len();

        if total_cells > 0 {
            writeln!(self.file, "CELL_DATA {}", total_cells)?;

            // Write cell type data
            writeln!(self.file, "SCALARS cell_type int 1")?;
            writeln!(self.file, "LOOKUP_TABLE default")?;

            if self.binary {
                // Write binary header
                self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
            }

            // Face type = 0
            for _ in &mesh.faces {
                if self.binary {
                    self.file.write_all(&0i32.to_le_bytes())?;
                } else {
                    writeln!(self.file, "0")?;
                }
            }

            // Tetrahedron type = 1
            for _ in &mesh.tetrahedrons {
                if self.binary {
                    self.file.write_all(&1i32.to_le_bytes())?;
                } else {
                    writeln!(self.file, "1")?;
                }
            }

            // Hexahedron type = 2
            for _ in &mesh.hexahedrons {
                if self.binary {
                    self.file.write_all(&2i32.to_le_bytes())?;
                } else {
                    writeln!(self.file, "2")?;
                }
            }

            // Prism type = 3
            for _ in &mesh.prisms {
                if self.binary {
                    self.file.write_all(&3i32.to_le_bytes())?;
                } else {
                    writeln!(self.file, "3")?;
                }
            }
        }

        Ok(())
    }

    /// Write point data
    fn write_point_data(&mut self, mesh: &Mesh3D) -> Result<(), std::io::Error> {
        let point_count = mesh.vertices.len();

        if point_count > 0 {
            writeln!(self.file, "POINT_DATA {}", point_count)?;

            // Write normals if available
            if mesh.vertices.iter().any(|v| v.normal.is_some()) {
                writeln!(self.file, "VECTORS normals float")?;

                if self.binary {
                    // Write binary header
                    self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
                }

                for vertex in &mesh.vertices {
                    if let Some(normal) = vertex.normal {
                        if self.binary {
                            let nx = normal[0] as f32;
                            let ny = normal[1] as f32;
                            let nz = normal[2] as f32;
                            self.file.write_all(&nx.to_le_bytes())?;
                            self.file.write_all(&ny.to_le_bytes())?;
                            self.file.write_all(&nz.to_le_bytes())?;
                        } else {
                            writeln!(self.file, "{} {} {}", normal[0], normal[1], normal[2])?;
                        }
                    } else {
                        if self.binary {
                            self.file.write_all(&0f32.to_le_bytes())?;
                            self.file.write_all(&0f32.to_le_bytes())?;
                            self.file.write_all(&0f32.to_le_bytes())?;
                        } else {
                            writeln!(self.file, "0 0 0")?;
                        }
                    }
                }
            }

            // Write colors if available
            if mesh.vertices.iter().any(|v| v.color.is_some()) {
                writeln!(self.file, "SCALARS color float 4")?;
                writeln!(self.file, "LOOKUP_TABLE default")?;

                if self.binary {
                    // Write binary header
                    self.file.write_all(&[0, 0, 0, 0])?; // VTK binary header
                }

                for vertex in &mesh.vertices {
                    if let Some(color) = vertex.color {
                        if self.binary {
                            let r = color[0] as f32;
                            let g = color[1] as f32;
                            let b = color[2] as f32;
                            let a = color[3] as f32;
                            self.file.write_all(&r.to_le_bytes())?;
                            self.file.write_all(&g.to_le_bytes())?;
                            self.file.write_all(&b.to_le_bytes())?;
                            self.file.write_all(&a.to_le_bytes())?;
                        } else {
                            writeln!(
                                self.file,
                                "{} {} {} {}",
                                color[0], color[1], color[2], color[3]
                            )?;
                        }
                    } else {
                        if self.binary {
                            self.file.write_all(&1f32.to_le_bytes())?;
                            self.file.write_all(&1f32.to_le_bytes())?;
                            self.file.write_all(&1f32.to_le_bytes())?;
                            self.file.write_all(&1f32.to_le_bytes())?;
                        } else {
                            writeln!(self.file, "1 1 1 1")?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

/// VTK file format reader
pub struct VtkReader {
    reader: std::io::BufReader<std::fs::File>,
    binary: bool,
}

impl VtkReader {
    /// Create a new VTK reader
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        Ok(Self {
            reader,
            binary: false,
        })
    }

    /// Read mesh from VTK file
    pub fn read_mesh(&mut self) -> Result<Mesh3D, std::io::Error> {
        let mut mesh = Mesh3D::new();

        // Read header and determine format
        self.read_header()?;

        // Read dataset type
        let dataset_type = self.read_dataset_type()?;

        match dataset_type.as_str() {
            "UNSTRUCTURED_GRID" => {
                self.read_unstructured_grid(&mut mesh)?;
            }
            "POLYDATA" => {
                self.read_polydata(&mut mesh)?;
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unsupported dataset type: {}", dataset_type),
                ));
            }
        }

        Ok(mesh)
    }

    /// Read header
    fn read_header(&mut self) -> Result<(), std::io::Error> {
        let mut line = String::new();

        // Read first line
        self.reader.read_line(&mut line)?;
        if !line.trim().starts_with("# vtk DataFile Version") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not a valid VTK file",
            ));
        }

        // Read second line (title)
        line.clear();
        self.reader.read_line(&mut line)?;

        // Read third line (format)
        line.clear();
        self.reader.read_line(&mut line)?;
        let format = line.trim();
        self.binary = format == "BINARY";

        Ok(())
    }

    /// Read dataset type
    fn read_dataset_type(&mut self) -> Result<String, std::io::Error> {
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.starts_with("DATASET") {
                return Ok(trimmed.split_whitespace().nth(1).unwrap_or("").to_string());
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No dataset type found",
        ))
    }

    /// Read unstructured grid
    fn read_unstructured_grid(&mut self, mesh: &mut Mesh3D) -> Result<(), std::io::Error> {
        let mut line = String::new();

        // Read points
        while self.reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if trimmed.starts_with("POINTS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid points count")
                    })?;
                    self.read_points(mesh, count)?;
                }
            } else if trimmed.starts_with("CELLS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid cells count")
                    })?;
                    self.read_cells(mesh, count)?;
                }
            } else if trimmed.starts_with("CELL_TYPES") {
                // Skip cell types for now
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid cell types count",
                        )
                    })?;
                    self.skip_cell_types(count)?;
                }
            } else if trimmed.starts_with("POINT_DATA") || trimmed.starts_with("CELL_DATA") {
                // Skip data sections for now
                break;
            }
            line.clear();
        }

        Ok(())
    }

    /// Read polydata
    fn read_polydata(&mut self, mesh: &mut Mesh3D) -> Result<(), std::io::Error> {
        let mut line = String::new();

        // Read points
        while self.reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if trimmed.starts_with("POINTS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid points count")
                    })?;
                    self.read_points(mesh, count)?;
                }
            } else if trimmed.starts_with("POLYGONS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid polygons count",
                        )
                    })?;
                    self.read_polygons(mesh, count)?;
                }
            } else if trimmed.starts_with("POINT_DATA") || trimmed.starts_with("CELL_DATA") {
                // Skip data sections for now
                break;
            }
            line.clear();
        }

        Ok(())
    }

    /// Read points
    fn read_points(&mut self, mesh: &mut Mesh3D, count: usize) -> Result<(), std::io::Error> {
        if self.binary {
            // Skip binary header
            let mut header = [0u8; 4];
            self.reader.read_exact(&mut header)?;
        }

        for _ in 0..count {
            let (x, y, z) = if self.binary {
                let mut buffer = [0u8; 4];
                self.reader.read_exact(&mut buffer)?;
                let x = f32::from_le_bytes(buffer) as f64;

                self.reader.read_exact(&mut buffer)?;
                let y = f32::from_le_bytes(buffer) as f64;

                self.reader.read_exact(&mut buffer)?;
                let z = f32::from_le_bytes(buffer) as f64;

                (x, y, z)
            } else {
                let mut line = String::new();
                self.reader.read_line(&mut line)?;
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() < 3 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid point data",
                    ));
                }

                let x = parts[0].parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid point x coordinate",
                    )
                })?;

                let y = parts[1].parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid point y coordinate",
                    )
                })?;

                let z = parts[2].parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid point z coordinate",
                    )
                })?;

                (x, y, z)
            };

            mesh.add_vertex(Point::new(x, y, z));
        }

        Ok(())
    }

    /// Read cells
    fn read_cells(&mut self, mesh: &mut Mesh3D, count: usize) -> Result<(), std::io::Error> {
        if self.binary {
            // Skip binary header
            let mut header = [0u8; 4];
            self.reader.read_exact(&mut header)?;
        }

        for _ in 0..count {
            let vertices = if self.binary {
                let mut buffer = [0u8; 4];
                self.reader.read_exact(&mut buffer)?;
                let n = i32::from_le_bytes(buffer) as usize;

                let mut vertices = Vec::with_capacity(n);
                for _ in 0..n {
                    self.reader.read_exact(&mut buffer)?;
                    let index = i32::from_le_bytes(buffer) as usize;
                    vertices.push(index);
                }
                vertices
            } else {
                let mut line = String::new();
                self.reader.read_line(&mut line)?;
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid cell data",
                    ));
                }

                let n: usize = parts[0].parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid cell vertex count",
                    )
                })?;

                if parts.len() < n + 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Insufficient cell vertex indices",
                    ));
                }

                let mut vertices = Vec::with_capacity(n);
                for i in 1..=n {
                    let index: usize = parts[i].parse().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid cell vertex index",
                        )
                    })?;
                    vertices.push(index);
                }
                vertices
            };

            mesh.add_face(vertices);
        }

        Ok(())
    }

    /// Read polygons
    fn read_polygons(&mut self, mesh: &mut Mesh3D, count: usize) -> Result<(), std::io::Error> {
        if self.binary {
            // Skip binary header
            let mut header = [0u8; 4];
            self.reader.read_exact(&mut header)?;
        }

        for _ in 0..count {
            let vertices = if self.binary {
                let mut buffer = [0u8; 4];
                self.reader.read_exact(&mut buffer)?;
                let n = i32::from_le_bytes(buffer) as usize;

                let mut vertices = Vec::with_capacity(n);
                for _ in 0..n {
                    self.reader.read_exact(&mut buffer)?;
                    let index = i32::from_le_bytes(buffer) as usize;
                    vertices.push(index);
                }
                vertices
            } else {
                let mut line = String::new();
                self.reader.read_line(&mut line)?;
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid polygon data",
                    ));
                }

                let n: usize = parts[0].parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid polygon vertex count",
                    )
                })?;

                if parts.len() < n + 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Insufficient polygon vertex indices",
                    ));
                }

                let mut vertices = Vec::with_capacity(n);
                for i in 1..=n {
                    let index: usize = parts[i].parse().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid polygon vertex index",
                        )
                    })?;
                    vertices.push(index);
                }
                vertices
            };

            mesh.add_face(vertices);
        }

        Ok(())
    }

    /// Skip cell types
    fn skip_cell_types(&mut self, count: usize) -> Result<(), std::io::Error> {
        if self.binary {
            // Skip binary header
            let mut header = [0u8; 4];
            self.reader.read_exact(&mut header)?;

            // Skip cell types
            let mut buffer = [0u8; 4];
            for _ in 0..count {
                self.reader.read_exact(&mut buffer)?;
            }
        } else {
            // Skip ASCII cell types
            let mut line = String::new();
            for _ in 0..count {
                self.reader.read_line(&mut line)?;
                line.clear();
            }
        }

        Ok(())
    }
}
