mod natural;
pub use natural::*;

mod zahl;
pub use zahl::*;

mod real;
pub use real::*;

use std::fmt::{Debug, Display};

use crate::{Expressions, InnerExpressions};

#[derive(Debug, Clone)]
pub enum Value<T> {
    Constant(T),
    Variable(String),
    Expression(Expressions),
}

impl<T> Value<T> {
    pub fn new_variable(name: &str) -> Self {
        Value::Variable(name.to_string())
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Value::Constant(_))
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, Value::Constant(_))
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

impl<T, O> MyFrom<Value<T>> for Value<O> where T: MyInto<O> {
    fn my_from(from: Value<T>) -> Self {
        match from {
            Value::Constant(inner) => Value::Constant(inner.my_into()),
            Value::Variable(name) => Value::Variable(name.clone()),
            Value::Expression(expression) => Value::Expression(expression.clone()),
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(t: T) -> Self {
        Value::Constant(t)
    }
}

#[derive(Debug, Clone)]
pub enum Types {
    Natural(Natural),
    Zahl(Zahl),
    Real(Real),
}

impl<T: Display> Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::<T>::Constant(inner) => Display::fmt(&inner, f),
            Value::<T>::Variable(name) => Display::fmt(&name, f),
            Value::<T>::Expression(expression) => Display::fmt(&expression, f),
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
            Types::Natural(natural) => natural.get_type(),
            Types::Zahl(zahl) => zahl.get_type(),
            Types::Real(real) => real.get_type(),
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Types::Natural(natural) => natural.is_value(),
            Types::Zahl(zahl) => zahl.is_value(),
            Types::Real(real) => real.is_value(),
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            Types::Natural(natural) => natural.is_variable(),
            Types::Zahl(zahl) => zahl.is_variable(),
            Types::Real(real) => real.is_variable(),
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

impl From<Types> for InnerExpressions {
    fn from(t: Types) -> Self {
        InnerExpressions::Type(t)
    }
}

impl<T> From<Value<T>> for InnerExpressions where Value<T>: Into<Types> {
    fn from(real: Value<T>) -> Self {
        InnerExpressions::Type(real.into())
    }
}