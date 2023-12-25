pub mod code_generator;

use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, values::PointerValue, types::BasicTypeEnum, basic_block::BasicBlock, module::Module};

#[derive(Eq, PartialEq)]
pub struct VariableInfo<'ctx> {
    type_: BasicTypeEnum<'ctx>,
    alloca: PointerValue<'ctx>,
}

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    symbol_table: Vec<HashMap<String, VariableInfo<'ctx>>>,
    break_block_stack: Vec<BasicBlock<'ctx>>,
    continue_block_stack: Vec<BasicBlock<'ctx>>,
}
