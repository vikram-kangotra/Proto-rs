pub mod code_generator;

use inkwell::builder::Builder;

pub struct CodeGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
}
