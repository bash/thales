//! Contains linear transformations

use crate::matrix::Matrix;

mod rotation;
pub use rotation::*;

/// A linear transformation that can be applied to a point.
pub trait LinearTransformation {
    /// Returns the matrix that represents this linear transformation.
    fn to_matrix(&self) -> Matrix;
}

impl LinearTransformation for Matrix {
    fn to_matrix(&self) -> Matrix {
        self.clone()
    }
}
