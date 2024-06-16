use crate::errors::error::*;
use super::ast::*;
use super::token::*;

pub struct Parser<'a> {
    prev_token : Option<&'a Token>,
    curr_token : Option<&'a Token>,
    token_i : usize,
    tokens : &'a Vec<Token>,
    errorstack : &'a mut ErrorStack,
}
impl<'a> Parser<'a> {
    pub fn new(tokens : &'a Vec<Token>, errorstack : &'a mut ErrorStack) -> Parser<'a> {
        Parser { prev_token: None, curr_token: tokens.get(0), token_i : 0, tokens, errorstack }
    }
    pub fn advance(&mut self) {
        self.token_i += 1;
        self.prev_token = self.curr_token;
        self.curr_token = self.tokens.get(self.token_i);
    }
    pub fn parse_compound(&mut self) -> Option<ASTNode> {
        let mut comp_ast = ASTNode::new(AST::COMPOUND { compound_value: ( Vec::new()) }, self.curr_token.unwrap().einfo.clone());
        None
    }
    pub fn parse_comp_expr(&mut self) -> Option<ASTNode> {
        let ast_left = self.parse_comp_term();
        while(self.curr_token.unwrap().kind == TokenType::AND ||
              self.curr_token.unwrap().kind == TokenType::OR) {
                return None;
              } 
              None
    }
    pub fn parse_comp_term(&mut self) -> Option<ASTNode> {
        None
    }
    pub fn parse_factor(&mut self) -> Option<ASTNode> {
        None
    }
    pub fn parse_mono(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            match tok.kind {
                TokenType::INT(_) => { return self.parse_num(); }
                TokenType::STRING(_) => { return self.parse_string(); }
                _ => None
            }
        } else {
            None
        }
    }
    pub fn parse_term(&mut self) -> Option<ASTNode> {
        None
    }
    pub fn parse_string(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            self.advance();
            match &tok.kind {
                TokenType::STRING(s) => { return Some(ASTNode::new(AST::STRING { str_value : s.to_string() }, tok.einfo.clone())); }
                _ => { self.errorstack.errors.push(GError::new_from_tok(ETypes::TokenError, "Expected token 'STRING'", tok.einfo.clone()));
                return None; }
            }; 
        } else {
            None
        }
    }
    pub fn parse_num(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            self.advance();
            match tok.kind {
                TokenType::INT(x) => { return Some(ASTNode::new(AST::INT { int_value : x}, tok.einfo.clone()))}
                _ => { self.errorstack.errors.push(GError::new_from_tok(ETypes::TokenError, "Expected token 'INT'", tok.einfo.clone()));
                return None; }
            }
        } else {
            None
        }
    }
    
}