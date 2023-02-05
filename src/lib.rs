use std::fmt;
use std::fmt::Display;

pub mod lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    content: String,
    file_pos: FilePos,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    UNKNOWN,
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
}

#[derive(Debug,PartialEq, Copy,Clone)]
pub struct FilePos {
    pub line: u32,
    pub character: u32,
}

impl FilePos {
    pub fn new(line: u32, character: u32) -> FilePos {
        FilePos { line, character }
    }
    pub fn next_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
    pub fn next_character(&mut self) {
        self.character += 1;
    }
}

impl Display for FilePos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}:{}", self.line, self.character)
    }
}


impl Token {
    pub fn new(ttype: TokenType, content: String, file_pos: FilePos) -> Token {
        Token { ttype, content, file_pos }
    }

    pub fn to_string(&self) -> String{
        let out = match self.ttype {
            TokenType::NUMBER => format!("NUMBER({})", self.content),
            TokenType::IDENTIFIER => format!("IDENTIFIER({})", self.content),
            TokenType::ENDOFLINE => format!("ENDOFLINE"),
            TokenType::UNKNOWN => format!("UNKOWNCHAR({}) in line {}", self.content, self.file_pos),
            TokenType::STRINGLITERAL => format!("STRINGLITERAL(\"{}\")", self.content),
            TokenType::CHARACTERLITERAL => format!("CHARACTERLITERAL('{}')", self.content),
            _ => format!("Not handled yet({})", self.content),

        };

        out
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
