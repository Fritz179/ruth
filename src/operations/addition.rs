use std::fmt::Display;

use crate::{Expressions, Types, Value};
use super::BinaryOperation;

use super::OperationTrait;

pub trait Add<Rhs = Self> where {
    type Output;

    fn add(self, other: Rhs) -> Result<Self::Output, String>;
}

impl<L, R, O> Add<Value<R>> for Value<L> where
    Value<L>: Into<Expressions>,
    Value<R>: Into<Expressions>,
    Value<O>: Into<Types>,
    L: Add<R, Output = Value<O>>,
{
    type Output = Types;

    fn add(self, rhs: Value<R>) -> Result<Self::Output, String> {
        match (self, rhs) {
            (Value::<L>::Constant(lhs), Value::<R>::Constant(rhs)) => Ok((lhs.add(rhs))?.into()),
            (lhs, rhs) => Ok(Value::<O>::Expression(Addition::new(lhs.into(), rhs.into()).into()).into()),
        }
    }
}

pub trait TypeAdd {
    fn type_add(self, right: Types) -> Result<Types, String>;
}

#[derive(Debug, Clone)]
pub struct Addition {
    pub left: Expressions,
    pub right: Expressions,
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

    fn solve(&self) -> Result<Types, String> {
        match self.left.solve()? {
            Types::Real(left) => Ok((left.type_add(self.right.solve()?))?),
            Types::Natural(left) => Ok((left.type_add(self.right.solve()?))?),
            Types::Zahl(left) => Ok((left.type_add(self.right.solve()?))?),
        }
    }
}

impl BinaryOperation for Addition {
    fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}