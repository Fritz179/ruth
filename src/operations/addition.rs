use std::fmt::Display;

use crate::{BinaryOperation, Expressions, Operation, Types};

use super::OperationTrait;

pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, other: Rhs) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Addition {
    pub left: Expressions,
    pub right: Expressions,
}

impl From<Addition> for Operation {
    fn from(addition: Addition) -> Self {
        Operation::Addition(addition)
    }
}

impl Display for Addition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#} + {:#})", self.left, self.right)
        } else {
            write!(f, "({} + {})", self.left, self.right)
        }
    }
}

impl OperationTrait for Addition {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Addition::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> Types {
        match (self.left.solve(), self.right.solve()) {
            (Types::Real(left), Types::Real(right)) => (left.add(right)).into(),
        }
    }
}

impl BinaryOperation for Addition {
    fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}