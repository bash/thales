//! Contains linear transformations

use crate::vector::Vector;
use std::ops::Mul;

mod rotation;
pub use rotation::*;

/// A 2x2 matrix
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix {
    /// TODO
    pub a: f64,
    /// TODO
    pub b: f64,
    /// TODO
    pub c: f64,
    /// TODO
    pub d: f64,
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.a * rhs.x + self.b * rhs.y,
            y: self.c * rhs.x + self.d * rhs.y,
        }
    }
}

/// A linear transformation that can be applied to a point.
pub trait LinearTransformation {
    /// TODO
    fn as_matrix(&self) -> Matrix;
}
