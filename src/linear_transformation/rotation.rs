use super::{LinearTransformation, Matrix};
use crate::radians::Radians;

/// TODO
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    /// TODO
    Clockwise,
    /// TODO
    Counterclockwise,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Counterclockwise
    }
}

/// A rotation
#[derive(Debug, PartialEq, Clone)]
pub struct Rotation {
    angle: Radians,
    direction: Direction,
}

impl Rotation {
    /// TODO
    pub fn new(angle: Radians) -> Self {
        Self::new_with_direction(angle, Direction::default())
    }

    /// TODO
    pub fn new_with_direction(angle: Radians, direction: Direction) -> Self {
        Self { angle, direction }
    }
}

impl LinearTransformation for Rotation {
    fn as_matrix(&self) -> Matrix {
        let direction = match self.direction {
            Direction::Counterclockwise => 1.0,
            Direction::Clockwise => -1.0,
        };
        let angle = direction * self.angle.value();
        let (sin, cos) = angle.sin_cos();
        Matrix {
            a: cos,
            b: -sin,
            c: sin,
            d: cos,
        }
    }
}
