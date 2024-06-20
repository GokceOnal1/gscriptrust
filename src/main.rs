use gscriptrust::lexer::*;
use gscriptrust::error::*;
use gscriptrust::parser::*;
use gscriptrust::visitor::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let errorstack = Rc::new(RefCell::new(ErrorStack::new()));
    let mut lexer = Lexer::new("entry/entry.gsc", Rc::clone(&errorstack));
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, Rc::clone(&errorstack));
    let ast_compound = parser.parse_compound().unwrap();

    let mut visitor = Visitor::new(Rc::clone(&errorstack));
    visitor.visit(&ast_compound);

    let errorstack = visitor.errorstack;
    errorstack.borrow().print_dump();
}
