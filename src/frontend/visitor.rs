use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr};

use super::{stmt::{ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt}, expr::{VariableExpr, VarAssignExpr, CallExpr, ListExpr, IndexExpr}, value::{Value, IntegerValue, FloatingValue}};

pub trait Visitor<'ctx> {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Value<'ctx>;
    fn visit_variable_expr(&mut self, _expr: &VariableExpr) -> Value<'ctx> { unimplemented!("VariableExpr") }
    fn visit_var_assign_expr(&mut self, _expr: &VarAssignExpr<'ctx>) -> Value<'ctx> { unimplemented!("VarAssignExpr") }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_unary_expr_int(&mut self, value: IntegerValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_unary_expr_float(&mut self, value: FloatingValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx>;

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_binary_expr_int_int(&mut self, left: IntegerValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_binary_expr_int_float(&mut self, left: IntegerValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_binary_expr_float_int(&mut self, left: FloatingValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx>;
    fn visit_binary_expr_float_float(&mut self, left: FloatingValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx>;

    fn visit_call_expr(&mut self, _expr: &CallExpr<'ctx>) -> Value<'ctx> { unimplemented!("CallExpr") }
    fn visit_list_expr(&mut self, _expr: &ListExpr<'ctx>) -> Value<'ctx> { unimplemented!("ListExpr") }
    fn visit_index_expr(&mut self, _expr: &IndexExpr<'ctx>) -> Value<'ctx> { unimplemented!("IndexExpr") }

    fn visit_expr_stmt(&mut self, _stmt: &ExprStmt<'ctx>) { unimplemented!("ExprStmt") }
    fn visit_var_decl_stmt(&mut self, _stmt: &VarDeclStmt<'ctx>) { unimplemented!("VarDeclStmt") }
    fn visit_return_stmt(&mut self, _stmt: &ReturnStmt<'ctx>) { unimplemented!("ReturnStmt") }
    fn visit_block_stmt(&mut self, _stmt: &BlockStmt<'ctx>) { unimplemented!("BlockStmt") }
    fn visit_if_stmt(&mut self, _stmt: &IfStmt<'ctx>) { unimplemented!("IfStmt") }
    fn visit_while_stmt(&mut self, _stmt: &WhileStmt<'ctx>) { unimplemented!("WhileStmt") }
    fn visit_break_stmt(&mut self, _stmt: &BreakStmt) { unimplemented!("BreakStmt") }
    fn visit_continue_stmt(&mut self, _stmt: &ContinueStmt) { unimplemented!("ContinueStmt") }

    fn visit_function_decl_stmt(&mut self, _stmt: &FunctionDeclStmt) { unimplemented!("FunctionDeclStmt") }
    fn visit_function_def_stmt(&mut self, _tmt: &FunctionDefStmt<'ctx>) { unimplemented!("FunctionDefStmt") }
}
