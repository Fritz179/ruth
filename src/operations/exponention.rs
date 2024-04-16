use std::fmt::Display;

use crate::{Expressions, Types, Wrapper};
use super::BinaryOperation;

use super::OperationTrait;

pub trait Exp<Rhs = Self> where {
    type Output;

    fn exp(self, other: Rhs) -> Result<Self::Output, String>;
}

impl<L, R, O> Exp<Wrapper<R>> for Wrapper<L> where
    Wrapper<L>: Into<Expressions>,
    Wrapper<R>: Into<Expressions>,
    Wrapper<O>: Into<Types>,
    O: Into<Wrapper<O>>,
    L: Exp<R, Output = O>,
{
    type Output = Types;

    fn exp(self, rhs: Wrapper<R>) -> Result<Self::Output, String> {
        match (self, rhs) {
            (Wrapper::<L>::Constant(lhs), Wrapper::<R>::Constant(rhs)) => Ok((lhs.exp(rhs))?.into().into()),
            (lhs, rhs) => Ok(Wrapper::<O>::Expression(Exponentiation::new(lhs.into(), rhs.into()).into()).into()),
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
            Types::Zahl(left) => Ok((left.type_exp(self.right.solve()?))?),
        }
    }
}

impl BinaryOperation for Exponentiation {
    fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}