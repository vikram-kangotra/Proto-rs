use crate::frontend::expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::frontend::lexer::Lexer;
use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

use std::iter::Peekable;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl<'ctx> Parser {

    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expr<'ctx> + 'ctx> {
        self.term()
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
                Box::new(LiteralExpr::new(token.literal.unwrap().parse::<i64>().unwrap()))
            },
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
