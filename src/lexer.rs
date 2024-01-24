use crate::token::{Token, TokenType};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Lexer{
    pub source:String,
    pub curr_char:char,
    pub curr_pos:isize,
}

impl Lexer{
    pub fn new(content:String) -> Self{
        let mut lexer = Lexer{
            source : content + "\n",
            curr_char : '\0',
            curr_pos : -1,
        };
        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self){
        self.curr_pos += 1;
        if self.curr_pos >= self.source.len() as isize {
            self.curr_char = '\0'; //EOF
        }
        else{
            self.curr_char = self.source.chars().nth(self.curr_pos as usize).unwrap();
        }
    }

    pub fn peek(&self)-> char{
        if self.curr_pos + 1 > self.source.len() as isize{
            return '\0';
        }
        self.source.chars().nth((self.curr_pos + 1) as usize).unwrap()
    }

    pub fn get_token(&mut self) -> Option<Token>{
        self.skip_whitespaces();
        self.skip_comments();
        let token = match self.curr_char{
            '+' => Some(Token::new(self.curr_char.to_string(),TokenType::PLUS)),
            '-' => Some(Token::new(self.curr_char.to_string(),TokenType::MINUS)),
            '*' => Some(Token::new(self.curr_char.to_string(),TokenType::ASTERISK)),
            '/' => Some(Token::new(self.curr_char.to_string(),TokenType::SLASH)),
            '\n' => Some(Token::new(self.curr_char.to_string(),TokenType::NEWLINE)),
            '\0' => Some(Token::new(self.curr_char.to_string(),TokenType::EOF)),
            '=' => {
                if self.peek() == '='{
                    let last_char = self.curr_char;
                    self.next_char();
                    let token_text = format!("{}{}",last_char,self.curr_char);
                    Some(Token::new(token_text,TokenType::EQEQ))
                }
                else{
                    Some(Token::new(self.curr_char.to_string(),TokenType::EQ))
                }
            },

            '>' => {
                if self.peek() == '='{
                    let last_char = self.curr_char;
                    self.next_char();
                    let token_text = format!("{}{}",last_char,self.curr_char);
                    Some(Token::new(token_text,TokenType::GTEQ))
                }
                else{
                    Some(Token::new(self.curr_char.to_string(),TokenType::GT))
                }
            },

            '<' => {
                if self.peek() == '='{
                    let last_char = self.curr_char;
                    self.next_char();
                    let token_text = format!("{}{}",last_char,self.curr_char);
                    Some(Token::new(token_text,TokenType::LTEQ))
                }
                else{
                    Some(Token::new(self.curr_char.to_string(),TokenType::LT))
                }
            },

            '!' => {
                if self.peek() == '='{
                    let last_char = self.curr_char;
                    self.next_char();
                    let token_text = format!("{}{}",last_char,self.curr_char);
                    Some(Token::new(token_text,TokenType::NOTEQ))
                }
                else{
                    let message =format!("{}, {}",String::from("Expected != but got !"),self.peek());
                    self.abort(message);
                    None
                }
            },
            //string
            '"' =>{
                self.next_char();
                let start_pos = self.curr_pos as usize;
                while self.curr_char != '"' {
                    if self.curr_char == '\r' || self.curr_char == '\n' || self.curr_char == '\n' || self.curr_char == '\t' || self.curr_char == '\\' || self.curr_char == '%' {
                        self.abort(String::from("Illegal character in string"));
                    }
                    self.next_char();
                }
                let token_text:&str = &self.source[start_pos..self.curr_pos as usize];
                Some(Token::new(token_text.to_string(),TokenType::STRING))
            }

            //digit
            _ if self.curr_char.is_digit(10) =>{
                let start_pos = self.curr_pos as usize;
                while self.peek().is_digit(10){
                    self.next_char();
                }
                if self.peek() == '.'{
                    self.next_char();
                    if !self.peek().is_digit(10){
                        self.abort(String::from("Illegal character in number"));
                        return None;
                    }
                    while self.peek().is_digit(10){
                        self.next_char();
                    }
                }
                let token_text: &str = &self.source[start_pos..(self.curr_pos + 1) as usize];
                Some(Token::new(token_text.to_string(),TokenType::NUMBER))
            }
            //text
            _ if self.curr_char.is_alphabetic() => {
                let start_pos = self.curr_pos as usize;
                while self.peek().is_alphanumeric(){
                    self.next_char();
                }
                let token_text:&str = &self.source[start_pos..(self.curr_pos + 1) as usize];
                let keyword = Lexer::check_keyword(token_text.to_string());
                if keyword == None{
                    Some(Token::new(token_text.to_string(),TokenType::IDENT))
                }
                else{
                    Some(Token::new(token_text.to_string(),keyword.unwrap()))
                }
            }





            _ => None,
        };
        self.next_char();
        token
    }

    fn check_keyword(token_text : String) -> Option<TokenType>{
        for token_type in TokenType::iter(){
            if token_type.as_ref() == token_text && token_type.value() >= 100 && token_type.value() < 200{
                return Some(token_type);
            }
        }
        None

    }

    fn abort(&self,message:String){
        panic!("Lexing error, {}",message);
    }

    fn skip_whitespaces(&mut self){
        while self.curr_char == ' ' || self.curr_char == '\t' || self.curr_char == '\r'{
            self.next_char();
        }
    }

    fn skip_comments(&mut self){
        if self.curr_char == '#'{
            while self.curr_char != '\n'{
                self.next_char();
            }
        }
    }

}
