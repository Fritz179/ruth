use crate::{operations::Exponentiation, Addition, Expressions, InnerExpressions, Multiplication, Rule};

use super::operations::OperationTrait;

static DISTRIBUTIVITY: Rule = Rule {
    matches: &|expression: &Expressions| {
        let mul = expression.is_multiplication()?;
        let add = mul.right.is_addition()?;

        Some(Addition {
            left: Multiplication::new(mul.left.clone(), add.right).into(), 
            right: Multiplication::new(mul.left, add.left).into()
        }.into())
    },
    name: "Distributivity",
    description: "x * (a + b) = x * a + x * b"
};

// static EXPONENT_TO_MULTIPLICATION: Rule = Rule {
//     matches: &|expression: &Expressions| {
//         let exp = expression.is_exponentiation()?;

//         Some(Exponentiation {
//             left: exp.left, 
//             right: ::new(mul.left, add.left).into()
//         }.into())
//     },
//     name: "EXPONENT_TO_MULTIPLICATION",
//     description: "(a + b) ** x = (a + b) * (a + b) ** (x - 1))"
// };

static CONST_EVALUATION: Rule = Rule {
    matches: &|expression: &Expressions| {
        let result = expression.is_operation()?.solve().unwrap();

        if result.is_value() {
            Some(result.clone().into())
        } else {
            None
        }
    },
    name: "Constant Evaluation",
    description: "1 + 1 = 2"
};

static RULES: [&Rule; 2] = [
    &DISTRIBUTIVITY, 
    &CONST_EVALUATION
];

pub struct Match {
    pub rule: &'static Rule,
    pub result: InnerExpressions
}

impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}: {}", self.rule.name, self.rule.description, self.result)
    }
}

impl Match {
    pub fn result(&self) -> InnerExpressions {
        // TODO: Should not clone
        self.result.clone()
    }
}

pub fn find_all_rules(expression: &Expressions) -> Vec<Match> {
    let mut rules: Vec<Match> = vec![];

    for rule in RULES.iter() {
        if let Some(result) = (rule.matches)(expression) {
            rules.push(Match {
                rule,
                result
            });
        }
    }

    rules
}