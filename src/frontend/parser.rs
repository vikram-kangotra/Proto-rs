use inkwell::context::Context;

use crate::frontend::expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::frontend::lexer::Lexer;
use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

use std::iter::Peekable;

use super::expr::{VariableExpr, VarAssignExpr, CallExpr};
use super::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt};

pub struct Parser<'ctx> {
    context: &'ctx Context,
    lexer: Peekable<Lexer>,
}

impl<'ctx> Parser<'ctx> {

    pub fn new(context: &'ctx Context , lexer: Lexer) -> Self {
        Self {
            context,
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Stmt<'ctx> + 'ctx>> {
        let mut statements = Vec::new();

        while let Some(_) = self.lexer.peek() {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        match self.lexer.peek().unwrap_or(&Token::default()).kind {
            TokenKind::Let => self.var_decl_statement(),
            TokenKind::Return => self.return_statement(),
            TokenKind::LeftBrace => self.block_statement(),
            TokenKind::If => self.if_statement(),
            TokenKind::While => self.while_statement(),
            TokenKind::Break => self.break_statement(),
            TokenKind::Continue => self.continue_statement(),
            TokenKind::Function => self.function_decl_def_statement(),
            _ => self.expression_statement(),
        }
    }

    fn consume(&mut self, kind: TokenKind) {
        if self.lexer.peek().unwrap_or(&Token::default()).kind == kind {
            self.lexer.next();
        } else {
            panic!("Expected {:?} but got {:?}", kind, self.lexer.peek().unwrap_or(&Token::default()).kind);
        }
    }

    fn is_type(&self, type_: &str) -> bool {
        match type_ {
            "i8" | "i16" | "i32" | "i64" => true,
            _ => false,
        }
    }

    fn var_decl_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Let);
        if let TokenKind::Ident(name) = self.lexer.next().unwrap().kind {

            let type_ = if self.lexer.peek().unwrap_or(&Token::default()).kind == TokenKind::Colon {
                self.consume(TokenKind::Colon);
                match self.lexer.next().map(|token| token.kind) {
                    Some(TokenKind::Ident(type_)) if self.is_type(&type_) => Some(type_),
                    Some(kind) => panic!("Expected type but got {:?}", kind),
                    _ => panic!("Expected type but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind),
                }
            } else {
                None
            };

            self.consume(TokenKind::Assign);
            let initializer = self.expression();
            self.consume(TokenKind::Semicolon);
            Box::new(VarDeclStmt::new(name, type_, initializer) as VarDeclStmt<'ctx>)
        } else {
            panic!("Expected identifier but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind);
        }
    }

    fn block_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::LeftBrace);
        let mut statements = Vec::new();

        while self.lexer.peek().unwrap_or(&Token::default()).kind != TokenKind::RightBrace {
            statements.push(self.statement());
        }

        self.consume(TokenKind::RightBrace);
        Box::new(BlockStmt::new(statements) as BlockStmt<'ctx>)
    }

    fn if_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::If);
        let condition = self.expression();
        let then_branch = self.statement();
        let else_branch = if self.lexer.peek().unwrap_or(&Token::default()).kind == TokenKind::Else {
            self.lexer.next();
            Some(self.statement())
        } else {
            None
        };

        Box::new(IfStmt::new(condition, then_branch, else_branch) as IfStmt<'ctx>)
    }

    fn while_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::While);
        let condition = self.expression();
        let body = self.statement();

        Box::new(WhileStmt::new(condition, body) as WhileStmt<'ctx>)
    }

    fn break_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Break);
        self.consume(TokenKind::Semicolon);
        Box::new(BreakStmt::new() as BreakStmt)
    }

    fn continue_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Continue);
        self.consume(TokenKind::Semicolon);
        Box::new(ContinueStmt::new() as ContinueStmt)
    }

    fn function_decl_def_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Function);
        if let TokenKind::Ident(name) = self.lexer.next().unwrap_or_default().kind {
            self.consume(TokenKind::LeftParen);
            let mut params = Vec::new();
            while self.lexer.peek().unwrap_or(&Token::default()).kind != TokenKind::RightParen {
                if let TokenKind::Ident(name) = self.lexer.next().unwrap_or_default().kind {
                    params.push(name);
                } else {
                    panic!("Expected identifier but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind);
                }
                if self.lexer.peek().unwrap_or(&Token::default()).kind != TokenKind::RightParen {
                    self.consume(TokenKind::Comma);
                }
            }
            self.consume(TokenKind::RightParen);

            if let TokenKind::LeftBrace = self.lexer.peek().unwrap_or(&Token::default()).kind {
                let body = self.block_statement();
                Box::new(FunctionDefStmt::new(name, params, body) as FunctionDefStmt<'ctx>)
            } else {
                self.consume(TokenKind::Semicolon);
                Box::new(FunctionDeclStmt::new(name, params) as FunctionDeclStmt)
            }
        } else {
            panic!("Expected identifier but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind);
        }
    }

    fn expression_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        let expr = self.expression();
        self.consume(TokenKind::Semicolon);
        Box::new(ExprStmt::new(expr) as ExprStmt<'ctx>)
    }

    fn return_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Return);
        let expr = self.expression();
        self.consume(TokenKind::Semicolon);
        Box::new(ReturnStmt::new(expr) as ReturnStmt<'ctx>)
    }

    fn expression(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        let mut left = self.comparison();

        while let TokenKind::NotEqual | TokenKind::Equal = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.comparison();
            left = Box::new(BinaryExpr::new(left, op, right) as BinaryExpr<'ctx>);
        }
        left
    }

    fn comparison(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        let mut left = self.term();

        while let TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.term();
            left = Box::new(BinaryExpr::new(left, op, right) as BinaryExpr<'ctx>);
        }
        left
    }

    fn term(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        let mut left = self.factor();

        while let TokenKind::Plus | TokenKind::Minus = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.factor();
            left = Box::new(BinaryExpr::new(left, op, right) as BinaryExpr<'ctx>);
        }
        left
    }

    fn factor(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        let mut left = self.unary(); 
        while let TokenKind::Asterisk | TokenKind::Slash | TokenKind::Remainder = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.unary();
            left = Box::new(BinaryExpr::new(left, op, right) as BinaryExpr<'ctx>);
        }
        left
    }

    fn unary(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        if let TokenKind::Minus | TokenKind::Plus = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.unary();
            Box::new(UnaryExpr::new(op, right) as UnaryExpr<'ctx>)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {

        let token = self.lexer.next().unwrap_or(Token::default());

        match token.kind {
            TokenKind::Char(value) => {
                Box::new(LiteralExpr::new_char(self.context, value) as LiteralExpr<'ctx>)
            }
            TokenKind::Int(value) => {
                let value = value.parse::<i128>().unwrap();
                Box::new(LiteralExpr::new_int(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::Float(value) => {
                let value = value.parse::<f64>().unwrap();
                Box::new(LiteralExpr::new_float(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::False => {
                Box::new(LiteralExpr::new_bool(self.context, true) as LiteralExpr<'ctx>)
            }
            TokenKind::True => {
                Box::new(LiteralExpr::new_bool(self.context, false) as LiteralExpr<'ctx>)
            }
            TokenKind::LeftParen => {
                let expr = self.expression();
                self.consume(TokenKind::RightParen);
                expr
            },
            TokenKind::Ident(name) => {
                if self.lexer.peek().unwrap().kind == TokenKind::Assign {
                    self.lexer.next();
                    let value = self.expression();
                    Box::new(VarAssignExpr::new(name, value) as VarAssignExpr<'ctx>)
                } else if self.lexer.peek().unwrap().kind == TokenKind::LeftParen {
                    self.lexer.next();
                    let mut args = Vec::new();
                    while self.lexer.peek().unwrap().kind != TokenKind::RightParen {
                        args.push(self.expression());
                        if self.lexer.peek().unwrap().kind != TokenKind::RightParen {
                            self.consume(TokenKind::Comma);
                        }
                    }
                    self.consume(TokenKind::RightParen);
                    Box::new(CallExpr::new(name, args) as CallExpr<'ctx>)
                } else {
                    Box::new(VariableExpr::new(name))
                }
            },
            TokenKind::Illegal(lexeme) => panic!("Syntax Error (line: {}, column: {}): Illegal token '{}'", token.line, token.column, lexeme),
            _ => panic!("Syntax Error (line: {}, column: {}): Unexpected token '{:?}'", token.line, token.column, token.kind),
        }
    }

}
