mod addition;
use std::fmt::Display;

pub use addition::*;

mod multiplication;
use enum_dispatch::enum_dispatch;
pub use multiplication::*;

use crate::{Expressions, InnerExpressions, Types};

#[enum_dispatch]
pub trait OperationTrait: Into<Operation> + Display {
    fn get_children(&self) -> Vec<Expressions>;
    fn copy(&self) -> Expressions;
    fn solve(&self) -> Result<Types, String>;
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
#[enum_dispatch(OperationTrait)]
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
