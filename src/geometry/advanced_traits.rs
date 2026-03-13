/// Generic curve trait supporting sampling, interpolation, derivatives, projection, etc.
use crate::geometry::traits::GetCoord;

pub trait Curve {
    type Point: GetCoord;
    type Vector: GetCoord;
    /// Requires Point/Vector to implement GetCoord trait
    /// Sample point
    fn sample(&self, t: f64) -> Self::Point;
    /// First derivative
    fn derivative(&self, t: f64) -> Self::Vector;
    /// Second derivative
    fn second_derivative(&self, t: f64) -> Self::Vector;
    /// Project point
    fn project(&self, point: &Self::Point) -> f64;
    /// Find closest point
    fn closest_point(&self, point: &Self::Point) -> Self::Point;
    /// Curve length
    fn length(&self, t0: f64, t1: f64) -> f64;
}

/// Generic surface trait supporting sampling, derivatives, projection, etc.
pub trait Surface {
    type Point: GetCoord;
    type Vector: GetCoord;
    /// Sample point
    fn sample(&self, u: f64, v: f64) -> Self::Point;
    /// First derivative
    fn derivative(&self, u: f64, v: f64) -> (Self::Vector, Self::Vector);
    /// Second derivative
    fn second_derivative(
        &self,
        u: f64,
        v: f64,
    ) -> ((Self::Vector, Self::Vector), (Self::Vector, Self::Vector));
    /// Project point
    fn project(&self, point: &Self::Point) -> (f64, f64);
    /// Find closest point
    fn closest_point(&self, point: &Self::Point) -> Self::Point;
    /// Surface area
    fn area(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> f64;
}
