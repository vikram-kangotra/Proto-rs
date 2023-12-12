pub mod code_generator;

use inkwell::{builder::Builder, context::Context};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
}
