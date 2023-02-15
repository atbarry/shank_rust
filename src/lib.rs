use std::fmt;
use std::fmt::Display;
use std::convert::AsRef;
use strum_macros::AsRefStr;

pub mod lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    value: Option<String>,
    line: u32,
}

#[derive(Clone, Debug, PartialEq, AsRefStr)]
pub enum TokenType {
    INDENT,
    DEDENT,
    IDENTIFIER,
    NUMBER,
    ENDOFLINE,
    CHARACTERLITERAL,
    STRINGLITERAL,
    IF,
    ELSIF,
    ELSE,
    THEN,
    WHILE,
    FOR,
    REPEAT,
    UNTIL,
    FROM,
    TO,
    DEFINE,
    VAR,
    ARRAY,
    OF,
    VARIABLES,
    CONSTANTS,
    INTEGER,
    REAL,
    BOOLEAN,
    MOD,
    PLUS, // Chacters
    MINUS, 
    MULTIPLY, 
    DIVIDE,
    EQUALS, 
    NOTEQUAL,
    LESSTHAN,
    LESSTHANOREQUAL,
    GREATERTHAN,
    GREATERTHANOREQUAL,
    ASSIGN, 
    COLON, 
    SEMICOLON, 
    COMMA, 
    LEFTPARENTHESIS, 
    RIGHTPARENTHESIS,
    RIGHTSQUAREBRACKET, 
    LEFTSQUAREBRACKET, 
}

impl Token {
    pub fn new_with_value(ttype: TokenType, value: &str, line: u32) -> Token {
        Token { ttype, value: Some(value.to_owned()), line }
    }

    pub fn new(ttype: TokenType, line: u32) -> Token {
        Token { ttype, value: None, line }
    }

    pub fn token_type(&self) -> TokenType {
        self.ttype.to_owned()
    }

    pub fn to_string(&self) -> String{
        format!("Yeah I did not do this yet")
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            return write!(f, "{}({})", self.ttype.as_ref(), value)
        }
        return write!(f, "{}", self.ttype.as_ref())
    }
}

/// Returns the corresponding tokentype if it matches
/// a keyword, otherwise it returns a tokentype of identifier
pub fn keyword_check(s: &str) -> TokenType {
    use TokenType::*;
    match s {
        "if" => IF,
        "elsif" => ELSIF,
        "else" => ELSE,
        "then" => THEN,
        "while" => WHILE,
        "for" => REPEAT,
        "until" => UNTIL,
        "from" => FROM,
        "to" => TO,
        "define" => DEFINE,
        "var" => VAR,
        "array" => ARRAY,
        "of" => OF,
        "variables" => VARIABLES,
        "constants" => CONSTANTS,
        "integer" => INTEGER,
        "real" => REAL,
        "boolean" => BOOLEAN,
        "mod" => MOD,
        _ => IDENTIFIER, 
    }
}
