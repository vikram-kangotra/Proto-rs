pub mod code_generator;

use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, values::{PointerValue, BasicValueEnum, FunctionValue}, types::BasicTypeEnum, basic_block::BasicBlock, module::Module};

use crate::frontend::stmt::FunctionDeclStmt;

#[derive(Eq, PartialEq)]
pub struct VariableInfo<'ctx> {
    type_: BasicTypeEnum<'ctx>,
    alloca: PointerValue<'ctx>,
}

#[derive(Eq, PartialEq)]
pub struct FunctionInfo<'ctx> {
    func_decl: FunctionDeclStmt,
    params: HashMap<String, BasicValueEnum<'ctx>>,
}

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    symbol_table: Vec<HashMap<String, VariableInfo<'ctx>>>,
    function_table: HashMap<FunctionValue<'ctx>, FunctionInfo<'ctx>>,
    break_block_stack: Vec<BasicBlock<'ctx>>,
    continue_block_stack: Vec<BasicBlock<'ctx>>,
}
