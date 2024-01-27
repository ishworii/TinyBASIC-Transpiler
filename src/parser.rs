use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashSet;
use crate::emitter::Emitter;

#[derive(Debug)]
pub struct Parser{
    lexer : Lexer,
    emitter:Emitter,
    curr_token : Token,
    peek_token : Token,
    symbols : HashSet<String>,
    labels_declared : HashSet<String>,
    labels_gotoed : HashSet<String>,
}


impl Parser{
    pub fn new(lexer: Lexer,emitter: Emitter) -> Self{
        let mut parser = Parser{
            lexer,
            emitter,
            curr_token : Token::default(),
            peek_token: Token::default(),
            symbols : HashSet::new(),
            labels_declared : HashSet::new(),
            labels_gotoed : HashSet::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    //check if current token matches
    pub fn check_token(&mut self,token_type: TokenType) -> bool{
        self.curr_token.token_type == token_type

    }

    //check if next token matches
    pub fn check_peek(&mut self, token_type: TokenType) -> bool{
        self.peek_token.token_type == token_type
    }

    //try to match the current token, if not match, error
    pub fn match_token(&mut self, token_type:TokenType){
        if !self.check_token(token_type){
            let message = format!("Expected {}, got {}",token_type.as_ref(),self.curr_token.token_type.as_ref());
            self.abort(message);
        }
        self.next_token();
    }

    //abort function
    fn abort(&self,message:String){
        panic!("Parsing error, {}",message);
    }

    //advances the next token
    pub fn next_token(&mut self){
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token().unwrap();
    }

    //parser rules

    //program ::= {statement}
    pub fn program(&mut self){
        // println!("PROGRAM");
        self.emitter.header_line(String::from("#include<stdio.h>"));
        self.emitter.header_line(String::from("int main(void) {"));

        //remove newlines at the start if any
        while self.check_token(TokenType::NEWLINE){
            self.next_token();
        }

        while !self.check_token(TokenType::EOF){
            self.statement();
        }
        self.emitter.emit_line(String::from("return 0;"));
        self.emitter.emit_line(String::from("}"));

        //check if labels referenced by GOTO are declared
        for label in &self.labels_gotoed{
            if !self.labels_declared.contains(label){
                let message = format!("Attempting GOTO to an undeclared label, {}",label);
                self.abort(message);
            }
        }

        //write to file
        self.emitter.write_to_file().expect("Failed to open output C file");
    }

    //statement ::= "PRINT" (expression | string) nl
    pub fn statement(&mut self){
        //PRINT (expression|string)
        if self.check_token(TokenType::PRINT){
            // println!("STATEMENT-PRINT");
            self.next_token();

            if self.check_token(TokenType::STRING){
                let code = format!("printf(\"{}\\n\");",self.curr_token.text);
                self.emitter.emit_line(code);
                self.next_token();
            }
            else{
                self.emitter.emit_line(String::from("printf(\"%.2f\\n\", (float)("));
                self.expression();
                self.emitter.emit_line(String::from("));"));
            }
        }
        //"IF" comparison "THEN" {statement} "ENDIF"
        else if self.check_token(TokenType::IF){
            // println!("STATEMENT-IF");
            self.next_token();
            self.emitter.emit_line(String::from("if("));
            self.comparison();

            self.match_token(TokenType::THEN);
            self.new_line();
            self.emitter.emit_line(String::from("){"));

            //zero or more statements in the body
            while !self.check_token(TokenType::ENDIF){
                self.statement();
            }
            self.match_token(TokenType::ENDIF);
            self.emitter.emit_line(String::from("}"));
        }
        //"WHILE" comparison "REPEAT" {statement} "END  WHILE"
        else if self.check_token(TokenType::WHILE){
            // println!("STATEMENT-WHILE");
            self.next_token();
            self.emitter.emit_line(String::from("while("));
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.new_line();
            self.emitter.emit_line(String::from("){"));

            while !self.check_token(TokenType::ENDWHILE){
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE);
            self.emitter.emit_line(String::from("}"));

        }
        //LABEL ident
        else if self.check_token(TokenType::LABEL){
            // println!("STATEMENT-LABEL");
            self.next_token();

            //make sure this label doesn't already exist
            if self.labels_declared.contains(&self.curr_token.text){
                let message = format!("Label already exists {}",self.curr_token.text);
                self.abort(message);
            }
            self.labels_declared.insert(self.curr_token.text.clone());
            let code = format!("{}:",self.curr_token.text);
            self.emitter.emit_line(code);

            self.match_token(TokenType::IDENT);
        }
        //GOTO ident
        else if self.check_token(TokenType::GOTO){
            // println!("STATEMENT-GOTO");
            self.next_token();
            self.labels_gotoed.insert(self.curr_token.text.clone());
            let code = format!("goto {} ;",self.curr_token.text);
            self.emitter.emit_line(code);
            self.match_token(TokenType::IDENT);
        }
        //"LET" ident = expression
        else if self.check_token(TokenType::LET){
            // println!("STATEMENT-LET");
            self.next_token();

            //if variable doesn't exist, declare it
            if !self.symbols.contains(&self.curr_token.text){
                self.symbols.insert(self.curr_token.text.clone());
                let code = format!("float {};",self.curr_token.text);
                self.emitter.header_line(code);
            }

            let code = format!("{}=",self.curr_token.text);
            self.emitter.emit(code);
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            self.expression();
            self.emitter.emit_line(String::from(";"));
        }
        //INPUT ident
        else if self.check_token(TokenType::INPUT){
            // println!("STATEMENT-INPUT");
            self.next_token();
            if !self.symbols.contains(&self.curr_token.text){
                self.symbols.insert(self.curr_token.text.clone());
                let code = format!("float {};",self.curr_token.text);
                self.emitter.header_line(code);
            }
            let code = format!("if(0==scanf(\"%f\",&{})) {{",self.curr_token.text);
            self.emitter.emit_line(code);
            let code = format!("{} = 0;",self.curr_token.text);
            self.emitter.emit_line(code);
            self.emitter.emit(String::from("scanf(\"%"));
            self.emitter.emit_line(String::from("*s\");"));
            self.emitter.emit_line(String::from("}"));
            self.match_token(TokenType::IDENT);
        }
        else{
            let message = format!("Invalid statement at {} ({})",self.curr_token.text,self.curr_token.token_type.as_ref());
            self.abort(message);
        }
        self.new_line();

    }

    //nl ::= '\n'+
    pub fn new_line(&mut self){
        //println!("NEWLINE");
        // match at least one new line
        self.match_token(TokenType::NEWLINE);
        //allow for multiple new lines
        while self.check_token(TokenType::NEWLINE){
            self.next_token();
        }
    }

    //comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    pub fn comparison(&mut self){
        // println!("COMPARISON");
        self.expression();
        if self.is_comparison_operator(){
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
            self.expression();
        }
        else{
            let message = format!("Expected comparison operator at {}",self.curr_token.text);
            self.abort(message);
        }

        while self.is_comparison_operator(){
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
            self.expression();
        }
    }

    pub fn is_comparison_operator(&mut self) -> bool{
        self.check_token(TokenType::GT) || self.check_token(TokenType::GTEQ) || self.check_token(TokenType::LT) || self.check_token(TokenType::LTEQ) || self.check_token(TokenType::EQEQ) || self.check_token(TokenType::NOTEQ)
    }

    //expression ::= term {( "-" | "+" ) term}
    pub fn expression(&mut self){
        // println!("EXPRESSION");
        self.term();
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS){
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
            self.term();
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    pub fn term(&mut self){
        //println!("TERM");
        self.unary();
        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH){
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
            self.unary();
        }
    }

    //unary ::= ["+" | "-"] primary
    pub fn unary(&mut self){
        //println!("UNARY");
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS){
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
        }
        self.primary();
    }

    //primary ::= number | ident
    pub fn primary(&mut self){
        //println!("Primary ({})",self.curr_token.text);
        if self.check_token(TokenType::NUMBER) {
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
        }
        else if self.check_token(TokenType::IDENT){
            //ensure variable already exists
            if !self.symbols.contains(&self.curr_token.text.clone()){
                let message = format!("Referencing variable before assignment, {}",self.curr_token.text);
                self.abort(message);
            }
            self.emitter.emit(self.curr_token.text.clone());
            self.next_token();
        }
        else{
            let message = format!("Unexpected token at {}",self.curr_token.text);
            // println!("{}", message);
            self.abort(message);
        }
    }

}