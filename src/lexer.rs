#![allow(dead_code, unused_imports)]
use core::fmt;
use std::{fs, fmt::Debug, thread::current};        
use crate::{Token, TokenType, keyword_check};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    previous_indentation: i32,
    comment_state: CommentState,
    line_num: u32,
}

#[derive(PartialEq)]
pub struct LexError {
    pub msg: String,
    pub line_num: u32, 
}

impl LexError {
    fn new(msg: &str, line_num: u32) -> Self{
        LexError { msg: msg.to_owned(), line_num }
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} located at line{}", self.msg, self.line_num)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MachineState {
    Start,
    Word,
    Number,
    NumberDecimal,
    String,
    CharBeginning,
    CharMid,
}

#[derive(Debug, PartialEq)]
enum CommentState {
    NoComment,
    IsComment,
    Done
}

fn peak(v: &Vec<char>, line_pos: usize) -> char {
    if line_pos + 1 < v.len() {
        return v[line_pos + 1];
    } 

    '\0'
}

pub fn lex_file(path: &str) -> Lexer {
    let file_str = match fs::read_to_string(path) {
        Err(e) => panic!("{e}"),
        Ok(s) => s,
    };

    match Lexer::lex(&file_str) {
        Err(e) => panic!("{} located at {} in {}", e.msg, e.line_num, path),
        Ok(l) => l
    }
}

impl Lexer {
    fn new() -> Lexer {
        Lexer { 
            tokens: vec![], 
            previous_indentation: 0,
            comment_state: CommentState::NoComment,
            line_num: 0, 
        }
    }

    pub fn lex(file_string: &str) -> Result<Lexer, LexError> {
        let mut lexer = Lexer::new();
        for line in file_string.lines() {
            lexer.line_num += 1;
            lexer.lex_line(line)?;
        }

        lexer.print_tokens();
        Ok(lexer)
    }

    pub fn lex_line(&mut self, lstr: &str) -> Result<(), LexError> {
        let mut str_buffer = String::new();
        let mut indentation = 0;
        let mut current_state = MachineState::Start;
        let mut prev_state;

        //looping through every character in the file string
        let line_chars: Vec<char> = lstr.chars().collect();
        let mut line_pos = 0;
        while line_pos  < line_chars.len() {
            line_pos += 1;
            let current_char = line_chars[line_pos - 1];
            // minus 1 because of the way it loops 

            if self.check_comment(current_char)? {
                continue;
            }

            indentation = self.count_indentation(indentation, current_char); 
            if indentation >= 0 {
                continue;
            }

            prev_state = current_state;
            current_state = self.update_machine(current_state, current_char)?;

            if current_state != MachineState::Start {
                str_buffer.push(current_char);
                continue;
            }

            if prev_state == MachineState::Start {
                line_pos += self.check_character(current_char, peak(&line_chars, line_pos - 1))?;
                continue;
            }

            // Then add the tokens and reset the str_buffer
            self.add_token_from_state(prev_state, &str_buffer);
            str_buffer = String::new();
            

            // checks for char tokens or incorrect characters
            if prev_state != MachineState::CharMid || prev_state != MachineState::String {
                line_pos += self.check_character(current_char, peak(&line_chars, line_pos - 1))?;
            }
        }

        if self.comment_state == CommentState::Done {
            self.comment_state = CommentState::NoComment;
        }

        self.finish_line(current_state, &str_buffer)?;
        self.add_token(TokenType::ENDOFLINE);

        println!("{:?}",lstr);
        Ok(())
    }

        
    // TODO Throw errors for improper otherwise use the normal state to token function
    fn finish_line(&mut self, end_state: MachineState, str_buffer: &str) -> Result<(), LexError> {
        match end_state {
            MachineState::CharMid | MachineState::CharBeginning => {
                return Err(LexError::new("No closing ' in CHARLITERAL", self.line_num));
            }
            MachineState::String => {
                return Err(LexError::new("No closing \" in STRINGLITERAL", self.line_num));
            }
            _ => self.add_token_from_state(end_state, str_buffer), // Otherwise add the token
        }

        Ok(())
    }

    fn count_indentation(&mut self, mut indentation: i32, input_char: char) -> i32 {
        if indentation < 0 {
            return indentation;
        }

        if input_char == '\t' {
            return indentation + 4;
        } else if input_char == ' ' {
            return indentation + 1;
        } 

        let curr = indentation / 4;
        let prev = self.previous_indentation / 4;
        let diff = curr - prev;

        for _ in 0..(diff.abs()) {
            if diff.is_positive() {
                self.add_token(TokenType::INDENT);
            } else {
                self.add_token(TokenType::DEDENT);
            }
        }

        self.previous_indentation = indentation;
        indentation = -1;

        return indentation;
    }

    fn check_comment(&mut self, input_char: char) -> Result<bool,LexError> {
        match self.comment_state {
            CommentState::NoComment => {
                if input_char == '{' {
                    self.comment_state = CommentState::IsComment;
                }
            }
            CommentState::IsComment => {
                if input_char == '}' {
                    self.comment_state = CommentState::Done;
                }
            }
            CommentState::Done => {
                if !input_char.is_ascii_whitespace() {
                    return Err(LexError::new("No code after a comment", self.line_num));
                }
            }
        }

        Ok(self.comment_state != CommentState::NoComment)
    }

    fn update_machine(&self, mut state: MachineState, input_char: char) 
        -> Result<MachineState, LexError>
    {
        match state {
            MachineState::Start => { 
                if input_char.is_alphabetic() {
                    state = MachineState::Word;
                } else if input_char.is_digit(10) {
                    state = MachineState::Number;
                } else if input_char == '.' {
                    state = MachineState::NumberDecimal
                } else if input_char == '"' {
                    state = MachineState::String;
                } else if input_char == '\'' {
                    state = MachineState::CharBeginning;
                }
            }
            MachineState::Word => {
                if !input_char.is_ascii_alphanumeric() {
                    state = MachineState::Start;
                }
            }
            MachineState::Number => {
                if input_char == '.' {
                    state = MachineState::NumberDecimal;
                } else if !input_char.is_digit(10) {
                    state = MachineState::Start;
                }
            }
            MachineState::NumberDecimal => {
                if input_char == '.' {
                    return Err(LexError::new("Two decimals in number", self.line_num));
                } else if !input_char.is_digit(10) {
                    state = MachineState::Start;
                }
            }
            MachineState::String => {
                if input_char == '"' {
                    state = MachineState::Start;
                }
            }
            MachineState::CharBeginning => {
                state = MachineState::CharMid;
            }
            MachineState::CharMid => {
                if input_char == '\'' {
                    state = MachineState::Start;
                } else { 
                    return Err(LexError::new("No closing ' in CHARLITERAL",self.line_num));
                }
            }
        }

        Ok(state)
    }

    fn add_token_from_state(&mut self, state: MachineState, str_buffer: &str) {
        match state {
            MachineState::Start => {},
            MachineState::Word => self.add_token_with_val(keyword_check(str_buffer), str_buffer),
            MachineState::Number => self.add_token_with_val(TokenType::NUMBER, str_buffer),
            MachineState::NumberDecimal => self.add_token_with_val(TokenType::NUMBER, str_buffer),
            MachineState::CharMid | MachineState::CharBeginning => {
                let mut s = str_buffer.to_owned();
                s.remove(0);
                self.add_token_with_val(TokenType::CHARACTERLITERAL, &s);
            }
            MachineState::String => {
                let mut s = str_buffer.to_owned();
                s.remove(0);
                self.add_token_with_val(TokenType::STRINGLITERAL, &s);
            }
        }
    }

    // Returns true if it used peak 
    fn check_character( &mut self, current_char: char, next_char: char) -> Result<usize, LexError> {
        if current_char.is_ascii_whitespace() {
            return Ok(0);
        }

        let mut look_ahead_addition = 0;
        match current_char {
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            '*' => self.add_token(TokenType::MULTIPLY),
            '/' => self.add_token(TokenType::DIVIDE),
            ';' => self.add_token(TokenType::SEMICOLON),
            ',' => self.add_token(TokenType::COMMA),
            '=' => self.add_token(TokenType::EQUALS),
            '(' => self.add_token(TokenType::LEFTPARENTHESIS),
            ')' => self.add_token(TokenType::RIGHTPARENTHESIS),
            '[' => self.add_token(TokenType::LEFTSQUAREBRACKET),
            ']' => self.add_token(TokenType::RIGHTSQUAREBRACKET),
            ':' => {
                if next_char == '=' {
                    self.add_token(TokenType::ASSIGN);
                    look_ahead_addition = 1;
                } else {
                    self.add_token(TokenType::COLON)
                }
            }
            '>' => {
                if next_char == '=' {
                    self.add_token(TokenType::GREATERTHANOREQUAL);
                    look_ahead_addition = 1;
                } else {
                    self.add_token(TokenType::GREATERTHAN);
                }
            }
            '<' => {
                if next_char == '>' {
                    self.add_token(TokenType::NOTEQUAL);
                    look_ahead_addition = 1;
                } else if next_char == '=' {
                    self.add_token(TokenType::LESSTHANOREQUAL);
                    look_ahead_addition = 1;
                } else {
                    self.add_token(TokenType::LESSTHAN);
                }
            }
            c => return Err(LexError::new(&format!("Unknown char '{}'", c), self.line_num)),
        }

        Ok(look_ahead_addition)
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.tokens.push(Token::new(ttype, self.line_num));
    }

    fn add_token_with_val(&mut self, ttype: TokenType, str_buffer: &str) {
        self.tokens.push(Token::new_with_value(ttype, str_buffer, self.line_num));
    }

    pub fn print_tokens(&self) {
        println!("");
        for token in &self.tokens {
            if token.token_type() == TokenType::ENDOFLINE {
                println!("{}", token);
            } else {
                print!("{} ", token);
            }
        }
        println!("");
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
