use gscriptrust::lexer::Lexer;
use gscriptrust::token::Token;

fn main() {
    let mut lexer = Lexer::new("entry/entry.gsc");
    lexer.lex();
    Token::print_toks(&lexer.tokens);
}
