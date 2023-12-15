use std::collections::HashMap;

use inkwell::FloatPredicate;
use inkwell::context::Context;
use inkwell::values::{IntValue, FloatValue};
use inkwell::{builder::Builder, values::BasicValueEnum};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::frontend::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt};
use crate::frontend::visitor::Visitor;
use crate::frontend::token::TokenKind;

use super::VariableInfo;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context,builder: &'ctx Builder<'ctx>) -> CodeGenerator<'ctx> {
        CodeGenerator {
            context,
            builder,
            symbol_table: HashMap::new(),
        }
    }

    pub fn generate_code(&mut self, stmt: &dyn Stmt<'ctx>) {
        stmt.accept(self)
    }
}

impl<'ctx> Visitor<'ctx> for CodeGenerator<'ctx> {

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt<'ctx>) {
        stmt.expr.accept(self);
    }

    fn visit_var_decl_stmt(&mut self, stmt: &VarDeclStmt<'ctx>) {
        let name = stmt.name.to_owned();
        let value = stmt.expr.as_ref().accept(self);

        let alloca = self.builder.build_alloca(value.get_type(), &name);
        self.builder.build_store(alloca, value);

        let variable_info = VariableInfo {
            type_: value.get_type(),
            alloca,
        };
        self.symbol_table.insert(name, variable_info);
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt<'ctx>) {
        let value = stmt.expr.as_ref().accept(self);
        self.builder.build_return(Some(&value));
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr<'ctx>) -> BasicValueEnum<'ctx> {
        expr.value
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> BasicValueEnum<'ctx> {
        let name = expr.name.to_owned();
        let variable_info = self.symbol_table.get(&name).unwrap();
        let type_ = variable_info.type_;
        let alloca = variable_info.alloca;
        self.builder.build_load(type_, alloca, &name)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let operand = expr.right.as_ref().accept(self);

        match operand {
            BasicValueEnum::IntValue(value) => self.visit_unary_expr_int(value, expr),
            BasicValueEnum::FloatValue(value) => self.visit_unary_expr_float(value, expr),
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
        let left = expr.left.as_ref().accept(self);
        let right = expr.right.as_ref().accept(self);

        match (left, right) {
            (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_int_int(left, right, expr),
            (BasicValueEnum::IntValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_int_float(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_float_int(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_float_float(left, right, expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr_int_int(&mut self, left: IntValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx> {

        if left.get_type() != right.get_type() {
            if left.get_type().get_bit_width() > right.get_type().get_bit_width() {
                let right = self.builder.build_int_z_extend(right, left.get_type(), "z_extend");
                return self.visit_binary_expr_int_int(left, right, expr);
            } else {
                let left = self.builder.build_int_z_extend(left, right.get_type(), "z_extend");
                return self.visit_binary_expr_int_int(left, right, expr);
            }
        }

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_int_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_int_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_int_mul(left, right, "mul").into(),
            TokenKind::Slash => {
                let left = self.builder.build_signed_int_to_float(left, self.context.f64_type(), "int_to_float");
                let right = self.builder.build_signed_int_to_float(right, self.context.f64_type(), "int_to_float");

                if right.is_null() {
                    panic!("Division by zero");
                }

                self.builder.build_float_div(left, right, "div").into()
            }
            TokenKind::Remainder => self.builder.build_int_signed_rem(left, right, "rem").into(),
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
        let left = self.builder.build_signed_int_to_float(left, right.get_type(), "int_to_float");

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_float_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_float_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_float_mul(left, right, "mul").into(),
            TokenKind::Slash => {
                if right.is_null() {
                    panic!("Division by zero");
                }
                self.builder.build_float_div(left, right, "div").into()
            }
            TokenKind::Remainder => self.builder.build_float_rem(left, right, "rem").into(),
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
        let right = self.builder.build_signed_int_to_float(right, left.get_type(), "int_to_float");

        match expr.op.kind {
            TokenKind::Plus => self.builder.build_float_add(left, right, "add").into(),
            TokenKind::Minus => self.builder.build_float_sub(left, right, "sub").into(),
            TokenKind::Asterisk => self.builder.build_float_mul(left, right, "mul").into(),
            TokenKind::Slash => {
                if right.is_null() {
                    panic!("Division by zero");
                }
                self.builder.build_float_div(left, right, "div").into()
            }
            TokenKind::Remainder => self.builder.build_float_rem(left, right, "rem").into(),
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
            TokenKind::Slash => {
                if right.is_null() {
                    panic!("Division by zero");
                }
                self.builder.build_float_div(left, right, "div").into()
            }
            TokenKind::Remainder => self.builder.build_float_rem(left, right, "rem").into(),
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
