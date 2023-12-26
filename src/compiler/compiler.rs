use std::path::Path;

use inkwell::{context::Context, targets::{FileType, InitializationConfig, RelocMode, CodeModel, Target, TargetMachine}, OptimizationLevel};

use crate::frontend::{lexer::Lexer, parser::Parser};
use crate::code_generator::CodeGenerator;

use std::fs::read_to_string;

use crate::Compiler;

impl<'ctx> Compiler<'ctx> {

    pub fn new(context: &'ctx Context, source_file: &String) -> Self {

        let source = read_to_string(source_file).unwrap();
        let lexer = Lexer::new(source);
        let parser = Parser::new(lexer);

        let module = context.create_module(source_file);
        let builder = context.create_builder();

        Self {
            parser,
            generator: CodeGenerator::new(context, module, builder),
        }

    }

    pub fn compile(&mut self) -> Result<(), String> {

        let stmts = self.parser.parse();

        for stmt in stmts {
            self.generator.generate_code(stmt.as_ref());
        }
        
        Ok(())
    }

    pub fn generate_output(&mut self, output_filename: &Path, filetype: FileType, verbose: bool) -> Result<(), String> {

        if verbose {
            println!("Generated LLVM IR:");
            println!("{}", self.generator.get_module().print_to_string().to_string());
        }

        Target::initialize_all(&InitializationConfig::default());

        let target_triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        let target = Target::from_triple(&target_triple).map_err(|e| e.to_string())?;

        let target_machine = target.create_target_machine(
            &target_triple,
            &cpu,
            &features,
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or("Could not create target machine")?;

        target_machine
            .write_to_file(&self.generator.get_module(),
                           filetype, 
                           output_filename)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

}
