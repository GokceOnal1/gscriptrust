use gscriptrust::lexer::*;
use gscriptrust::token::*;
use gscriptrust::error::*;

fn main() {
    let mut errorstack = ErrorStack::new();
    let mut lexer = Lexer::new("entry/entry.gsc", &mut errorstack);
    lexer.lex();
    Token::print_toks(&lexer.tokens);
    errorstack.print_dump();
}
