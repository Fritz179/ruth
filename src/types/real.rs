use std::fmt::{Debug, Display};

use crate::{Add, Types, Value};

#[derive(Debug, Clone)]
pub struct InnerReal(pub f64);
pub type Real = Value<InnerReal>;

impl Real {
    pub fn new(value: f64) -> Self {
        Self::Value(InnerReal(value))
    }
}

impl Add for InnerReal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        InnerReal(self.0 + rhs.0)
    }
}

impl Display for InnerReal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl From<Real> for Types {
    fn from(real: Real) -> Self {
        Types::Real(real)
    }
}