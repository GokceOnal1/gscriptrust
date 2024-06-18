use gscriptrust::lexer::*;
use gscriptrust::token::*;
use gscriptrust::error::*;
use gscriptrust::ast::*;
use gscriptrust::parser::*;
use gscriptrust::visitor::*;

fn main() {
    let mut errorstack = ErrorStack::new();
    let mut lexer = Lexer::new("entry/entry.gsc", &mut errorstack);
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, &mut errorstack);
    let ast_compound = parser.parse_compound().unwrap();

    let mut visitor = Visitor::new(errorstack);
    visitor.visit(&ast_compound).print();

    let errorstack = visitor.errorstack;
    errorstack.print_dump();
}
