use super::{LinearTransformation, Matrix};
use crate::radians::Radians;

/// TODO
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RotationDirection {
    /// TODO
    Clockwise,
    /// TODO
    Counterclockwise,
}

impl Default for RotationDirection {
    fn default() -> Self {
        Self::Counterclockwise
    }
}

/// A rotation
#[derive(Debug, PartialEq, Clone)]
pub struct Rotation {
    angle: Radians,
    direction: RotationDirection,
}

/// Builder used to construct a [`Rotation`].
#[derive(Debug, PartialEq, Clone)]
pub struct RotationBuilder {
    angle: Radians,
    direction: RotationDirection,
}

impl RotationBuilder {
    /// Creates a new [`RotationBuilder`].
    pub fn new(angle: Radians) -> Self {
        Self {
            angle,
            direction: RotationDirection::default(),
        }
    }

    /// Configures the direction of this rotation.
    pub fn direction(mut self, direction: RotationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Finalizes this builder, returning a [`Rotation`].
    pub fn build(self) -> Rotation {
        let Self { angle, direction } = self;
        Rotation { angle, direction }
    }
}

impl LinearTransformation for Rotation {
    fn to_matrix(&self) -> Matrix {
        let direction = match self.direction {
            RotationDirection::Counterclockwise => 1.0,
            RotationDirection::Clockwise => -1.0,
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
