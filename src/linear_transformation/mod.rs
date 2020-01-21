//! Contains linear transformations

use crate::vector::Vector;
use std::ops::Mul;

mod rotation;
pub use rotation::*;

/// A 2x2 matrix
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix {
    /// Row 1, Column 1
    pub a: f64,
    /// Row 1, Column 2
    pub b: f64,
    /// Row 2, Column 1
    pub c: f64,
    /// Row 2, Column 2
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
    /// Returns the matrix that represents this linear transformation.
    fn as_matrix(&self) -> Matrix;
}

impl LinearTransformation for Matrix {
    fn as_matrix(&self) -> Matrix {
        self.clone()
    }
}
