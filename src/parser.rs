use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenKind;

use std::iter::Peekable;
use proto_rs_macros::generate_ast;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expr> {
        self.term()
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut left = self.factor();

        while let TokenKind::Plus | TokenKind::Minus = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.factor();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        left
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut left = self.unary(); 
        while let TokenKind::Asterisk | TokenKind::Slash = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.unary();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        left
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if let TokenKind::Minus | TokenKind::Plus = self.lexer.peek().unwrap_or(&Token::default()).kind {
            let op = self.lexer.next().unwrap();
            let right = self.unary();
            Box::new(UnaryExpr::new(op, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        match self.lexer.peek().unwrap_or(&Token::default()).kind {
            TokenKind::Int => {
                let token = self.lexer.next().unwrap();
                Box::new(LiteralExpr::new(token.literal.unwrap().parse::<f64>().unwrap()))
            },
            _ => panic!("Unexpected token"),
        }
    }

}

pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> f64;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> f64;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> f64;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> f64;
}

#[generate_ast]
pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub op: Token,
    pub right: Box<dyn Expr>,
}

#[generate_ast]
pub struct LiteralExpr {
    pub value: f64,
}

#[generate_ast]
pub struct UnaryExpr {
    pub op: Token,
    pub right: Box<dyn Expr>,
}
