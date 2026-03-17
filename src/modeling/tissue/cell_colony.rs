use crate::foundation::{handle::Handle, StandardReal};
use crate::geometry::{sphere::Sphere, Point, Vector};
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

    /// Generate the cell as a solid
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
            CellType::Ellipsoidal(major, minor) => {
                // TODO: Implement ellipsoidal cell geometry
                let sphere = Sphere::new(self.position, self.size / 2.0);
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
                )))));
                let mut shell = TopoDsShell::new();
                shell.add_face(face);
                solid.add_shell(Handle::new(Arc::new(shell)));
            }
            CellType::RodShaped(length_ratio) => {
                // TODO: Implement rod-shaped cell geometry
                let sphere = Sphere::new(self.position, self.size / 2.0);
                let face = Handle::new(Arc::new(TopoDsFace::with_surface(Handle::new(Arc::new(
                    crate::geometry::surface_enum::SurfaceEnum::Sphere(sphere),
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
}

impl CellColony {
    /// Create a new cell colony
    pub fn new(cells: Vec<Cell>, density: StandardReal) -> Self {
        // Calculate bounding box
        let mut min_point = Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max_point = Point::new(f64::MIN, f64::MIN, f64::MIN);

        for cell in &cells {
            let radius = cell.size / 2.0;
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
        use rand::RngExt;
        let mut rng = rand::rng();
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

    /// Generate the colony as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        for cell in &self.cells {
            let cell_solid = cell.to_solid();
            // TODO: Merge cells into a single solid
            // For now, just add each cell as a separate shell
            for shell in cell_solid.shells() {
                solid.add_shell(shell.clone());
            }
        }

        solid
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

    /// Generate the biofilm as a solid
    pub fn to_solid(&self) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();

        // TODO: Implement biofilm geometry generation
        // For now, return a simple solid
        let mut shell = TopoDsShell::new();
        shell.add_face(Handle::new(Arc::new(self.base_surface.clone())));
        solid.add_shell(Handle::new(Arc::new(shell)));

        solid
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
    fn test_cell_collision() {
        let position1 = Point::new(0.0, 0.0, 0.0);
        let position2 = Point::new(0.5, 0.0, 0.0);

        let cell1 = Cell::spherical(1.0, position1);
        let cell2 = Cell::spherical(1.0, position2);

        assert!(cell1.collides_with(&cell2));
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
}
