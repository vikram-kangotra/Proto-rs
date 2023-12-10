use inkwell::values::IntValue;
use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr};

pub trait Visitor<'ctx> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> IntValue<'ctx>;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> IntValue<'ctx>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> IntValue<'ctx>;
}
