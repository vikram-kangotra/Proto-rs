use std::collections::HashMap;

use inkwell::FloatPredicate;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{IntValue, FloatValue, BasicMetadataValueEnum};
use inkwell::{builder::Builder, values::BasicValueEnum};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr, VariableExpr, VarAssignExpr, CallExpr, IntType, LiteralType, FloatType};
use crate::frontend::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt};
use crate::frontend::visitor::Visitor;
use crate::frontend::token::TokenKind;

use super::{VariableInfo, FunctionInfo};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>) -> CodeGenerator<'ctx> {
        CodeGenerator {
            context,
            module,
            builder,
            symbol_table: vec![HashMap::new()],
            function_table: HashMap::new(),
            break_block_stack: vec![],
            continue_block_stack: vec![],
        }
    }

    pub fn generate_code(&mut self, stmt: &dyn Stmt<'ctx>) {
        stmt.accept(self)
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    fn enter_scope(&mut self) {
        self.symbol_table.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn get_type(&self, type_: &str) -> BasicTypeEnum<'ctx> {
        match type_ {
            "i8" => self.context.i8_type().into(),
            "i16" => self.context.i16_type().into(),
            "i32" => self.context.i32_type().into(),
            "i64" => self.context.i64_type().into(),
            "f32" => self.context.f32_type().into(),
            "f64" => self.context.f64_type().into(),
            _ => panic!("Unknown type {}", type_),
        }
    }

    fn extract_string(&self, string: &str) -> String { 
        if string.starts_with("\"") && string.ends_with("\"") {
            string[1..string.len() - 1].to_owned()
        } else {
            string.to_owned()
        }
    }
 
    fn check_type_match(&self, expected: &str, actual: BasicTypeEnum<'ctx>) {
        let expected = self.extract_string(expected);
        let actual = self.extract_string(&actual.to_string());
        if expected != actual {
            panic!("Expected {}, got {}", expected, actual);
        }
    }
}

impl<'ctx> Visitor<'ctx> for CodeGenerator<'ctx> {

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt<'ctx>) {
        stmt.expr.accept(self);
    }

    fn visit_var_decl_stmt(&mut self, stmt: &VarDeclStmt<'ctx>) {
        let name = &stmt.name;
        let value = stmt.expr.accept(self);

        if let Some(type_) = &stmt.type_ {
            self.check_type_match(type_, value.get_type());
        }

        let alloca = self.builder.build_alloca(value.get_type(), name);
        self.builder.build_store(alloca, value);

        let variable_info = VariableInfo {
            type_: value.get_type(),
            alloca,
        };
        if let Some(scope) = self.symbol_table.last_mut() {
            scope.insert(name.to_owned(), variable_info);
        }
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt<'ctx>) {
        let value = stmt.expr.accept(self);
       
        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let return_type = self.function_table.get(&function).unwrap().return_type.as_ref();

        if let Some(return_type) = return_type {
            self.check_type_match(&return_type.to_string(), value.get_type());
        } else {
            self.function_table.get_mut(&function).unwrap().return_type = Some(value.get_type());
        }

        self.builder.build_return(Some(&value));

        let end_block = self.context.append_basic_block(function, "end");
        self.builder.position_at_end(end_block);
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt<'ctx>) {
        self.enter_scope();
        for stmt in &stmt.stmts {
            stmt.accept(self);
        }
        self.exit_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt<'ctx>) {
        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let cond_block = self.context.append_basic_block(function, "if_cond");
        let then_block = self.context.append_basic_block(function, "then");
        let end_block = self.context.append_basic_block(function, "if_end");

        let else_block = if stmt.otherwise.is_some() {
            self.context.append_basic_block(function, "else")
        } else {
            end_block
        };

        self.builder.build_unconditional_branch(cond_block);
        self.builder.position_at_end(cond_block);
        let condition = stmt.cond.accept(self);
        let condition = condition.into_int_value();

        self.builder.build_conditional_branch(condition, then_block, else_block);

        self.builder.position_at_end(then_block);
        stmt.then.accept(self);
        self.builder.build_unconditional_branch(end_block);

        if let Some(otherwise) = &stmt.otherwise {
            self.builder.position_at_end(else_block);
            otherwise.accept(self);
            self.builder.build_unconditional_branch(end_block);
        }

        self.builder.position_at_end(end_block);
    }
    
    fn visit_while_stmt(&mut self, stmt: &WhileStmt<'ctx>) {
        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let cond_block = self.context.append_basic_block(function, "while_cond");
        let body_block = self.context.append_basic_block(function, "body");
        let end_block = self.context.append_basic_block(function, "while_end");

        self.builder.build_unconditional_branch(cond_block);

        self.builder.position_at_end(cond_block);
        let condition = stmt.cond.accept(self);
        let condition = condition.into_int_value();
        self.builder.build_conditional_branch(condition, body_block, end_block);

        self.builder.position_at_end(body_block);

        self.continue_block_stack.push(cond_block);
        self.break_block_stack.push(end_block);
        stmt.body.accept(self);
        self.break_block_stack.pop();
        self.continue_block_stack.pop();

        self.builder.build_unconditional_branch(cond_block);

        self.builder.position_at_end(end_block);
    }

    fn visit_break_stmt(&mut self, _stmt: &BreakStmt) {
        let break_block = self.break_block_stack.last();
        if let Some(break_block) = break_block {
            self.builder.build_unconditional_branch(*break_block);
            let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
            let end_block = self.context.append_basic_block(function, "break_end");
            self.builder.position_at_end(end_block);
        } else {
            panic!("Break statement outside of loop");
        }
    }

    fn visit_continue_stmt(&mut self, _stmt: &ContinueStmt) {
        let continue_block = self.continue_block_stack.last();
        if let Some(continue_block) = continue_block {
            self.builder.build_unconditional_branch(*continue_block);
            let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
            let end_block = self.context.append_basic_block(function, "continue_end");
            self.builder.position_at_end(end_block);
        } else {
            panic!("Continue statement outside of loop");
        }
    }

    fn visit_function_decl_stmt(&mut self, stmt: &FunctionDeclStmt) {
        let name = &stmt.name;
        let params = &stmt.params;
        let param_types = params.iter().map(|param| self.get_type(&param.type_).into()).collect::<Vec<BasicMetadataTypeEnum>>();

        let function_type = if let Some(return_type) = &stmt.return_type {
            match return_type.as_str() {
                "()"=> self.context.void_type().fn_type(&param_types, false),
                "i8" => self.context.i8_type().fn_type(&param_types, false), 
                "i16" => self.context.i16_type().fn_type(&param_types, false),
                "i32" => self.context.i32_type().fn_type(&param_types, false),
                "i64" => self.context.i64_type().fn_type(&param_types, false),
                _ => panic!("Unsupported return type"),
            }
        } else {
            self.context.void_type().fn_type(&param_types, false)
        };

        if self.module.get_function(name).is_some() {
            panic!("Function `{}` already exists", name);
        }

        self.module.add_function(name, function_type, None);
    }

    fn visit_function_def_stmt(&mut self, stmt: &FunctionDefStmt<'ctx>) {
        stmt.func_decl.accept(self);

        let function = self.module.get_function(&stmt.func_decl.name).unwrap();

        let mut params = HashMap::new();
        for (i, param) in function.get_param_iter().enumerate() {
            param.set_name(&stmt.func_decl.params[i].name);
            params.insert(stmt.func_decl.params[i].name.clone(), param);
        }

        let function_info = FunctionInfo {
            params,
            return_type: if let Some(return_type) = &stmt.func_decl.return_type {
                Some(self.get_type(return_type))
            } else {
                None
            },
        };

        self.function_table.insert(function, function_info);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        stmt.body.accept(self);
    }

    fn visit_call_expr(&mut self, expr: &CallExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let name = &expr.callee;

        let function = match self.module.get_function(name) {
            Some(function) => function,
            None => panic!("Function '{}' not defined", name),
        };

        if expr.args.len() != function.count_params() as usize {
            panic!("Function '{}' takes {} arguments, but {} were supplied", name, function.count_params(), expr.args.len());
        }

        let args = expr.args.iter().map(|arg| arg.accept(self).into()).collect::<Vec<BasicMetadataValueEnum>>();

        let ret_value = self.builder
            .build_call(function, &args, &name)
            .try_as_basic_value().left();

        ret_value.unwrap_or_else(|| self.context.i32_type().const_zero().into())
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> BasicValueEnum<'ctx> {
        match expr.value {
            LiteralType::Int(value) => {
                match value {
                    IntType::U8(value) => self.context.i8_type().const_int(value as u64, false).into(),
                    IntType::U16(value) => self.context.i16_type().const_int(value as u64, false).into(),
                    IntType::U32(value) => self.context.i32_type().const_int(value as u64, false).into(),
                    IntType::U64(value) => self.context.i64_type().const_int(value, false).into(),
                }
            }
            LiteralType::Float(value) => {
                match value {
                    FloatType::F32(value) => self.context.f32_type().const_float(value as f64).into(),
                    FloatType::F64(value) => self.context.f64_type().const_float(value).into(),
                }
            }
            LiteralType::Bool(value) => self.context.bool_type().const_int(value as u64, false).into(),
            LiteralType::Char(value) => self.context.i8_type().const_int(value as u64, false).into(),
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> BasicValueEnum<'ctx> {
        let name = &expr.name;

        for scope in self.symbol_table.iter().rev() {
            if let Some(variable_info) = scope.get(name) {
                let alloca = variable_info.alloca;
                return self.builder.build_load(variable_info.type_, alloca, &name);
            }
        }

        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        if let Some(param) = self.function_table.get(&function).unwrap().params.get(name) {
            return param.clone();
        }

        panic!("Variable '{}' not found in current scope", name);
    }
    
    fn visit_var_assign_expr(&mut self, expr: &VarAssignExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let name = &expr.name;
        let value = expr.value.accept(self);

        for scope in self.symbol_table.iter().rev() {
            if let Some(variable_info) = scope.get(name) {
                let alloca = variable_info.alloca;
                self.builder.build_store(alloca, value);
                return value;
            }
        }

        panic!("Variable '{}' not found in current scope", name);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> BasicValueEnum<'ctx> {
        let operand = expr.right.accept(self);

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
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);

        match (left, right) {
            (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_int_int(left, right, expr),
            (BasicValueEnum::IntValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_int_float(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_float_int(left, right, expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_float_float(left, right, expr),
            _ => panic!("Unexpected token: left: {:?}, right: {:?}", left, right),
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
