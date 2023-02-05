use shank_rust::lexer::Lexer;

fn main() {
    let lexer = Lexer::lex_file("src/test.shank"); 
    lexer.print_tokens();
}
