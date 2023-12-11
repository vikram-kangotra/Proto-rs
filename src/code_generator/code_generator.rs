use inkwell::values::IntValue;
use inkwell::{context::Context, builder::Builder, values::BasicValueEnum};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{Expr, BinaryExpr, LiteralExpr, UnaryExpr};
use crate::frontend::visitor::Visitor;
use crate::frontend::token::TokenKind;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(builder: &'ctx Builder<'ctx>) -> CodeGenerator<'ctx> {
        CodeGenerator {
            builder,
        }
    }

    pub fn generate_code(&mut self, expr: &dyn Expr<'ctx>) -> BasicValueEnum<'ctx> {
        expr.accept(self)
    }
}

impl<'ctx> Visitor<'ctx> for CodeGenerator<'ctx> {

    fn visit_literal_expr(&mut self, expr: &LiteralExpr<'ctx>) -> BasicValueEnum<'ctx> {
        expr.value
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let operand = self.generate_code(expr.right.as_ref());

        match operand {
            BasicValueEnum::IntValue(value) => self.visit_unary_expr_i64(value, expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_i64(&mut self, value: IntValue<'ctx>, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Minus => self.builder.build_int_neg(value, "neg").into(),
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let left = self.generate_code(expr.left.as_ref());
        let right = self.generate_code(expr.right.as_ref());

        match (left, right) {
            (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_i64_i64(left, right, expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr_i64_i64(&mut self, left: IntValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Plus => self.builder.build_int_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_int_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_int_mul(left, right, "mul").into(),
            TokenKind::Slash => self.builder.build_int_signed_div(left, right, "div").into(),
            _ => panic!("Unexpected token"),
        }
    }
}
