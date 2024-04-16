use std::{cell::RefCell, fmt::{Debug, Display}, ops::Deref, rc::Rc};

mod types;
use types::*;

mod operations;
use operations::*;

mod rules;
use rules::*;

#[derive(Debug, Clone)]
enum InnerExpressions {
    Type(Types),
    Operation(Operation),
}

#[derive(Debug, Clone)]
pub struct Expressions(Rc<RefCell<InnerExpressions>>);

impl Expressions {
    fn new(inner: InnerExpressions) -> Self {
        Self(Rc::new(RefCell::new(inner)))
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.0.as_ref().borrow().deref() {
                InnerExpressions::Type(types) => write!(f, "({:#}: {})", types, types.get_type()),
                InnerExpressions::Operation(operation) => write!(f, "({:#}: {})", operation, operation.solve().get_type()),
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

    fn solve(&self) -> Types {
        match self.0.as_ref().borrow().deref() {
            InnerExpressions::Type(types) => types.solve(),
            InnerExpressions::Operation(opeartion) => opeartion.solve(),
        }
    }
}

impl<T: Into<InnerExpressions>> From<T> for Expressions {
    fn from(t: T) -> Self {
        Expressions::new(t.into())
    }
}

trait Rule: Debug + Display {
    fn apply(&self) -> InnerExpressions;
    fn matches(expression: &Expressions) -> Option<Self> where Self: Sized;
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
        "[<index> | top]"
    }

    fn get_description(&self) -> &'static str {
        "Prints current children or select child by index"
    }

    fn execute(&self, state: &mut State, args: &[&str]) {
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
}

struct TypeCommand {}

impl Command for TypeCommand {
    fn get_name(&self) -> &'static str {
        "type"
    }

    fn get_description(&self) -> &'static str {
        "Prints current equation with types"
    }

    fn execute(&self, state: &mut State, _args: &[&str]) {
        println!("{:#}", state.current);
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
        "[<index>]"
    }

    fn execute(&self, state: &mut State, args: &[&str]) {
        let rules = find_all_rules(&state.selection);

        if let Some(index) = args.first() {
            match index.parse::<usize>() {
                Ok(index) => {    
                    if index < rules.len() {
                        println!("Applying rule: {}", rules[index]);
                        *state.selection.0.borrow_mut() = rules[index].apply();

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
}

static COMMANDS: [&dyn Command; 5] = [
    &HelpCommand {},
    &HistoryCommand {},
    &ChildrenCommand {},
    &TypeCommand {},
    &RulesCommand {},
];

fn main() -> std::io::Result<()> {
    let equation: Expressions = Multiplication::new(Real::new(2.0).into(), Addition::new(Real::new(3.0).into(), Real::new(4.0).into()).into()).into();
    
    let mut state = State {
        history: vec![equation.copy()],
        selection: equation.clone(),
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