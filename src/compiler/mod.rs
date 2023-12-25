pub mod compiler;

use inkwell::{context::Context, module::Module};

use crate::code_generator::CodeGenerator;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    generator: CodeGenerator<'ctx>,
}
