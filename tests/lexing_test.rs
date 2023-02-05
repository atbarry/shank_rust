#![allow(dead_code)]
#![allow(unused_imports)]

use shank_rust::*;
use shank_rust::lexer::{Lexer, LexError};

#[test]
fn file_pos_check() {
    let lex_str = "hi";
    let tokens = vec![
        Token::new(TokenType::IDENTIFIER, "hi".to_owned(), FilePos::new(1,2)),
        //Token::new(TokenType::ENDOFLINE, "\n".to_owned(), FilePos::new(1,4)),
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();

    assert_eq!(lexer.tokens(), tokens);
}

//#[test]
fn keyword_check() {
    let lex_str = "how to while\n";
    let tokens = vec![
        Token::new(TokenType::IDENTIFIER, "how".to_owned(), FilePos::new(1,3)),
        Token::new(TokenType::TO, "to".to_owned(), FilePos::new(1,6)),
        Token::new(TokenType::WHILE, "while".to_owned(), FilePos::new(1,12)),
        Token::new(TokenType::ENDOFLINE, "\n".to_owned(), FilePos::new(1,13)),
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();

    assert_eq!(lexer.tokens(), tokens);
}

