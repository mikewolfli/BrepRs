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
            min: Point::new(f64::MAX, f64::MAX, f64::MAX),
            max: Point::new(f64::MIN, f64::MIN, f64::MIN),
        }
    }

    pub fn from_point(point: Point) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn from_points(points: &[Point]) -> Self {
        if points.is_empty() {
            return Self::empty();
        }

        let mut min = points[0];
        let mut max = points[0];

        for point in points.iter().skip(1) {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
        }

        Self { min, max }
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
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn size(&self) -> (f64, f64, f64) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn size_x(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn size_y(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn size_z(&self) -> f64 {
        self.max.z - self.min.z
    }

    pub fn volume(&self) -> f64 {
        let (sx, sy, sz) = self.size();
        sx * sy * sz
    }

    pub fn diagonal(&self) -> f64 {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty()
    }

    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }

        BoundingBox {
            min: Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn merge_point(&mut self, point: &Point) {
        if self.is_empty() {
            self.min = *point;
            self.max = *point;
            return;
        }

        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn contains_box(&self, other: &BoundingBox) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.min.x <= other.min.x
            && self.max.x >= other.max.x
            && self.min.y <= other.min.y
            && self.max.y >= other.max.y
            && self.min.z <= other.min.z
            && self.max.z >= other.max.z
    }

    pub fn expand(&mut self, delta: f64) {
        if self.is_empty() {
            return;
        }

        self.min.x -= delta;
        self.min.y -= delta;
        self.min.z -= delta;
        self.max.x += delta;
        self.max.y += delta;
        self.max.z += delta;
    }

    pub fn expanded(&self, delta: f64) -> BoundingBox {
        if self.is_empty() {
            return self.clone();
        }

        BoundingBox {
            min: Point::new(
                self.min.x - delta,
                self.min.y - delta,
                self.min.z - delta,
            ),
            max: Point::new(
                self.max.x + delta,
                self.max.y + delta,
                self.max.z + delta,
            ),
        }
    }

    pub fn intersection(&self, other: &BoundingBox) -> BoundingBox {
        if !self.intersects(other) {
            return BoundingBox::empty();
        }

        BoundingBox {
            min: Point::new(
                self.min.x.max(other.min.x),
                self.min.y.max(other.min.y),
                self.min.z.max(other.min.z),
            ),
            max: Point::new(
                self.max.x.min(other.max.x),
                self.max.y.min(other.max.y),
                self.max.z.min(other.max.z),
            ),
        }
    }

    pub fn surface_area(&self) -> f64 {
        let (sx, sy, sz) = self.size();
        2.0 * (sx * sy + sy * sz + sz * sx)
    }

    pub fn corner(&self, index: usize) -> Point {
        match index % 8 {
            0 => Point::new(self.min.x, self.min.y, self.min.z),
            1 => Point::new(self.max.x, self.min.y, self.min.z),
            2 => Point::new(self.max.x, self.max.y, self.min.z),
            3 => Point::new(self.min.x, self.max.y, self.min.z),
            4 => Point::new(self.min.x, self.min.y, self.max.z),
            5 => Point::new(self.max.x, self.min.y, self.max.z),
            6 => Point::new(self.max.x, self.max.y, self.max.z),
            7 => Point::new(self.min.x, self.max.y, self.max.z),
            _ => self.center(),
        }
    }

    pub fn corners(&self) -> [Point; 8] {
        [
            Point::new(self.min.x, self.min.y, self.min.z),
            Point::new(self.max.x, self.min.y, self.min.z),
            Point::new(self.max.x, self.max.y, self.min.z),
            Point::new(self.min.x, self.max.y, self.min.z),
            Point::new(self.min.x, self.min.y, self.max.z),
            Point::new(self.max.x, self.min.y, self.max.z),
            Point::new(self.max.x, self.max.y, self.max.z),
            Point::new(self.min.x, self.max.y, self.max.z),
        ]
    }

    pub fn longest_axis(&self) -> usize {
        let (sx, sy, sz) = self.size();
        if sx >= sy && sx >= sz {
            0
        } else if sy >= sz {
            1
        } else {
            2
        }
    }

    pub fn shortest_axis(&self) -> usize {
        let (sx, sy, sz) = self.size();
        if sx <= sy && sx <= sz {
            0
        } else if sy <= sz {
            1
        } else {
            2
        }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_creation() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0));
        assert!(!bb.is_empty());
        assert_eq!(bb.size(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_bounding_box_empty() {
        let bb = BoundingBox::empty();
        assert!(bb.is_empty());
    }

    #[test]
    fn test_bounding_box_from_points() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 3.0),
            Point::new(-1.0, -2.0, -3.0),
        ];
        let bb = BoundingBox::from_points(&points);
        assert_eq!(bb.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(bb.max, Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_bounding_box_contains() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        assert!(bb.contains(&Point::new(0.5, 0.5, 0.5)));
        assert!(!bb.contains(&Point::new(1.5, 0.5, 0.5)));
    }

    #[test]
    fn test_bounding_box_center() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 4.0, 6.0));
        let center = bb.center();
        assert_eq!(center, Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_bounding_box_volume() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(bb.volume(), 24.0);
    }

    #[test]
    fn test_bounding_box_diagonal() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 4.0, 0.0));
        assert!((bb.diagonal() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box_merge() {
        let bb1 = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let bb2 = BoundingBox::new(Point::new(0.5, 0.5, 0.5), Point::new(2.0, 2.0, 2.0));
        let merged = bb1.merge(&bb2);
        assert_eq!(merged.min, Point::new(0.0, 0.0, 0.0));
        assert_eq!(merged.max, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_bounding_box_intersects() {
        let bb1 = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let bb2 = BoundingBox::new(Point::new(0.5, 0.5, 0.5), Point::new(2.0, 2.0, 2.0));
        let bb3 = BoundingBox::new(Point::new(2.0, 2.0, 2.0), Point::new(3.0, 3.0, 3.0));

        assert!(bb1.intersects(&bb2));
        assert!(!bb1.intersects(&bb3));
    }

    #[test]
    fn test_bounding_box_expand() {
        let mut bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        bb.expand(1.0);
        assert_eq!(bb.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bb.max, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_bounding_box_intersection() {
        let bb1 = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 2.0, 2.0));
        let bb2 = BoundingBox::new(Point::new(1.0, 1.0, 1.0), Point::new(3.0, 3.0, 3.0));
        let intersection = bb1.intersection(&bb2);
        assert_eq!(intersection.min, Point::new(1.0, 1.0, 1.0));
        assert_eq!(intersection.max, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_bounding_box_corners() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let corners = bb.corners();
        assert_eq!(corners.len(), 8);
        assert_eq!(corners[0], Point::new(0.0, 0.0, 0.0));
        assert_eq!(corners[6], Point::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_bounding_box_surface_area() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0));
        assert_eq!(bb.surface_area(), 22.0);
    }

    #[test]
    fn test_bounding_box_axes() {
        let bb = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 2.0, 1.0));
        assert_eq!(bb.longest_axis(), 0);
        assert_eq!(bb.shortest_axis(), 2);
    }
}
