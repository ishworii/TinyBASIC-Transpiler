use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;

mod lexer;
mod token;
mod parser;

fn main() {
    env::set_var("RUST BACKTRACE","1");
    let args:Vec<String> = env::args().collect();
    if args.len() != 2{
        panic!("Command line arguments incorrect!");
    }
    println!("Tiny Compiler");
    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file.");
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    parser.program();
    println!("Parsing completed!");
}