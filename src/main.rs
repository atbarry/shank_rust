use shank_rust::lexer::lex_file;
use shank_rust::Token;

fn main() {
    let lexer = lex_file("src/test.shank"); 
    lexer.print_tokens();

}
