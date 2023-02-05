#![allow(dead_code, unused_imports)]
use std::fs;        
use shank_rust::{Token, TokenType, keyword_check};

pub struct Lexer {
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

struct LexError {
    location: (u32, u32), // line, col
    etype: ErrorType 
}

pub enum ErrorType {
    UnknownChar(char),
}

enum WordNumState {
    Start,
    Word,
    Number,
    NumberDecimal,
}
     
impl Lexer {
    fn new() -> Lexer {
        Lexer { tokens: vec![], errors: vec![] }
    }

    pub fn lex(lstr: &str) -> Lexer {
        //  other, letters, numbers, decimal point
        let smachine = [[ 0, 1, 3, 2], 
                        [ 0, 1, 1, 0],
                        [ 0, 0, 2, 0],
                        [ 0, 0, 3, 2]];

        let mut lexer = Lexer::new();
        let mut old_state: usize;
        let mut state: usize = 0;
        let mut token_val = String::new();

        let mut line = 0; // keeps track of line number
        let mut line_pos = 0; 
       
        //looping through every character in the file string
        for current_char in lstr.chars() {
            line_pos += 1;
            old_state = state; 

            // figure out what the char converts to
            // in the state machine 
            let char_smachine_conversion = 
            if current_char.is_alphabetic() { 1 }
            else if current_char.is_digit(10) { 2 } 
            else if current_char == '.' { 3 }
            else { 0 };

            state = smachine[old_state][char_smachine_conversion]; 

            // Only time we add token is when state
            // transfers to 0 so continue otherwise
            if state != 0 {
                token_val.push(current_char);
                continue;
            }

            // generate token based on state transfer
            let transition_token = match old_state {
                1 => Some(keyword_check(&token_val)),
                2|3 => Some(TokenType::NUMBER), 
                _ => None,
            };

            // check the character
            // I think these are lambda transitions in state machine
            let lambda_token = match current_char {
                '\n' => {
                    line = line + 1;
                    line_pos = 0;
                    Some(TokenType::ENDOFLINE)
                },
                ' '|'\r'|'\t' => None,  
                '+'|'-'|'/'|'=' => None,
                c => {
                    panic!("Unknown char {:?} at {}:{}",c, line, line_pos)
                },
            };

            lexer.add_potential_token(transition_token, token_val, line);
            lexer.add_potential_token(lambda_token, current_char.to_string(), line);

            token_val = String::new();
        }

        println!("{:?}",lstr);
        lexer
    }
    
    pub fn lex_file(path: &str) -> Lexer {
        let file_str = match fs::read_to_string(path) {
            Err(e) => panic!("{e}"),
            Ok(s) => s,
        };

        Lexer::lex(&file_str)
    }

    fn add_potential_token(&mut self, ttype_option: Option<TokenType>, content: String, line_num: u32) {
        if let Some(ttype) = ttype_option {
            self.tokens.push(Token::new(ttype, content, line_num));
        }
    }

    pub fn print_tokens(&self) {
        println!("");
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }
}