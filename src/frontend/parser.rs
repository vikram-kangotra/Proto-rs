use inkwell::context::Context;

use crate::frontend::expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::frontend::lexer::Lexer;
use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

use std::iter::Peekable;

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

    pub fn parse(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        self.expression()
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
        while let TokenKind::Asterisk | TokenKind::Slash = self.lexer.peek().unwrap_or(&Token::default()).kind {
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
            TokenKind::Int => {
                let token = self.lexer.next().unwrap();
                let value = token.lexeme.unwrap().parse::<i64>().unwrap();
                Box::new(LiteralExpr::new_int(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::Float => {
                let token = self.lexer.next().unwrap();
                let value = token.lexeme.unwrap().parse::<f64>().unwrap();
                Box::new(LiteralExpr::new_float(self.context, value) as LiteralExpr<'ctx>)
            },
            TokenKind::False => {
                self.lexer.next();
                Box::new(LiteralExpr::new_int(self.context, 0) as LiteralExpr<'ctx>)
            }
            TokenKind::True => {
                self.lexer.next();
                Box::new(LiteralExpr::new_int(self.context, 1) as LiteralExpr<'ctx>)
            }
            TokenKind::LParen => {
                self.lexer.next();
                let expr = self.expression();
                if let TokenKind::RParen = self.lexer.peek().unwrap_or(&Token::default()).kind {
                    self.lexer.next();
                } else {
                    panic!("Syntax Error: Expected closing parenthesis");
                }
                expr
            },
            _ => panic!("Unexpected token"),
        }
    }

}
