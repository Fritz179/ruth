use std::fmt::Display;

use crate::{Expressions, Types, Value};

use super::OperationTrait;

pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(self, other: Rhs) -> Result<Self::Output, String>;
}

impl<L, R, O> Mul<Value<R>> for Value<L> where
    Value<L>: Into<Expressions>,
    Value<R>: Into<Expressions>,
    Value<O>: Into<Types>,
    L: Mul<R, Output = Value<O>>,
{
    type Output = Types;

    fn mul(self, rhs: Value<R>) -> Result<Self::Output, String> {
        match (self, rhs) {
            (Value::<L>::Constant(lhs), Value::<R>::Constant(rhs)) => Ok((lhs.mul(rhs))?.into()),
            (lhs, rhs) => Ok(Value::<O>::Expression(Multiplication::new(lhs.into(), rhs.into()).into()).into()),
        }
    }
}

pub trait TypeMul {
    fn type_mul(self, right: Types) -> Result<Types, String>;
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

    fn solve(&self) -> Result<Types, String> {
        match self.left.solve()? {
            Types::Real(left) => Ok((left.type_mul(self.right.solve()?))?),
            Types::Natural(left) => Ok((left.type_mul(self.right.solve()?))?),
        }
    }
}