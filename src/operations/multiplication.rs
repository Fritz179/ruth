use std::fmt::Display;

use crate::{Expression, Expressions, InnerExpressions};

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

impl From<Multiplication> for Expressions {
    fn from(multiplication: Multiplication) -> Self {
        Expressions::new(InnerExpressions::Multiplication(multiplication))
    }
}

impl Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.left, self.right)
    }
}

impl Expression for Multiplication {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Multiplication::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> crate::Types {
        todo!()
    }
}