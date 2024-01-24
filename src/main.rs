use crate::lexer::Lexer;
use crate::token::TokenType;

mod lexer;
mod token;

fn main() {
    let source = String::from("+- \"This is a string\" # This is a comment!\n */");
    let mut lexer = Lexer::new(source);
    let mut token = lexer.get_token().unwrap();
    while token.token_type != TokenType::EOF{
        println!("{:?}",token.token_type);
        token = lexer.get_token().unwrap();
    }
}