use std::ops::Add;
use std::ops::Mul;

use svg::node::element::path::Parameters;

#[derive(Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
    pub height: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, height: f64) -> Self {
        Position { x, y, height }
    }

    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.height - self.y
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert!((self.height - rhs.height).abs() <= std::f64::EPSILON);
        Self::new(self.x + rhs.x, self.y + rhs.y, self.height)
    }
}

impl Mul<f64> for Position {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.height)
    }
}

impl Into<Parameters> for Position {
    fn into(self) -> Parameters {
        Parameters::from((self.x(), self.y()))
    }
}
