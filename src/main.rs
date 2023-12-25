mod code_generator;
mod compiler;
mod frontend;

use clap::{Arg, ArgAction, Command, crate_version, crate_authors, crate_name, crate_description};

use std::path::Path;
use crate::compiler::Compiler;
use inkwell::{targets::FileType, context::Context};

fn main() {

    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("INPUT")
                .help("source proto file to compile")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .short('o')
                .long("output")
                .help("output file")
                .required(true),
        )
        .arg(
            Arg::new("ASSEMBLY")
                .short('S')
                .help("output assembly file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("VERBOSE")
                .short('v')
                .help("verbose output")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let source_file = matches.get_one::<String>("INPUT").unwrap();

    let context = Context::create(); 
    let mut compiler = Compiler::new(&context, source_file);

    compiler.compile().unwrap();

    let output_file = matches.get_one::<String>("OUTPUT").unwrap();
    let output_file = Path::new(output_file);

    let filetype = if matches.get_flag("ASSEMBLY") {
        FileType::Assembly
    } else {
        FileType::Object
    };
    
    let verbose = matches.get_flag("VERBOSE");

    compiler.generate_output(output_file, filetype, verbose).unwrap();
}
