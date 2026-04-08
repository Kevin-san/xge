
use crate::ast::*;
use lexer::Token;
use anyhow::Result;

pub struct Parser {
}

impl Parser {
    pub fn new(_source: &str) -> Self {
        Parser {}
    }

    pub fn parse_module(&mut self) -> Result<Module> {
        Ok(Module { items: Vec::new() })
    }
}

