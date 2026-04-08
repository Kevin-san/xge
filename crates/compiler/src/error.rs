
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Lexical error")]
    LexError,
    #[error("Syntax error")]
    ParseError,
    #[error("Semantic error")]
    SemanticError,
    #[error("Type error")]
    TypeError,
    #[error("Lowering error")]
    LowerError,
    #[error("Code generation error")]
    CodegenError,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

