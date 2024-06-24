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
            AST::STRING{..} | AST::INT{..} | AST::FLOAT{..} | AST::BOOL{..} => { return node.clone(); } ,
            AST::BINOP{..} => { return self.visit_binop(node); }
            AST::UNOP{..} => { return self.visit_unop(node); }
            AST::VAR_DEF{..} => { return self.visit_variable_definition(node); }
            AST::VAR{..} => { return self.visit_variable(node); }
            AST::FUNC_CALL { .. } => { return self.visit_function_call(node); },
            AST::FUNC_DEF {..} => { return self.visit_function_definition(node); },
            AST::RETURN{..} => { return self.visit_return(node); },
            AST::IF{..} => { return self.visit_if(node); }
            AST::COMPOUND{ compound_value } => { 
                for ast in compound_value { 
                    let res = self.visit(ast); 
                    if let AST::RETURN{..} = res.kind {
                        return res;
                    }
                }
                return ASTNode::new_noop(); 
            }
            _ => return ASTNode::new_noop()
        }
    }
    //fix this later... (this is too messy)
    pub fn visit_binop(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::BINOP{ left, op, right} => {
                let nleft = self.visit(left);
                let nright = self.visit(right);
                match (nleft.kind, op, nright.kind) {
                    (AST::INT{ int_value: x } , TokenType::DEQL, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::BOOL{ bool_value : x == y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::DEQL, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::BOOL{ bool_value : x as f32 == y}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::DEQL, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::BOOL{ bool_value : x == y as f32}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::DEQL, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::BOOL{ bool_value : x == y}, node.einfo.clone())
                    },
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
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MOD, AST::FLOAT{ float_value : y }) => {
                        if y == 0.0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x % y}, node.einfo.clone())
                        }    
                    },
                    (AST::INT{ int_value: x } , TokenType::MOD, AST::INT{ int_value : y }) => {
                        if y == 0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::INT{ int_value : x % y}, node.einfo.clone())
                        }    
                    },
                    (AST::INT{ int_value: x } , TokenType::MOD, AST::FLOAT{ float_value : y }) => {
                        if y == 0.0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x as f32 % y}, node.einfo.clone())
                        }    
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::MOD, AST::INT{ int_value : y }) => {
                        if y == 0 {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", node.einfo.clone()));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        } else {
                            return ASTNode::new(AST::FLOAT{ float_value : x % y as f32}, node.einfo.clone())
                        }    
                    },
                    _ => return ASTNode::new_noop()
                }
            },
            _ => return ASTNode::new_noop()
        }

    }
    pub fn visit_unop(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::UNOP { op, body } => {
                let body_val = self.visit(body);
                match (op, &body_val.kind) {
                    (TokenType::MIN, AST::INT{int_value}) => {
                        return ASTNode::new(AST::INT{ int_value : -int_value}, node.einfo.clone());
                    }
                    (TokenType::MIN, AST::FLOAT{float_value}) => {
                        return ASTNode::new(AST::FLOAT{float_value : -float_value}, node.einfo.clone());
                    }
                    _ => return ASTNode::new_noop()
                }
            }
            _ => return ASTNode::new_noop()
        }
    }
    pub fn visit_function_call(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::FUNC_CALL { name, args } => {
                match name.as_str() {
                    "write" => return self.std_func_write(args),
                    _ => {}
                }
                //cloning here because of borrowing rules
                let curr_scope = self.current_scope.clone();
                let fdef_option = curr_scope.resolve_func(name.clone());
                if let Some(fdef) =  fdef_option {
                    match &fdef.kind {
                        AST::FUNC_DEF { name: _, body: fdef_body, args: fdef_args } => {
                            if args.len() != fdef_args.len() {
                                //improper args error
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function '{}' requires {} argument(s), not {}", name, fdef_args.len(), args.len()).as_str(), node.einfo.clone()));
                                return ASTNode::new_noop();
                            }
                            // -- FIXED --
                            // by implementing Scope::get_root_scope
                            // !! ISSUE !!
                            //setting self.current_scope as the parent DOES NOT WORK in recursive scenarios
                            //solution: set the parent as some kind of global scope
                            // !! END ISSUE !!
                            let mut func_scope = Scope::new(Some(Box::new(Scope::get_root_scope(self.current_scope.clone()))));
                            for (argdef, arg) in fdef_args.iter().zip(args.iter()) {
                                let arg_val = self.visit(arg);

                                if let AST::VAR_DEF{name: argdef_name, ..} = &argdef.kind {
                                    if let Err(s) = func_scope.add_var(&ASTNode::new(AST::VAR_DEF { name: argdef_name.clone() , value: Box::new(arg_val) }, arg.einfo.clone())) {
                                        println!("{}",s);
                                    }
                                } else {
                                    //this should never happen...
                                    eprintln!("func argdef error");
                                    return ASTNode::new_noop();
                                }
                                
                            }
                            let cscope = self.current_scope.clone();
                            self.current_scope = func_scope;
                            let res = self.visit(&fdef_body);
                            self.current_scope = cscope;
                            if let AST::RETURN { value } = res.kind {
                                *value
                            } else {
                                res
                            }

                        },
                        _ => ASTNode::new_noop()
                    }
                } else {
                    //function is not defined error
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function '{}' does not exist in the current scope",name).as_str(), node.einfo.clone()));
                    ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
         
    }
    pub fn visit_return(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::RETURN{value} => ASTNode::new(AST::RETURN{value: Box::new(self.visit(&value))}, node.einfo.clone()),
            _ => ASTNode::new_noop()
        }
    }
    pub fn visit_function_definition(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::FUNC_DEF { .. } => {
                if let Err(s) = self.current_scope.add_func(node) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionDefinitionError, s.as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                }
                node.clone()
            },
            _ => return ASTNode::new_noop()
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
    pub fn visit_if(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::IF {
                conditions, bodies, else_body
            } => {
                for (cond, body) in conditions.iter().zip(bodies.iter()) {
                    let cond_val = self.visit(cond);
                    match &cond_val.kind {
                        AST::BOOL { bool_value } => {
                            if *bool_value {
                                let res = self.visit(body);
                                if let AST::RETURN{..} = res.kind {
                                    return res;
                                } else {
                                    return ASTNode::new_noop();
                                }
                            }
                        },
                        _ => { self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ConditionalError, "Expected conditional expression", cond.einfo.clone())); return ASTNode::new_noop()}
                    }
                }
                if let Some(b) = else_body {
                    let res = self.visit(&b);
                    if let AST::RETURN{..} = res.kind {
                        return res;
                    } else {
                        return ASTNode::new_noop();
                    }
                } else {
                    return ASTNode::new_noop();
                }

            },
            _ => ASTNode::new_noop()
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