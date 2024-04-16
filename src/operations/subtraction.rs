use std::fmt::Display;

use crate::InnerZahl;
use crate::{Expressions, Types, Value};
use super::BinaryOperation;

use super::OperationTrait;

pub trait Sub<Rhs = Self> where {
    type Output;

    fn sub(self, other: Rhs) -> Result<Self::Output, String>;
}

impl<L, R, O> Sub<Value<R>> for Value<L> where
    Value<L>: Into<Expressions>,
    Value<R>: Into<Expressions>,
    Value<O>: Into<Types>,
    L: Sub<R, Output = Value<O>>,
{
    type Output = Types;

    fn sub(self, rhs: Value<R>) -> Result<Self::Output, String> {
        match (self, rhs) {
            (Value::<L>::Constant(lhs), Value::<R>::Constant(rhs)) => Ok((lhs.sub(rhs))?.into()),
            (lhs, rhs) => Ok(Value::<O>::Expression(Subtraction::new(lhs.into(), rhs.into()).into()).into()),
        }
    }
}

pub trait TypeSub {
    fn type_sub(self, right: Types) -> Result<Types, String>;
}

#[derive(Debug, Clone)]
pub struct Subtraction {
    pub left: Expressions,
    pub right: Expressions,
}

impl Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#} - {:#})", self.left, self.right)
        } else {
            write!(f, "({} - {})", self.left, self.right)
        }
    }
}

impl OperationTrait for Subtraction {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Subtraction::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> Result<Types, String> {
        match self.left.solve()? {
            Types::Natural(left) => Ok((left.enlarge::<InnerZahl>().type_sub(self.right.solve()?))?),
            Types::Real(left) => Ok((left.type_sub(self.right.solve()?))?),
            Types::Zahl(left) => Ok((left.type_sub(self.right.solve()?))?),
        }
    }
}

impl BinaryOperation for Subtraction {
    fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}