mod lexer;
mod token;
mod parser;
mod interpreter;

use std::io::Write;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn eval(source: &str) -> f64 {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let expr = parser.parse();

    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(expr);

    result
}

fn repl() {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let result = eval(&input);

        println!("{}", result);
    }
}

fn main() {

    if std::env::args().len() == 1 {
        repl();
    } else {
        let args: Vec<String> = std::env::args().collect();
        let filename = &args[1];
        let source = std::fs::read_to_string(filename).unwrap();
        let result = eval(&source);
        println!("{}", result);
    }

}
