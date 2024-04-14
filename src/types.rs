#[derive(Debug, Clone)]
pub enum Value<T: Display + Clone + Debug> {
    Value(T),
    Variable(String),
}

impl<T: Display + Clone + Debug> Value<T> where Value<T>: Into<Types> {
    pub fn new_variable(name: &str) -> Self {
        Self::Variable(name.to_string())
    }
}

impl<T: Display + Clone + Debug> Expression for Value<T> where Value<T>: Into<Types> {
    fn get_children(&self) -> Vec<Expressions> {
        vec![]
    }

    fn copy(&self) -> Expressions {
        match self {
            Value::Value(value) => <Value<T> as Into<Types>>::into(Self::Value(value.clone())).into(),
            Value::Variable(name) => <Value<T> as Into<Types>>::into(Self::new_variable(name)).into(),
        }
    }

    fn solve(&self) -> Types {
        self.clone().into()
    }
}

impl<T: Display + Clone + Debug> Display for Value<T> where Value<T>: Into<Types>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(value) => Display::fmt(value, f),
            Value::Variable(name) => write!(f, "{}", name),
        }
    }
}

// TODO: don't?
impl<A: Display + Clone + Debug, B: Display + Clone + Debug, C: Display + Clone + Debug> Add<Value<B>> for Value<A> 
    where 
        A: Add<B, Output = C>, 
        Value<A>: Into<Types>, 
        Value<B>: Into<Types>,
        Value<C>: Into<Types>,
{
    type Output = Value<C>;

    fn add(self, rhs: Value<B>) -> Self::Output {
        match (self, rhs) {
            // Constant propagation
            (Value::Value(left), Value::Value(right)) => {
                let sum = left.add(right);
                Value::Value(sum)
            },

            // Type Propagation
            (lhs, rhs) => {
                let addition = Addition::new(lhs.into().into(), rhs.into().into());
                Value::<C>::new_variable(&format!("{}", addition))
            }
        }
    }

}

mod real;
pub use real::Real;

use std::fmt::{Debug, Display};

use crate::{Add, Addition, Expression, Expressions, InnerExpressions};

#[derive(Debug, Clone)]
pub enum Types {
    Real(Real),
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Real(real) => Display::fmt(&real, f),
        }
    }
}

impl Expression for Types {
    fn get_children(&self) -> Vec<Expressions> {
        match self {
            Types::Real(real) => real.get_children(),
        }
    }

    fn copy(&self) -> Expressions {
        match self {
            Types::Real(real) => real.copy(),
        }
    }

    fn solve(&self) -> Types {
        match self {
            Types::Real(real) => real.solve(),
        }
    }
}

impl<T: Into<Types>> From<T> for Expressions {
    fn from(t: T) -> Self {
        Expressions::new(InnerExpressions::Type(t.into()))
    }
}