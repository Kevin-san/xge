
use crate::error::*;
use lexer::Lexer;
use parser::Parser;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub optimize: bool,
    pub target: Target,
    pub output_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Native,
    Wasm32,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            optimize: true,
            target: Target::Native,
            output_file: None,
        }
    }
}

pub struct Compiler {
    options: CompilerOptions,
    diagnostics: Diagnostics,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Compiler {
            options,
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<CompileOutput, CompileError> {
        let _lexer = Lexer::new(source);
        let mut parser = Parser::new(source);
        let _ast = parser.parse_module()?;
        
        Ok(CompileOutput {
            assembly: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CompileOutput {
    pub assembly: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Diagnostics {
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: Diagnostic) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: Diagnostic) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub kind: DiagnosticKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

