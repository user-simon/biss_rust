pub mod function;

use std::iter;

pub use function::Function;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum Precedence {
    L6, // ||
    L5, // && ^^
    L4, // == != < <= > >=
    L3, // + -
    L2, // * /
    L1, // ^, functions, unary operators, variables, literals
}

pub enum Expression {
    Literal {
        as_bool: bool,
        value: f64,
    },
    FunctionCall {
        function: &'static Function,
        args: Vec<Expression>,
    },
    Variable {
        identifier: String,
    }
}

impl Expression {
    pub fn precedence(&self) -> Precedence {
        match self {
            Expression::FunctionCall{ function, .. } => function.precedence,
            _ => Precedence::L1,
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match &self {
            Expression::Literal { as_bool, value } => {
                match as_bool {
                    true  => (*value != 0.0).to_string(),
                    false => value.to_string()
                }
            },
            Expression::FunctionCall { function, args } => {
                // TODO operator syntax
                let identifier = function.identifier;
                let args_str = args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{identifier}({args_str})")
            },
            Expression::Variable { identifier } => {
                identifier.clone()
            },
        }
    }
}
