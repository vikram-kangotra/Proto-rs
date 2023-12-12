pub mod code_generator;

use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, values::PointerValue, types::BasicTypeEnum};

#[derive(Eq, PartialEq)]
pub struct VariableInfo<'ctx> {
    type_: BasicTypeEnum<'ctx>,
    alloca: PointerValue<'ctx>,
}

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    symbol_table: HashMap<String, VariableInfo<'ctx>>,
}
