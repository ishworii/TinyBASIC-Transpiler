use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;
use crate::emitter::Emitter;

mod lexer;
mod token;
mod parser;
mod emitter;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2{
        panic!("Command line arguments incorrect!");
    }
    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file.");
    let lexer = Lexer::new(source);
    let emitter = Emitter::new(String::from("out.c"));
    let mut parser = Parser::new(lexer,emitter);
    parser.program();

    println!("Parsing completed!");
}