use std::result;

use crate::ast::{self, function};
use self::function::Function;
use self::tokenizer::TokenStream;

mod tokenizer;

pub type Result<T> = result::Result<T, String>;

/// Implemented as an operator-precedence parser using precedence climbing, based on specification
/// in [Wikipedia: Operator-precedence parser](https://en.wikipedia.org/wiki/Operator-precedence_parser)
pub fn parse(string: &str) -> Result<ast::Expression> {
    let mut tokens = TokenStream::new(string, function::identifiers());
    parse_expression(&mut tokens)
}

fn parse_expression(tokens: &mut TokenStream) -> Result<ast::Expression> {
    parse_precedence(parse_primary(tokens)?, ast::Precedence::L1, tokens)
}

fn parse_precedence(lhs: ast::Expression, min_precedence: ast::Precedence, tokens: &mut TokenStream) -> Result<ast::Expression> {
    let op = peek_function(tokens, 2);

    while op.map(|f| f.precedence >= min_precedence).unwrap_or(false) {
        tokens.discard();
        let rhs = parse_primary(tokens)?;

    };
    todo!()
}

fn parse_primary(tokens: &mut TokenStream) -> Result<ast::Expression> {
    let token = tokens.read().ok_or("expected an expression")?;

    if let Some('(') = token.symbol() {
        let nested = parse_expression(tokens)?;

        match tokens.read().and_then(|t| t.symbol()) {
            Some(')') => Ok(nested),
            _ => Err("expected ')'".into())
        }
    } else if let Some(string) = token.string() {

        todo!()
    } else {
        todo!()
    }
}

fn peek_function(tokens: &mut TokenStream, arity: u8) -> Option<&'static Function> {
    tokens.peek()
        .and_then(|t| t.string())
        .and_then(|s| function::get(s, arity))
}
