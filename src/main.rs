use gscriptrust::lexer::*;
use gscriptrust::error::*;
use gscriptrust::parser::*;
use gscriptrust::visitor::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "0");

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        GError::command_line(1, "Expected 2 command line arguments: \n'gscript [filename]'");
        std::process::exit(1);
    }
    if args[1] != "gscript" {
        GError::command_line(2, "No support for commands other than 'gscript' has been implemented");
        std::process::exit(1);
    }
    let filename = args[2].clone();
    let errorstack = Rc::new(RefCell::new(ErrorStack::new()));
    let mut lexer = Lexer::new(format!("entry/{}", filename).as_str(), Rc::clone(&errorstack));
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, Rc::clone(&errorstack));
    let ast_compound = parser.parse_compound().unwrap();

    let mut visitor = Visitor::new(Rc::clone(&errorstack));
    visitor.visit(&ast_compound);

    let errorstack = visitor.errorstack;
    errorstack.borrow().print_dump();
}
