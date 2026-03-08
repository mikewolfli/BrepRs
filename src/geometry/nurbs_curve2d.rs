use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct NurbsCurve2D {
    poles: Vec<Point>,
    weights: Vec<Standard_Real>,
    knots: Vec<Standard_Real>,
    multiplicities: Vec<i32>,
    degree: i32,
    is_rational: bool,
    is_periodic: bool,
}

impl NurbsCurve2D {
    pub fn new(
        degree: i32,
        poles: Vec<Point>,
        weights: Vec<Standard_Real>,
        knots: Vec<Standard_Real>,
        multiplicities: Vec<i32>,
    ) -> Self {
        assert!(!poles.is_empty(), "Poles cannot be empty");
        assert_eq!(poles.len(), weights.len(), "Poles and weights must have the same length");
        assert!(degree >= 0, "Degree must be non-negative");
        assert!(!knots.is_empty(), "Knots cannot be empty");
        assert_eq!(knots.len(), multiplicities.len(), "Knots and multiplicities must have the same length");

        let is_rational = weights.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON);

        Self {
            degree,
            poles,
            weights,
            knots,
            multiplicities,
            is_rational,
            is_periodic: false,
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    pub fn nb_knots(&self) -> i32 {
        self.knots.len() as i32
    }

    pub fn poles(&self) -> &[Point] {
        &self.poles
    }

    pub fn weights(&self) -> &[Standard_Real] {
        &self.weights
    }

    pub fn knots(&self) -> &[Standard_Real] {
        &self.knots
    }

    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
    }

    pub fn pole(&self, index: i32) -> Option<&Point> {
        if index >= 0 && (index as usize) < self.poles.len() {
            Some(&self.poles[index as usize])
        } else {
            None
        }
    }

    pub fn weight(&self, index: i32) -> Option<Standard_Real> {
        if index >= 0 && (index as usize) < self.weights.len() {
            Some(self.weights[index as usize])
        } else {
            None
        }
    }

    pub fn knot(&self, index: i32) -> Option<Standard_Real> {
        if index >= 0 && (index as usize) < self.knots.len() {
            Some(self.knots[index as usize])
        } else {
            None
        }
    }

    pub fn multiplicity(&self, index: i32) -> Option<i32> {
        if index >= 0 && (index as usize) < self.multiplicities.len() {
            Some(self.multiplicities[index as usize])
        } else {
            None
        }
    }

    pub fn set_pole(&mut self, index: i32, pole: Point) -> bool {
        if index >= 0 && (index as usize) < self.poles.len() {
            self.poles[index as usize] = pole;
            true
        } else {
            false
        }
    }

    pub fn set_weight(&mut self, index: i32, weight: Standard_Real) -> bool {
        if index >= 0 && (index as usize) < self.weights.len() {
            self.weights[index as usize] = weight;
            self.is_rational = self.weights.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON);
            true
        } else {
            false
        }
    }

    pub fn set_knot(&mut self, index: i32, knot: Standard_Real) -> bool {
        if index >= 0 && (index as usize) < self.knots.len() {
            self.knots[index as usize] = knot;
            true
        } else {
            false
        }
    }

    pub fn set_multiplicity(&mut self, index: i32, multiplicity: i32) -> bool {
        if index >= 0 && (index as usize) < self.multiplicities.len() {
            self.multiplicities[index as usize] = multiplicity;
            true
        } else {
            false
        }
    }

    pub fn insert_knot(&mut self, knot: Standard_Real, multiplicity: i32) -> bool {
        let insert_index = self.knots.binary_search_by(|probe| {
            probe.partial_cmp(&knot).unwrap_or(std::cmp::Ordering::Equal)
        });

        match insert_index {
            Ok(idx) => {
                self.multiplicities[idx] += multiplicity;
            }
            Err(idx) => {
                self.knots.insert(idx, knot);
                self.multiplicities.insert(idx, multiplicity);
            }
        }

        true
    }

    pub fn remove_knot(&mut self, index: i32, multiplicity: i32) -> bool {
        if index < 0 || (index as usize) >= self.knots.len() {
            return false;
        }

        let idx = index as usize;
        if self.multiplicities[idx] <= multiplicity {
            self.knots.remove(idx);
            self.multiplicities.remove(idx);
        } else {
            self.multiplicities[idx] -= multiplicity;
        }

        true
    }

    pub fn increase_degree(&mut self, new_degree: i32) -> bool {
        if new_degree <= self.degree {
            return false;
        }

        let degree_diff = new_degree - self.degree;
        for _ in 0..degree_diff {
            self.increase_degree_by_one();
        }

        self.degree = new_degree;
        true
    }

    fn increase_degree_by_one(&mut self) {
        let n = self.poles.len();
        if n == 0 {
            return;
        }

        let mut new_poles = Vec::with_capacity(n + 1);
        let mut new_weights = Vec::with_capacity(n + 1);

        new_poles.push(self.poles[0]);
        new_weights.push(self.weights[0]);

        for i in 1..n {
            let alpha = i as Standard_Real / n as Standard_Real;
            let pole = Point::new(
                (1.0 - alpha) * self.poles[i - 1].x + alpha * self.poles[i].x,
                (1.0 - alpha) * self.poles[i - 1].y + alpha * self.poles[i].y,
                (1.0 - alpha) * self.poles[i - 1].z + alpha * self.poles[i].z,
            );
            let weight = (1.0 - alpha) * self.weights[i - 1] + alpha * self.weights[i];

            new_poles.push(pole);
            new_weights.push(weight);

            new_poles.push(self.poles[i]);
            new_weights.push(self.weights[i]);
        }

        self.poles = new_poles;
        self.weights = new_weights;
    }

    pub fn position(&self, parameter: Standard_Real) -> Point {
        if self.poles.is_empty() {
            return Point::origin();
        }

        let n = self.poles.len() - 1;
        let p = self.degree;

        let mut result = Point::origin();
        let mut weight_sum = 0.0;

        for i in 0..=n {
            let basis = self.basis_function(i, p, parameter);
            let weighted_basis = basis * self.weights[i];

            result.x += weighted_basis * self.poles[i].x;
            result.y += weighted_basis * self.poles[i].y;
            result.z += weighted_basis * self.poles[i].z;
            weight_sum += weighted_basis;
        }

        if weight_sum.abs() > STANDARD_REAL_EPSILON {
            result.x /= weight_sum;
            result.y /= weight_sum;
            result.z /= weight_sum;
        }

        result
    }

    pub fn d1(&self, parameter: Standard_Real) -> Vector {
        if self.poles.len() < 2 {
            return Vector::zero();
        }

        let epsilon = 0.0001;
        let pos_plus = self.position(parameter + epsilon);
        let pos_minus = self.position(parameter - epsilon);

        Vector::new(
            (pos_plus.x - pos_minus.x) / (2.0 * epsilon),
            (pos_plus.y - pos_minus.y) / (2.0 * epsilon),
            (pos_plus.z - pos_minus.z) / (2.0 * epsilon),
        )
    }

    pub fn d2(&self, parameter: Standard_Real) -> Vector {
        let epsilon = 0.0001;
        let d1_plus = self.d1(parameter + epsilon);
        let d1_minus = self.d1(parameter - epsilon);

        Vector::new(
            (d1_plus.x - d1_minus.x) / (2.0 * epsilon),
            (d1_plus.y - d1_minus.y) / (2.0 * epsilon),
            (d1_plus.z - d1_minus.z) / (2.0 * epsilon),
        )
    }

    fn basis_function(&self, i: usize, p: i32, u: Standard_Real) -> Standard_Real {
        if p == 0 {
            return self.basis_function_zero_degree(i, u);
        }

        let p_usize = p as usize;

        let left = if i > 0 {
            let knot_i = self.get_knot(i);
            let knot_i_plus_p = self.get_knot(i + p_usize);
            
            if (knot_i_plus_p - knot_i).abs() < STANDARD_REAL_EPSILON {
                0.0
            } else {
                ((u - knot_i) / (knot_i_plus_p - knot_i)) * self.basis_function(i, p - 1, u)
            }
        } else {
            0.0
        };

        let right = if i < self.poles.len() - 1 {
            let knot_i_plus_1 = self.get_knot(i + 1);
            let knot_i_plus_p_plus_1 = self.get_knot(i + p_usize + 1);
            
            if (knot_i_plus_p_plus_1 - knot_i_plus_1).abs() < STANDARD_REAL_EPSILON {
                0.0
            } else {
                ((knot_i_plus_p_plus_1 - u) / (knot_i_plus_p_plus_1 - knot_i_plus_1)) * 
                self.basis_function(i + 1, p - 1, u)
            }
        } else {
            0.0
        };

        left + right
    }

    fn basis_function_zero_degree(&self, i: usize, u: Standard_Real) -> Standard_Real {
        let knot_i = self.get_knot(i);
        let knot_i_plus_1 = self.get_knot(i + 1);

        if u >= knot_i && u < knot_i_plus_1 {
            1.0
        } else if u == knot_i_plus_1 && i == self.poles.len() - 1 {
            1.0
        } else {
            0.0
        }
    }

    fn get_knot(&self, index: usize) -> Standard_Real {
        if self.knots.is_empty() {
            return 0.0;
        }

        let mut total_multiplicity = 0;
        for (knot_idx, &mult) in self.multiplicities.iter().enumerate() {
            total_multiplicity += mult as usize;
            if index < total_multiplicity {
                return self.knots[knot_idx];
            }
        }

        self.knots[self.knots.len() - 1]
    }

    pub fn is_rational(&self) -> bool {
        self.is_rational
    }

    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    pub fn set_periodic(&mut self, is_periodic: bool) {
        self.is_periodic = is_periodic;
    }

    pub fn is_closed(&self, tolerance: Standard_Real) -> bool {
        if self.poles.len() < 2 {
            return true;
        }
        self.poles.first().unwrap().distance(self.poles.last().unwrap()) <= tolerance
    }

    pub fn continuity(&self) -> i32 {
        if self.poles.len() < 2 {
            return 0;
        }
        
        let start_tangent = self.d1(self.knots[0]);
        let end_tangent = self.d1(self.knots[self.knots.len() - 1]);
        
        if start_tangent.magnitude() < STANDARD_REAL_EPSILON || end_tangent.magnitude() < STANDARD_REAL_EPSILON {
            return 0;
        }

        let normalized_start = start_tangent.normalized();
        let normalized_end = end_tangent.normalized();

        if normalized_start.is_equal(&normalized_end, STANDARD_REAL_EPSILON) {
            1
        } else {
            0
        }
    }

    pub fn first_parameter(&self) -> Standard_Real {
        if self.knots.is_empty() {
            0.0
        } else {
            self.knots[0]
        }
    }

    pub fn last_parameter(&self) -> Standard_Real {
        if self.knots.is_empty() {
            1.0
        } else {
            self.knots[self.knots.len() - 1]
        }
    }
}

impl Default for NurbsCurve2D {
    fn default() -> Self {
        Self {
            degree: 1,
            poles: vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            weights: vec![1.0, 1.0],
            knots: vec![0.0, 1.0],
            multiplicities: vec![2, 2],
            is_rational: false,
            is_periodic: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nurbs_curve2d_creation() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let weights = vec![1.0, 1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        assert_eq!(curve.degree(), 1);
        assert_eq!(curve.nb_poles(), 3);
    }

    #[test]
    fn test_nurbs_curve2d_position() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        
        let start = curve.position(0.0);
        assert_eq!(start, Point::origin());
        
        let end = curve.position(1.0);
        assert_eq!(end, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_nurbs_curve2d_set_pole() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let mut curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        assert!(curve.set_pole(1, Point::new(2.0, 0.0, 0.0)));
        assert_eq!(curve.pole(1), Some(&Point::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_nurbs_curve2d_insert_knot() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let mut curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        assert!(curve.insert_knot(0.5, 1));
        assert_eq!(curve.nb_knots(), 3);
    }

    #[test]
    fn test_nurbs_curve2d_is_rational() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        assert!(!curve.is_rational());
    }

    #[test]
    fn test_nurbs_curve2d_first_last_parameter() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let multiplicities = vec![2, 2];
        let curve = NurbsCurve2D::new(1, poles, weights, knots, multiplicities);
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 1.0);
    }
}
