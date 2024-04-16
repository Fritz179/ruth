use crate::{Addition, Expressions, InnerExpressions, Multiplication, OperationTrait, Rule};

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

static RULES: [&'static Rule; 2] = [
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
                rule: *rule,
                result
            });
        }
    }

    rules
}