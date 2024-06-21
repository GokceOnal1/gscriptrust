use crate::errors::error::*;
use crate::parsing::ast::*;
use crate::parsing::token::*;
use crate::scope::*;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Visitor {
    current_scope : Scope,
    pub errorstack : Rc<RefCell<ErrorStack>>
}
impl Visitor {
    pub fn new(errorstack : Rc<RefCell<ErrorStack>>) -> Visitor {
        Visitor { errorstack, current_scope : Scope::new(None) }
    }
    pub fn visit(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::STRING{..} | AST::INT{..} | AST::FLOAT{..} => { return node.clone(); } ,
            AST::BINOP{..} => { return self.visit_binop(node); }
            AST::VAR_DEF{..} => { return self.visit_variable_definition(node); }
            AST::VAR{..} => { return self.visit_variable(node); }
            AST::FUNC_CALL { .. } => { return self.visit_function_call(node); },
           /*VERY TEMPORARY*/ AST::FUNC_DEF {..} => return node.clone(),
            AST::COMPOUND{ compound_value } => { compound_value.iter().for_each(|ast|  { self.visit(ast); }); return ASTNode::new_noop(); }
            _ => return ASTNode::new_noop()
        }
    }
    pub fn visit_binop(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::BINOP{ left, op, right} => {
                let nleft = self.visit(left);
                let nright = self.visit(right);
                match (nleft.kind, op, nright.kind) {
                    (AST::INT{ int_value: x } , TokenType::PLS, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::INT{ int_value : x + y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::PLS, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x as f32 + y}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::PLS, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x + y as f32}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::PLS, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x + y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::MIN, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::INT{ int_value : x - y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::MIN, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x as f32 - y}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MIN, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x - y as f32}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MIN, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x - y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::MUL, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::INT{ int_value : x * y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::MUL, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x as f32 * y}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MUL, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x * y as f32}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MUL, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x * y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::DIV, AST::INT{ int_value : y }) => {
                        if y == 0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::INT{ int_value : x / y}, node.einfo.clone())
                        }    
                    },
                    (AST::INT{ int_value: x } , TokenType::DIV, AST::FLOAT{ float_value : y }) => {
                        if y == 0.0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x as f32 / y}, node.einfo.clone())
                        }    
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::DIV, AST::INT{ int_value : y }) => {
                        if y == 0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x / y as f32}, node.einfo.clone())
                        }    
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::DIV, AST::FLOAT{ float_value : y }) => {
                        if y == 0.0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x / y}, node.einfo.clone())
                        }    
                    }
                    _ => return ASTNode::new_noop()
                }
            },
            _ => return ASTNode::new_noop()
        }

    }
    pub fn visit_function_call(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::FUNC_CALL { name, args } => {
                match name.as_str() {
                    "write" => return self.std_func_write(args),
                    _ => return ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
         
    }
    pub fn visit_variable_definition(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::VAR_DEF { name, value } => {
                let val = self.visit(value);
                let var_def = ASTNode::new(AST::VAR_DEF { name: name.to_string(), value: Box::new(val) }, node.einfo.clone());
                let res = self.current_scope.add_var(&var_def );
                if let Err(s) = res {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::VariableDefinitionError, s.as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                }
                var_def

            },
            _ => ASTNode::new_noop()
        }
    }
    pub fn visit_variable(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::VAR { name } => {
                let val = self.current_scope.resolve_var(name.to_string());
                if let Some(var_def) = val {
                    match &var_def.kind {
                        AST::VAR_DEF { name: _ , value } => {
                            return *value.clone()
                        },
                        _ => return ASTNode::new_noop()
                    }
                } else {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::IdentifierError, format!("Variable '{}' does not exist in the current scope", name).as_str(), node.einfo.clone()));
                    return ASTNode::new_noop();
                }
            },
            _ => return ASTNode::new_noop()
        }
    }
    //MOVE THIS TO DIFFERENT FILE LATER
    pub fn std_func_write(&mut self, args : &Vec<ASTNode> ) -> ASTNode {
        for arg in args {
            self.visit(arg).print()
        }
        ASTNode::new_noop()
    }
}