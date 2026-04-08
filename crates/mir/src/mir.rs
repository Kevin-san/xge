
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MirIdent(pub String);

impl fmt::Display for MirIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct MirModule {
    pub functions: Vec<MirFunction>,
    pub classes: Vec<MirClass>,
}

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: MirIdent,
    pub params: Vec<MirParam>,
    pub return_type: Option<MirType>,
    pub blocks: Vec<MirBasicBlock>,
}

#[derive(Debug, Clone)]
pub struct MirClass {
    pub name: MirIdent,
    pub fields: Vec<MirField>,
}

#[derive(Debug, Clone)]
pub struct MirParam {
    pub name: MirIdent,
    pub ty: MirType,
}

#[derive(Debug, Clone)]
pub struct MirField {
    pub name: MirIdent,
    pub ty: MirType,
}

#[derive(Debug, Clone)]
pub struct MirBasicBlock {
    pub id: usize,
    pub statements: Vec<MirStatement>,
    pub terminator: MirTerminator,
}

#[derive(Debug, Clone)]
pub enum MirStatement {
    Assign(MirIdent, MirExpr),
    Expr(MirExpr),
    Declare(MirIdent, MirType),
}

#[derive(Debug, Clone)]
pub enum MirTerminator {
    Return(Option<MirExpr>),
    Goto(usize),
    If(MirExpr, usize, usize),
    Loop(usize),
}

#[derive(Debug, Clone)]
pub enum MirExpr {
    Literal(MirLiteral),
    Ident(MirIdent),
    Binary(MirBinaryOp, Box<MirExpr>, Box<MirExpr>),
    Unary(MirUnaryOp, Box<MirExpr>),
    Call(Box<MirExpr>, Vec<MirExpr>),
    Member(Box<MirExpr>, MirIdent),
}

#[derive(Debug, Clone)]
pub enum MirLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    None,
}

#[derive(Debug, Clone)]
pub enum MirBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum MirUnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum MirType {
    Ident(MirIdent),
    Array(Box<MirType>),
    Optional(Box<MirType>),
}

