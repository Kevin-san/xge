
use crate::ast::*;
use lexer::{Lexer, Token};
use anyhow::{bail, Result};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next().unwrap_or(Token::Eof);
        Parser {
            lexer,
            current,
        }
    }

    pub fn parse_module(&mut self) -> Result<Module> {
        let mut items = Vec::new();
        while !self.is_eof() {
            items.push(self.parse_item()?);
        }
        Ok(Module { items })
    }

    fn parse_item(&mut self) -> Result<Item> {
        match &self.current {
            Token::Fn => self.parse_function(),
            Token::Class => self.parse_class(),
            _ => self.parse_stmt().map(Item::Stmt),
        }
    }

    fn parse_function(&mut self) -> Result<Item> {
        self.expect(Token::Fn)?;
        let name = self.parse_ident()?;
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;
        let return_type = self.parse_return_type()?;
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let body = self.parse_block()?;
        self.expect(Token::Dedent)?;
        Ok(Item::Function(Function {
            name,
            params,
            return_type,
            body,
        }))
    }

    fn parse_class(&mut self) -> Result<Item> {
        self.expect(Token::Class)?;
        let name = self.parse_ident()?;
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let mut body = Vec::new();
        while !matches!(&self.current, Token::Dedent | Token::Eof) {
            body.push(self.parse_item()?);
        }
        self.expect(Token::Dedent)?;
        Ok(Item::Class(Class {
            name,
            body,
        }))
    }

    fn parse_params(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();
        if !self.is_token(&Token::RParen) {
            loop {
                let name = self.parse_ident()?;
                self.expect(Token::Colon)?;
                let ty = self.parse_type()?;
                params.push(Param { name, ty });
                if !self.is_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }
        Ok(params)
    }

    fn parse_return_type(&mut self) -> Result<Option<Type>> {
        if self.is_token(&Token::Arrow) {
            self.advance();
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        match &self.current {
            Token::Ident(name) => {
                let ident = Ident(name.clone());
                self.advance();
                if self.is_token(&Token::LBracket) {
                    self.advance();
                    self.expect(Token::RBracket)?;
                    Ok(Type::Array(Box::new(Type::Ident(ident))))
                } else {
                    Ok(Type::Ident(ident))
                }
            }
            _ => bail!("Expected type, got {:?}", self.current),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match &self.current {
            Token::Let => self.parse_let_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::For => self.parse_for_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr().map(Stmt::Expr),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        self.expect(Token::Let)?;
        let name = self.parse_ident()?;
        self.expect(Token::Colon)?;
        let ty = self.parse_type()?;
        self.expect(Token::Eq)?;
        let value = self.parse_expr()?;
        self.expect(Token::Newline)?;
        Ok(Stmt::Let(LetStmt {
            name,
            ty,
            value,
        }))
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.expect(Token::If)?;
        let condition = self.parse_expr()?;
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let then_branch = self.parse_block()?;
        self.expect(Token::Dedent)?;
        let else_branch = if self.is_token(&Token::Else) {
            self.advance();
            self.expect(Token::Newline)?;
            self.expect(Token::Indent)?;
            let branch = self.parse_block()?;
            self.expect(Token::Dedent)?;
            Some(branch)
        } else {
            None
        };
        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt> {
        self.expect(Token::For)?;
        let name = self.parse_ident()?;
        self.expect(Token::In)?;
        let iter = self.parse_expr()?;
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let body = self.parse_block()?;
        self.expect(Token::Dedent)?;
        Ok(Stmt::For(ForStmt {
            name,
            iter,
            body,
        }))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.expect(Token::While)?;
        let condition = self.parse_expr()?;
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let body = self.parse_block()?;
        self.expect(Token::Dedent)?;
        Ok(Stmt::While(WhileStmt {
            condition,
            body,
        }))
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.expect(Token::Return)?;
        let value = if !self.is_token(&Token::Newline) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Token::Newline)?;
        Ok(Stmt::Return(ReturnStmt {
            value,
        }))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !matches!(&self.current, Token::Dedent | Token::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr> {
        let left = self.parse_logical()?;
        if self.is_token(&Token::Eq) {
            self.advance();
            let value = self.parse_assignment()?;
            Ok(Expr::Assign(AssignExpr {
                target: Box::new(left),
                value: Box::new(value),
            }))
        } else {
            Ok(left)
        }
    }

    fn parse_logical(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;
        while matches!(&self.current, Token::And | Token::Or) {
            let op = match &self.current {
                Token::And => BinaryOp::And,
                Token::Or => BinaryOp::Or,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_additive()?;
        while matches!(&self.current, Token::EqEq | Token::NotEq | Token::Lt | Token::LtEq | Token::Gt | Token::GtEq) {
            let op = match &self.current {
                Token::EqEq => BinaryOp::Eq,
                Token::NotEq => BinaryOp::Ne,
                Token::Lt => BinaryOp::Lt,
                Token::LtEq => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::GtEq => BinaryOp::Ge,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplicative()?;
        while matches!(&self.current, Token::Plus | Token::Minus) {
            let op = match &self.current {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        while matches!(&self.current, Token::Star | Token::Slash | Token::Percent) {
            let op = match &self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        match &self.current {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                }))
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                }))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match &self.current {
            Token::Integer(i) => {
                let lit = Literal::Integer(*i);
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::Float(f) => {
                let lit = Literal::Float(*f);
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::StringLiteral(s) => {
                let lit = Literal::String(s.clone());
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::CharLiteral(c) => {
                let lit = Literal::Char(*c);
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::True => {
                let lit = Literal::Bool(true);
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::False => {
                let lit = Literal::Bool(false);
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::None => {
                let lit = Literal::None;
                self.advance();
                Ok(Expr::Literal(lit))
            }
            Token::Ident(name) => {
                let ident = Ident(name.clone());
                self.advance();
                if self.is_token(&Token::LParen) {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call(CallExpr {
                        callee: Box::new(Expr::Ident(ident)),
                        args,
                    }))
                } else if self.is_token(&Token::Dot) {
                    self.advance();
                    let member = self.parse_ident()?;
                    Ok(Expr::Member(MemberExpr {
                        object: Box::new(Expr::Ident(ident)),
                        member,
                    }))
                } else {
                    Ok(Expr::Ident(ident))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => bail!("Expected expression, got {:?}", self.current),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.is_token(&Token::RParen) {
            loop {
                args.push(self.parse_expr()?);
                if !self.is_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }
        Ok(args)
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        match &self.current {
            Token::Ident(name) => {
                let ident = Ident(name.clone());
                self.advance();
                Ok(ident)
            }
            _ => bail!("Expected identifier, got {:?}", self.current),
        }
    }

    fn expect(&mut self, token: Token) -> Result<()> {
        if self.is_token(&token) {
            self.advance();
            Ok(())
        } else {
            bail!("Expected {:?}, got {:?}", token, self.current)
        }
    }

    fn is_token(&self, token: &Token) -> bool {
        match (&self.current, token) {
            (Token::Ident(a), Token::Ident(b)) => a == b,
            (Token::Integer(a), Token::Integer(b)) => a == b,
            (Token::Float(a), Token::Float(b)) => a == b,
            (Token::StringLiteral(a), Token::StringLiteral(b)) => a == b,
            (Token::CharLiteral(a), Token::CharLiteral(b)) => a == b,
            _ => std::mem::discriminant(&self.current) == std::mem::discriminant(token),
        }
    }

    fn is_eof(&self) -> bool {
        matches!(&self.current, Token::Eof)
    }

    fn advance(&mut self) {
        self.current = self.lexer.next().unwrap_or(Token::Eof);
    }
}

