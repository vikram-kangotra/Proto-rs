use inkwell::context::Context;

use crate::frontend::expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::frontend::lexer::Lexer;
use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

use std::iter::Peekable;

use super::expr::VariableExpr;
use super::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt};

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

    fn var_decl_statement(&mut self) -> Box<dyn Stmt<'ctx> + 'ctx> {
        self.consume(TokenKind::Let);
        let name = self.lexer.next().unwrap().lexeme.unwrap(); // error handling
        self.consume(TokenKind::Assign);
        let expr = self.expression();
        self.consume(TokenKind::Semicolon);
        Box::new(VarDeclStmt::new(name, expr) as VarDeclStmt<'ctx>)
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

        match self.lexer.peek().unwrap_or(&Token::default()).kind {
            TokenKind::Char => {
                let token = self.lexer.next().unwrap();
                let value = token.lexeme.unwrap().chars().next().unwrap();
                Box::new(LiteralExpr::new_char(self.context, value) as LiteralExpr<'ctx>)
            }
            TokenKind::Int => {
                let token = self.lexer.next().unwrap();
                let value = token.lexeme.unwrap().parse::<i128>().unwrap();
                Box::new(LiteralExpr::new_int(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::Float => {
                let token = self.lexer.next().unwrap();
                let value = token.lexeme.unwrap().parse::<f64>().unwrap();
                Box::new(LiteralExpr::new_float(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::False => {
                self.lexer.next();
                Box::new(LiteralExpr::new_int8(self.context, 0) as LiteralExpr<'ctx>)
            }
            TokenKind::True => {
                self.lexer.next();
                Box::new(LiteralExpr::new_int8(self.context, 1) as LiteralExpr<'ctx>)
            }
            TokenKind::LParen => {
                self.lexer.next();
                let expr = self.expression();
                self.consume(TokenKind::RParen);
                expr
            },
            TokenKind::Ident => {
                let token = self.lexer.next().unwrap();
                let name = token.lexeme.unwrap();
                Box::new(VariableExpr::new(name))
            },
            TokenKind::Illegal => {
                let token = self.lexer.next().unwrap();
                panic!("Syntax Error (line: {}, column: {}): {}", token.line, token.column, token.lexeme.unwrap());
            }
            _ => {
                let token = self.lexer.next().unwrap();
                panic!("Syntax Error (line: {}, column: {}): {:?} '{}'", token.line, token.column, token.kind, token.lexeme.unwrap());
            }
        }
    }

}
