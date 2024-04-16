mod addition;
use std::fmt::Display;

pub use addition::*;

mod multiplication;
pub use multiplication::*;

use crate::{Expressions, InnerExpressions};

pub trait OperationTrait: Into<Operation> + Display {
    fn get_children(&self) -> Vec<Expressions>;
    fn copy(&self) -> Expressions;
    fn solve(&self) -> crate::Types;
}

pub trait BinaryOperation: OperationTrait {
    fn new(left: Expressions, right: Expressions) -> Self;
}

impl<T: Into<Operation>> From<T> for InnerExpressions {
    fn from(operation: T) -> Self {
        InnerExpressions::Operation(operation.into())
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Addition(Addition),
    Multiplication(Multiplication),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Addition(addition) => Display::fmt(&addition, f),
            Operation::Multiplication(multiplication) => Display::fmt(&multiplication, f),
        }
    }
}

impl OperationTrait for Operation {
    fn copy(&self) -> Expressions {
        match self {
            Operation::Addition(addition) => addition.copy(),
            Operation::Multiplication(multiplication) => multiplication.copy(),
        }
    }

    fn get_children(&self) -> Vec<Expressions> {
        match self {
            Operation::Addition(addition) => addition.get_children(),
            Operation::Multiplication(multiplication) => multiplication.get_children(),
        }
    }

    fn solve(&self) -> crate::Types {
        match self {
            Operation::Addition(addition) => addition.solve(),
            Operation::Multiplication(multiplication) => multiplication.solve(),
        }
    }
}