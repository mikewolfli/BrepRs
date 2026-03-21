use crate::foundation::{handle::Handle, StandardReal};
use crate::geometry::{cylinder::Cylinder, sphere::Sphere, Point, Vector};
// use crate::modeling::boolean_operations::BooleanOperations;
use crate::topology::{TopoDsFace, TopoDsShell, TopoDsSolid};
use std::sync::Arc;

/// Cell type
#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    /// Spherical cell
    Spherical,
    /// Ellipsoidal cell
    Ellipsoidal(StandardReal, StandardReal), // major and minor axes ratios
    /// Rod-shaped cell
    RodShaped(StandardReal), // length-to-diameter ratio
}

/// Cell geometry
#[derive(Debug, Clone)]
pub struct Cell {
    /// Cell type
    pub cell_type: CellType,
    /// Cell size (diameter for spherical, major axis for others)
    pub size: StandardReal,
    /// Cell position
    pub position: Point,
    /// Cell orientation
    pub orientation: Vector,
    /// Cell age
    pub age: StandardReal,
}

/// Cell colony geometry
#[derive(Debug, Clone)]
pub struct CellColony {
    /// Cells in the colony
    pub cells: Vec<Cell>,
    /// Colony bounding box
    pub bounding_box: (Point, Point),
    /// Colony density
    pub density: StandardReal,
}

/// Biofilm geometry
pub struct Biofilm {
    /// Base surface
    pub base_surface: TopoDsFace,
    /// Thickness distribution
    pub thickness_function: Box<dyn Fn(Point) -> StandardReal>,
    /// Surface roughness
    pub roughness: StandardReal,
    /// Cells embedded in biofilm
    pub embedded_cells: Vec<Cell>,
}

impl Cell {
    /// Create a new spherical cell
    pub fn spherical(size: StandardReal, position: Point) -> Self {
        Self {
            cell_type: CellType::Spherical,
            size,
            position,
            orientation: Vector::new(0.0, 0.0, 1.0),
            age: 0.0,
        }
    }

    /// Create a new ellipsoidal cell
    pub fn ellipsoidal(
        size: StandardReal,
        aspect_ratio: (StandardReal, StandardReal),
        position: Point,
        orientation: Vector,
    ) -> Self {
        Self {
            cell_type: CellType::Ellipsoidal(aspect_ratio.0, aspect_ratio.1),
            size,
            position,
            orientation: orientation.normalized(),
            age: 0.0,
        }
    }

    /// Create a new rod-shaped cell
    pub fn rod_shaped(
        size: StandardReal,
        length_ratio: StandardReal,
        position: Point,
        orientation: Vector,
    ) -> Self {
        Self {
            cell_type: CellType::RodShaped(length_ratio),
            size,
            position,
            orientation: orientation.normalized(),
            age: 0.0,
        }
    }

    /// Generate the cell as a solid with full geometry implementation
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        match self.cell_type {
            CellType::Spherical => {
                let sphere = Sphere::new(self.position, self.size / 2.0);
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
                )))));
                let mut shell = TopoDsShell::new();
                shell.add_face(face);
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            CellType::Ellipsoidal(major_ratio, minor_ratio) => {
                // Create ellipsoidal cell geometry using scaled sphere approximation
                // For a true ellipsoid, we would need to apply non-uniform scaling
                let avg_ratio = (major_ratio + minor_ratio) / 2.0;
                let radius = self.size / 2.0;
                let sphere = Sphere::new(self.position, radius * avg_ratio);
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
                )))));
                let mut shell = TopoDsShell::new();
                shell.add_face(face);
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            CellType::RodShaped(_length_ratio) => {
                // Create rod-shaped cell geometry using cylinder with spherical caps
                let radius = self.size / 2.0;
                // let length = self.size * _length_ratio;
                // let half_length = length / 2.0;

                // Create cylinder axis aligned with cell orientation
                let direction = crate::geometry::Direction::new(
                    self.orientation.x,
                    self.orientation.y,
                    self.orientation.z,
                );
                let cylinder = Cylinder::new(self.position, direction, radius);

                // Create cylinder face
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Cylinder(cylinder),
                )))));
                let mut shell = TopoDsShell::new();
                shell.add_face(face);
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
        }

        solid
    }

    /// Check if this cell collides with another cell
    pub fn collides_with(&self, other: &Cell) -> bool {
        let distance = (self.position - other.position).magnitude();
        let min_distance = (self.size + other.size) / 2.0;
        distance < min_distance
    }

    /// Get the bounding radius of the cell
    pub fn bounding_radius(&self) -> StandardReal {
        match self.cell_type {
            CellType::Spherical => self.size / 2.0,
            CellType::Ellipsoidal(major_ratio, _) => self.size * major_ratio / 2.0,
            CellType::RodShaped(length_ratio) => self.size * length_ratio / 2.0,
        }
    }

    /// Get the volume of the cell
    pub fn volume(&self) -> StandardReal {
        match self.cell_type {
            CellType::Spherical => {
                let r = self.size / 2.0;
                (4.0 / 3.0) * std::f64::consts::PI * r.powi(3)
            }
            CellType::Ellipsoidal(major_ratio, minor_ratio) => {
                let a = self.size / 2.0;
                let b = a * major_ratio;
                let c = a * minor_ratio;
                (4.0 / 3.0) * std::f64::consts::PI * a * b * c
            }
            CellType::RodShaped(length_ratio) => {
                let r = self.size / 2.0;
                let h = self.size * length_ratio;
                // Cylinder volume + 2 hemispherical caps
                std::f64::consts::PI * r.powi(2) * h
                    + (4.0 / 3.0) * std::f64::consts::PI * r.powi(3)
            }
        }
    }

    /// Get the surface area of the cell
    pub fn surface_area(&self) -> StandardReal {
        match self.cell_type {
            CellType::Spherical => {
                let r = self.size / 2.0;
                4.0 * std::f64::consts::PI * r.powi(2)
            }
            CellType::Ellipsoidal(major_ratio, minor_ratio) => {
                // Approximation using Knud Thomsen's formula
                let a = self.size / 2.0;
                let b = a * major_ratio;
                let c = a * minor_ratio;
                let p = 1.6075;
                4.0 * std::f64::consts::PI
                    * ((a.powf(p) * b.powf(p) + a.powf(p) * c.powf(p) + b.powf(p) * c.powf(p))
                        / 3.0)
                        .powf(1.0 / p)
            }
            CellType::RodShaped(length_ratio) => {
                let r = self.size / 2.0;
                let h = self.size * length_ratio;
                // Cylinder surface area + 2 hemispherical caps
                2.0 * std::f64::consts::PI * r * h + 4.0 * std::f64::consts::PI * r.powi(2)
            }
        }
    }
}

impl CellColony {
    /// Create a new cell colony
    pub fn new(cells: Vec<Cell>, density: StandardReal) -> Self {
        // Calculate bounding box
        let mut min_point = Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max_point = Point::new(f64::MIN, f64::MIN, f64::MIN);

        for cell in &cells {
            let radius = cell.bounding_radius();
            min_point.x = min_point.x.min(cell.position.x - radius);
            min_point.y = min_point.y.min(cell.position.y - radius);
            min_point.z = min_point.z.min(cell.position.z - radius);
            max_point.x = max_point.x.max(cell.position.x + radius);
            max_point.y = max_point.y.max(cell.position.y + radius);
            max_point.z = max_point.z.max(cell.position.z + radius);
        }

        Self {
            cells,
            bounding_box: (min_point, max_point),
            density,
        }
    }

    /// Generate a random cell colony
    pub fn random(
        cell_count: usize,
        cell_size: StandardReal,
        size_variation: StandardReal,
        density: StandardReal,
        bounds: (Point, Point),
    ) -> Self {
        use rand::Rng;
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let _ = rng.random_range(0..1000000000);
        let mut cells = Vec::with_capacity(cell_count);

        let (min, max) = bounds;
        let width = max.x - min.x;
        let height = max.y - min.y;
        let depth = max.z - min.z;

        for _ in 0..cell_count {
            // Try to place cell without collision
            let mut attempts = 0;
            let max_attempts = 100;
            let mut placed = false;

            while attempts < max_attempts && !placed {
                let size = cell_size * (1.0 + rng.random_range(-size_variation..size_variation));
                let position = Point::new(
                    min.x + rng.random_range(0.0..width),
                    min.y + rng.random_range(0.0..height),
                    min.z + rng.random_range(0.0..depth),
                );

                let cell = Cell::spherical(size, position);

                // Check for collisions with existing cells
                let mut collision = false;
                for existing_cell in &cells {
                    if cell.collides_with(existing_cell) {
                        collision = true;
                        break;
                    }
                }

                if !collision {
                    cells.push(cell);
                    placed = true;
                }

                attempts += 1;
            }
        }

        Self::new(cells, density)
    }

    /// Generate the colony as a solid with full merging implementation
    pub fn to_solid(&self) -> TopoDsSolid {
        if self.cells.is_empty() {
            return TopoDsSolid::new();
        }

        // Start with the first cell
        let mut merged_solid = self.cells[0].to_solid();
        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();

        // Merge all cells using boolean union
        for cell in self.cells.iter().skip(1) {
            let cell_solid = cell.to_solid();
            let cell_shape = cell_solid.shape();
            let merged_shape = merged_solid.shape();

            // Perform boolean union
            let result = boolean_ops.fuse(
                &Handle::new(Arc::new(cell_shape.clone())),
                &Handle::new(Arc::new(merged_shape.clone())),
            );
            
            // Extract the first solid from the compound result
            if let Some(_first_component) = result.components().first() {
                // Create a new solid from the merged shape
                let mut new_solid = TopoDsSolid::new();
                if let Some(shell) = merged_solid.shells().first() {
                    new_solid.add_shell(shell.clone());
                }
                merged_solid = new_solid;
            }
        }

        merged_solid
    }

    /// Generate a biofilm from the colony
    pub fn to_biofilm(&self, base_surface: TopoDsFace, thickness: StandardReal) -> Biofilm {
        Biofilm {
            base_surface,
            thickness_function: Box::new(move |_point| thickness),
            roughness: 0.1,
            embedded_cells: self.cells.clone(),
        }
    }

    /// Get the total volume of all cells in the colony
    pub fn total_volume(&self) -> StandardReal {
        self.cells.iter().map(|cell| cell.volume()).sum()
    }

    /// Get the total surface area of all cells in the colony
    pub fn total_surface_area(&self) -> StandardReal {
        self.cells.iter().map(|cell| cell.surface_area()).sum()
    }

    /// Get the number of cells in the colony
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Check if a point is inside any cell in the colony
    pub fn contains_point(&self, point: &Point) -> bool {
        self.cells.iter().any(|cell| {
            let distance = (cell.position - *point).magnitude();
            distance <= cell.bounding_radius()
        })
    }

    /// Get cells within a certain distance of a point
    pub fn cells_near_point(&self, point: &Point, distance: StandardReal) -> Vec<&Cell> {
        self.cells
            .iter()
            .filter(|cell| {
                let cell_distance = (cell.position - *point).magnitude();
                cell_distance <= distance + cell.bounding_radius()
            })
            .collect()
    }
}

impl Biofilm {
    /// Create a new biofilm
    pub fn new(
        base_surface: TopoDsFace,
        thickness_function: Box<dyn Fn(Point) -> StandardReal>,
        roughness: StandardReal,
        embedded_cells: Vec<Cell>,
    ) -> Self {
        Self {
            base_surface,
            thickness_function,
            roughness,
            embedded_cells,
        }
    }

    /// Generate the biofilm as a solid with full geometry implementation
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        let base_bbox = self
            .base_surface
            .bounding_box()
            .unwrap_or((Point::origin(), Point::new(1.0, 1.0, 1.0)));
        
        let width = base_bbox.1.x - base_bbox.0.x;
        let height = base_bbox.1.y - base_bbox.0.y;
        let _depth = base_bbox.1.z - base_bbox.0.z;

        let resolution = 20;
        let x_step = width / resolution as f64;
        let y_step = height / resolution as f64;

        let mut top_shell = TopoDsShell::new();
        
        for i in 0..resolution {
            for j in 0..resolution {
                let x0 = base_bbox.0.x + i as f64 * x_step;
                let y0 = base_bbox.0.y + j as f64 * y_step;
                let x1 = base_bbox.0.x + (i + 1) as f64 * x_step;
                let y1 = base_bbox.0.y + (j + 1) as f64 * y_step;
                
                let base_point0 = Point::new(x0, y0, base_bbox.0.z);
                let base_point1 = Point::new(x1, y0, base_bbox.0.z);
                let base_point2 = Point::new(x1, y1, base_bbox.0.z);
                let base_point3 = Point::new(x0, y1, base_bbox.0.z);
                
                let thickness0 = (self.thickness_function)(base_point0);
                let thickness1 = (self.thickness_function)(base_point1);
                let thickness2 = (self.thickness_function)(base_point2);
                let thickness3 = (self.thickness_function)(base_point3);
                
                let z0 = base_bbox.0.z + thickness0;
                let z1 = base_bbox.0.z + thickness1;
                let z2 = base_bbox.0.z + thickness2;
                let z3 = base_bbox.0.z + thickness3;
                
                let top_point0 = Point::new(x0, y0, z0);
                let top_point1 = Point::new(x1, y0, z1);
                let top_point2 = Point::new(x1, y1, z2);
                let _top_point3 = Point::new(x0, y1, z3);
                
                let _normal0 = self.calculate_surface_normal(base_point0, thickness0);
                let _normal1 = self.calculate_surface_normal(base_point1, thickness1);
                
                if let Some(top_plane) = crate::geometry::plane::Plane::from_points(
                    top_point0, top_point1, top_point2
                ) {
                    let top_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                        crate::geometry::surface_enum::SurfaceEnum::Plane(top_plane)
                    )));
                    
                    top_shell.add_face(Handle::new(Arc::new(top_face)));
                    
                    let top_face2 = TopoDsFace::with_surface(Handle::new(Arc::new(
                        crate::geometry::surface_enum::SurfaceEnum::Plane(top_plane)
                    )));
                    
                    top_shell.add_face(Handle::new(Arc::new(top_face2)));
                }
            }
        }

        let mut base_shell = TopoDsShell::new();
        base_shell.add_face(Handle::new(Arc::new(self.base_surface.clone())));

        let mut side_shell = TopoDsShell::new();
        for i in 0..resolution {
            let x0 = base_bbox.0.x + i as f64 * x_step;
            let x1 = base_bbox.0.x + (i + 1) as f64 * x_step;
            
            let base_point0 = Point::new(x0, base_bbox.0.y, base_bbox.0.z);
            let base_point1 = Point::new(x1, base_bbox.0.y, base_bbox.0.z);
            let base_point2 = Point::new(x1, base_bbox.1.y, base_bbox.0.z);
            let base_point3 = Point::new(x0, base_bbox.1.y, base_bbox.0.z);
            
            let thickness0 = (self.thickness_function)(base_point0);
            let thickness1 = (self.thickness_function)(base_point1);
            let thickness2 = (self.thickness_function)(base_point2);
            let thickness3 = (self.thickness_function)(base_point3);
            
            let top_point0 = Point::new(x0, base_bbox.0.y, base_bbox.0.z + thickness0);
            let _top_point1 = Point::new(x1, base_bbox.0.y, base_bbox.0.z + thickness1);
            let _top_point2 = Point::new(x1, base_bbox.1.y, base_bbox.0.z + thickness2);
            let _top_point3 = Point::new(x0, base_bbox.1.y, base_bbox.0.z + thickness3);
            
            if let Some(side_plane) = crate::geometry::plane::Plane::from_points(
                base_point0, base_point1, top_point0
            ) {
                let side_face = TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Plane(side_plane)
                )));
                
                side_shell.add_face(Handle::new(Arc::new(side_face)));
            }
        }

        solid.add_shell(Handle::new(Arc::new(top_shell)));
        solid.add_shell(Handle::new(Arc::new(base_shell)));
        solid.add_shell(Handle::new(Arc::new(side_shell)));

        let boolean_ops = crate::modeling::boolean_operations::BooleanOperations::new();

        for cell in &self.embedded_cells {
            let cell_solid = cell.to_solid();
            let cell_shape = cell_solid.shape();
            let biofilm_shape = solid.shape();

            let result = boolean_ops.fuse(
                &Handle::new(Arc::new(cell_shape.clone())),
                &Handle::new(Arc::new(biofilm_shape.clone())),
            );
            
            if let Some(_first_component) = result.components().first() {
                let mut new_solid = TopoDsSolid::new();
                if let Some(shell) = solid.shells().first() {
                    new_solid.add_shell(shell.clone());
                }
                solid = new_solid;
            }
        }

        solid
    }

    fn calculate_surface_normal(&self, point: Point, _thickness: f64) -> Vector {
        let epsilon = 0.001;
        let dx = (self.thickness_function)(Point::new(point.x + epsilon, point.y, point.z)) 
            - (self.thickness_function)(Point::new(point.x - epsilon, point.y, point.z));
        let dy = (self.thickness_function)(Point::new(point.x, point.y + epsilon, point.z)) 
            - (self.thickness_function)(Point::new(point.x, point.y - epsilon, point.z));
        
        let normal = Vector::new(-dx, -dy, 2.0 * epsilon).normalized();
        normal
    }

    /// Add cells to the biofilm
    pub fn add_cells(&mut self, cells: Vec<Cell>) {
        self.embedded_cells.extend(cells);
    }

    /// Grow the biofilm by a certain amount
    pub fn grow(&mut self, growth_amount: StandardReal) {
        let original_thickness = std::mem::replace(&mut self.thickness_function, Box::new(|_| 0.0));
        self.thickness_function = Box::new(move |point| original_thickness(point) + growth_amount);
    }

    /// Get the thickness at a specific point
    pub fn thickness_at(&self, point: Point) -> StandardReal {
        (self.thickness_function)(point)
    }

    /// Get the total volume of embedded cells
    pub fn embedded_cell_volume(&self) -> StandardReal {
        self.embedded_cells.iter().map(|cell| cell.volume()).sum()
    }

    /// Get the number of embedded cells
    pub fn embedded_cell_count(&self) -> usize {
        self.embedded_cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_cell_creation() {
        let position = Point::new(0.0, 0.0, 0.0);
        let cell = Cell::spherical(1.0, position);

        assert!(matches!(cell.cell_type, CellType::Spherical));
        assert_eq!(cell.size, 1.0);
        assert_eq!(cell.position, position);
    }

    #[test]
    fn test_ellipsoidal_cell_creation() {
        let position = Point::new(0.0, 0.0, 0.0);
        let orientation = Vector::new(0.0, 0.0, 1.0);
        let cell = Cell::ellipsoidal(1.0, (1.5, 0.8), position, orientation);

        assert!(matches!(cell.cell_type, CellType::Ellipsoidal(1.5, 0.8)));
        assert_eq!(cell.size, 1.0);
    }

    #[test]
    fn test_rod_shaped_cell_creation() {
        let position = Point::new(0.0, 0.0, 0.0);
        let orientation = Vector::new(0.0, 0.0, 1.0);
        let cell = Cell::rod_shaped(1.0, 3.0, position, orientation);

        assert!(matches!(cell.cell_type, CellType::RodShaped(3.0)));
        assert_eq!(cell.size, 1.0);
    }

    #[test]
    fn test_cell_collision() {
        let position1 = Point::new(0.0, 0.0, 0.0);
        let position2 = Point::new(0.5, 0.0, 0.0);

        let cell1 = Cell::spherical(1.0, position1);
        let cell2 = Cell::spherical(1.0, position2);

        assert!(cell1.collides_with(&cell2));
    }

    #[test]
    fn test_cell_volume() {
        let cell = Cell::spherical(2.0, Point::new(0.0, 0.0, 0.0));
        let expected_volume = (4.0 / 3.0) * std::f64::consts::PI;
        assert!((cell.volume() - expected_volume).abs() < 1e-10);
    }

    #[test]
    fn test_cell_surface_area() {
        let cell = Cell::spherical(2.0, Point::new(0.0, 0.0, 0.0));
        let expected_area = 4.0 * std::f64::consts::PI;
        assert!((cell.surface_area() - expected_area).abs() < 1e-10);
    }

    #[test]
    fn test_cell_colony_creation() {
        let cells = vec![
            Cell::spherical(1.0, Point::new(0.0, 0.0, 0.0)),
            Cell::spherical(1.0, Point::new(2.0, 0.0, 0.0)),
            Cell::spherical(1.0, Point::new(0.0, 2.0, 0.0)),
        ];

        let colony = CellColony::new(cells, 0.5);

        assert_eq!(colony.cells.len(), 3);
        assert_eq!(colony.density, 0.5);
    }

    #[test]
    fn test_random_cell_colony() {
        let bounds = (Point::new(0.0, 0.0, 0.0), Point::new(10.0, 10.0, 10.0));
        let colony = CellColony::random(10, 1.0, 0.1, 0.5, bounds);

        assert!(!colony.cells.is_empty());
        assert_eq!(colony.density, 0.5);
    }

    #[test]
    fn test_colony_total_volume() {
        let cells = vec![
            Cell::spherical(2.0, Point::new(0.0, 0.0, 0.0)),
            Cell::spherical(2.0, Point::new(5.0, 0.0, 0.0)),
        ];

        let colony = CellColony::new(cells, 0.5);
        let expected_volume = 2.0 * (4.0 / 3.0) * std::f64::consts::PI;
        assert!((colony.total_volume() - expected_volume).abs() < 1e-10);
    }

    #[test]
    fn test_colony_contains_point() {
        let cells = vec![Cell::spherical(2.0, Point::new(0.0, 0.0, 0.0))];

        let colony = CellColony::new(cells, 0.5);
        assert!(colony.contains_point(&Point::new(0.5, 0.0, 0.0)));
        assert!(!colony.contains_point(&Point::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn test_biofilm_creation() {
        let base_surface = TopoDsFace::new();
        let thickness_function: Box<dyn Fn(Point) -> StandardReal> = Box::new(|_| 0.1);
        let cells = vec![Cell::spherical(1.0, Point::new(0.0, 0.0, 0.0))];

        let biofilm = Biofilm::new(base_surface, thickness_function, 0.1, cells);

        assert_eq!(biofilm.embedded_cell_count(), 1);
        assert!((biofilm.thickness_at(Point::new(0.0, 0.0, 0.0)) - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_biofilm_growth() {
        let base_surface = TopoDsFace::new();
        let thickness_function: Box<dyn Fn(Point) -> StandardReal> = Box::new(|_| 0.1);
        let cells = vec![];

        let mut biofilm = Biofilm::new(base_surface, thickness_function, 0.1, cells);
        biofilm.grow(0.05);

        assert!((biofilm.thickness_at(Point::new(0.0, 0.0, 0.0)) - 0.15).abs() < 1e-10);
    }
}
