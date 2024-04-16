use std::{fmt::Display, ops::Deref};

use crate::{Addition, Expressions, InnerExpressions, Multiplication, Operation, OperationTrait, Rule, Types};

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
    fn matches(expression: &Expressions) -> Option<Self> {
        if let InnerExpressions::Operation(Operation::Multiplication(multiplication)) = expression.0.as_ref().borrow().deref() {
            if let InnerExpressions::Operation(Operation::Addition(ref addition)) = multiplication.right.0.as_ref().borrow().deref() {
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

        Addition {
            left: Multiplication::new(a.clone(), b).into(), 
            right: Multiplication::new(a, c).into()
        }.into()
    }
}

#[derive(Debug, Clone)]
pub struct ConstEvaluation {
    result: Types
}

impl Display for ConstEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstEvaluation({})", self.result)
    }
}

impl Rule for ConstEvaluation {
    fn matches(expression: &Expressions) -> Option<Self> {
        if let InnerExpressions::Operation(operation) = expression.0.as_ref().borrow().deref() {
            let result = operation.solve().unwrap();

            if result.is_value() {
                return Some(ConstEvaluation {
                    result: result.clone()
                });
            }
        }

        None
    }

    fn apply(&self) -> InnerExpressions {
        self.result.clone().into()
    }
}

pub fn find_all_rules(expression: &Expressions) -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];

    if let Some(rule) = Distributivity::matches(expression) {
        rules.push(Box::new(rule));
    }

    if let Some(rule) = ConstEvaluation::matches(expression) {
        rules.push(Box::new(rule));
    }

    rules
}