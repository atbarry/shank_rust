#![allow(dead_code, unused_imports)]
use std::fs;        
use shank_rust::{Token, TokenType, FilePos, keyword_check};

pub struct Lexer {
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

pub struct LexError {
    msg: String,
    location: FilePos, // file_pos, col
}

impl LexError {
    fn new(msg: &str, location: FilePos) -> Self{
        LexError { msg: msg.to_owned(), location}
    }
}

#[derive(Debug, Copy, Clone)]
enum WNState {
    Start,
    Word,
    Number,
    NumberDecimal,
}

enum SCLitState {
    Start,
    String,
    CharBeginning,
    CharMid,
}

struct WNMachine {
    state: WNState,
}

struct SCLitMachine {
    state: SCLitState
}

struct CommentMachine {
    comment: bool,
}

impl WNMachine {
    fn new() -> Self {
        Self {
            state: WNState::Start,
        }
    }

    fn update(&mut self, input_char: char, str_buffer: &str, file_pos: FilePos) 
        -> Result<Option<TokenType>, LexError>
    {
        let mut ttype = None;
        match self.state {
            WNState::Start => { 
                if input_char.is_alphabetic() {
                    self.state = WNState::Word;
                } else if input_char.is_digit(10) {
                    self.state = WNState::Number;
                } else if input_char == '.' {
                    self.state = WNState::NumberDecimal
                }
            }
            WNState::Word => {
                if !input_char.is_ascii_alphanumeric() {
                    let t = keyword_check(str_buffer);
                    ttype = Some(t);
                }
            }
            WNState::Number => {
                if input_char == '.' {
                    self.state = WNState::NumberDecimal;
                } else if !input_char.is_digit(10) {
                    ttype = Some(TokenType::NUMBER);
                }
            }
            WNState::NumberDecimal => {
                if input_char == '.' {
                dbg!("made it here");
                    return Err(LexError::new("Two decimals in number", file_pos));
                } else if !input_char.is_digit(10) {
                    ttype = Some(TokenType::NUMBER);
                }
            }
        }

        if ttype.is_some() {
            self.state = WNState::Start;
        }

        Ok(ttype)
    }
}


impl CommentMachine {
    fn new() -> Self { Self{ comment: false } }
    fn is_comment(&mut self, input_char: char) -> bool {
        if !self.comment && input_char == '{' {
            self.comment = true;
        }
        else if self.comment && input_char == '}' {
            self.comment = false;
            return true; // this iteration is still
                         // part of the comment so return 
                         // true. 
        }
        self.comment
    }
}


     
impl Lexer {
    fn new() -> Lexer {
        Lexer { tokens: vec![], errors: vec![] }
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
        let mut str_buffer = String::new();

        let mut wnmachine = WNMachine::new();
        let mut comment_machine = CommentMachine::new();
        let mut file_pos = FilePos::new(1, 0); // keeps track of file_pos number
       
        //looping through every character in the file string
        for current_char in lstr.chars() {
            file_pos.next_character();

            if comment_machine.is_comment(current_char) {
                str_buffer = String::new();
                continue;
            }

            if let Some(ttype) = wnmachine.update(current_char, &str_buffer, file_pos)? {
                lexer.add_token(ttype, &str_buffer, file_pos);
            } else {
                str_buffer.push(current_char);
                continue;
            }

            // checks for char tokens or incorrect characters
            lexer.check_character(current_char, &str_buffer, file_pos);

            if current_char == '\n' {
                file_pos.next_line()
            }

            str_buffer = String::new();
        }

        println!("{:?}",lstr);
        Ok(lexer)
    }

    fn check_character(
        &mut self, 
        current_char: char, 
        str_buffer: &str, 
        file_pos: FilePos
    ) {
        match current_char {
            '\n' => {
                self.add_token(TokenType::ENDOFLINE, str_buffer, file_pos);
            },
            ' '|'\r'|'\t' => {} ,  
            '+'|'-'|'/'|'=' => {} ,
            c => {
                panic!("Unknown char {:?} at {}",c, file_pos);
            },
        };
    }

    fn add_token(&mut self, ttype: TokenType, content: &str ,file_pos: FilePos) {
        self.tokens.push(Token::new(ttype, content.to_owned(), file_pos));
    }

    pub fn print_tokens(&self) {
        println!("");
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }
}
