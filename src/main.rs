use gscriptrust::lexer::*;
use gscriptrust::token::*;
use gscriptrust::error::*;
use gscriptrust::ast::*;
use gscriptrust::parser::*;

fn main() {
    let mut errorstack = ErrorStack::new();
    let mut lexer = Lexer::new("entry/entry.gsc", &mut errorstack);
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, &mut errorstack);
    parser.parse_compound().unwrap().print();
    // let test_tok = lexer.tokens.get(0).unwrap();
    // let test_binop = AST::BINOP {
    //     left : Box::new(ASTNode::new(AST::INT{ int_value: 5 }, test_tok.einfo.clone())),
    //     op : TokenType::PLS,
    //     right : Box::new(ASTNode::new(AST::INT{int_value:10}, test_tok.einfo.clone()))
    // };
    // let test_binop_node = ASTNode::new(test_binop, test_tok.einfo.clone());
    // test_binop_node.print();
    //Token::print_toks(&lexer.tokens);
    errorstack.print_dump();
}
