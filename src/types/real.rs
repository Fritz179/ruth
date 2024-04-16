use std::fmt::Display;

use crate::{Add, Addition, BinaryOperation, InnerExpressions, Mul, Multiplication, Types, Value};

pub type Real = Value<f64>;

impl Real {
    pub fn new(value: f64) -> Self {
        Self::Value(value)
    }

    pub fn get_type(&self) -> &str {
        "Real"
    }
}

impl Add for Real {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Real::Value(lhs), Real::Value(rhs)) => Real::Value(lhs + rhs),
            (lhs, rhs) => Real::Variable(format!("{}", Addition::new(lhs.into(), rhs.into()))),
        }
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Real::Value(lhs), Real::Value(rhs)) => Real::Value(lhs * rhs),
            (lhs, rhs) => Real::Variable(format!("{}", Multiplication::new(lhs.into(), rhs.into()))),
        }
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Real::Value(value) => write!(f, "{}", value),
            Real::Variable(name) => write!(f, "{}", name),
        }
    }
}

impl From<Real> for Types {
    fn from(real: Real) -> Self {
        Types::Real(real)
    }
}

impl From<Real> for InnerExpressions {
    fn from(real: Real) -> Self {
        InnerExpressions::Type(Types::Real(real))
    }
}