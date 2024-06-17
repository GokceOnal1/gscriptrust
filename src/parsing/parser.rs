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
        let mut comp_ast = ASTNode::new(AST::COMPOUND { 
            compound_value: ( Vec::new()) 
        }, self.curr_token.unwrap().einfo.clone());
        match comp_ast.kind {
            AST::COMPOUND { ref mut compound_value } => { compound_value.push(self.parse_comp_expr().unwrap_or(ASTNode::new(AST::NOOP, self.prev_token.unwrap().einfo.clone()))) },
            _ => { return None }
        }
        while let Some(tok) = self.curr_token {
            if tok.kind != TokenType::SEMI {
                break;
            } else {
                self.advance();
                match comp_ast.kind {
                    AST::COMPOUND { ref mut compound_value } => { compound_value.push(self.parse_comp_expr().unwrap_or(ASTNode::new(AST::NOOP, self.prev_token.unwrap().einfo.clone()))) },
                    _ => { return None }
                }
            }
        }
        Some(comp_ast)
    }
    pub fn parse_comp_expr(&mut self) -> Option<ASTNode> {
        let mut ast_left = self.parse_comp_term()?;
            while let Some(tok) = self.curr_token {
                match tok.kind {
                    TokenType::AND | TokenType::OR => {
                        let op = tok.kind.clone();
                        self.advance();
                        let right = self.parse_comp_term().unwrap();

                        let ast_binop = AST::BINOP {
                            left : Box::new(ast_left),
                            op,
                            right : Box::new(right)
                        }; 

                        ast_left = ASTNode::new(ast_binop, tok.einfo.clone());
                        
                    }
                    _ => {
                        break;
                    }
                }
            }
            Some(ast_left)
    }
    pub fn parse_comp_term(&mut self) -> Option<ASTNode> {
        let mut ast_left = self.parse_expr()?;
            while let Some(tok) = self.curr_token {
                match tok.kind {
                    TokenType::DEQL | TokenType::LT | TokenType::LTE | TokenType::GT 
                    | TokenType::GTE | TokenType::NEQ => {
                        let op = tok.kind.clone();
                        self.advance();
                        let right = self.parse_expr().unwrap();

                        let ast_binop = AST::BINOP {
                            left : Box::new(ast_left),
                            op,
                            right : Box::new(right)
                        }; 

                        ast_left = ASTNode::new(ast_binop, tok.einfo.clone());
                        
                    }
                    _ => {
                        break;
                    }
                }
            }
            Some(ast_left)
    }
    pub fn parse_factor(&mut self) -> Option<ASTNode> {
        None
    }
    pub fn parse_mono(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            match tok.kind {
                TokenType::INT(_) | TokenType::FLOAT(_) => { return self.parse_num(); }
                TokenType::STRING(_) => { return self.parse_string(); }
                TokenType::EOF => { return Some(ASTNode::new(AST::EOF, self.curr_token.unwrap().einfo.clone())); }
                _ => None
            }
        } else {
            None
        }
    }
    //DONE
    pub fn parse_term(&mut self) -> Option<ASTNode> {
        let mut ast_left = self.parse_mono()?;
            while let Some(tok) = self.curr_token {
                match tok.kind {
                    TokenType::MUL | TokenType::DIV | TokenType::MOD => {
                        let op = tok.kind.clone();
                        self.advance();
                        let right = self.parse_mono().unwrap();

                        let ast_binop = AST::BINOP {
                            left : Box::new(ast_left),
                            op,
                            right : Box::new(right)
                        }; 

                        ast_left = ASTNode::new(ast_binop, tok.einfo.clone());
                        
                    }
                    _ => {
                        break;
                    }
                }
            }
            Some(ast_left)

    }
    //DONE
    ///parses expression
    pub fn parse_expr(&mut self) -> Option<ASTNode> {
        let mut ast_left = self.parse_term()?;
            while let Some(tok) = self.curr_token {
                match tok.kind {
                    TokenType::PLS | TokenType::MIN => {
                        let op = tok.kind.clone();
                        self.advance();
                        let right = self.parse_term().unwrap();

                        let ast_binop = AST::BINOP {
                            left : Box::new(ast_left),
                            op,
                            right : Box::new(right)
                        }; 

                        ast_left = ASTNode::new(ast_binop, tok.einfo.clone());
                        
                    }
                    _ => {
                        break;
                    }
                }
            }
            Some(ast_left)
    }
    //DONE
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
    //DONE
    pub fn parse_num(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            self.advance();
            match tok.kind {
                TokenType::INT(x) => { return Some(ASTNode::new(AST::INT { int_value : x}, tok.einfo.clone()))}
                TokenType::FLOAT(x) => {return Some(ASTNode::new(AST::FLOAT { float_value: x }, tok.einfo.clone()))}
                _ => { self.errorstack.errors.push(GError::new_from_tok(ETypes::TokenError, "Expected token 'INT' or 'FLOAT'", tok.einfo.clone()));
                return None; }
            }
        } else {
            None
        }
    }
    
}