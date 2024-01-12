use std::collections::HashMap;

use inkwell::FloatPredicate;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{FloatValue, IntValue, BasicMetadataValueEnum, ArrayValue};
use inkwell::{builder::Builder, values::BasicValueEnum};
use crate::code_generator::CodeGenerator;
use crate::frontend::expr::{BinaryExpr, LiteralExpr, UnaryExpr, VariableExpr, VarAssignExpr, CallExpr, ListExpr, IndexExpr};
use crate::frontend::stmt::{Stmt, ExprStmt, VarDeclStmt, ReturnStmt, BlockStmt, IfStmt, WhileStmt, BreakStmt, ContinueStmt, FunctionDeclStmt, FunctionDefStmt};
use crate::frontend::type_::{Type, LiteralType, self};
use crate::frontend::value::{self, Value, IntegerValue, FloatingValue};
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

    fn get_type(&self, type_: &Type) -> BasicTypeEnum<'ctx> {
        match type_ {
            Type::Literal(LiteralType::Int(type_::IntType::U8)) |
            Type::Literal(LiteralType::Int(type_::IntType::I8)) => self.context.i8_type().into(),
            Type::Literal(LiteralType::Int(type_::IntType::U16)) |
            Type::Literal(LiteralType::Int(type_::IntType::I16)) => self.context.i16_type().into(),
            Type::Literal(LiteralType::Int(type_::IntType::U32)) |
            Type::Literal(LiteralType::Int(type_::IntType::I32)) => self.context.i32_type().into(),
            Type::Literal(LiteralType::Int(type_::IntType::U64)) |
            Type::Literal(LiteralType::Int(type_::IntType::I64)) => self.context.i64_type().into(),
            Type::Literal(LiteralType::Float(type_::FloatType::F32)) => self.context.f32_type().into(),
            Type::Literal(LiteralType::Float(type_::FloatType::F64)) => self.context.f64_type().into(),
            _ => panic!("Unknown type {:?}", type_),
        }
    }

    fn extract_string(&self, string: &str) -> String { 
        if string.starts_with("\"") && string.ends_with("\"") {
            string[1..string.len() - 1].to_owned()
        } else {
            string.to_owned()
        }
    }
 
    fn check_type_match(&self, expected: &str, actual: &str) {
        let expected = self.extract_string(expected);
        let actual = self.extract_string(&actual.to_string());
        if expected != actual {
            panic!("Expected {}, got {}", expected, actual);
        }
    }

    fn get_variable_info(&self, name: &str) -> Option<&VariableInfo<'ctx>> {
        for scope in self.symbol_table.iter().rev() {
            if let Some(variable_info) = scope.get(name) {
                return Some(variable_info);
            }
        }

        None
    }
}

impl<'ctx> Visitor<'ctx> for CodeGenerator<'ctx> {

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt<'ctx>) {
        stmt.expr.accept(self);
    }

    fn visit_var_decl_stmt(&mut self, stmt: &VarDeclStmt<'ctx>) {
        let name = &stmt.name;
        let value = stmt.expr.accept(self).as_llvm_basic_value_enum();

        match stmt.type_ {
            Type::Literal(_) => {}
            Type::Inferred => {}
            Type::List(_, _) => {}
            Type::Void => {}
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
        let value = stmt.expr.accept(self).as_llvm_basic_value_enum();
       
        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let return_type = self.function_table.get(&function).unwrap().return_type.as_ref();

        if let Some(return_type) = return_type {
            self.check_type_match(&return_type.to_string(), &value.get_type().to_string());
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
        let condition = stmt.cond.accept(self).as_llvm_basic_value_enum().into_int_value();

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
        let condition = stmt.cond.accept(self).as_llvm_basic_value_enum().into_int_value(); 
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

        let return_type = &stmt.return_type;
        let function_type = match return_type {
            Type::Literal(LiteralType::Int(_)) => self.get_type(return_type).into_int_type().fn_type(&param_types, false),
            Type::Literal(LiteralType::Float(_)) => self.get_type(return_type).into_float_type().fn_type(&param_types, false),
            Type::Void => self.context.void_type().fn_type(&param_types, false),
            Type::Inferred => self.context.void_type().fn_type(&param_types, false),
            _ => panic!("Unsupported return type"),
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

        let return_type = &stmt.func_decl.return_type;
        let return_type = match return_type {
            Type::Literal(LiteralType::Int(_)) => Some(self.get_type(return_type)),
            Type::Literal(LiteralType::Float(_)) => Some(self.get_type(return_type)),
            Type::Inferred => {
                if stmt.func_decl.name == "main" {
                    Some(self.context.i32_type().into())
                } else {
                    None
                }
            }
            _ => panic!("Unsupported return type"),
        };

        let function_info = FunctionInfo {
            params,
            return_type: return_type.into(), 
        };

        self.function_table.insert(function, function_info);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        stmt.body.accept(self);
    }

    fn visit_call_expr(&mut self, expr: &CallExpr<'ctx>) -> Value<'ctx> {
        let name = &expr.callee;

        let function = match self.module.get_function(name) {
            Some(function) => function,
            None => panic!("Function '{}' not defined", name),
        };

        if expr.args.len() != function.count_params() as usize {
            panic!("Function '{}' takes {} arguments, but {} were supplied", name, function.count_params(), expr.args.len());
        }

        let args = expr.args.iter().map(|arg| arg.accept(self).as_llvm_basic_value_enum().into()).collect::<Vec<BasicMetadataValueEnum>>();

        let ret_value = self.builder
            .build_call(function, &args, &name)
            .try_as_basic_value().left();

        Value::LLVMBasicValueEnum(ret_value.unwrap_or_else(|| self.context.i32_type().const_zero().into()))
    }

    fn visit_list_expr(&mut self, expr: &ListExpr<'ctx>) -> Value<'ctx> {

        let mut values = Vec::new();
        for value in &expr.values {
            let value = value.accept(self);
            values.push(value.as_llvm_basic_value_enum());
        }

        let first = values.first().expect("List must have at least one value");
        let type_ = first.get_type();

        for value in &values {
            if value.get_type() != type_ {
                panic!("List values must all be of the same type. Expected {}, found {}", type_, value.get_type());
            }
        }

        match type_ {
            BasicTypeEnum::IntType(_) => {
                let values = values.iter().map(|value| value.into_int_value()).collect::<Vec<IntValue>>();
                let array = type_.into_int_type().const_array(&values);
                Value::LLVMBasicValueEnum(array.into())
            }
            BasicTypeEnum::FloatType(_) => {
                let values = values.iter().map(|value| value.into_float_value()).collect::<Vec<FloatValue>>();
                let array = type_.into_float_type().const_array(&values);
                Value::LLVMBasicValueEnum(array.into())
            }
            BasicTypeEnum::ArrayType(_) => {
                let values = values.iter().map(|value| value.into_array_value()).collect::<Vec<ArrayValue>>();
                let array = type_.into_array_type().const_array(&values);
                Value::LLVMBasicValueEnum(array.into())
            }
            _ => panic!("Unsupported list type"),
        }
    }

    fn visit_index_expr(&mut self, expr: &IndexExpr<'ctx>) -> Value<'ctx> {

        let indices = expr.indices
            .iter()
            .map(|index| index.accept(self).as_llvm_basic_value_enum().into_int_value())
            .collect::<Vec<IntValue>>();

        let variable_info = self.get_variable_info(&expr.variable.name)
                                .expect(format!("Variable '{}' not defined", expr.variable.name).as_str());

        let mut address = variable_info.alloca;
        let mut type_ = variable_info.type_;
        let mut value = self.builder.build_load(type_, address, "value");

        for index in indices {
            if !type_.is_array_type() {
                panic!("Cannot index non-array type");
            }

            let array_type = type_.into_array_type();
            let len = array_type.len();

            let len = self.context.i32_type().const_int(len as u64, false);
            let comparison = self.builder.build_int_compare(inkwell::IntPredicate::ULT, index, len, "comparison");

            let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
            let continue_block = self.context.append_basic_block(function, "continue");
            let error_block = self.context.append_basic_block(function, "error");

            self.builder.build_conditional_branch(comparison, continue_block, error_block);

            self.builder.position_at_end(error_block);
            let exit_fn = self.module.get_function("exit").unwrap();
            self.builder.build_call(exit_fn, &[self.context.i32_type().const_int(1, false).into()], "exit");

            self.builder.build_unconditional_branch(continue_block);

            self.builder.position_at_end(continue_block);

            type_ = array_type.get_element_type();

            unsafe {
                address = self.builder.build_gep(type_, address, &[index], "index");
                value = self.builder.build_load(type_, address, "value");
                type_ = value.get_type();
            }
        }

        Value::LLVMBasicValueEnum(value)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Value<'ctx> {
        match expr.value {
            value::LiteralValue::Int(value) => {
                match value {
                    value::IntValue::I8(value) => self.context.i8_type().const_int(value as u64, false).into(),
                    value::IntValue::I16(value) => self.context.i16_type().const_int(value as u64, false).into(),
                    value::IntValue::I32(value) => self.context.i32_type().const_int(value as u64, false).into(),
                    value::IntValue::I64(value) => self.context.i64_type().const_int(value as u64, false).into(),
                }
            }
            value::LiteralValue::Float(value) => {
                match value {
                    value::FloatValue::F32(value) => self.context.f32_type().const_float(value as f64).into(),
                    value::FloatValue::F64(value) => self.context.f64_type().const_float(value).into(),
                }
            }
            value::LiteralValue::Bool(value) => self.context.bool_type().const_int(value as u64, false).into(),
            value::LiteralValue::Char(value) => self.context.i8_type().const_int(value as u64, false).into(),
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Value<'ctx> {
        let name = &expr.name;

        if let Some(variable_info) = self.get_variable_info(name) {
            let alloca = variable_info.alloca;
            return self.builder.build_load(variable_info.type_, alloca, &name).into();
        }

        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        if let Some(param) = self.function_table.get(&function).unwrap().params.get(name) {
            return param.clone().into();
        }

        panic!("Variable '{}' not found in current scope", name);
    }
    
    fn visit_var_assign_expr(&mut self, expr: &VarAssignExpr<'ctx>) -> Value<'ctx> {
        let name = &expr.name;
        let value = expr.value.accept(self);

        for scope in self.symbol_table.iter().rev() {
            if let Some(variable_info) = scope.get(name) {
                let alloca = variable_info.alloca;
                self.builder.build_store(alloca, value.as_llvm_basic_value_enum());
                return value;
            }
        }

        panic!("Variable '{}' not found in current scope", name);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<'ctx>) -> Value<'ctx> {
        let operand = expr.right.accept(self);

        match operand {
            Value::LLVMBasicValueEnum(BasicValueEnum::IntValue(value)) => self.visit_unary_expr_int(value.into(), expr),
            Value::LLVMBasicValueEnum(BasicValueEnum::FloatValue(value)) => self.visit_unary_expr_float(value.into(), expr),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_int(&mut self, value: IntegerValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx> {

        let value: IntValue = value.into();

        match expr.op.kind {
            TokenKind::Minus => self.builder.build_int_neg(value, "neg").into(),
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_unary_expr_float(&mut self, value: FloatingValue<'ctx>, expr: &UnaryExpr<'ctx>) -> Value<'ctx> {

        let value: FloatValue = value.into();

        match expr.op.kind {
            TokenKind::Minus => self.builder.build_float_neg(value, "neg").into(),
            TokenKind::Plus => value.into(),
            _ => panic!("Unexpected token"),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr<'ctx>) -> Value<'ctx> {
        let left = expr.left.accept(self).as_llvm_basic_value_enum();
        let right = expr.right.accept(self).as_llvm_basic_value_enum();

        match (left, right) {
            (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_int_int(left.into(), right.into(), expr),
            (BasicValueEnum::IntValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_int_float(left.into(), right.into(), expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::IntValue(right)) => self.visit_binary_expr_float_int(left.into(), right.into(), expr),
            (BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) => self.visit_binary_expr_float_float(left.into(), right.into(), expr),
            _ => panic!("Unexpected token: left: {:?}, right: {:?}", left, right),
        }
    }

    fn visit_binary_expr_int_int(&mut self, left: IntegerValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> {

        let left: IntValue = left.into();
        let right: IntValue = right.into();

        if left.get_type() != right.get_type() {
            if left.get_type().get_bit_width() > right.get_type().get_bit_width() {
                let right = self.builder.build_int_z_extend(right, left.get_type(), "z_extend");
                return self.visit_binary_expr_int_int(left.into(), right.into(), expr);
            } else {
                let left = self.builder.build_int_z_extend(left, right.get_type(), "z_extend");
                return self.visit_binary_expr_int_int(left.into(), right.into(), expr);
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

    fn visit_binary_expr_int_float(&mut self, left: IntegerValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> {

        let left: IntValue = left.into();
        let right: FloatValue = right.into();

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

    fn visit_binary_expr_float_int(&mut self, left: FloatingValue<'ctx>, right: IntegerValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> {

        let left: FloatValue = left.into();
        let right: IntValue = right.into();

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

    fn visit_binary_expr_float_float(&mut self, left: FloatingValue<'ctx>, right: FloatingValue<'ctx>, expr: &BinaryExpr<'ctx>) -> Value<'ctx> {

        let left: FloatValue = left.into();
        let right: FloatValue = right.into();
        
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
