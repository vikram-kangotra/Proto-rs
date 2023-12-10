pub mod compiler;

use inkwell::{builder::Builder, context::Context, module::Module};

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}
