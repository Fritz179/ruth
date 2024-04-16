mod natural;
use enum_dispatch::enum_dispatch;
pub use natural::*;

mod zahl;
pub use zahl::*;

mod real;
pub use real::*;

use std::fmt::{Debug, Display};

use crate::{Expressions, InnerExpressions};

#[enum_dispatch]
pub trait TypeTrait {
    fn is_value(&self) -> bool;
    fn is_variable(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Wrapper<T> {
    Constant(T),
    Variable(String),
    Expression(Expressions),
}

impl<T> Wrapper<T> {
    pub fn new_variable(name: &str) -> Self {
        Wrapper::Variable(name.to_string())
    }
}

impl<T> TypeTrait for Wrapper<T> {
    fn is_value(&self) -> bool {
        matches!(self, Wrapper::Constant(_))
    }

    fn is_variable(&self) -> bool {
        matches!(self, Wrapper::Constant(_))
    }
}

pub trait MyFrom<From> {
    fn my_from(from: From) -> Self;
}

pub trait MyInto<Into> {
    fn my_into(self) -> Into;
}

impl<From, Into> MyInto<Into> for From where Into: MyFrom<From> {
    fn my_into(self) -> Into {
        Into::my_from(self)
    }
} 

impl<T, O> MyFrom<Wrapper<T>> for Wrapper<O> where T: MyInto<O> {
    fn my_from(from: Wrapper<T>) -> Self {
        match from {
            Wrapper::Constant(inner) => Wrapper::Constant(inner.my_into()),
            Wrapper::Variable(name) => Wrapper::Variable(name.clone()),
            Wrapper::Expression(expression) => Wrapper::Expression(expression.clone()),
        }
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch(TypeTrait)]
pub enum Types {
    Natural(WrappedNatural),
    Zahl(WrappedZahl),
    Real(WrappedReal),
}

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wrapper::<T>::Constant(inner) => Display::fmt(&inner, f),
            Wrapper::<T>::Variable(name) => Display::fmt(&name, f),
            Wrapper::<T>::Expression(expression) => Display::fmt(&expression, f),
        }
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Real(real) => Display::fmt(&real, f),
            Types::Natural(natural) => Display::fmt(&natural, f),
            Types::Zahl(zahl) => Display::fmt(&zahl, f),
        }
    }
}

impl Types {
    pub fn get_type(&self) -> &str {
        match self {
            Types::Real(wrapped) => wrapped.get_type(),
            Types::Natural(wrapped) => wrapped.get_type(),
            Types::Zahl(wrapped) => wrapped.get_type(),
        }
    }

    pub fn get_children(&self) -> Vec<Expressions> {
        vec![]
    }

    pub fn copy(&self) -> Expressions {
        self.clone().into()
    }

    pub fn solve(&self) -> Types {
        self.clone()
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(t: T) -> Self {
        Wrapper::Constant(t)
    }
}

// impl<T> From<T> for Types where 
//     T: Into<Wrapper<T>>,
//     Wrapper<T>: Into<Types>,
// {
//     fn from(t: T) -> Self {
//         t.into().into()
//     }
// }

// impl<T: Into<Wrapper<T>>> From<T> for Expressions where Wrapper<T>: Into<Types> {
//     fn from(t: T) -> Self {
//         Expressions::new(InnerExpressions::Type(t.into().into()))
//     }
// }

// impl From<Types> for InnerExpressions {
//     fn from(t: Types) -> Self {
//         InnerExpressions::Type(t)
//     }
// }

// impl<T: Into<Wrapper<T>>> From<T> for Types where Wrapper<T>: Into<Types> {
//     fn from(real: T) -> Self {
//         Wrapper::Constant(real).into()
//     }
// }

impl From<Types> for InnerExpressions {
    fn from(value: Types) -> Self {
        InnerExpressions::Type(value)
    }
}

impl<T> From<Wrapper<T>> for InnerExpressions where Wrapper<T>: Into<Types> {
    fn from(real: Wrapper<T>) -> Self {
        InnerExpressions::Type(real.into())
    }
}