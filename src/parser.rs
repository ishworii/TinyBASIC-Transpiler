use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser{
    lexer : Lexer,
    curr_token : Token,
    peek_token : Token,
}


impl Parser{
    pub fn new(lexer: Lexer) -> Self{
        let mut parser = Parser{
            lexer,
            curr_token : Token::default(),
            peek_token: Token::default(),
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
        panic!("Lexing error, {}",message);
    }

    //advances the next token
    pub fn next_token(&mut self){
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token().unwrap();
    }

    //parser rules

    //program ::= {statement}
    pub fn program(&mut self){
        println!("PROGRAM");

        //remove newlines at the start if any
        while self.check_token(TokenType::NEWLINE){
            self.next_token();
        }

        while !self.check_token(TokenType::EOF){
            self.statement();
        }
    }

    //statement ::= "PRINT" (expression | string) nl
    pub fn statement(&mut self){
        //PRINT (expression|string)
        if self.check_token(TokenType::PRINT){
            println!("STATEMENT-PRINT");
            self.next_token();

            if self.check_token(TokenType::STRING){
                self.next_token();
            }
            else{
                self.expression();
            }
        }
        //"IF" comparison "THEN" {statement} "ENDIF"
        else if self.check_token(TokenType::IF){
            println!("STATEMENT-IF");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::THEN);
            self.new_line();

            //zero or more statements in the body
            while !self.check_token(TokenType::ENDIF){
                self.statement();
            }
            self.match_token(TokenType::ENDIF);
        }
        //"WHILE" comparison "REPEAT" {statement} "END WHILE"
        else if self.check_token(TokenType::WHILE){
            println!("STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.new_line();

            while !self.check_token(TokenType::ENDWHILE){
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE);

        }
        //LABEL ident
        else if self.check_token(TokenType::LABEL){
            println!("STATEMENT-LABEL");
            self.next_token();
            self.match_token(TokenType::IDENT);
        }
        //GOTO ident
        else if self.check_token(TokenType::GOTO){
            println!("STATEMENT-GOTO");
            self.next_token();
            self.match_token(TokenType::IDENT);
        }
        //"LET" ident = expression
        else if self.check_token(TokenType::LET){
            println!("STATEMENT-LET");
            self.next_token();
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            self.expression();
        }
        //INPUT ident
        else if self.check_token(TokenType::INPUT){
            println!("STATEMENT-INPUT");
            self.next_token();
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
        println!("NEWLINE");
        // match at least one new line
        self.match_token(TokenType::NEWLINE);
        //allow for multiple new lines
        while self.check_token(TokenType::NEWLINE){
            self.next_token();
        }
    }

    //comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    pub fn comparison(&mut self){
        println!("COMPARISON");
        self.expression();
        if self.is_comparison_operator(){
            self.next_token();
            self.expression();
        }
        else{
            let message = format!("Expected comparison operator at {}",self.curr_token.text);
            self.abort(message);
        }

        while self.is_comparison_operator(){
            self.next_token();
            self.expression();
        }
    }

    pub fn is_comparison_operator(&mut self) -> bool{
        self.check_token(TokenType::GT) || self.check_token(TokenType::GTEQ) || self.check_token(TokenType::LT) || self.check_token(TokenType::LTEQ) || self.check_token(TokenType::EQEQ) || self.check_token(TokenType::NOTEQ)
    }

    //expression ::= term {( "-" | "+" ) term}
    pub fn expression(&mut self){
        println!("EXPRESSION");
        self.term();
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS){
            self.next_token();
            self.term();
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    pub fn term(&mut self){
        println!("TERM");
        self.unary();
        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH){
            self.next_token();
            self.unary();
        }
    }

    //unary ::= ["+" | "-"] primary
    pub fn unary(&mut self){
        println!("UNARY");
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS){
            self.next_token();
        }
        self.primary();
    }

    //primary ::= number | ident
    pub fn primary(&mut self){
        println!("Primary ({})",self.curr_token.text);
        if self.check_token(TokenType::NUMBER) || self.check_token(TokenType::IDENT){
            self.next_token();
        }
        else{
            let message = format!("Unexpected token at {}",self.curr_token.text);
            self.abort(message);
        }
    }

}