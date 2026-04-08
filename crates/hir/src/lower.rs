use crate::hir::*;
use ::parser::*;

pub fn lower_module(module: Module) -> HirModule {
    let items = module.items.into_iter().map(lower_item).collect();
    HirModule { items }
}

fn lower_item(item: Item) -> HirItem {
    match item {
        Item::Function(func) => HirItem::Function(lower_function(func)),
        Item::Class(class) => HirItem::Class(lower_class(class)),
        Item::Expr(expr) => HirItem::Expr(lower_expr(expr)),
        Item::Stmt(_) => todo!(),
    }
}

fn lower_function(func: Function) -> HirFunction {
    let name = lower_ident(func.name);
    let params = func.params.into_iter().map(lower_param).collect();
    let return_type = func.return_type.map(lower_type);
    let body = func.body.into_iter().map(lower_stmt).collect();
    HirFunction {
        name,
        params,
        return_type,
        body,
    }
}

fn lower_class(class: Class) -> HirClass {
    let name = lower_ident(class.name);
    let body = class.body.into_iter().map(lower_item).collect();
    HirClass {
        name,
        body,
    }
}

fn lower_param(param: Param) -> HirParam {
    let name = lower_ident(param.name);
    let ty = lower_type(param.ty);
    HirParam {
        name,
        ty,
    }
}

fn lower_stmt(stmt: Stmt) -> HirStmt {
    match stmt {
        Stmt::Let(let_stmt) => HirStmt::Let(lower_let_stmt(let_stmt)),
        Stmt::If(if_stmt) => HirStmt::If(lower_if_stmt(if_stmt)),
        Stmt::For(for_stmt) => HirStmt::For(lower_for_stmt(for_stmt)),
        Stmt::While(while_stmt) => HirStmt::While(lower_while_stmt(while_stmt)),
        Stmt::Return(return_stmt) => HirStmt::Return(lower_return_stmt(return_stmt)),
        Stmt::Expr(expr) => HirStmt::Expr(lower_expr(expr)),
    }
}

fn lower_let_stmt(let_stmt: LetStmt) -> HirLetStmt {
    let name = lower_ident(let_stmt.name);
    let ty = lower_type(let_stmt.ty);
    let value = lower_expr(let_stmt.value);
    HirLetStmt {
        name,
        ty,
        value,
    }
}

fn lower_if_stmt(if_stmt: IfStmt) -> HirIfStmt {
    let condition = lower_expr(if_stmt.condition);
    let then_branch = if_stmt.then_branch.into_iter().map(lower_stmt).collect();
    let else_branch = if_stmt.else_branch.map(|branch| branch.into_iter().map(lower_stmt).collect());
    HirIfStmt {
        condition,
        then_branch,
        else_branch,
    }
}

fn lower_for_stmt(for_stmt: ForStmt) -> HirForStmt {
    let name = lower_ident(for_stmt.name);
    let iter = lower_expr(for_stmt.iter);
    let body = for_stmt.body.into_iter().map(lower_stmt).collect();
    HirForStmt {
        name,
        iter,
        body,
    }
}

fn lower_while_stmt(while_stmt: WhileStmt) -> HirWhileStmt {
    let condition = lower_expr(while_stmt.condition);
    let body = while_stmt.body.into_iter().map(lower_stmt).collect();
    HirWhileStmt {
        condition,
        body,
    }
}

fn lower_return_stmt(return_stmt: ReturnStmt) -> HirReturnStmt {
    let value = return_stmt.value.map(lower_expr);
    HirReturnStmt {
        value,
    }
}

fn lower_expr(expr: Expr) -> HirExpr {
    match expr {
        Expr::Literal(lit) => HirExpr::Literal(lower_literal(lit)),
        Expr::Ident(ident) => HirExpr::Ident(lower_ident(ident)),
        Expr::Binary(binary) => HirExpr::Binary(lower_binary_expr(binary)),
        Expr::Unary(unary) => HirExpr::Unary(lower_unary_expr(unary)),
        Expr::Call(call) => HirExpr::Call(lower_call_expr(call)),
        Expr::Member(member) => HirExpr::Member(lower_member_expr(member)),
        Expr::Assign(assign) => HirExpr::Assign(lower_assign_expr(assign)),
    }
}

fn lower_literal(literal: Literal) -> HirLiteral {
    match literal {
        Literal::Integer(i) => HirLiteral::Integer(i),
        Literal::Float(f) => HirLiteral::Float(f),
        Literal::String(s) => HirLiteral::String(s),
        Literal::Char(c) => HirLiteral::Char(c),
        Literal::Bool(b) => HirLiteral::Bool(b),
        Literal::None => HirLiteral::None,
    }
}

fn lower_ident(ident: Ident) -> HirIdent {
    HirIdent(ident.0)
}

fn lower_binary_expr(binary: BinaryExpr) -> HirBinaryExpr {
    let left = Box::new(lower_expr(*binary.left));
    let op = lower_binary_op(binary.op);
    let right = Box::new(lower_expr(*binary.right));
    HirBinaryExpr {
        left,
        op,
        right,
    }
}

fn lower_unary_expr(unary: UnaryExpr) -> HirUnaryExpr {
    let op = lower_unary_op(unary.op);
    let expr = Box::new(lower_expr(*unary.expr));
    HirUnaryExpr {
        op,
        expr,
    }
}

fn lower_call_expr(call: CallExpr) -> HirCallExpr {
    let callee = Box::new(lower_expr(*call.callee));
    let args = call.args.into_iter().map(lower_expr).collect();
    HirCallExpr {
        callee,
        args,
    }
}

fn lower_member_expr(member: MemberExpr) -> HirMemberExpr {
    let object = Box::new(lower_expr(*member.object));
    let member = lower_ident(member.member);
    HirMemberExpr {
        object,
        member,
    }
}

fn lower_assign_expr(assign: AssignExpr) -> HirAssignExpr {
    let target = Box::new(lower_expr(*assign.target));
    let value = Box::new(lower_expr(*assign.value));
    HirAssignExpr {
        target,
        value,
    }
}

fn lower_binary_op(op: BinaryOp) -> HirBinaryOp {
    match op {
        BinaryOp::Add => HirBinaryOp::Add,
        BinaryOp::Sub => HirBinaryOp::Sub,
        BinaryOp::Mul => HirBinaryOp::Mul,
        BinaryOp::Div => HirBinaryOp::Div,
        BinaryOp::Mod => HirBinaryOp::Mod,
        BinaryOp::Eq => HirBinaryOp::Eq,
        BinaryOp::Ne => HirBinaryOp::Ne,
        BinaryOp::Lt => HirBinaryOp::Lt,
        BinaryOp::Le => HirBinaryOp::Le,
        BinaryOp::Gt => HirBinaryOp::Gt,
        BinaryOp::Ge => HirBinaryOp::Ge,
        BinaryOp::And => HirBinaryOp::And,
        BinaryOp::Or => HirBinaryOp::Or,
    }
}

fn lower_unary_op(op: UnaryOp) -> HirUnaryOp {
    match op {
        UnaryOp::Not => HirUnaryOp::Not,
        UnaryOp::Neg => HirUnaryOp::Neg,
    }
}

fn lower_type(ty: Type) -> HirType {
    match ty {
        Type::Ident(ident) => HirType::Ident(lower_ident(ident)),
        Type::Array(inner) => HirType::Array(Box::new(lower_type(*inner))),
        Type::Optional(inner) => HirType::Optional(Box::new(lower_type(*inner))),
    }
}
