use crate::token::TokenKind;
use crate::parser::{Expr, BinaryExpr, LiteralExpr, Visitor, UnaryExpr};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn eval(&mut self, expr: Box<dyn Expr>) -> f64 {
        expr.accept(self)
    }
}

impl Visitor for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> f64 {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);

        match expr.op.kind {
            TokenKind::Plus => left + right,
            TokenKind::Minus => left - right,
            TokenKind::Asterisk => left * right,
            TokenKind::Slash => left / right,
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> f64 {
        expr.value
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> f64 {
        let right = expr.right.accept(self);

        match expr.op.kind {
            TokenKind::Minus => -right,
            TokenKind::Plus => right,
            _ => panic!("Unexpected token"),
        }
    }
}
