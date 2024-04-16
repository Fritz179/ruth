use std::fmt::Display;

use crate::{Expressions, Types, Value};
use super::BinaryOperation;

use super::OperationTrait;

pub trait Exp<Rhs = Self> where {
    type Output;

    fn exp(self, other: Rhs) -> Result<Self::Output, String>;
}

impl<L, R, O> Exp<Value<R>> for Value<L> where
    Value<L>: Into<Expressions>,
    Value<R>: Into<Expressions>,
    Value<O>: Into<Types>,
    L: Exp<R, Output = Value<O>>,
{
    type Output = Types;

    fn exp(self, rhs: Value<R>) -> Result<Self::Output, String> {
        match (self, rhs) {
            (Value::<L>::Constant(lhs), Value::<R>::Constant(rhs)) => Ok((lhs.exp(rhs))?.into()),
            (lhs, rhs) => Ok(Value::<O>::Expression(Exponentiation::new(lhs.into(), rhs.into()).into()).into()),
        }
    }
}

pub trait TypeExp {
    fn type_exp(self, right: Types) -> Result<Types, String>;
}

#[derive(Debug, Clone)]
pub struct Exponentiation {
    pub left: Expressions,
    pub right: Expressions,
}

impl Display for Exponentiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#} ** {:#})", self.left, self.right)
        } else {
            write!(f, "({} ** {})", self.left, self.right)
        }
    }
}

impl OperationTrait for Exponentiation {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Exponentiation::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> Result<Types, String> {
        match self.left.solve()? {
            Types::Real(left) => Ok((left.type_exp(self.right.solve()?))?),
            Types::Natural(left) => Ok((left.type_exp(self.right.solve()?))?),
        }
    }
}

impl BinaryOperation for Exponentiation {
    fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}