pub struct Token {
    ttype: TokenType,
    content: String,
    line_num: u32,
}

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

impl Token {
    pub fn new(ttype: TokenType, content: String, line_num: u32) -> Token {
        Token { ttype, content, line_num }
    }

    pub fn to_string(&self) -> String{
        let out = match self.ttype {
            TokenType::NUMBER => format!("NUMBER({})", self.content),
            TokenType::IDENTIFIER => format!("IDENTIFIER({})", self.content),
            TokenType::ENDOFLINE => format!("ENDOFLINE"),
            TokenType::UNKNOWN => format!("UNKOWNCHAR({}) in line {}", self.content, self.line_num),
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
