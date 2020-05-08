use crate::vector::Vector;
use std::ops::Mul;

/// A 2x2 matrix
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

impl Matrix {
    /// The identity [`Matrix`].
    pub const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
    };
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

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d + rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_does_nothing() {
        let vector = Vector { x: 5.0, y: 3.5 };
        let transformed_vector = vector.transform(Matrix::IDENTITY);
        assert_eq!(vector, transformed_vector);
    }

    #[test]
    fn multiplying_by_identity_matrix_does_nothing() {
        let matrix = Matrix {
            a: 5.3,
            b: 1.2,
            c: 7.8,
            d: 2.4,
        };
        let result = Matrix::IDENTITY * matrix;
        assert_eq!(matrix, result);
    }
}
