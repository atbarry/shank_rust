#![allow(dead_code)]
#![allow(unused_imports)]

use shank_rust::*;
use shank_rust::lexer::{Lexer, LexError};
use common::compare_tokens;
mod common;


#[test]
fn character_fine1() {
    let lex_str = "variables x = 1";
    let tokens = vec![
        tokenize!(VARIABLES, "variables", 1),
        tokenize!(IDENTIFIER, "x", 1),
        tokenize!(EQUALS, 1),
        tokenize!(NUMBER, "1", 1)
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();
    compare_tokens(lexer.tokens(), tokens);
}

#[test]
fn character_fine2() {
    let lex_str = ">< > <>";
    let tokens = vec![
        tokenize!(GREATERTHAN),
        tokenize!(LESSTHAN),
        tokenize!(GREATERTHAN),
        tokenize!(NOTEQUAL),
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();
    compare_tokens(lexer.tokens(), tokens);
}

// Indenting
#[test]
fn indent_fine1() {
    let lex_str = "    hello\n   hi";
    let tokens = vec![
        tokenize!(INDENT, 1),
        tokenize!(IDENTIFIER, "hello", 1),
        tokenize!(ENDOFLINE, 1),
        tokenize!(DEDENT, 2),
        tokenize!(IDENTIFIER, "hi", 2)
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();
    compare_tokens(lexer.tokens(), tokens);
}

#[test]
fn comment_fine1() {
    let lex_str = "{This is a comment}\nhi";
    let tokens = vec![
        tokenize!(ENDOFLINE, 1),
        tokenize!(IDENTIFIER, "hi", 2)
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();
    compare_tokens(lexer.tokens(), tokens);
}

#[test]
fn comment_fine2() {
    let lex_str = "{This is another \n comment on a new line}\nhi";
    let tokens = vec![
        tokenize!(ENDOFLINE, 1),
        tokenize!(ENDOFLINE, 2),
        tokenize!(IDENTIFIER, "hi", 3)
    ]; 

    let lexer = Lexer::lex(lex_str).unwrap();
    compare_tokens(lexer.tokens(), tokens);
}

#[test]
#[should_panic]
fn comment_bad1() {
    let lex_str = "{This comment should fail}hi";
    Lexer::lex(lex_str).unwrap();
}

#[test]
#[should_panic]
fn comment_bad2() {
    let lex_str = "{This comment should fail\n}hi";
    Lexer::lex(lex_str).unwrap();
}
