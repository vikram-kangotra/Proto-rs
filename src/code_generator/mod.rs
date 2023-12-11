pub mod code_generator;

use inkwell::{builder::Builder, context::Context};

pub struct CodeGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
}
