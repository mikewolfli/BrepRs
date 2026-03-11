use crate::geometry::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: Point::origin(),
            max: Point::origin(),
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        (self.min.x <= point.x && point.x <= self.max.x)
            && (self.min.y <= point.y && point.y <= self.max.y)
            && (self.min.z <= point.z && point.z <= self.max.z)
    }

    pub fn center(&self) -> Point {
        Point::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0
        )
    }
}
