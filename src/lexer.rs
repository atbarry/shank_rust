#![allow(dead_code, unused_imports)]
use core::fmt;
use std::{fs, fmt::Debug};        
use crate::{Token, TokenType, FilePos, keyword_check};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    state: MachineState,
    comment_flag: bool,
    str_buffer: String,
    file_pos: FilePos,
}

#[derive(PartialEq)]
pub struct LexError {
    pub msg: String,
    pub location: FilePos, // file_pos, col
}

impl LexError {
    fn new(msg: &str, location: FilePos) -> Self{
        LexError { msg: msg.to_owned(), location}
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} located at {}", self.msg, self.location)
    }
}

#[derive(Debug, Copy, Clone)]
enum MachineState {
    Start,
    Word,
    Number,
    NumberDecimal,
    String,
    CharBeginning,
    CharMid,
}
     
impl Lexer {
    fn new() -> Lexer {
        Lexer { 
            tokens: vec![], 
            state: MachineState::Start,
            comment_flag: false,
            str_buffer: String::new(),
            file_pos: FilePos::new(1,0) // 1, 0 because of the way the program loops
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn lex_file(path: &str) -> Lexer {
        let file_str = match fs::read_to_string(path) {
            Err(e) => panic!("{e}"),
            Ok(s) => s,
        };

        match Lexer::lex(&file_str) {
            Err(e) => panic!("{} located at {} in {}", e.msg, e.location, path),
            Ok(l) => l
        }
    }

    pub fn lex(lstr: &str) -> Result<Lexer, LexError> {
        let mut lexer = Lexer::new();
       
        //looping through every character in the file string
        for current_char in lstr.chars() {
            lexer.file_pos.next_character();

            if lexer.is_comment(current_char) {
                lexer.str_buffer = String::new();
                continue;
            }

            if let Some(ttype) = lexer.update_machine(current_char)? {
                lexer.add_token(ttype);
            } else {
                lexer.str_buffer.push(current_char);
                continue;
            }

            lexer.str_buffer = String::from(current_char);

            // checks for char tokens or incorrect characters
            lexer.check_character(current_char);

            if current_char == '\n' {
                lexer.file_pos.next_line()
            }
        }

        println!("{:?}",lstr);
        Ok(lexer)
    }

    fn is_comment(&mut self, input_char: char) -> bool {
        if !self.comment_flag && input_char == '{' {
            self.comment_flag = true;
        }
        else if self.comment_flag && input_char == '}' {
            self.comment_flag = false;
            return true; // this iteration is still
                         // part of the comment so return 
                         // true. 
        }
        self.comment_flag
    }

    fn update_machine(&mut self, input_char: char) 
        -> Result<Option<TokenType>, LexError>
    {
        dbg!(&input_char);
        let mut ttype = None;
        match self.state {
            MachineState::Start => { 
                if input_char.is_alphabetic() {
                    self.state = MachineState::Word;
                } else if input_char.is_digit(10) {
                    self.state = MachineState::Number;
                } else if input_char == '.' {
                    self.state = MachineState::NumberDecimal
                } else if input_char == '"' {
                    self.state = MachineState::String;
                } else if input_char == '\'' {
                    self.state = MachineState::CharBeginning;
                }
            }
            MachineState::Word => {
                if !input_char.is_ascii_alphanumeric() {
                    let t = keyword_check(&self.str_buffer);
                    ttype = Some(t);
                }
            }
            MachineState::Number => {
                if input_char == '.' {
                    self.state = MachineState::NumberDecimal;
                } else if !input_char.is_digit(10) {
                    ttype = Some(TokenType::NUMBER);
                }
            }
            MachineState::NumberDecimal => {
                if input_char == '.' {
                    return Err(LexError::new("Two decimals in number", self.file_pos));
                } else if !input_char.is_digit(10) {
                    ttype = Some(TokenType::NUMBER);
                }
            }
            MachineState::String => {
                dbg!("made it here");
                if input_char == '"' {
                    self.str_buffer.remove(0); // remove the first " from the buffer
                    ttype = Some(TokenType::STRINGLITERAL);
                }
            }
            MachineState::CharBeginning => {
                self.state = MachineState::CharMid;
            }
            MachineState::CharMid => {
                if input_char == '\'' {
                    self.str_buffer.remove(0); // remove the first " from the buffer
                    ttype = Some(TokenType::CHARACTERLITERAL);
                } else { 
                    return Err(
                        LexError::new("No closing ' in CHARLITERAL",self.file_pos)
                    );
                }
            }
        }

        if ttype.is_some() {
            self.state = MachineState::Start;
        }

        Ok(ttype)
    }

    fn check_character(
        &mut self, 
        current_char: char, 
    ) {
        match current_char {
            '\n' => {
                self.add_token(TokenType::ENDOFLINE);
            },
            ' '|'\r'|'\t' => {} ,  
            '\''|'"' => {},
            '+'|'-'|'=' => {} ,
            c => {
                panic!("Unknown char {:?} at {}",c, self.file_pos);
            },
        };
    }

    fn add_token(&mut self, ttype: TokenType) {
        let s = self.str_buffer.trim().to_owned();
        self.tokens.push(Token::new(ttype,s, self.file_pos));
    }

    pub fn print_tokens(&self) {
        println!("");
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }
}
