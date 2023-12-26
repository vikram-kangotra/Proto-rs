pub mod compiler;

use crate::{code_generator::CodeGenerator, frontend::parser::Parser};

pub struct Compiler<'ctx> {
    parser: Parser,
    generator: CodeGenerator<'ctx>,
}
