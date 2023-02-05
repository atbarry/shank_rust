use lexer::Lexer;
mod lexer;

fn main() {
    let lexer = Lexer::lex_file("src/test.shank"); 
    lexer.print_tokens();
}
