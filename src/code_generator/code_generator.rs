use inkwell::{context::Context, builder::Builder, values::IntValue};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{Expr, BinaryExpr, LiteralExpr, UnaryExpr};
use crate::frontend::visitor::Visitor;
use crate::frontend::token::TokenKind;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, builder: &'ctx Builder<'ctx>) -> CodeGenerator<'ctx> {
        CodeGenerator {
            context,
            builder,
        }
    }

    pub fn generate_code(&mut self, expr: &dyn Expr<'ctx>) -> IntValue<'ctx> {
        expr.accept(self)
    }
}

impl<'ctx> Visitor<'ctx> for CodeGenerator<'ctx> {

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> IntValue<'ctx> {
        self.context.i64_type().const_int(expr.value as u64, false)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> IntValue<'ctx> {
        let operand = self.generate_code(expr.right.as_ref());

        match expr.op.kind {
            TokenKind::Minus => self.builder.build_int_neg(operand, "neg"),
            TokenKind::Plus => operand,
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> IntValue<'ctx> {
        let left = self.generate_code(expr.left.as_ref());
        let right = self.generate_code(expr.right.as_ref());

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_int_add(left, right, "add"),
            TokenKind::Minus => self.builder.build_int_sub(left, right, "sub"),
            TokenKind::Asterisk => self.builder.build_int_mul(left, right, "mul"),
            TokenKind::Slash => self.builder.build_int_signed_div(left, right, "div"),
            _ => panic!("Unexpected token"),
        }
    }
}
