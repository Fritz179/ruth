use std::fmt::Display;

use crate::{Expression, Expressions, InnerExpressions, Types, Add};

#[derive(Debug, Clone)]
pub struct Addition {
    pub left: Expressions,
    pub right: Expressions,
}

impl Addition {
    pub fn new(left: Expressions, right: Expressions) -> Self {
        Self { left, right }
    }
}

impl From<Addition> for Expressions {
    fn from(addition: Addition) -> Self {
        Expressions::new(InnerExpressions::Addition(addition))
    }
}

impl Display for Addition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.left, self.right)
    }
}

impl Expression for Addition {
    fn get_children(&self) -> Vec<Expressions> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn copy(&self) -> Expressions {
        Addition::new(self.left.copy(), self.right.copy()).into()
    }

    fn solve(&self) -> Types {
        match (self.left.solve(), self.right.solve()) {
            (Types::Real(left), Types::Real(right)) => (left.add(right)).into(),
            _ => panic!("Invalid addition")
        }
    }
}