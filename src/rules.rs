use std::{fmt::Display, ops::Deref};

use crate::{Addition, Expressions, InnerExpressions, Multiplication, Rule};

#[derive(Debug, Clone)]
pub struct Distributivity {
    expression: Expressions,
    addition: Addition
}

impl Display for Distributivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Distributivity({}, {})", self.expression, self.addition)
    }
}

impl Rule for Distributivity {
    // fn get_children(&self) -> Vec<Expressions> {
    //     vec![self.expression.clone(), Expressions(InnerExpressions::Addition(self.addition.clone()))]
    // }

    fn matches(expression: &Expressions) -> Option<Self> {
        if let InnerExpressions::Multiplication(multiplication) = expression.0.as_ref().borrow().deref() {
            if let InnerExpressions::Addition(ref addition) = multiplication.right.0.as_ref().borrow().deref() {
                return Some(Distributivity {
                    expression: multiplication.left.clone(),
                    addition: addition.clone()
                });
            }
        }

        None
    }

    fn apply(&self) -> InnerExpressions {
        let a = self.expression.clone();
        let b = self.addition.left.clone();
        let c = self.addition.right.clone();

        InnerExpressions::Addition(Addition {
            left: Multiplication::new(a.clone(), b).into(), 
            right: Multiplication::new(a, c).into()
        })
    }
}