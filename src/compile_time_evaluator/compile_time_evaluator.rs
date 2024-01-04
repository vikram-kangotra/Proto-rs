use crate::frontend::{visitor::Visitor, stmt::{FunctionDefStmt, FunctionDeclStmt, ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt}, expr::{LiteralExpr, VariableExpr, VarAssignExpr, UnaryExpr, BinaryExpr, CallExpr}, value::{Value, LiteralValue, IntegerValue, FloatingValue, IntValue, FloatValue}, token::TokenKind};

use super::CompileTimeEvaluator;

impl CompileTimeEvaluator {
    pub fn new() -> Self {
        Self
    }
}

impl<'ctx> Visitor<'ctx> for CompileTimeEvaluator {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Value<'ctx> { 
        expr.value.into()
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Value<'ctx> { panic!("CompileTimeEvaluator::visit_variable_expr() should never be called") }
    fn visit_var_assign_expr(&mut self, expr: &VarAssignExpr<'ctx>) -> Value<'ctx> { panic!("CompileTimeEvaluator::visit_var_assign_expr() should never be called") }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> Value<'ctx> { 
        let operand = expr.right.accept(self);

        match operand {
            Value::Literal(LiteralValue::Int(value)) => self.visit_unary_expr_int(value.into(), expr),
            Value::Literal(LiteralValue::Float(value)) => self.visit_unary_expr_float(value.into(), expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_int(&mut self, value: IntegerValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx> { 

        match expr.op.kind {
            TokenKind::Minus => {
                let value: IntValue = value.into();
                let value = -value;
                value.into()
            }
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_float(&mut self, value: FloatingValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx> { 

        match expr.op.kind {
            TokenKind::Minus => {
                match value {
                    FloatingValue::Float(FloatValue::F32(value)) => Value::Literal(LiteralValue::Float(FloatValue::F32(-value))),
                    FloatingValue::Float(FloatValue::F64(value)) => Value::Literal(LiteralValue::Float(FloatValue::F64(-value))),
                    _ => panic!("Unexpected type"),
                }
            }
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> Value<'ctx> { 

        let left = expr.left.accept(self).as_literal();
        let right = expr.right.accept(self).as_literal();

        match (left, right) {
            (LiteralValue::Int(left), LiteralValue::Int(right)) => self.visit_binary_expr_int_int(left.into(), right.into(), expr),
            (LiteralValue::Int(left), LiteralValue::Float(right)) => self.visit_binary_expr_int_float(left.into(), right.into(), expr),
            (LiteralValue::Float(left), LiteralValue::Int(right)) => self.visit_binary_expr_float_int(left.into(), right.into(), expr),
            (LiteralValue::Float(left), LiteralValue::Float(right)) => self.visit_binary_expr_float_float(left.into(), right.into(), expr),
            _ => panic!("Unexpected token: left: {:?}, right: {:?}", left, right),
        }
    }

    fn visit_binary_expr_int_int(&mut self, left: IntegerValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> { 

        let left_: IntValue = left.into();
        let right_: IntValue = right.into();

        let left = match (left_, right_) {
            (IntValue::I8(left), IntValue::I16(_)) => IntValue::I16(left as i16),
            (IntValue::I8(left), IntValue::I32(_)) => IntValue::I32(left as i32),
            (IntValue::I8(left), IntValue::I64(_)) => IntValue::I64(left as i64),
            (IntValue::I16(left), IntValue::I32(_)) => IntValue::I32(left as i32),
            (IntValue::I16(left), IntValue::I64(_)) => IntValue::I64(left as i64),
            (IntValue::I32(left), IntValue::I64(_)) => IntValue::I64(left as i64),
            _ => left_,
        };

        let right = match (left_, right_) {
            (IntValue::I16(_), IntValue::I8(right)) => IntValue::I16(right as i16),
            (IntValue::I32(_), IntValue::I8(right)) => IntValue::I32(right as i32),
            (IntValue::I64(_), IntValue::I8(right)) => IntValue::I64(right as i64),
            (IntValue::I32(_), IntValue::I16(right)) => IntValue::I32(right as i32),
            (IntValue::I64(_), IntValue::I16(right)) => IntValue::I64(right as i64),
            (IntValue::I64(_), IntValue::I32(right)) => IntValue::I64(right as i64),
            _ => right_,
        };

        match expr.op.kind {
            TokenKind::Plus => (left + right).into(),
            TokenKind::Minus => (left - right).into(),
            TokenKind::Asterisk => (left * right).into(),
            TokenKind::Slash => {
                if right.is_zero() {
                    panic!("Division by zero");
                }

                (left / right).into()
            }
            TokenKind::Remainder => {
                if right.is_zero() {
                    panic!("Division by zero");
                }

                (left % right).into()
            }
            TokenKind::Greater => (left > right).into(),
            TokenKind::GreaterEqual => (left >= right).into(),
            TokenKind::Less => (left < right).into(),
            TokenKind::LessEqual => (left <= right).into(),
            TokenKind::Equal => (left == right).into(),
            TokenKind::NotEqual => (left != right).into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr_int_float(&mut self, left: IntegerValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> { 

        let left: FloatingValue = left.into();
        let right: FloatingValue = right.into();

        return self.visit_binary_expr_float_float(left.into(), right.into(), expr);
    }

    fn visit_binary_expr_float_int(&mut self, left: FloatingValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> { 

        let left: FloatingValue = left.into();
        let right: FloatingValue = right.into();

        return self.visit_binary_expr_float_float(left.into(), right.into(), expr);
    }

    fn visit_binary_expr_float_float(&mut self, left: FloatingValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> { 

        let left_: FloatValue = left.into();
        let right_: FloatValue = right.into();

        let left = match (left_, right_) {
            (FloatValue::F32(left), FloatValue::F64(_)) => FloatValue::F64(left as f64),
            _ => left_,
        };

        let right = match (left_, right_) {
            (FloatValue::F64(_), FloatValue::F32(right)) => FloatValue::F64(right as f64),
            _ => right_,
        };

        match expr.op.kind {
            TokenKind::Plus => (left + right).into(),
            TokenKind::Minus => (left - right).into(),
            TokenKind::Asterisk => (left * right).into(),
            TokenKind::Slash => {
                if right.is_zero() {
                    panic!("Division by zero");
                }

                (left / right).into()
            }
            TokenKind::Remainder => {
                if right.is_zero() {
                    panic!("Division by zero");
                }

                (left % right).into()
            }
            TokenKind::Greater => (left > right).into(),
            TokenKind::GreaterEqual => (left >= right).into(),
            TokenKind::Less => (left < right).into(),
            TokenKind::LessEqual => (left <= right).into(),
            TokenKind::Equal => (left == right).into(),
            TokenKind::NotEqual => (left != right).into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_call_expr(&mut self, expr: &CallExpr<'ctx>) -> Value<'ctx> { panic!("CompileTimeEvaluator::visit_call_expr() should never be called") }

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_expr_stmt() should never be called") }
    fn visit_var_decl_stmt(&mut self, stmt: &VarDeclStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_var_decl_stmt() should never be called") }
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_return_stmt() should never be called") }
    fn visit_block_stmt(&mut self, stmt: &BlockStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_block_stmt() should never be called") }
    fn visit_if_stmt(&mut self, stmt: &IfStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_if_stmt() should never be called") }
    fn visit_while_stmt(&mut self, stmt: &WhileStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_while_stmt() should never be called") }
    fn visit_break_stmt(&mut self, _stmt: &BreakStmt) { panic!("CompileTimeEvaluator::visit_break_stmt() should never be called") }
    fn visit_continue_stmt(&mut self, _stmt: &ContinueStmt) { panic!("CompileTimeEvaluator::visit_continue_stmt() should never be called") }

    fn visit_function_decl_stmt(&mut self, stmt: &FunctionDeclStmt) { panic!("CompileTimeEvaluator::visit_function_decl_stmt() should never be called") }
    fn visit_function_def_stmt(&mut self, _stmt: &FunctionDefStmt<'ctx>) { panic!("CompileTimeEvaluator::visit_function_def_stmt() should never be called") }
}
