use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NurbsSurface {
    poles: Vec<Vec<Point>>,
    weights: Vec<Vec<StandardReal>>,
    u_knots: Vec<StandardReal>,
    v_knots: Vec<StandardReal>,
    u_multiplicities: Vec<i32>,
    v_multiplicities: Vec<i32>,
    u_degree: i32,
    v_degree: i32,
    is_rational: bool,
    is_u_periodic: bool,
    is_v_periodic: bool,
}

impl NurbsSurface {
    pub fn new(
        u_degree: i32,
        v_degree: i32,
        poles: Vec<Vec<Point>>,
        weights: Vec<Vec<StandardReal>>,
        u_knots: Vec<StandardReal>,
        v_knots: Vec<StandardReal>,
        u_multiplicities: Vec<i32>,
        v_multiplicities: Vec<i32>,
    ) -> Self {
        assert!(!poles.is_empty(), "Poles cannot be empty");
        assert_eq!(poles.len(), weights.len(), "Poles and weights must have the same number of U rows");
        if !poles.is_empty() {
            assert_eq!(poles[0].len(), weights[0].len(), "Poles and weights must have the same number of V columns");
        }
        assert!(u_degree >= 0, "U degree must be non-negative");
        assert!(v_degree >= 0, "V degree must be non-negative");
        assert!(!u_knots.is_empty(), "U knots cannot be empty");
        assert!(!v_knots.is_empty(), "V knots cannot be empty");
        assert_eq!(u_knots.len(), u_multiplicities.len(), "U knots and multiplicities must have the same length");
        assert_eq!(v_knots.len(), v_multiplicities.len(), "V knots and multiplicities must have the same length");

        let is_rational = weights.iter().any(|row| row.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON));

        Self {
            u_degree,
            v_degree,
            poles,
            weights,
            u_knots,
            v_knots,
            u_multiplicities,
            v_multiplicities,
            is_rational,
            is_u_periodic: false,
            is_v_periodic: false,
        }
    }

    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    pub fn nb_u_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    pub fn nb_v_poles(&self) -> i32 {
        if self.poles.is_empty() {
            0
        } else {
            self.poles[0].len() as i32
        }
    }

    pub fn nb_u_knots(&self) -> i32 {
        self.u_knots.len() as i32
    }

    pub fn nb_v_knots(&self) -> i32 {
        self.v_knots.len() as i32
    }

    pub fn poles(&self) -> &[Vec<Point>] {
        &self.poles
    }

    pub fn weights(&self) -> &[Vec<StandardReal>] {
        &self.weights
    }

    pub fn u_knots(&self) -> &[StandardReal] {
        &self.u_knots
    }

    pub fn v_knots(&self) -> &[StandardReal] {
        &self.v_knots
    }

    pub fn u_multiplicities(&self) -> &[i32] {
        &self.u_multiplicities
    }

    pub fn v_multiplicities(&self) -> &[i32] {
        &self.v_multiplicities
    }

    pub fn pole(&self, u_index: i32, v_index: i32) -> Option<&Point> {
        if u_index >= 0 && (u_index as usize) < self.poles.len() {
            let u_row = &self.poles[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                Some(&u_row[v_index as usize])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn weight(&self, u_index: i32, v_index: i32) -> Option<StandardReal> {
        if u_index >= 0 && (u_index as usize) < self.weights.len() {
            let u_row = &self.weights[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                Some(u_row[v_index as usize])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn u_knot(&self, index: i32) -> Option<StandardReal> {
        if index >= 0 && (index as usize) < self.u_knots.len() {
            Some(self.u_knots[index as usize])
        } else {
            None
        }
    }

    pub fn v_knot(&self, index: i32) -> Option<StandardReal> {
        if index >= 0 && (index as usize) < self.v_knots.len() {
            Some(self.v_knots[index as usize])
        } else {
            None
        }
    }

    pub fn u_multiplicity(&self, index: i32) -> Option<i32> {
        if index >= 0 && (index as usize) < self.u_multiplicities.len() {
            Some(self.u_multiplicities[index as usize])
        } else {
            None
        }
    }

    pub fn v_multiplicity(&self, index: i32) -> Option<i32> {
        if index >= 0 && (index as usize) < self.v_multiplicities.len() {
            Some(self.v_multiplicities[index as usize])
        } else {
            None
        }
    }

    pub fn set_pole(&mut self, u_index: i32, v_index: i32, pole: Point) -> bool {
        if u_index >= 0 && (u_index as usize) < self.poles.len() {
            let u_row = &mut self.poles[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                u_row[v_index as usize] = pole;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_weight(&mut self, u_index: i32, v_index: i32, weight: StandardReal) -> bool {
        if u_index >= 0 && (u_index as usize) < self.weights.len() {
            let u_row = &mut self.weights[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                u_row[v_index as usize] = weight;
                self.is_rational = self.weights.iter().any(|row| row.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_u_knot(&mut self, index: i32, knot: StandardReal) -> bool {
        if index >= 0 && (index as usize) < self.u_knots.len() {
            self.u_knots[index as usize] = knot;
            true
        } else {
            false
        }
    }

    pub fn set_v_knot(&mut self, index: i32, knot: StandardReal) -> bool {
        if index >= 0 && (index as usize) < self.v_knots.len() {
            self.v_knots[index as usize] = knot;
            true
        } else {
            false
        }
    }

    pub fn set_u_multiplicity(&mut self, index: i32, multiplicity: i32) -> bool {
        if index >= 0 && (index as usize) < self.u_multiplicities.len() {
            self.u_multiplicities[index as usize] = multiplicity;
            true
        } else {
            false
        }
    }

    pub fn set_v_multiplicity(&mut self, index: i32, multiplicity: i32) -> bool {
        if index >= 0 && (index as usize) < self.v_multiplicities.len() {
            self.v_multiplicities[index as usize] = multiplicity;
            true
        } else {
            false
        }
    }

    pub fn insert_u_knot(&mut self, knot: StandardReal, multiplicity: i32) -> bool {
        let insert_index = self.u_knots.binary_search_by(|probe| {
            probe.partial_cmp(&knot).unwrap_or(std::cmp::Ordering::Equal)
        });

        match insert_index {
            Ok(idx) => {
                self.u_multiplicities[idx] += multiplicity;
            }
            Err(idx) => {
                self.u_knots.insert(idx, knot);
                self.u_multiplicities.insert(idx, multiplicity);
            }
        }

        true
    }

    pub fn insert_v_knot(&mut self, knot: StandardReal, multiplicity: i32) -> bool {
        let insert_index = self.v_knots.binary_search_by(|probe| {
            probe.partial_cmp(&knot).unwrap_or(std::cmp::Ordering::Equal)
        });

        match insert_index {
            Ok(idx) => {
                self.v_multiplicities[idx] += multiplicity;
            }
            Err(idx) => {
                self.v_knots.insert(idx, knot);
                self.v_multiplicities.insert(idx, multiplicity);
            }
        }

        true
    }

    pub fn position(&self, u_parameter: StandardReal, v_parameter: StandardReal) -> Point {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return Point::origin();
        }

        let u_degree = self.u_degree;
        let v_degree = self.v_degree;

        let mut result = Point::origin();
        let mut weight_sum = 0.0;

        for i in 0..self.nb_u_poles() {
            for j in 0..self.nb_v_poles() {
                let u_basis = self.basis_function(i, u_degree, u_parameter, true);
                let v_basis = self.basis_function(j, v_degree, v_parameter, false);
                let basis = u_basis * v_basis;
                let weighted_basis = basis * self.weights[i as usize][j as usize];

                result.x += weighted_basis * self.poles[i as usize][j as usize].x;
                result.y += weighted_basis * self.poles[i as usize][j as usize].y;
                result.z += weighted_basis * self.poles[i as usize][j as usize].z;
                weight_sum += weighted_basis;
            }
        }

        if weight_sum.abs() > STANDARD_REAL_EPSILON {
            result.x /= weight_sum;
            result.y /= weight_sum;
            result.z /= weight_sum;
        }

        result
    }

    pub fn d1(&self, u_parameter: StandardReal, v_parameter: StandardReal, u_direction: bool) -> Vector {
        let epsilon = 0.0001;
        if u_direction {
            let pos_plus = self.position(u_parameter + epsilon, v_parameter);
            let pos_minus = self.position(u_parameter - epsilon, v_parameter);
            Vector::new(
                (pos_plus.x - pos_minus.x) / (2.0 * epsilon),
                (pos_plus.y - pos_minus.y) / (2.0 * epsilon),
                (pos_plus.z - pos_minus.z) / (2.0 * epsilon),
            )
        } else {
            let pos_plus = self.position(u_parameter, v_parameter + epsilon);
            let pos_minus = self.position(u_parameter, v_parameter - epsilon);
            Vector::new(
                (pos_plus.x - pos_minus.x) / (2.0 * epsilon),
                (pos_plus.y - pos_minus.y) / (2.0 * epsilon),
                (pos_plus.z - pos_minus.z) / (2.0 * epsilon),
            )
        }
    }

    pub fn d2(&self, u_parameter: StandardReal, v_parameter: StandardReal) -> Vector {
        let epsilon = 0.0001;
        let d1_plus = self.d1(u_parameter + epsilon, v_parameter, true);
        let d1_minus = self.d1(u_parameter - epsilon, v_parameter, true);

        Vector::new(
            (d1_plus.x - d1_minus.x) / (2.0 * epsilon),
            (d1_plus.y - d1_minus.y) / (2.0 * epsilon),
            (d1_plus.z - d1_minus.z) / (2.0 * epsilon),
        )
    }

    fn basis_function(&self, i: i32, p: i32, u: StandardReal, is_u: bool) -> StandardReal {
        if p == 0 {
            return self.basis_function_zero_degree(i, u, is_u);
        }

        let p_usize = p as usize;
        let i_usize = i as usize;

        let left = if i > 0 {
            let knot_i = self.get_knot(i_usize, is_u);
            let knot_i_plus_p = self.get_knot(i_usize + p_usize, is_u);
            
            if (knot_i_plus_p - knot_i).abs() < STANDARD_REAL_EPSILON {
                0.0
            } else {
                ((u - knot_i) / (knot_i_plus_p - knot_i)) * self.basis_function(i, p - 1, u, is_u)
            }
        } else {
            0.0
        };

        let right = if i < self.nb_u_poles() - 1 {
            let knot_i_plus_1 = self.get_knot(i_usize + 1, is_u);
            let knot_i_plus_p_plus_1 = self.get_knot(i_usize + p_usize + 1, is_u);
            
            if (knot_i_plus_p_plus_1 - knot_i_plus_1).abs() < STANDARD_REAL_EPSILON {
                0.0
            } else {
                ((knot_i_plus_p_plus_1 - u) / (knot_i_plus_p_plus_1 - knot_i_plus_1)) * 
                self.basis_function(i + 1, p - 1, u, is_u)
            }
        } else {
            0.0
        };

        left + right
    }

    fn basis_function_zero_degree(&self, i: i32, u: StandardReal, is_u: bool) -> StandardReal {
        let i_usize = i as usize;
        let knot_i = self.get_knot(i_usize, is_u);
        let knot_i_plus_1 = self.get_knot(i_usize + 1, is_u);

        if u >= knot_i && u < knot_i_plus_1 {
            1.0
        } else if u == knot_i_plus_1 && i == self.nb_u_poles() - 1 {
            1.0
        } else {
            0.0
        }
    }

    fn get_knot(&self, index: usize, is_u: bool) -> StandardReal {
        if is_u {
            if self.u_knots.is_empty() {
                return 0.0;
            }

            let mut total_multiplicity = 0;
            for (knot_idx, &mult) in self.u_multiplicities.iter().enumerate() {
                total_multiplicity += mult as usize;
                if index < total_multiplicity {
                    return self.u_knots[knot_idx];
                }
            }

            self.u_knots[self.u_knots.len() - 1]
        } else {
            if self.v_knots.is_empty() {
                return 0.0;
            }

            let mut total_multiplicity = 0;
            for (knot_idx, &mult) in self.v_multiplicities.iter().enumerate() {
                total_multiplicity += mult as usize;
                if index < total_multiplicity {
                    return self.v_knots[knot_idx];
                }
            }

            self.v_knots[self.v_knots.len() - 1]
        }
    }

    pub fn is_rational(&self) -> bool {
        self.is_rational
    }

    pub fn is_u_periodic(&self) -> bool {
        self.is_u_periodic
    }

    pub fn is_v_periodic(&self) -> bool {
        self.is_v_periodic
    }

    pub fn set_u_periodic(&mut self, is_periodic: bool) {
        self.is_u_periodic = is_periodic;
    }

    pub fn set_v_periodic(&mut self, is_periodic: bool) {
        self.is_v_periodic = is_periodic;
    }

    pub fn is_u_closed(&self, tolerance: StandardReal) -> bool {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return true;
        }
        
        for j in 0..self.nb_v_poles() {
            if let (Some(first), Some(last)) = (self.pole(0, j), self.pole(self.nb_u_poles() - 1, j)) {
                if first.distance(last) > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_v_closed(&self, tolerance: StandardReal) -> bool {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return true;
        }
        
        for i in 0..self.nb_u_poles() {
            if let (Some(first), Some(last)) = (self.pole(i, 0), self.pole(i, self.nb_v_poles() - 1)) {
                if first.distance(last) > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn first_u_parameter(&self) -> StandardReal {
        if self.u_knots.is_empty() {
            0.0
        } else {
            self.u_knots[0]
        }
    }

    pub fn last_u_parameter(&self) -> StandardReal {
        if self.u_knots.is_empty() {
            1.0
        } else {
            self.u_knots[self.u_knots.len() - 1]
        }
    }

    pub fn first_v_parameter(&self) -> StandardReal {
        if self.v_knots.is_empty() {
            0.0
        } else {
            self.v_knots[0]
        }
    }

    pub fn last_v_parameter(&self) -> StandardReal {
        if self.v_knots.is_empty() {
            1.0
        } else {
            self.v_knots[self.v_knots.len() - 1]
        }
    }

    /// Get the point on the surface at (u, v) parameters
    pub fn value(&self, u: f64, v: f64) -> Point {
        self.position(u as StandardReal, v as StandardReal)
    }

    /// Get the normal at (u, v) parameters
    pub fn normal(&self, u: f64, v: f64) -> Vector {
        let u_dir = self.d1(u as StandardReal, v as StandardReal, true);
        let v_dir = self.d1(u as StandardReal, v as StandardReal, false);
        u_dir.cross(&v_dir).normalized()
    }

    /// Get the parameter range of the surface
    pub fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        ((self.first_u_parameter() as f64, self.last_u_parameter() as f64), 
         (self.first_v_parameter() as f64, self.last_v_parameter() as f64))
    }
}

impl Default for NurbsSurface {
    fn default() -> Self {
        Self {
            u_degree: 1,
            v_degree: 1,
            poles: vec![vec![Point::origin(), Point::new(1.0, 0.0, 0.0)]],
            weights: vec![vec![1.0, 1.0]],
            u_knots: vec![0.0, 1.0],
            v_knots: vec![0.0, 1.0],
            u_multiplicities: vec![2, 2],
            v_multiplicities: vec![2, 2],
            is_rational: false,
            is_u_periodic: false,
            is_v_periodic: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nurbs_surface_creation() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let u_multiplicities = vec![2, 2];
        let v_multiplicities = vec![2, 2];
        let surface = NurbsSurface::new(1, 1, poles, weights, u_knots, v_knots, u_multiplicities, v_multiplicities);
        assert_eq!(surface.u_degree(), 1);
        assert_eq!(surface.v_degree(), 1);
    }

    #[test]
    fn test_nurbs_surface_position() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let u_multiplicities = vec![2, 2];
        let v_multiplicities = vec![2, 2];
        let surface = NurbsSurface::new(1, 1, poles, weights, u_knots, v_knots, u_multiplicities, v_multiplicities);
        
        let corner = surface.position(0.0, 0.0);
        assert_eq!(corner, Point::origin());
        
        let opposite_corner = surface.position(1.0, 1.0);
        assert_eq!(opposite_corner, Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_nurbs_surface_is_rational() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let u_multiplicities = vec![2, 2];
        let v_multiplicities = vec![2, 2];
        let surface = NurbsSurface::new(1, 1, poles, weights, u_knots, v_knots, u_multiplicities, v_multiplicities);
        assert!(!surface.is_rational());
    }

    #[test]
    fn test_nurbs_surface_first_last_parameter() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let u_multiplicities = vec![2, 2];
        let v_multiplicities = vec![2, 2];
        let surface = NurbsSurface::new(1, 1, poles, weights, u_knots, v_knots, u_multiplicities, v_multiplicities);
        assert_eq!(surface.first_u_parameter(), 0.0);
        assert_eq!(surface.last_u_parameter(), 1.0);
        assert_eq!(surface.first_v_parameter(), 0.0);
        assert_eq!(surface.last_v_parameter(), 1.0);
    }
}
