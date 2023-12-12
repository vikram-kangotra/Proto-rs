use inkwell::FloatPredicate;
use inkwell::context::Context;
use inkwell::values::{IntValue, FloatValue};
use inkwell::{builder::Builder, values::BasicValueEnum};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{Expr, BinaryExpr, LiteralExpr, UnaryExpr};
use crate::frontend::visitor::Visitor;
use crate::frontend::token::TokenKind;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context,builder: &'ctx Builder<'ctx>) -> CodeGenerator<'ctx> {
        CodeGenerator {
            context,
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
            BasicValueEnum::IntValue(value) => self.visit_unary_expr_int(value, expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_int(&mut self, value: IntValue<'ctx>, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Minus => self.builder.build_int_neg(value, "neg").into(),
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_float(&mut self, value: FloatValue<'ctx>, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Minus => self.builder.build_float_neg(value, "neg").into(),
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let left = self.generate_code(expr.left.as_ref());
        let right = self.generate_code(expr.right.as_ref());

        match (left, right) {
            (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_int_int(left, right, expr),
            (BasicValueEnum::IntValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_int_float(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_float_int(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_float_float(left, right, expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr_int_int(&mut self, left: IntValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Plus => self.builder.build_int_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_int_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_int_mul(left, right, "mul").into(),
            TokenKind::Slash => {
                let left = self.builder.build_signed_int_to_float(left, self.context.f64_type(), "int_to_float");
                let right = self.builder.build_signed_int_to_float(right, self.context.f64_type(), "int_to_float");
                self.builder.build_float_div(left, right, "div").into()
            }
            TokenKind::Greater => self.builder.build_int_compare(inkwell::IntPredicate::SGT, left, right, "gt").into(),
            TokenKind::GreaterEqual => self.builder.build_int_compare(inkwell::IntPredicate::SGE, left, right, "ge").into(),
            TokenKind::Less => self.builder.build_int_compare(inkwell::IntPredicate::SLT, left, right, "lt").into(),
            TokenKind::LessEqual => self.builder.build_int_compare(inkwell::IntPredicate::SLE, left, right, "le").into(),
            TokenKind::Equal => self.builder.build_int_compare(inkwell::IntPredicate::EQ, left, right, "eq").into(),
            TokenKind::NotEqual => self.builder.build_int_compare(inkwell::IntPredicate::NE, left, right, "ne").into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr_int_float(&mut self, left: IntValue<'ctx>, right: FloatValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let left = self.builder.build_signed_int_to_float(left, self.context.f64_type(), "int_to_float");

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_float_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_float_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_float_mul(left, right, "mul").into(),
            TokenKind::Slash => self.builder.build_float_div(left, right, "div").into(),
            TokenKind::Greater => self.builder.build_float_compare(FloatPredicate::OGT, left, right, "gt").into(),
            TokenKind::GreaterEqual => self.builder.build_float_compare(FloatPredicate::OGE, left, right, "ge").into(),
            TokenKind::Less => self.builder.build_float_compare(FloatPredicate::OLT, left, right, "lt").into(),
            TokenKind::LessEqual => self.builder.build_float_compare(FloatPredicate::OLE, left, right, "le").into(),
            TokenKind::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, left, right, "eq").into(),
            TokenKind::NotEqual => self.builder.build_float_compare(FloatPredicate::ONE, left, right, "ne").into(),
            _ => panic!("Unexpected token"),
        } 
    }

    fn visit_binary_expr_float_int(&mut self, left: FloatValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let right = self.builder.build_signed_int_to_float(right, self.context.f64_type(), "int_to_float");

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_float_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_float_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_float_mul(left, right, "mul").into(),
            TokenKind::Slash => self.builder.build_float_div(left, right, "div").into(),
            TokenKind::Greater => self.builder.build_float_compare(FloatPredicate::OGT, left, right, "gt").into(),
            TokenKind::GreaterEqual => self.builder.build_float_compare(FloatPredicate::OGE, left, right, "ge").into(),
            TokenKind::Less => self.builder.build_float_compare(FloatPredicate::OLT, left, right, "lt").into(),
            TokenKind::LessEqual => self.builder.build_float_compare(FloatPredicate::OLE, left, right, "le").into(),
            TokenKind::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, left, right, "eq").into(),
            TokenKind::NotEqual => self.builder.build_float_compare(FloatPredicate::ONE, left, right, "ne").into(),
            _ => panic!("Unexpected token"),
        } 
    }

    fn visit_binary_expr_float_float(&mut self, left: FloatValue<'ctx>, right: FloatValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        match expr.op.kind {
            TokenKind::Plus => self.builder.build_float_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_float_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_float_mul(left, right, "mul").into(),
            TokenKind::Slash => self.builder.build_float_div(left, right, "div").into(),
            TokenKind::Greater => self.builder.build_float_compare(FloatPredicate::OGT, left, right, "gt").into(),
            TokenKind::GreaterEqual => self.builder.build_float_compare(FloatPredicate::OGE, left, right, "ge").into(),
            TokenKind::Less => self.builder.build_float_compare(FloatPredicate::OLT, left, right, "lt").into(),
            TokenKind::LessEqual => self.builder.build_float_compare(FloatPredicate::OLE, left, right, "le").into(),
            TokenKind::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, left, right, "eq").into(),
            TokenKind::NotEqual => self.builder.build_float_compare(FloatPredicate::ONE, left, right, "ne").into(),
            _ => panic!("Unexpected token"),
        } 
    }
}
