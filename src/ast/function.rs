use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

use crate::ast;

pub enum Associativity {
    Left,
    Right,
}

pub enum Variant {
    Routine,
    Operator, 
}

pub struct Function {
    pub identifier: &'static str,
    pub variant: Variant,
    pub arity: u8,
    pub precedence: ast::Precedence,
    pub is_commutative: bool,
    pub associativity: Associativity,
}

pub fn get(identifier: &str, arity: u8) -> Option<&'static Function> {
    LUT.get(&signature_of(identifier, arity))
        .and_then(|&index| FUNCTIONS.get(index))
}

pub fn identifiers() -> &'static HashSet<&'static str> {
    &IDENTIFIERS
}

fn signature_of(identifier: &str, arity: u8) -> String {
    format!("{identifier}{arity}")
}

const FUNCTIONS: &[Function] = {
    use ast::Precedence::*;
    use Associativity::*;
    use Variant::*;
    &[
        // arithmetic operators
        Function {
            identifier: "-",
            variant: Operator,
            arity: 1,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "^",
            variant: Operator,
            arity: 2,
            precedence: L1,
            is_commutative: false,
            associativity: Right,
        },
        Function {
            identifier: "*",
            variant: Operator,
            arity: 2,
            precedence: L2,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "/",
            variant: Operator,
            arity: 2,
            precedence: L2,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "%",
            variant: Operator,
            arity: 2,
            precedence: L2,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "+",
            variant: Operator,
            arity: 2,
            precedence: L3,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "-",
            variant: Operator,
            arity: 2,
            precedence: L3,
            is_commutative: false,
            associativity: Left,
        },

        // logical operators
        Function {
            identifier: "!",
            variant: Operator,
            arity: 1,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "==",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "!=",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "<",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "<=",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: ">",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: ">=",
            variant: Operator,
            arity: 2,
            precedence: L4,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "&&",
            variant: Operator,
            arity: 2,
            precedence: L5,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "^^",
            variant: Operator,
            arity: 2,
            precedence: L5,
            is_commutative: false,
            associativity: Left,
        },
        Function {
            identifier: "||",
            variant: Operator,
            arity: 2,
            precedence: L6,
            is_commutative: false,
            associativity: Left,
        },

        // routines
        Function {
            identifier: "sqrt",
            variant: Routine,
            arity: 1,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "abs",
            variant: Routine,
            arity: 1,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "min",
            variant: Routine,
            arity: 2,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
        Function {
            identifier: "max",
            variant: Routine,
            arity: 2,
            precedence: L1,
            is_commutative: true,
            associativity: Left,
        },
    ]
};

lazy_static! {
    static ref LUT: HashMap<String, usize> = {
        FUNCTIONS.iter()
            .enumerate()
            .map(|(i, f)|
                (signature_of(f.identifier, f.arity), i)
            )
            .collect()
    };

    static ref IDENTIFIERS: HashSet<&'static str> = {
        FUNCTIONS.iter()
            .map(|f| f.identifier)
            .collect()
    };
}
