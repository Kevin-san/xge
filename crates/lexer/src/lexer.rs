
use crate::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(_source: &str) -> Self {
        Lexer {
            tokens: Vec::new(),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

