use std::fmt::Display;

use crate::{Expressions, Operation, Types};

use super::OperationTrait;

pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(self, other: Rhs) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Multiplication {
    pub left: Expressions,
    pub right: Expressions,
}

impl Multiplication {
    pub fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}

impl From<Multiplication> for Operation {
    fn from(multiplication: Multiplication) -> Self {
        Operation::Multiplication(multiplication)
    }
}

impl Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#} * {:#})", self.left, self.right)
        } else {
            write!(f, "({} * {})", self.left, self.right)
        }
    }
}

impl OperationTrait for Multiplication {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Multiplication::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> crate::Types {
        match (self.left.solve(), self.right.solve()) {
            (Types::Real(left), Types::Real(right)) => (left.mul(right)).into(),
        }
    }
}