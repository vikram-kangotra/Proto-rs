use inkwell::values::{BasicValueEnum, IntValue, FloatValue};
use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr};

use super::{stmt::{ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt}, expr::{VariableExpr, VarAssignExpr, CallExpr}};

pub trait Visitor<'ctx> {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> BasicValueEnum<'ctx>;
    fn visit_var_assign_expr(&mut self, expr: &VarAssignExpr<'ctx>) -> BasicValueEnum<'ctx>;

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_unary_expr_int(&mut self, value: IntValue<'ctx>, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_unary_expr_float(&mut self, value: FloatValue<'ctx>, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx>;

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_binary_expr_int_int(&mut self, left: IntValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_binary_expr_int_float(&mut self, left: IntValue<'ctx>, right: FloatValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_binary_expr_float_int(&mut self, left: FloatValue<'ctx>, right: IntValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx>;
    fn visit_binary_expr_float_float(&mut self, left: FloatValue<'ctx>, right: FloatValue<'ctx>, expr: &BinaryExpr<'ctx>) -> BasicValueEnum<'ctx>;

    fn visit_call_expr(&mut self, _expr: &CallExpr<'ctx>) -> BasicValueEnum<'ctx>;

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt<'ctx>);
    fn visit_var_decl_stmt(&mut self, stmt: &VarDeclStmt<'ctx>);
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt<'ctx>);
    fn visit_block_stmt(&mut self, stmt: &BlockStmt<'ctx>);
    fn visit_if_stmt(&mut self, stmt: &IfStmt<'ctx>);
    fn visit_while_stmt(&mut self, stmt: &WhileStmt<'ctx>);
    fn visit_break_stmt(&mut self, _stmt: &BreakStmt);
    fn visit_continue_stmt(&mut self, _stmt: &ContinueStmt);

    fn visit_function_decl_stmt(&mut self, stmt: &FunctionDeclStmt);
    fn visit_function_def_stmt(&mut self, _tmt: &FunctionDefStmt<'ctx>);
}
