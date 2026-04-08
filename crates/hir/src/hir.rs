
use std::fmt;

#[derive(Debug, Clone)]
pub struct HirModule {
    pub items: Vec<HirItem>,
}

#[derive(Debug, Clone)]
pub enum HirItem {
    Function(HirFunction),
    Class(HirClass),
    Expr(HirExpr),
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: HirIdent,
    pub params: Vec<HirParam>,
    pub return_type: Option<HirType>,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub struct HirClass {
    pub name: HirIdent,
    pub body: Vec<HirItem>,
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: HirIdent,
    pub ty: HirType,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let(HirLetStmt),
    If(HirIfStmt),
    For(HirForStmt),
    While(HirWhileStmt),
    Return(HirReturnStmt),
    Expr(HirExpr),
}

#[derive(Debug, Clone)]
pub struct HirLetStmt {
    pub name: HirIdent,
    pub ty: HirType,
    pub value: HirExpr,
}

#[derive(Debug, Clone)]
pub struct HirIfStmt {
    pub condition: HirExpr,
    pub then_branch: Vec<HirStmt>,
    pub else_branch: Option<Vec<HirStmt>>,
}

#[derive(Debug, Clone)]
pub struct HirForStmt {
    pub name: HirIdent,
    pub iter: HirExpr,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub struct HirWhileStmt {
    pub condition: HirExpr,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub struct HirReturnStmt {
    pub value: Option<HirExpr>,
}

#[derive(Debug, Clone)]
pub enum HirExpr {
    Literal(HirLiteral),
    Ident(HirIdent),
    Binary(HirBinaryExpr),
    Unary(HirUnaryExpr),
    Call(HirCallExpr),
    Member(HirMemberExpr),
    Assign(HirAssignExpr),
}

#[derive(Debug, Clone)]
pub struct HirBinaryExpr {
    pub left: Box<HirExpr>,
    pub op: HirBinaryOp,
    pub right: Box<HirExpr>,
}

#[derive(Debug, Clone)]
pub struct HirUnaryExpr {
    pub op: HirUnaryOp,
    pub expr: Box<HirExpr>,
}

#[derive(Debug, Clone)]
pub struct HirCallExpr {
    pub callee: Box<HirExpr>,
    pub args: Vec<HirExpr>,
}

#[derive(Debug, Clone)]
pub struct HirMemberExpr {
    pub object: Box<HirExpr>,
    pub member: HirIdent,
}

#[derive(Debug, Clone)]
pub struct HirAssignExpr {
    pub target: Box<HirExpr>,
    pub value: Box<HirExpr>,
}

#[derive(Debug, Clone)]
pub enum HirBinaryOp {
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
pub enum HirUnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum HirLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    None,
}

#[derive(Debug, Clone)]
pub enum HirType {
    Ident(HirIdent),
    Array(Box<HirType>),
    Optional(Box<HirType>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirIdent(pub String);

impl fmt::Display for HirIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

