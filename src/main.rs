#![allow(clippy::new_ret_no_self)]

use std::{cell::RefCell, fmt::{Debug, Display}, ops::{Add, Deref}, rc::Rc};

#[derive(Debug, Clone, Copy)]
struct Real {
    value: f64,
}

impl Real {
    fn new(value: f64) -> Expressions {
       Expressions(Rc::new(RefCell::new(InnerExpressions::Real(Real { value }))))
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Real {
    type Output = Expressions;

    fn add(self, rhs: Self) -> Self::Output {
        Real::new(self.value + rhs.value)
    }
}

impl Expression for Real {
    fn get_children(&self) -> Vec<Expressions> {
        vec![]
    }

    fn copy(&self) -> Expressions {
        Real::new(self.value)
    }
}

#[derive(Debug, Clone)]
struct Addition {
    left: Expressions,
    right: Expressions,
}

impl Addition {
    fn new(left: Expressions, right: Expressions) -> Expressions {
        Expressions(Rc::new(RefCell::new(InnerExpressions::Addition(Self { left, right }))))
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
        Addition::new(self.left.copy(), self.right.copy())
    }
}

trait Expression: Display + Debug + Clone {
    fn get_children(&self) -> Vec<Expressions>;
    fn copy(&self) -> Expressions;
}

#[derive(Debug, Clone)]
struct Multiplication {
    left: Expressions,
    right: Expressions,
}

impl Multiplication {
    fn new(left: Expressions, right: Expressions) -> Expressions {
        Expressions(Rc::new(RefCell::new(InnerExpressions::Multiplication(Self { left, right }))))
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
        Multiplication::new(self.left.copy(), self.right.copy())
    }
}

#[derive(Debug, Clone)]
enum InnerExpressions {
    Real(Real),
    Addition(Addition),
    Multiplication(Multiplication),
}

#[derive(Debug, Clone)]
struct Expressions(Rc<RefCell<InnerExpressions>>);

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Real(real) => Display::fmt(&real, f),
            InnerExpressions::Addition(addition) => Display::fmt(&addition, f),
            InnerExpressions::Multiplication(multiplication) => Display::fmt(&multiplication, f),
        }
    }
}

impl Expression for Expressions {
    fn get_children(&self) -> Vec<Expressions> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Real(real) => real.get_children(),
            InnerExpressions::Addition(addition) => addition.get_children(),
            InnerExpressions::Multiplication(multiplication) => multiplication.get_children(),
        }
    }

    fn copy(&self) -> Expressions {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Real(real) => real.copy(),
            InnerExpressions::Addition(addition) => addition.copy(),
            InnerExpressions::Multiplication(multiplication) => multiplication.copy(),
        }
    }
}

/// RULES

trait Rule: Debug + Display {
    fn apply(&self) -> InnerExpressions;
    fn matches(expression: &Expressions) -> Option<Self> where Self: Sized;
    // fn get_children(&self) -> Vec<Expressions>;
}

#[derive(Debug, Clone)]
struct Distributivity {
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

        InnerExpressions::Addition(Addition {left: Multiplication::new(a.clone(), b), right: Multiplication::new(a, c)})
    }
}

fn find_all_rules(expression: &Expressions) -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];

    if let Some(rule) = Distributivity::matches(expression) {
        rules.push(Box::new(rule));
    }

    rules
}

struct State {
    // Current working equation
    current: Expressions,

    // Selction of current equation
    selection: Expressions,

    // History of equations, just copies
    history: Vec<Expressions>,
}

trait Command: Send + Sync {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
    fn get_usage(&self) -> &'static str {
        ""
    }

    fn execute(&self, state: &mut State, args: &[&str]);
}

struct HelpCommand {}

impl Command for HelpCommand {
    fn get_name(&self) -> &'static str {
        "help"
    }

    fn get_description(&self) -> &'static str {
        "Prints this help page"
    }

    fn execute(&self, _state: &mut State, _args: &[&str]) {
        println!("Commands:");
        for command in COMMANDS.iter() {
            let start = format!("{} {}", command.get_name(), command.get_usage());
            println!("    {:20}: {}", start, command.get_description());
        }
        println!("exit: Exits the program")
    }
}

struct HistoryCommand {}

impl Command for HistoryCommand {
    fn get_name(&self) -> &'static str {
        "history"
    }

    fn get_description(&self) -> &'static str {
        "Prints all equations history"
    }

    fn execute(&self, state: &mut State, _args: &[&str]) {
        println!("History:");
        for point in state.history.iter() {
            println!("{}", point);
        }
    }
}

struct ChildrenCommand {}

impl Command for ChildrenCommand {
    fn get_name(&self) -> &'static str {
        "children"
    }

    fn get_usage(&self) -> &'static str {
        "[index]"
    }

    fn get_description(&self) -> &'static str {
        "Prints current children or select child by index"
    }

    fn execute(&self, state: &mut State, args: &[&str]) {
        let children = state.selection.get_children();

        if let Some(index) = args.first() {
            match index.parse::<usize>() {
                Ok(index) => {    
                    if index < children.len() {
                        state.selection = children[index].clone();
                    } else {
                        println!("Index: {index} out of range: {}", children.len());
                    }
                },
                Err(error) =>{
                    println!("Invalid index: {error}");
                }
            }
        } else {
            println!("Current children");
            for (i, child) in children.iter().enumerate() {
                println!("{i}: {}", child);
            }
        }
    }
}

struct RulesCommand {}

impl Command for RulesCommand {
    fn get_name(&self) -> &'static str {
        "rules"
    }

    fn get_description(&self) -> &'static str {
        "Prints all rules"
    }

    fn get_usage(&self) -> &'static str {
        "[index]"
    }

    fn execute(&self, state: &mut State, args: &[&str]) {
        let rules = find_all_rules(&state.selection);

        if let Some(index) = args.first() {
            match index.parse::<usize>() {
                Ok(index) => {    
                    if index < rules.len() {
                        println!("Applying rule: {}", rules[index]);
                        *state.selection.0.borrow_mut() = rules[index].apply();

                        let copy = state.current.copy();

                        state.selection = copy.clone();
                        state.history.push(copy);
                    } else {
                        println!("Index: {index} out of range: {}", rules.len());
                    }
                },
                Err(error) =>{
                    println!("Invalid index: {error}");
                }
            }
        } else {
            println!("Current applicable rules");
            for (i, rule) in rules.iter().enumerate() {
                println!("{i}: {}", rule);
            }
        }
    }
}

static COMMANDS: [&dyn Command; 4] = [
    &HelpCommand {},
    &HistoryCommand {},
    &ChildrenCommand {},
    &RulesCommand {},
];

fn main() -> std::io::Result<()> {
    let equation = Multiplication::new(Real::new(2.0), Addition::new(Real::new(3.0), Real::new(4.0)));
    
    let mut state = State {
        selection: equation.clone(),
        history: vec![equation.copy()],
        current: equation,
    };

    HelpCommand{}.execute(&mut state, &[]);

    'outer: loop {
        println!();
        println!();
        println!("Equation: {}", state.current);
        println!("Selection: {}", state.selection);


        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut input = input.split_whitespace();

        let command_name = input.next().unwrap_or("");
        let args: Vec<&str> = input.collect();

        println!();
        println!();
        println!("Equation: {}", state.current);
        println!("Selection: {}", state.selection);
        println!("Command: {}", command_name);
        println!();

        for command in COMMANDS.iter() {
            if command.get_name() == command_name {
                command.execute(&mut state, &args);
                continue 'outer;
            }
        }

        if command_name == "exit" || command_name == "quit" || command_name == "q" {
            return Ok(());
        } else if !command_name.is_empty() {
            println!("Unknown command: {}", command_name);
            println!("enter <help> for help")
        }
    }
}