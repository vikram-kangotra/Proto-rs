use crate::frontend::expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::frontend::lexer::Lexer;
use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

use std::iter::Peekable;

use super::expr::{VariableExpr, VarAssignExpr, CallExpr, LiteralType, IntType, FloatType};
use super::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt, Param};

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl<'ctx> Parser {

    pub fn new(lexer: Lexer) -> Self {
        Self {
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
                let name = match self.lexer.next().map(|token| token.kind) {
                    Some(TokenKind::Ident(name)) => name,
                    _ => panic!("Expected identifier but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind),
                };
                self.consume(TokenKind::Colon);
                let type_ = match self.lexer.next().map(|token| token.kind) {
                    Some(TokenKind::Ident(type_)) if self.is_type(&type_) => type_,
                    Some(kind) => panic!("Expected type but got {:?}", kind),
                    _ => panic!("Expected type but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind),
                };
                params.push(Param::new(name, type_));

                if self.lexer.peek().unwrap_or(&Token::default()).kind != TokenKind::RightParen {
                    self.consume(TokenKind::Comma);
                }
            }
            self.consume(TokenKind::RightParen);

            let return_type = if self.lexer.peek().unwrap_or(&Token::default()).kind == TokenKind::RightArrow {
                self.lexer.next();

                match self.lexer.next().map(|token| token.kind) {
                    Some(TokenKind::LeftParen) => {
                        self.consume(TokenKind::RightParen);
                        None
                    }
                    Some(TokenKind::Ident(type_)) if self.is_type(&type_) => Some(type_),
                    Some(kind) => panic!("Expected type but got {:?}", kind),
                    _ => panic!("Expected type but got {:?}", self.lexer.peek().unwrap_or(&Token::default()).kind),
                }
            } else {
                None
            };

            let func_decl = FunctionDeclStmt::new(name, params, return_type);

            if let TokenKind::LeftBrace = self.lexer.peek().unwrap_or(&Token::default()).kind {
                let body = self.block_statement();
                Box::new(FunctionDefStmt::new(func_decl, body) as FunctionDefStmt<'ctx>)
            } else {
                self.consume(TokenKind::Semicolon);
                Box::new(func_decl)
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
                Box::new(LiteralExpr::new(LiteralType::Char(value)))
            }
            TokenKind::Int(value) => {
                let value = value.parse::<u64>().unwrap();
                match value {
                    value if value <= u8::MAX as u64 => Box::new(LiteralExpr::new(LiteralType::Int(IntType::U8(value as u8)))),
                    value if value <= u16::MAX as u64 => Box::new(LiteralExpr::new(LiteralType::Int(IntType::U16(value as u16)))),
                    value if value <= u32::MAX as u64 => Box::new(LiteralExpr::new(LiteralType::Int(IntType::U32(value as u32)))),
                    value if value <= u64::MAX as u64 => Box::new(LiteralExpr::new(LiteralType::Int(IntType::U64(value as u64)))),
                    _ => panic!("Integer literal out of range"),
                }
            },
            TokenKind::Float(value) => {
                let value = value.parse::<f64>().unwrap();
                match value {
                    value if value <= f32::MAX as f64 => Box::new(LiteralExpr::new(LiteralType::Float(FloatType::F32(value as f32)))), 
                    value if value <= f64::MAX as f64 => Box::new(LiteralExpr::new(LiteralType::Float(FloatType::F64(value as f64)))),
                    _ => panic!("Float literal out of range"),
                }
            },
            TokenKind::False => {
                Box::new(LiteralExpr::new(LiteralType::Bool(false)))
            }
            TokenKind::True => {
                Box::new(LiteralExpr::new(LiteralType::Bool(true)))
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
