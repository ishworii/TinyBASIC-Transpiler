use strum_macros::{EnumIter,AsRefStr,EnumString};
#[derive(EnumIter, AsRefStr, EnumString, Debug,PartialEq,Clone,Copy)]
pub enum TokenType{
    EOF = -1,
    NEWLINE = 0,
    NUMBER = 1,
    IDENT = 2,
    STRING = 3,
    //keywords
    LABEL = 101,
    GOTO =102,
    PRINT = 103,
    INPUT = 104,
    LET = 105,
    IF = 106,
    THEN=107,
    ENDIF=108,
    WHILE=109,
    REPEAT=110,
    ENDWHILE=111,
    //operators
    EQ = 201,
    PLUS=202,
    MINUS=203,
    ASTERISK=204,
    SLASH=205,
    EQEQ=206,
    NOTEQ=207,
    LT=208,
    LTEQ=209,
    GT=210,
    GTEQ=211,
}

impl TokenType{
    pub fn value(&self) -> i32{
        *self as i32
    }
}

#[derive(Debug, Clone)]
pub struct Token{
    pub text : String,
    pub token_type : TokenType,
}

impl Default for Token{
    fn default() -> Self {
        Token{text: String::from(""),token_type:TokenType::EOF}
    }
}

impl Token{
    pub fn new(token_text:String,token_kind: TokenType)-> Self{
        Token{
            text : token_text,
            token_type : token_kind,
        }
    }
}