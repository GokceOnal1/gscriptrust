use gscriptrust::lexer::*;
use gscriptrust::error::*;
use gscriptrust::parser::*;
use gscriptrust::visitor::*;

fn main() {
    let mut errorstack = ErrorStack::new();
    let mut lexer = Lexer::new("entry/entry.gsc", &mut errorstack);
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, &mut errorstack);
    let ast_compound = parser.parse_compound().unwrap();

    let mut visitor = Visitor::new(errorstack);
    visitor.visit(&ast_compound);

    let errorstack = visitor.errorstack;
    errorstack.print_dump();
}
