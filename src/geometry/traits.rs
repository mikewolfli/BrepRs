/// 全局容差常量
pub const TOLERANCE: f64 = 1e-8;
/// Trait for getting coordinates (x, y, z)
pub trait GetCoord {
    fn coord(&self) -> (f64, f64, f64);
}

/// Trait for setting coordinates (x, y, z)
pub trait SetCoord {
    fn set_coord(&mut self, x: f64, y: f64, z: f64);
}
