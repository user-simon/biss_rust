use std::collections::HashSet;

/// The smallest unit of data to be processed further by the parser.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Symbol(char),
    Constant(f64),
    String(&'a str),
}

impl<'a> Token<'a> {
    pub fn symbol(&self) -> Option<char> {
        match self {
            &Token::Symbol(c) => Some(c),
            _ => None
        }
    }

    pub fn constant(&self) -> Option<f64> {
        match self {
            &Token::Constant(v) => Some(v),
            _ => None
        }
    }

    pub fn string(&self) -> Option<&'a str> {
        match self {
            &Token::String(s) => Some(s),
            _ => None
        }
    }
}

/// Convenience class for the parser, allowing tokens to be read sequentially from a string.
pub struct TokenStream<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
}

impl<'a> TokenStream<'a> {
    /// Tokenizes input string. Tokens produced can then be read with [`read`](TokenStream::read) or
    /// [`peek`](TokenStream::peek).
    /// 
    /// `symbol_units` defines all tokens composed of symbols that are to be uniformly interpreted
    /// as strings, such that `">=-"` can be tokenized as `[">=", "-"]`, and not `['>', '=', '-']`
    /// as this aides in parsing.
    pub fn new(string: &'a str, symbol_units: &HashSet<&str>) -> Self {
        let mut tokens = Vec::new();
        
        for (category, segment) in categorize(string.as_ref()).into_iter() {
            match category {
                Category::Alpha => {
                    tokens.push(Token::String(segment));
                },
                Category::Digit => {
                    let content = segment.parse().unwrap();
                    // TODO handle multiple '.'
                    tokens.push(Token::Constant(content));
                },
                Category::Symbol => {
                    let mut start = 0;
                    while start < segment.len() {
                        let mut sub = &segment[start..];

                        loop {
                            if symbol_units.contains(sub) {
                                tokens.push(Token::String(sub));
                                break;
                            } else if sub.len() == 1 {
                                tokens.push(Token::Symbol(sub.chars().next().unwrap()));
                                break;
                            } else {
                                sub = &sub[0..sub.len() - 1];
                            }
                        };
                        start += sub.len();
                    }
                },
                Category::Whitespace => () // ignore whitespaces
            };
        }
        TokenStream {
            tokens,
            cursor: 0,
        }
    }

    /// Consumes a token from the stream.
    /// 
    /// # Returns
    /// [`None`] if all tokens have been read.
    pub fn read(&mut self) -> Option<&Token> {
        let out = self.tokens.get(self.cursor);
        self.cursor += 1;
        out
    }

    pub fn discard(&mut self) {
        self.cursor += 1;
    }

    /// Retrieves the next token from the stream without consuming it.
    /// 
    /// # Returns
    /// [`None`] if all tokens have been read.
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}

/// First phase of tokenization; splits string into segments containing only the same
/// [`Categories`](Category).
fn categorize(string: &str) -> Vec<(Category, &str)> {
    let mut segments = Vec::new();
    let mut prev_cat = Category::Alpha;
    let mut start = 0;

    for (i, c) in string.chars().enumerate() {
        let current_cat = Category::from(c);
        
        if i > 0 && prev_cat != current_cat {
            // add segment leading up to (but excluding) current char
            segments.push((prev_cat, &string[start..i]));
            // save start of next segment
            start = i;
        }
        prev_cat = current_cat;
    }
    segments.push((prev_cat, &string[start..]));
    segments
}

/// Utility defining all considered character categories.
#[derive(PartialEq, Debug)]
enum Category {
    Alpha,
    Digit,
    Whitespace,
    Symbol,
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        if c.is_alphabetic() || c == '_' {
            Category::Alpha
        } else if c.is_numeric() || c == '.' {
            Category::Digit
        } else if c.is_whitespace() {
            Category::Whitespace
        } else {
            Category::Symbol
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::function;

    use super::*;

    #[test]
    fn tokenize() {
        let data = [
            (
                "123abc()",
                vec![Token::Constant(123.0), Token::String("abc"), Token::Symbol('('), Token::Symbol(')')]
            ),
            (
                "      ",
                vec![]
            ),
            (
                "+===",
                vec![Token::String("+="), Token::String("==")]
            ),
            (
                "==+",
                vec![Token::String("=="), Token::String("+")]
            )
        ];
        for (input, output) in data {
            assert_eq!(TokenStream::new(input, function::identifiers()).tokens, output);
        }
    }

    #[test]
    fn categorize() {
        let data = [
            (
                "123abc+-*",
                vec![(Category::Digit, "123"), (Category::Alpha, "abc"), (Category::Symbol, "+-*")]
            ),
            (
                "1.3.4.5.6",
                vec![(Category::Digit, "1.3.4.5.6")]
            ),
            (
                "1 +  2",
                vec![(Category::Digit, "1"), (Category::Whitespace, " "), (Category::Symbol, "+"), (Category::Whitespace, "  "), (Category::Digit, "2")]
            )
        ];
        
        for (input, output) in data {
            assert_eq!(super::categorize(input), output);
        }
    }
}
