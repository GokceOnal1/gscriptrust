use crate::errors::error::*;
use super::ast::*;
use super::token::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<'a> {
    prev_token : Option<&'a Token>,
    curr_token : Option<&'a Token>,
    token_i : usize,
    tokens : &'a Vec<Token>,
    errorstack : Rc<RefCell<ErrorStack>>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens : &'a Vec<Token>, errorstack : Rc<RefCell<ErrorStack>>) -> Parser<'a> {
        Parser { prev_token: None, curr_token: tokens.get(0), token_i : 0, tokens, errorstack }
    }
    pub fn advance(&mut self) {
        self.token_i += 1;
        self.prev_token = self.curr_token;
        self.curr_token = self.tokens.get(self.token_i);
    }
    pub fn verify(&mut self, comparison : TokenType) {
        if self.curr_token.unwrap().kind != comparison {
            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TokenError, format!("Expected token {:?} but received {:?}", comparison, self.curr_token.unwrap().kind ).as_str(), self.curr_token.unwrap().einfo.clone()));
            self.errorstack.borrow().terminate_gs();
        }
    } 
    //DONE
    pub fn parse_compound(&mut self) -> Option<ASTNode> {
        let mut comp_ast = ASTNode::new(AST::COMPOUND { 
            compound_value: ( Vec::new()) 
        }, self.curr_token.unwrap().einfo.clone());
        match comp_ast.kind {
            AST::COMPOUND { ref mut compound_value } => { compound_value.push(self.parse_comp_expr().unwrap_or(ASTNode::new(AST::NOOP, self.curr_token.unwrap().einfo.clone()))) },
            _ => { return None }
        }
        while let Some(tok) = self.curr_token {
            if tok.kind != TokenType::SEMI {
                break;
            } else {
                self.advance();
                match comp_ast.kind {
                    AST::COMPOUND { ref mut compound_value } => { compound_value.push(self.parse_comp_expr().unwrap_or(ASTNode::new(AST::NOOP, self.curr_token.unwrap().einfo.clone()))) },
                    _ => { return None }
                }
            }
        }
        Some(comp_ast)
    }
    //DONE
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
    //DONE
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
                TokenType::ID(_) => { return self.parse_identifier(); }
                TokenType::EOF => { return Some(ASTNode::new(AST::EOF, self.curr_token.unwrap().einfo.clone())); }
                TokenType::LPR => {
                    self.advance();
                    let ast = self.parse_comp_expr();
                    if self.curr_token?.kind != TokenType::RPR {
                        //err
                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::SyntaxError, "Expected ')'", self.curr_token?.einfo.clone()));
                        self.errorstack.borrow().terminate_gs();
                        None
                    } else {
                        self.advance();
                        ast
                    }
                }
                TokenType::MIN => {
                    self.advance();
                    // -- TODO --
                    //handle invalid negative number error
                    let ast_body = self.parse_mono().unwrap_or(ASTNode::new_noop());
                    Some(ASTNode::new(AST::UNOP { op: TokenType::MIN, body: Box::new(ast_body)}, tok.einfo.clone()))
                }
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
                        let right = self.parse_term().unwrap_or(ASTNode::new_noop());

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
    pub fn parse_identifier(&mut self) -> Option<ASTNode> {
        match self.curr_token?.kind {
            TokenType::ID(ref name) => {
                match name.as_str() {
                    "assign" => self.parse_variable_definition(),
                    "funct" => self.parse_function_definition(),
                    "return" => self.parse_return(),
                    "if" => self.parse_if(),
                    "true" => {
                        let res = Some(ASTNode::new(AST::BOOL{ bool_value : true }, self.curr_token?.einfo.clone()));
                        self.advance();
                        res
                    },
                    "false" => {
                        let res = Some(ASTNode::new(AST::BOOL{bool_value : false}, self.curr_token?.einfo.clone()));
                        self.advance();
                        res
                    }
                    _ => self.parse_variable()
                }
            }
            _ => None
        }
    }
    //DONE
    pub fn parse_variable_definition(&mut self) -> Option<ASTNode> {
        self.advance();
        match &self.curr_token?.kind {
            TokenType::ID(name) => {
                let var_name = name;
                let e = self.curr_token?.einfo.clone();
                self.advance();
                self.verify(TokenType::EQL);
                self.advance();
                let var_value = self.parse_comp_expr()?;
                let var_def = ASTNode::new(AST::VAR_DEF { name: var_name.to_string(), value: Box::new(var_value) }, e);
                return Some(var_def);
            }
            _ => None 
        }
    }
    //DONE
    pub fn parse_variable(&mut self) -> Option<ASTNode> {
        self.advance();
        if self.curr_token?.kind == TokenType::LPR {
            return self.parse_function_call();
        } else if self.curr_token?.kind == TokenType::EQL {
            return self.parse_variable_reassign();
        } else {
            let var_name = match &self.prev_token?.kind {
                TokenType::ID(x) => x.to_owned(),
                _ => String::new()
            };
            return Some(ASTNode::new(AST::VAR { name : var_name }, self.prev_token?.einfo.clone()));
        }
        

    }
    //DONE
    pub fn parse_variable_reassign(&mut self) -> Option<ASTNode> {
        let var_name = match &self.prev_token?.kind {
            TokenType::ID(x) => x.to_owned(),
            _ => String::new()
        };
        self.advance(); //past the EQL
        let var_value = self.parse_comp_expr()?;
        Some(ASTNode::new(AST::VAR_REASSIGN { name: var_name, value: Box::new(var_value) }, self.prev_token?.einfo.clone()))
    }
    //DONE
    pub fn parse_function_definition(&mut self) -> Option<ASTNode> {
        self.advance(); //past the 'funct'
        let func_name = match &self.curr_token?.kind {
            TokenType::ID(x) => x.clone(),
            _ => String::new()
        };
        let e = self.curr_token?.einfo.clone();
        self.advance();
        self.verify(TokenType::LPR);
        self.advance();
        let mut func_args: Vec<ASTNode> = Vec::new();
        if self.curr_token?.kind != TokenType::RPR {
            func_args.push(self.parse_function_param().unwrap_or(ASTNode::new_noop()));
        }
        while let Some(tok) = self.curr_token {
            if tok.kind != TokenType::CMA {
                break;
            } else {
                self.advance();
                func_args.push(self.parse_function_param().unwrap_or(ASTNode::new_noop()));
            }
        }
        self.verify(TokenType::RPR);
        self.advance();
        self.verify(TokenType::LBR);
        self.advance();
        let func_body = self.parse_compound()?;
        self.verify(TokenType::RBR);
        self.advance();
        Some(ASTNode::new(AST::FUNC_DEF { body: Box::new(func_body), name: func_name, args: func_args }, e))
    }
    //DONE
    pub fn parse_function_param(&mut self) -> Option<ASTNode> {
        self.verify(TokenType::ID("param".to_string()));
        self.advance();
        let param_name = match &self.curr_token?.kind {
            TokenType::ID(x) => x.clone(),
            _ => String::new()
        };
        self.advance();
        Some(ASTNode::new(AST::VAR_DEF{name:param_name, value: Box::new(ASTNode::new_noop())}, self.curr_token?.einfo.clone()))
    }
    //DONE
    pub fn parse_function_call(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.prev_token {
            match &tok.kind {
                TokenType::ID(name) => {
                    let func_name = name;
                    let mut func_args = Vec::new();
                    self.advance();
                    if self.curr_token?.kind != TokenType::RPR {
                        func_args.push(self.parse_comp_expr().unwrap_or(ASTNode::new_noop()));
                        while let Some(curr_tok) = self.curr_token {
                            if curr_tok.kind != TokenType::CMA {
                                break;
                            } else {
                                self.advance();
                                func_args.push(self.parse_comp_expr().unwrap_or(ASTNode::new_noop()));
                            }
                        }
                    }
                    self.verify(TokenType::RPR);
                    self.advance();
                    Some(ASTNode::new(AST::FUNC_CALL {
                        name : func_name.to_string(), args : func_args
                    }, tok.einfo.clone()))
                }
                _ => None
            }
        } else {
            None
        }
    }
    //DONE
    pub fn parse_return(&mut self) -> Option<ASTNode> {
        self.advance();
        let return_value = self.parse_comp_expr().unwrap_or(ASTNode::new_noop());
        Some(ASTNode::new(AST::RETURN{value:Box::new(return_value)}, self.curr_token?.einfo.clone()))
    }
    pub fn parse_if(&mut self) -> Option<ASTNode> {
        let e = self.curr_token?.einfo.clone();
        self.advance(); //past 'if'
        self.verify(TokenType::LPR);
        self.advance();
        let mut conds : Vec<ASTNode> = Vec::new();
        conds.push(self.parse_comp_expr().unwrap_or(ASTNode::new_noop()));
        self.verify(TokenType::RPR);
        self.advance();
        self.verify(TokenType::LBR);
        self.advance();
        let mut bodies : Vec<ASTNode> = Vec::new();
        bodies.push(self.parse_compound().unwrap_or(ASTNode::new_noop()));
        self.verify(TokenType::RBR);
        self.advance();
        while self.curr_token.is_some() && self.curr_token?.kind == TokenType::ID("else".to_owned()) {
            self.advance(); //past 'else'
            if self.curr_token?.kind == TokenType::ID("if".to_owned()) {
                self.advance(); //past 'if'
                self.verify(TokenType::LPR);
                self.advance();
                conds.push(self.parse_comp_expr().unwrap_or(ASTNode::new_noop()));
                self.verify(TokenType::RPR);
                self.advance();
                self.verify(TokenType::LBR);
                self.advance();
                bodies.push(self.parse_compound().unwrap_or(ASTNode::new_noop()));
                self.verify(TokenType::RBR);
                self.advance();

            } else {
                break;
            }
        };
        let mut else_body = None;
        if self.prev_token?.kind == TokenType::ID("else".to_owned()) {
            self.verify(TokenType::LBR);
            self.advance();
            else_body = Some(Box::new(self.parse_compound().unwrap_or(ASTNode::new_noop())));
            self.verify(TokenType::RBR);
            self.advance();
        }
        Some(ASTNode::new(AST::IF { conditions : conds, bodies, else_body }, e))
    }
    //DONE
    pub fn parse_string(&mut self) -> Option<ASTNode> {
        if let Some(tok) = self.curr_token {
            self.advance();
            match &tok.kind {
                TokenType::STRING(s) => { return Some(ASTNode::new(AST::STRING { str_value : s.to_string() }, tok.einfo.clone())); }
                _ => { self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TokenError, "Expected token 'STRING'", tok.einfo.clone()));
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
                _ => { self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TokenError, "Expected token 'INT' or 'FLOAT'", tok.einfo.clone()));
                return None; }
            }
        } else {
            None
        }
    }
    
}