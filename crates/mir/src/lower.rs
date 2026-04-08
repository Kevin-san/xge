
use crate::mir::*;
use hir::hir::*;
use anyhow::Result;

pub fn lower_hir_to_mir(hir: HirModule) -> Result<MirModule> {
    let mut functions = Vec::new();
    let mut classes = Vec::new();

    for item in hir.items {
        match item {
            HirItem::Function(func) => functions.push(lower_function(func)),
            HirItem::Class(class) => classes.push(lower_class(class)),
            _ => (),
        }
    }

    Ok(MirModule {
        functions,
        classes,
    })
}

fn lower_function(hir: HirFunction) -> MirFunction {
    let name = lower_ident(hir.name);
    let params = hir.params.into_iter().map(lower_param).collect();
    let return_type = hir.return_type.map(lower_type);
    let blocks = lower_body(hir.body);
    MirFunction {
        name,
        params,
        return_type,
        blocks,
    }
}

fn lower_class(hir: HirClass) -> MirClass {
    let name = lower_ident(hir.name);
    let fields = Vec::new(); // TODO: Implement field lowering
    MirClass {
        name,
        fields,
    }
}

fn lower_param(hir: HirParam) -> MirParam {
    let name = lower_ident(hir.name);
    let ty = lower_type(hir.ty);
    MirParam {
        name,
        ty,
    }
}

fn lower_body(body: Vec<HirStmt>) -> Vec<MirBasicBlock> {
    let mut statements = Vec::new();
    for stmt in body {
        statements.extend(lower_stmt(stmt));
    }
    let block = MirBasicBlock {
        id: 0,
        statements,
        terminator: MirTerminator::Return(None),
    };
    vec![block]
}

fn lower_stmt(hir: HirStmt) -> Vec<MirStatement> {
    match hir {
        HirStmt::Let(let_stmt) => lower_let_stmt(let_stmt),
        HirStmt::If(if_stmt) => lower_if_stmt(if_stmt),
        HirStmt::For(for_stmt) => lower_for_stmt(for_stmt),
        HirStmt::While(while_stmt) => lower_while_stmt(while_stmt),
        HirStmt::Return(return_stmt) => lower_return_stmt(return_stmt),
        HirStmt::Expr(expr) => vec![MirStatement::Expr(lower_expr(expr))],
    }
}

fn lower_let_stmt(hir: HirLetStmt) -> Vec<MirStatement> {
    let name = lower_ident(hir.name);
    let ty = lower_type(hir.ty);
    let value = lower_expr(hir.value);
    vec![
        MirStatement::Declare(name.clone(), ty),
        MirStatement::Assign(name, value),
    ]
}

fn lower_if_stmt(hir: HirIfStmt) -> Vec<MirStatement> {
    // TODO: Implement proper if statement lowering with basic blocks
    let condition = lower_expr(hir.condition);
    let mut statements = vec![MirStatement::Expr(condition)];
    for stmt in hir.then_branch {
        statements.extend(lower_stmt(stmt));
    }
    if let Some(else_branch) = hir.else_branch {
        for stmt in else_branch {
            statements.extend(lower_stmt(stmt));
        }
    }
    statements
}

fn lower_for_stmt(hir: HirForStmt) -> Vec<MirStatement> {
    // TODO: Implement proper for loop lowering with basic blocks
    let _name = lower_ident(hir.name);
    let iter = lower_expr(hir.iter);
    let mut statements = vec![MirStatement::Expr(iter)];
    for stmt in hir.body {
        statements.extend(lower_stmt(stmt));
    }
    statements
}

fn lower_while_stmt(hir: HirWhileStmt) -> Vec<MirStatement> {
    // TODO: Implement proper while loop lowering with basic blocks
    let condition = lower_expr(hir.condition);
    let mut statements = vec![MirStatement::Expr(condition)];
    for stmt in hir.body {
        statements.extend(lower_stmt(stmt));
    }
    statements
}

fn lower_return_stmt(_hir: HirReturnStmt) -> Vec<MirStatement> {
    // Return is handled by the terminator, so we just return an empty vec here
    // The terminator will be set in lower_body
    vec![]
}

fn lower_expr(hir: HirExpr) -> MirExpr {
    match hir {
        HirExpr::Literal(lit) => MirExpr::Literal(lower_literal(lit)),
        HirExpr::Ident(ident) => MirExpr::Ident(lower_ident(ident)),
        HirExpr::Binary(binary) => lower_binary_expr(binary),
        HirExpr::Unary(unary) => lower_unary_expr(unary),
        HirExpr::Call(call) => lower_call_expr(call),
        HirExpr::Member(member) => lower_member_expr(member),
        HirExpr::Assign(assign) => lower_assign_expr(assign),
    }
}

fn lower_literal(hir: HirLiteral) -> MirLiteral {
    match hir {
        HirLiteral::Integer(i) => MirLiteral::Integer(i),
        HirLiteral::Float(f) => MirLiteral::Float(f),
        HirLiteral::String(s) => MirLiteral::String(s),
        HirLiteral::Char(c) => MirLiteral::Char(c),
        HirLiteral::Bool(b) => MirLiteral::Bool(b),
        HirLiteral::None => MirLiteral::None,
    }
}

fn lower_ident(hir: HirIdent) -> MirIdent {
    MirIdent(hir.0)
}

fn lower_binary_expr(hir: HirBinaryExpr) -> MirExpr {
    let op = lower_binary_op(hir.op);
    let left = Box::new(lower_expr(*hir.left));
    let right = Box::new(lower_expr(*hir.right));
    MirExpr::Binary(op, left, right)
}

fn lower_unary_expr(hir: HirUnaryExpr) -> MirExpr {
    let op = lower_unary_op(hir.op);
    let expr = Box::new(lower_expr(*hir.expr));
    MirExpr::Unary(op, expr)
}

fn lower_call_expr(hir: HirCallExpr) -> MirExpr {
    let callee = Box::new(lower_expr(*hir.callee));
    let args = hir.args.into_iter().map(lower_expr).collect();
    MirExpr::Call(callee, args)
}

fn lower_member_expr(hir: HirMemberExpr) -> MirExpr {
    let object = Box::new(lower_expr(*hir.object));
    let member = lower_ident(hir.member);
    MirExpr::Member(object, member)
}

fn lower_assign_expr(hir: HirAssignExpr) -> MirExpr {
    // In MIR, assignment is a statement, not an expression
    // TODO: Handle this properly
    let target = Box::new(lower_expr(*hir.target));
    let value = Box::new(lower_expr(*hir.value));
    MirExpr::Binary(MirBinaryOp::Eq, target, value)
}

fn lower_binary_op(hir: HirBinaryOp) -> MirBinaryOp {
    match hir {
        HirBinaryOp::Add => MirBinaryOp::Add,
        HirBinaryOp::Sub => MirBinaryOp::Sub,
        HirBinaryOp::Mul => MirBinaryOp::Mul,
        HirBinaryOp::Div => MirBinaryOp::Div,
        HirBinaryOp::Mod => MirBinaryOp::Mod,
        HirBinaryOp::Eq => MirBinaryOp::Eq,
        HirBinaryOp::Ne => MirBinaryOp::Ne,
        HirBinaryOp::Lt => MirBinaryOp::Lt,
        HirBinaryOp::Le => MirBinaryOp::Le,
        HirBinaryOp::Gt => MirBinaryOp::Gt,
        HirBinaryOp::Ge => MirBinaryOp::Ge,
        HirBinaryOp::And => MirBinaryOp::And,
        HirBinaryOp::Or => MirBinaryOp::Or,
    }
}

fn lower_unary_op(hir: HirUnaryOp) -> MirUnaryOp {
    match hir {
        HirUnaryOp::Not => MirUnaryOp::Not,
        HirUnaryOp::Neg => MirUnaryOp::Neg,
    }
}

fn lower_type(hir: HirType) -> MirType {
    match hir {
        HirType::Ident(ident) => MirType::Ident(lower_ident(ident)),
        HirType::Array(inner) => MirType::Array(Box::new(lower_type(*inner))),
        HirType::Optional(inner) => MirType::Optional(Box::new(lower_type(*inner))),
    }
}

