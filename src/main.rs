use std::{cell::RefCell, fmt::{Debug, Display}, ops::Deref, rc::Rc};

mod types;
use types::*;

pub mod operations;
use operations::{Addition, BinaryOperation, Exponentiation, Multiplication, Operation, OperationTrait};

mod rules;

#[derive(Debug, Clone)]
enum InnerExpressions {
    Type(Types),
    Operation(Operation),
}

impl Display for InnerExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerExpressions::Type(types) => Display::fmt(&types, f),
            InnerExpressions::Operation(operation) => Display::fmt(&operation, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expressions(Rc<RefCell<InnerExpressions>>);

impl Expressions {
    fn new(inner: InnerExpressions) -> Self {
        Self(Rc::new(RefCell::new(inner)))
    }

    fn is_operation(&self) -> Option<Operation> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Operation(operation) => Some(operation.clone()),
            _ => None,
        }
    }

    fn is_addition(&self) -> Option<Addition> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Operation(Operation::Addition(addition)) => Some(addition.clone()),
            _ => None,
        }
    }

    fn is_multiplication(&self) -> Option<Multiplication> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Operation(Operation::Multiplication(multiplication)) => Some(multiplication.clone()),
            _ => None,
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.0.as_ref().borrow().deref() {
                InnerExpressions::Type(types) => write!(f, "({:#}: {})", types, types.get_type()),
                InnerExpressions::Operation(operation) => write!(f, "({:#}: {})", operation, operation.solve().unwrap().get_type()),
            }
        } else {
            match self.0.as_ref().borrow().deref() {
                InnerExpressions::Type(types) => write!(f, "{}", types),
                InnerExpressions::Operation(operation) => write!(f, "{}", operation),
            }
        }
    }
}

impl Expressions {
    fn get_children(&self) -> Vec<Expressions> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Type(types) => types.get_children(),
            InnerExpressions::Operation(operation) => operation.get_children(),
        }
    }

    fn copy(&self) -> Expressions {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Type(types) => types.copy(),
            InnerExpressions::Operation(opeartion) => opeartion.copy(),
        }
    }

    fn solve(&self) -> Result<Types, String> {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Type(types) => Ok(types.solve()),
            InnerExpressions::Operation(opeartion) => opeartion.solve(),
        }
    }
}

impl<T: Into<InnerExpressions>> From<T> for Expressions {
    fn from(t: T) -> Self {
        Expressions::new(t.into())
    }
}

struct Rule {
    matches: &'static dyn Fn(&Expressions) -> Option<InnerExpressions>,
    name: &'static str,
    description: &'static str,
}

unsafe impl Send for Rule {}
unsafe impl Sync for Rule {}

struct State {
    // Current working equation
    current: Expressions,

    // Selction of current equation
    selection: Expressions,

    // History of equations, just copies
    history: Vec<Expressions>,
}

struct Command {
    name: &'static str,
    description: &'static str,
    usage: &'static str,

    execute: &'static dyn Fn(&mut State, &[&str]),
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}

static HELP_COMMAND: Command = Command {
    name: "help",
    description: "Prints this help page",
    usage: "",

    execute: &|_state: &mut State, _args: &[&str]| {
        println!("Commands:");
        for command in COMMANDS.iter() {
            let start = format!("{} {}", command.name, command.usage);
            println!("    {:20}: {}", start, command.description);
        }
    },
};

static HISTORY_COMMAND: Command = Command {
    name: "history",
    description: "Prints all equations history",
    usage: "",

    execute: &|state: &mut State, _args: &[&str]| {
        println!("History:");
        for point in state.history.iter() {
            println!("{}", point);
        }
    },
};

static CHILDREN_COMMAND: Command = Command {
    name: "children",
    description: "Prints current children or select child by index",
    usage: "[<index> | top]",

    execute: &|state: &mut State, args: &[&str]| {
        let children = state.selection.get_children();

        if let Some(index) = args.first() {
            if index == &"top" {
                state.selection = state.current.clone();
                return;
            }

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
};

static TYPE_COMMAND: Command = Command {
    name: "type",
    description: "Prints current equation with types",
    usage: "",

    execute: &|state: &mut State, _args: &[&str]| {
        println!("{:#}", state.selection);
    }
};

static RULES_COMMAND: Command = Command {
    name: "rules",
    description: "Prints all rules",
    usage: "[<index>]",

    execute: &|state: &mut State, args: &[&str]| {
        let rules = rules::find_all_rules(&state.selection);

        if let Some(index) = args.first() {
            match index.parse::<usize>() {
                Ok(index) => {    
                    if index < rules.len() {
                        println!("Applying rule: {}", rules[index]);
                        *state.selection.0.borrow_mut() = rules[index].result();

                        state.history.push(state.current.clone());

                        // detach from history
                        state.current = state.current.copy();
                        state.selection = state.current.clone();
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
};

static EXIT_COMMAND: Command = Command {
    name: "exit | quit | q",
    description: "Exits the program",
    usage: "",

    execute: &|_state: &mut State, _args: &[&str]| {
        std::process::exit(0);
    }
};

static COMMANDS: [&Command; 6] = [
    &HELP_COMMAND,
    &HISTORY_COMMAND,
    &CHILDREN_COMMAND,
    &TYPE_COMMAND,
    &RULES_COMMAND,
    &EXIT_COMMAND,
];

fn main() {
    // let equation: Expressions = Multiplication::new(Natural::new(2).into(), Addition::new(Real::new(3.0).into(), Real::new(4.0).into()).into()).into();
    let equation: Expressions = Exponentiation::new(
        Addition::new(Real::new_variable("b").into(), Real::new_variable("c").into()).into(),
        Natural::new_variable("a").into(), 
    ).into();

    let mut state = State {
        history: vec![equation.copy()],
        selection: equation.clone(),
        current: equation,
    };

    (HELP_COMMAND.execute)(&mut state, &[]);

    'outer: loop {
        println!();
        println!();
        println!("Equation: {}", state.current);
        println!("Selection: {}", state.selection);


        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
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
            if command.name == command_name {
                (command.execute)(&mut state, &args);
                continue 'outer;
            }
        }

        if !command_name.is_empty() {
            println!("Unknown command: {}", command_name);
            println!("enter <help> for help")
        }
    }
}