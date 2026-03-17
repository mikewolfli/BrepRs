use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::modeling::primitives::make_box;
use crate::topology::TopoDsSolid;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
    pub solid: Option<TopoDsSolid>,
}

impl Cube {
    pub fn new(size: f64, center: Option<Point>) -> Self {
        let center = center.unwrap_or(Point::origin());
        let solid = Some(make_box(size, size, size, Some(center)));
        
        Self {
            center,
            size,
            solid,
        }
    }

    pub fn from_solid(solid: TopoDsSolid, center: Point, size: f64) -> Self {
        Self {
            center,
            size,
            solid: Some(solid),
        }
    }

    pub fn solid(&self) -> Option<&TopoDsSolid> {
        self.solid.as_ref()
    }

    pub fn to_solid(&self) -> TopoDsSolid {
        self.solid.clone().unwrap_or_else(|| make_box(self.size, self.size, self.size, Some(self.center)))
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
        self.solid = Some(make_box(size, size, size, Some(self.center)));
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn set_center(&mut self, center: Point) {
        self.center = center;
        self.solid = Some(make_box(self.size, self.size, self.size, Some(center)));
    }

    pub fn volume(&self) -> f64 {
        self.size * self.size * self.size
    }

    pub fn surface_area(&self) -> f64 {
        6.0 * self.size * self.size
    }

    pub fn edge_length(&self) -> f64 {
        self.size
    }

    pub fn vertices(&self) -> Vec<Point> {
        let half_size = self.size / 2.0;
        let center = &self.center;
        
        vec![
            Point::new(center.x - half_size, center.y - half_size, center.z - half_size),
            Point::new(center.x + half_size, center.y - half_size, center.z - half_size),
            Point::new(center.x + half_size, center.y + half_size, center.z - half_size),
            Point::new(center.x - half_size, center.y + half_size, center.z - half_size),
            Point::new(center.x - half_size, center.y - half_size, center.z + half_size),
            Point::new(center.x + half_size, center.y - half_size, center.z + half_size),
            Point::new(center.x + half_size, center.y + half_size, center.z + half_size),
            Point::new(center.x - half_size, center.y + half_size, center.z + half_size),
        ]
    }

    pub fn scale(&mut self, factor: f64) {
        self.set_size(self.size * factor);
    }

    pub fn translated(&self, dx: f64, dy: f64, dz: f64) -> Self {
        let new_center = Point::new(
            self.center.x + dx,
            self.center.y + dy,
            self.center.z + dz
        );
        Self::new(self.size, Some(new_center))
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        let new_center = Point::new(
            self.center.x + dx,
            self.center.y + dy,
            self.center.z + dz
        );
        self.set_center(new_center);
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(1.0, Some(Point::origin()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_creation() {
        let cube = Cube::new(2.0, Some(Point::origin()));
        
        assert_eq!(cube.size(), 2.0);
        assert_eq!(cube.center(), &Point::origin());
        assert!(cube.solid().is_some());
        assert_eq!(cube.volume(), 8.0);
        assert_eq!(cube.surface_area(), 24.0);
        assert_eq!(cube.edge_length(), 2.0);
    }

    #[test]
    fn test_cube_vertices() {
        let cube = Cube::new(2.0, Some(Point::origin()));
        let vertices = cube.vertices();
        
        assert_eq!(vertices.len(), 8);
        assert!(vertices.contains(&Point::new(-1.0, -1.0, -1.0)));
        assert!(vertices.contains(&Point::new(1.0, -1.0, -1.0)));
        assert!(vertices.contains(&Point::new(1.0, 1.0, -1.0)));
        assert!(vertices.contains(&Point::new(-1.0, 1.0, -1.0)));
        assert!(vertices.contains(&Point::new(-1.0, -1.0, 1.0)));
        assert!(vertices.contains(&Point::new(1.0, -1.0, 1.0)));
        assert!(vertices.contains(&Point::new(1.0, 1.0, 1.0)));
        assert!(vertices.contains(&Point::new(-1.0, 1.0, 1.0)));
    }

    #[test]
    fn test_cube_scale() {
        let mut cube = Cube::new(2.0, Some(Point::origin()));
        cube.scale(1.5);
        
        assert_eq!(cube.size(), 3.0);
        assert_eq!(cube.volume(), 27.0);
        assert_eq!(cube.surface_area(), 54.0);
    }

    #[test]
    fn test_cube_translate() {
        let mut cube = Cube::new(2.0, Some(Point::origin()));
        cube.translate(1.0, 2.0, 3.0);
        
        assert_eq!(cube.center().x, 1.0);
        assert_eq!(cube.center().y, 2.0);
        assert_eq!(cube.center().z, 3.0);
    }
}
