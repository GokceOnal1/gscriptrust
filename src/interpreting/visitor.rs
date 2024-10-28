use crate::errors::error::*;
use crate::parsing::ast::*;
use crate::parsing::token::*;
use crate::scope::*;
use crate::stdfunc::*;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Visitor {
    current_scope : Rc<RefCell<Scope>>,
    pub errorstack : Rc<RefCell<ErrorStack>>,
    keywords : Vec<String>
}
impl Visitor {
    pub fn new(errorstack : Rc<RefCell<ErrorStack>>) -> Visitor {
        Visitor { 
            errorstack, 
            current_scope : Rc::new(RefCell::new(Scope::new(None))), 
            keywords : vec!["assign", "funct", "if", "param", "return", "blueprint", "new", "while", "break", "true", "false"].iter().map(|x| x.to_string()).collect()
        }
    }
    pub fn visit(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            //maybe change this to a deref of node?
            AST::STRING{..} | AST::INT{..} | AST::FLOAT{..} | AST::BOOL{..} | AST::BREAK | AST::OBJECT{..} => { return node.clone(); } ,
            AST::BINOP{..} => { return self.visit_binop(node); }
            AST::UNOP{..} => { return self.visit_unop(node); }
            AST::LIST{..} => {return self.visit_list(node); }
            AST::INDEX{..} => { return self.visit_index(node); }
            AST::LIST_REASSIGN { .. } => { return self.visit_list_reassign(node); }
            AST::VAR_DEF{..} => { return self.visit_variable_definition(node); }
            AST::VAR_REASSIGN{..} => { return self.visit_variable_reassign(node); }
            AST::VAR{..} => { return self.visit_variable(node); }
            AST::FUNC_CALL { .. } => { return self.visit_function_call(node, None); },
            AST::FUNC_DEF {..} => { return self.visit_function_definition(node); },
            AST::RETURN{..} => { return self.visit_return(node); },
            AST::IF{..} => { return self.visit_if(node); }
            AST::WHILE{..} => { return self.visit_while(node); }
            AST::CLASS{..} => { return self.visit_blueprint(node); }
            AST::NEW{..} => { return self.visit_new(node); }
            AST::OBJECT_INDEX { .. } => { return self.visit_obj_index(node); }
            AST::OBJECT_REASSIGN { .. } => { return self.visit_obj_reassign(node); }
            AST::COMPOUND{ compound_value } => { 
                for ast in compound_value { 
                    let res = self.visit(ast); 
                    if let AST::RETURN{..} | AST::BREAK = res.kind {
                        return res;
                    }
                }
                return ASTNode::new_noop(); 
            }
            _ => return ASTNode::new_noop()
        }
    }
    //visit_binop helper function
    fn node_to_int(&mut self, node : &ASTNode) -> Option<i32> {
        match node.kind {
            AST::FLOAT{ float_value } => Some(float_value as i32),
            AST::INT{ int_value } => Some(int_value),
            _ => None
        }
    }
    //visit_binop helper function
    fn node_to_float(&mut self, node : &ASTNode) -> Option<f32> {
        match node.kind {
            AST::FLOAT{float_value} => Some(float_value),
            AST::INT{int_value} => Some(int_value as f32),
            _ => { /*println!("{:?}", node.kind);*/ None }
        }
    }
    pub fn visit_binop(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::BINOP{ left, op, right} => {
                let nleft = self.visit(left);
                let nright = self.visit(right);
                match op {
                    TokenType::PLS => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::INT{ int_value: self.node_to_int(&nleft).unwrap() + self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::FLOAT { float_value: fleft.unwrap() + fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::MIN => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::INT{ int_value: self.node_to_int(&nleft).unwrap() - self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::FLOAT { float_value: fleft.unwrap() - fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::MUL => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::INT{ int_value: self.node_to_int(&nleft).unwrap() * self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::FLOAT { float_value: fleft.unwrap() * fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::DIV => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            let fright = self.node_to_int(&nright).unwrap();
                            if fright == 0 {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", nright.einfo.clone()));
                                self.errorstack.borrow().terminate_gs();
                                return ASTNode::new_noop();
                            }
                            return ASTNode::new(AST::INT{ int_value: self.node_to_int(&nleft).unwrap() / fright}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                if fright.unwrap() == 0.0 {
                                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", nright.einfo.clone()));
                                    self.errorstack.borrow().terminate_gs();
                                    return ASTNode::new_noop();
                                }
                                return ASTNode::new(AST::FLOAT { float_value: fleft.unwrap() / fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::MOD => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            let fright = self.node_to_int(&nright).unwrap();
                            if fright == 0 {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", nright.einfo.clone()));
                                self.errorstack.borrow().terminate_gs();
                                return ASTNode::new_noop();
                            }
                            return ASTNode::new(AST::INT{ int_value: self.node_to_int(&nleft).unwrap() % fright}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                if fright.unwrap() == 0.0 {
                                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::DivideByZeroError, "Cannot divide by zero", nright.einfo.clone()));
                                    self.errorstack.borrow().terminate_gs();
                                    return ASTNode::new_noop();
                                }
                                return ASTNode::new(AST::FLOAT { float_value: fleft.unwrap() % fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::DEQL => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value : self.node_to_int(&nleft).unwrap() == self.node_to_int(&nright).unwrap()}, node.einfo.clone());   
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                match (nleft.kind, nright.kind) {
                                    (AST::BOOL{bool_value : x}, AST::BOOL{bool_value : y}) => {
                                        return ASTNode::new(AST::BOOL { bool_value: x == y }, node.einfo.clone());
                                    },
                                    (AST::STRING{str_value: x}, AST::STRING{str_value : y}) => {
                                        //println!("{} == {}", x, y);
                                        return ASTNode::new(AST::BOOL { bool_value: x == y}, node.einfo.clone());
                                    },
                                    (AST::TYPE{type_value:t1}, AST::TYPE{type_value:t2} ) => {
                                        return ASTNode::new(AST::BOOL {bool_value : t1 == t2}, node.einfo.clone());
                                    },
                                    _ => return ASTNode::new_noop()
                                }
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() == fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::NEQ => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value : self.node_to_int(&nleft).unwrap() != self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                match (nleft.kind, nright.kind) {
                                    (AST::BOOL{bool_value : x}, AST::BOOL{bool_value : y}) => {
                                        return ASTNode::new(AST::BOOL { bool_value: x != y }, node.einfo.clone());
                                    },
                                    (AST::STRING{str_value: x}, AST::STRING{str_value : y}) => {
                                        return ASTNode::new(AST::BOOL { bool_value: x != y}, node.einfo.clone());
                                    },
                                    _ => return ASTNode::new_noop()
                                }
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() != fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::LT => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value: self.node_to_int(&nleft).unwrap() < self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() < fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::LTE => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value: self.node_to_int(&nleft).unwrap() <= self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() <= fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::GT => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value: self.node_to_int(&nleft).unwrap() > self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() > fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::GTE => {
                        if matches!(nleft.kind, AST::INT{..}) && matches!(nright.kind, AST::INT{..}) {
                            return ASTNode::new(AST::BOOL{ bool_value: self.node_to_int(&nleft).unwrap() >= self.node_to_int(&nright).unwrap()}, node.einfo.clone());
                        } else {
                            let fleft = self.node_to_float(&nleft);
                            let fright = self.node_to_float(&nright);
                            if fleft.is_none() || fright.is_none() {
                                return ASTNode::new_noop();
                            } else {
                                return ASTNode::new(AST::BOOL { bool_value: fleft.unwrap() >= fright.unwrap() }, node.einfo.clone());
                            }
                        }
                    },
                    TokenType::AND => {
                        match (nleft.kind, nright.kind) {
                            (AST::BOOL{bool_value: x}, AST::BOOL{bool_value: y}) => {
                                return ASTNode::new(AST::BOOL{bool_value: x && y}, node.einfo.clone());
                            },
                            _ => return ASTNode::new_noop()
                        }
                    },
                    TokenType::OR => {
                        match (nleft.kind, nright.kind) {
                            (AST::BOOL{bool_value: x}, AST::BOOL{bool_value: y}) => {
                                return ASTNode::new(AST::BOOL{bool_value: x || y}, node.einfo.clone());
                            },
                            _ => return ASTNode::new_noop()
                        }
                    },
                    
                    _ => { return ASTNode::new_noop() }
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
                    (TokenType::NOT, AST::BOOL{bool_value}) => {
                        return ASTNode::new(AST::BOOL{bool_value: !bool_value}, node.einfo.clone());
                    }
                    _ => return ASTNode::new_noop()
                }
            }
            _ => return ASTNode::new_noop()
        }
    }
    pub fn visit_function_call(&mut self, node : &ASTNode, extern_scope : Option<Rc<RefCell<Scope>>>) -> ASTNode {
        
        match &node.kind {
            AST::FUNC_CALL { name, args } => {
                match name.as_str() {
                    "write" => return std_func_write(self, args),
                    "read" => return std_func_read(self, node, args),
                    "ast_debug" => return std_func_debug(self, args), 
                    "type" => return std_func_type(self, node, args),
                    _ => {}
                }
                //Interesting how I need to store the borrowed currscope in a local variable
                //I think it's because since the b_currscope is declared after currscope, it 
                //is dropped first, meaning currscope can then be safely dropped
                let fdef_option = {
                    let currscope = self.current_scope.clone();
                    let b_currscope = currscope.borrow();
                    b_currscope.resolve_func(name.clone())
                };

                if let Some(fdef) = fdef_option {
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
                            
                            let func_scope = match extern_scope {
                                Some(s) => Rc::new(RefCell::new(Scope::new(Some(s.clone())))),
                                None => Rc::new(RefCell::new(Scope::new(Some(Scope::get_root_scope(self.current_scope.clone())))))
                            };
                            //println!("func_scope: {:#?}", func_scope);
                            for (argdef, arg) in fdef_args.iter().zip(args.iter()) {
                                let arg_val = self.visit(arg);

                                if let AST::VAR_DEF{name: argdef_name, ..} = &argdef.kind {
                                    if let Err(s) = func_scope.borrow_mut().add_var(&ASTNode::new(AST::VAR_DEF { name: argdef_name.clone() , value: Box::new(arg_val) }, arg.einfo.clone())) {
                                        println!("{}",s);
                                    }
                                } else {
                                    //this should never happen...
                                    eprintln!("func argdef error (should not ever be reached)");
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
            AST::FUNC_DEF { name, .. } => {
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionDefinitionError, "Illegal use of keyword for function definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
                if let Err(s) = self.current_scope.borrow_mut().add_func(node) {
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
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::VariableDefinitionError, "Illegal use of keyword for variable definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
                let val = self.visit(value);
               // println!("{:#?}", val);
                let var_def = ASTNode::new(AST::VAR_DEF { name: name.to_string(), value: Box::new(val) }, node.einfo.clone());
                let mut thingy = self.current_scope.borrow_mut();
                let res = thingy.add_var(&var_def );
                if let Err(s) = res {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::VariableDefinitionError, s.as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                }
                //println!("{:#?}", self.current_scope.borrow().resolve_var("a".to_string()));
                var_def

            },
            _ => ASTNode::new_noop()
        }
    }
    pub fn visit_variable_reassign(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::VAR_REASSIGN { name, value } => {
                let val = self.visit(value);
                let var_def = ASTNode::new(AST::VAR_DEF { name: name.clone(), value: Box::new(val) }, node.einfo.clone());
                let res = self.current_scope.borrow_mut().set_var(name.clone(), &var_def);
                if let Err(s) = res {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::VariableDefinitionError, s.as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                }
                var_def
            },
            _ => ASTNode::new_noop()
        }
    }
    // -- ISSUE --
    // calling 'clone' on node DOESN'T DEEP COPY all the fields if the fields are Rc RefCell
    // so cloning an object will not clone its scope or all the properties/methods within that scope
    pub fn visit_variable(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::VAR { name } => {
                if let Some(var_def) = self.current_scope.borrow().resolve_var(name.to_string()) {
                    match &var_def.borrow().kind {
                        AST::VAR_DEF { name: _ , value } => {

                            if let AST::OBJECT{class_name, scope} = &value.kind {
                               // println!("{:#?}", scope.borrow().parent.as_ref().unwrap().borrow().variables);
                              // println!("{:#?}", scope);
                                let scope = Scope::deep_clone(Some(scope.clone())).unwrap();
                                
                                return ASTNode::new(AST::OBJECT{class_name: class_name.clone(), scope }, node.einfo.clone())
                            }
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
                                if let AST::RETURN{..} | AST::BREAK = res.kind {
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
                    if let AST::RETURN{..} | AST::BREAK = res.kind {
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
    pub fn visit_while(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::WHILE { condition, body } => {
                // -- ISSUE --
                // scope is doing weird things
                // make parent Rc RefCell instead of owned
                // -- END ISSUE --
                // -- SOLUTION --
                // Take the memory of the parent of the temporary scope to reuse it on future iterations
                //kind of works for now but it might be a better idea to make parent an Rc Refcell reference instead of owned 
                    let origin = self.current_scope.clone();
                    loop {
                        self.current_scope = Rc::new(RefCell::new(Scope::new(Some(origin.clone()))));
                        let cond = self.visit(condition);
                        let cond_res = match cond.kind {
                            AST::BOOL { bool_value } => { bool_value },
                            _ => { return ASTNode::new_noop(); } //invalid conditional expression error
                        };
                        if !cond_res {
                            break;
                        }
                        let res = self.visit(body);
                        if let AST::RETURN{..} = res.kind {
                            self.current_scope = origin;
                            return res;
                        } else if let AST::BREAK = res.kind {
                            self.current_scope = origin;
                            return ASTNode::new_noop();
                        }
                    }
                    self.current_scope = origin;
                    ASTNode::new_noop()
            },
            _ => return ASTNode::new_noop()
        }
    }
    pub fn node_to_string(&mut self, node : &ASTNode) -> String {
        match &node.kind {
            AST::STRING{str_value} => str_value.clone(),
            AST::INT { int_value} => int_value.to_string(),
            AST::FLOAT { float_value} => float_value.to_string(),
            AST::BOOL {bool_value} => bool_value.to_string(),
            AST::LIST { contents } => {
                let mut s = String::new();
                let mut b = false;
                s.push('[');
                for c in contents {
                    if b {
                        s.push(',');
                        s.push(' ');
                    }
                    b = true;
                    s.push_str(&self.node_to_string(&c.borrow()));
                }
                s.push(']');
                s
            },
            AST::OBJECT { class_name, scope } => {
                //println!("{:#?}", scope);
                let mut s = String::new();
                s.push_str(&class_name);
                s.push_str(" instance { \n");
                let mut a = false;
                for( varname, vardef ) in &scope.borrow().variables {
                    if a {
                        s.push_str(",\n");
                    }
                    a = true;
                    s.push_str(&varname);
                    s.push_str(": ");
                    s.push('"');
                    match &vardef.borrow().kind {
                        AST::VAR_DEF {name: _, value} => {
                            let visited = self.visit(value);
                            s.push_str(&self.node_to_string(&visited));

                        },
                        _ => { }
                    }
                    s.push('"');
                }
                s.push_str(",\n");
                let mut b = false;
                for(funcname, funcdef) in &scope.borrow().functions {
                    if b {
                        s.push_str(",\n");
                    }
                    b = true;
                    s.push_str(&funcname);
                    s.push_str(": function(");
                    match &funcdef.kind {
                        AST::FUNC_DEF { name: _, body: _, args } => {
                            let mut f = false;
                            for arg in args {
                                if f {
                                    s.push_str(", ");
                                } else {
                                    s.push_str("params: ");
                                }
                                f = true;
                                match &arg.kind {
                                    AST::VAR_DEF{name, ..} => {
                                        s.push_str(&name);
                                    },
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }
                    s.push_str(")");
                }
                s.push_str("\n}");
                s
            },
            AST::TYPE{ type_value } => { type_value.to_string() }
            AST::NOOP => "no operation".to_string(),
            _ => format!("undefined: \n{:#?}", node).to_string()
        }
    }
    pub fn visit_list(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::LIST{contents} => {
                ASTNode::new(AST::LIST{contents : contents.iter().map(|x| Rc::new(RefCell::new(self.visit(&x.borrow())))).collect()}, node.einfo.clone())
            },
            _ => ASTNode::new_noop()
        }
    }
    pub fn visit_index(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::INDEX{target, indices} => {
                let mut combined_target = self.visit(target);
                for ind in indices {
                    let ind = self.visit(ind);
                    let ind_i: i32;
                    match ind.kind {
                        AST::INT { int_value } => ind_i = int_value,
                        _ => { 
                            //index is not a number error
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Expected integer to index list", ind.einfo.clone()));
                            return ASTNode::new_noop()
                        }
                    }
                    match combined_target.kind {
                        AST::LIST { contents } => {
                            if ind_i < 0 || ind_i as usize >= contents.len() {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, format!("Index {} is out of bounds for list of length {}", ind_i, contents.len()).as_str(), ind.einfo.clone()));
                                self.errorstack.borrow().terminate_gs();
                            }
                            combined_target = contents[ind_i as usize].borrow().clone()
                        },
                        _ => {
                            //target is not a list error
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Indexed target is not a list", combined_target.einfo.clone()));
                            return ASTNode::new_noop()
                        } 
                    }


                }
                combined_target
            }
            _ => ASTNode::new_noop()
        }
    }
    // -- ISSUE --
    // need to make AST::LIST's contents Vec<Rc<RefCell<ASTNode>>> instead of Vec<ASTNode>
    // to enabe getting a mutable reference to an inner element
    // -- SOLUTION --
    // did this
    pub fn visit_obj_index(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::OBJECT_INDEX { object, property } => {
                let obj = self.visit(object);
                match &obj.kind {
                    AST::OBJECT{ class_name, scope } => {
                        //println!("{:#?}", scope);
                        match &property.kind {
                            AST::VAR{name} => {
                                if let Some(val) = scope.borrow().resolve_var(name.clone()) {
                                    if let AST::VAR_DEF{name:_, value} = &val.borrow().kind {
                                        return *value.clone();
                                    }
                                    val.borrow().clone()
                                } else {
                                    //property does not exist error
                                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, format!("Property '{}' does not exist on blueprint '{}'", name.clone(), class_name.clone()).as_str(), property.einfo.clone()));
                                    ASTNode::new_noop()
                                }
                            }
                            AST::FUNC_CALL{..} => {
                                let oscope = self.current_scope.clone();
                                self.current_scope = scope.clone();
                                //added alternate scope arg to visit_function_call
                                //since we want globally defined blueprints to be visible within methods of other blueprints
                              //  println!("{:#?}", self.current_scope);
                                let res = self.visit_function_call(property, None);
                                self.current_scope = oscope;
                                res
                            }
                            AST::INDEX{..} => {
                                let oscope = self.current_scope.clone();
                                self.current_scope = scope.clone();
                                let res = self.visit_index(property);
                                self.current_scope = oscope;
                                res
                            }
                            _ => { println!("you are indexing something other than a method or property or list!"); ASTNode::new_noop() }
                        }
                        
                    }
                    _ => {
                        //indexed identifier is not an object error
                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::SyntaxError, "Invalid use of dot operator on non-object", node.einfo.clone()));
                        self.errorstack.borrow().terminate_gs();
                        ASTNode::new_noop()
                    }
                }
            }
            _ => ASTNode::new_noop()

        }
    }
    fn obj_index_mut(&mut self, node : &ASTNode) -> Rc<RefCell<ASTNode>> {
        match &node.kind {
            AST::OBJECT_INDEX { object, property } => {
                //println!("object side: {:#?}", object);
                //println!("property side: {:#?}", property);
                let obj = match &object.kind {
                    AST::OBJECT_INDEX { .. } => {
                        self.obj_index_mut(object)
                    },
                    AST::OBJECT{class_name:_, scope} => {
                        match &property.kind {
                            AST::VAR{name} => {
                                let vdef = scope.borrow().resolve_var(name.clone()).unwrap();
                                vdef
                            }
                            _ => {println!("something"); return Rc::new(RefCell::new(ASTNode::new_noop())) }
                        }
                    },
                    AST::VAR{name} => {
                        let odef = self.current_scope.borrow().resolve_var(name.clone()).unwrap();
                        odef
                    }
                    _ => { println!("NOOO!"); Rc::new(RefCell::new(ASTNode::new_noop())) }
                };
                let obj_b = obj.borrow();
                match &obj_b.kind {
                    AST::VAR_DEF { name: _, value } => {
                        //println!("prop name: {}", n);
                        match &value.kind {
                            AST::OBJECT{class_name:_, scope} => {
                                match &property.kind {
                                    AST::VAR{name} => {
                                        //println!("prop name: {}", name);
                                        if let Some(val) = scope.borrow().resolve_var(name.clone()) {
                                            //println!("val of p prop: {:#?}", val.clone());
                                            Rc::clone(&val)
                                        } else {
                                            println!("property does not exist in object");
                                            std::process::exit(1)
                                        }
                                    }
                                    AST::INDEX{..} => {
                                        let oscope = self.current_scope.clone();
                                        self.current_scope = scope.clone();
                                        let mut_list_ref = self.list_get_mut(&property);
                                        self.current_scope = oscope;
                                        Rc::clone(&mut_list_ref)
                                    }
                                    _ => { Rc::new(RefCell::new(ASTNode::new_noop())) }
                                }     
                            }
                            _ => { 
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::SyntaxError, "Invalid use of dot operator on non-object", property.einfo.clone()));
                                self.errorstack.borrow().terminate_gs();
                                Rc::new(RefCell::new(ASTNode::new_noop()))
                             }
                        }
                    }
                    _ => { println!("here"); std::process::exit(1) }
                }
            },
            _ => { println!("here1"); std::process::exit(1) }
        }
    }
    pub fn visit_obj_reassign(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::OBJECT_REASSIGN { object_index, value } => {
                match &object_index.kind {
                    AST::OBJECT_INDEX { object, property } => {
                        let mut ei = object.einfo.clone();
                        
                        let obj = match &object.kind {
                            //this is in the case that object is another OBJECT_INDEX
                            //as in var.x.y
                            AST::OBJECT_INDEX { object:_, property:pr } => { ei = pr.einfo.clone(); self.obj_index_mut(object)} ,
                            //and this is the case where object would literally just be the identifier for the object
                            //like the 'var' in var.x
                            AST::VAR{name} => self.current_scope.borrow().resolve_var(name.clone()).unwrap(),
                            _ => Rc::new(RefCell::new(ASTNode::new_noop()))
                        };
                        
                        let mut obj_b = obj.borrow_mut();
                        let new_value = self.visit(&value);
                        let new_value_einfo = new_value.einfo.clone();
                        let new_value_vardef = match &property.kind {
                            AST::VAR{name} => {
                                ASTNode::new(AST::VAR_DEF{name : name.clone(), value : Box::new(new_value.clone())}, new_value_einfo.clone())
                            }
                            _ => ASTNode::new_noop()
                        };
                        //obj_b is a var def in the case of a SIMPLE property, like var.x
                        if let AST::VAR_DEF { name: _, value:v } = &mut obj_b.kind {
                            if let AST::OBJECT { class_name: _, scope } = &v.kind {
                                match &property.kind {
                                    AST::VAR{name} => {
                                    
                                        let _ = scope.borrow_mut().set_var(name.clone(), &new_value_vardef);
                                    }
                                    AST::INDEX{..} => {
                                        let oscope = self.current_scope.clone();
                                        self.current_scope = scope.clone();
                                        let list_reassign = ASTNode::new(AST::LIST_REASSIGN{target: Box::new(*property.clone()), value: Box::new(new_value.clone())}, new_value_einfo.clone());
                                        self.visit_list_reassign(&list_reassign);
                                        self.current_scope = oscope;
                                        
                                    }
                                    _ => { return ASTNode::new_noop()}
                                }
                                
                            } else {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::SyntaxError, "Invalid use of dot operator on non-object", ei.clone()));
                                self.errorstack.borrow().terminate_gs();
                            }
                        //HOWEVER, obj_b will not be a VAR_DEF because of the stupid way I implemented this...
                        // it will instead be AST::OBJECT as a result of calling list_get_mut 
                        // basically this is the exact same code as above
                        // this bottom code will run when the gsc code looks like 'var.x[1].prop = 5'
                        } else if let AST::OBJECT { class_name: _, scope } = &mut obj_b.kind {
                            match &property.kind {
                                AST::VAR{name} => {
                                
                                    let _ = scope.borrow_mut().set_var(name.clone(), &new_value_vardef);
                                }
                                //This is if the gsc code looks like 'var.x.prop[1] = 5'
                                // it simply artificially creates a LIST_REASSIGN node and goes within the object's scope
                                AST::INDEX{..} => {
                                    let oscope = self.current_scope.clone();
                                    self.current_scope = scope.clone();
                                    let list_reassign = ASTNode::new(AST::LIST_REASSIGN{target: Box::new(*property.clone()), value: Box::new(new_value.clone())}, new_value_einfo.clone());
                                    self.visit_list_reassign(&list_reassign);
                                    self.current_scope = oscope;
                                    
                                }
                                _ => { return ASTNode::new_noop()}
                            }
                            
                        } else {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::SyntaxError, "Token indexed with dot is not an object", ei.clone()));
                            self.errorstack.borrow().terminate_gs();
                        }
                        return ASTNode::new_noop()

                    } 
                    _ => ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
    }
    //(old, for testing purposes)
    //need to analyze why this didn't work...

    //--ANALYSIS OF SEVERE ISSUE -- 
    // it didn't work because I was mutably borrowing list_ref_i when trying to extract
    //the list from the var_def match but then also later on doing the actual algorithm which
    //also borrowed mutably.
    //--SOLUTION--
    //just self.visit target like normal and overwrite the old list value with the new one
    //--REVISED FUNCTION--
    // -- UPDATE --
    // made LIST's contents Vec<Rc<RefCell<ASTNode>>> so 
    pub fn visit_list_reassign(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::LIST_REASSIGN { target, value } => {
                let value = self.visit(value);
                match &target.kind {
                    AST::INDEX { target: list, indices} => {
                        let list_name = match &list.kind {
                            AST::VAR{name} => name.clone(),
                            _ => String::new()
                        };
                        // the way this works is we gain a copy of the list referred to by 'list'
                        // then we store it in orig_list
                        // then iterate through the indices, using list_ref_i as a mutable reference to the contents
                        // of orig_list to finally change the desired value
                        // then we set orig_list to the actual list stored in the scope
                        let orig_list = self.visit(list);
                        let orig_list_ref = Rc::new(RefCell::new(orig_list));
                        let mut list_ref_i = orig_list_ref.clone();
                        
                        //at this point list_ref_i is a mutable reference to the list
                        //referred to by the identifier value of "list"
                        for(i, i_node) in indices.iter().enumerate() {
                            let i_val = self.visit(i_node);
                            let actual_i = match i_val.kind {
                                AST::INT{int_value} => int_value,
                                _ => {
                                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Expected integer to index list", i_val.einfo.clone()));
                                    self.errorstack.borrow().terminate_gs();
                                    0
                                }
                            };
                            //println!("{}", actual_i);
                            let e = list_ref_i.borrow().einfo.clone();
                            if i == indices.len() - 1 {
                                //now we know we have reached the point to actually set the variable
                                
                                match &mut list_ref_i.borrow_mut().kind {
                                    AST::LIST{contents} => {
                                        //println!("here");
                                        //make sure index is valid
                                        if actual_i < 0 || actual_i as usize >= contents.len() {
                                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, format!("Index {} is out of bounds for list of length {}", actual_i, contents.len()).as_str(), i_val.einfo.clone()));
                                            self.errorstack.borrow().terminate_gs();
                                        }
                                        contents[actual_i as usize] = Rc::new(RefCell::new(value.clone()));
                                        
                                    },
                                    _ => { 
                                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Indexed object is not a list", e.clone()));
                                        return ASTNode::new_noop() }  //handle error here
                                }
                            } else {
                                let thingy = list_ref_i.clone();
                                let mut thingy2 = thingy.borrow_mut();
                                match &mut thingy2.kind {
                                    AST::LIST {contents} => {
                                        //make sure index is valid  
                                        if actual_i < 0 || actual_i as usize >= contents.len() {
                                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, format!("Index {} is out of bounds for list of length {}", actual_i, contents.len()).as_str(), i_val.einfo.clone()));
                                            self.errorstack.borrow().terminate_gs();
                                        }
                                        list_ref_i = contents[actual_i as usize].clone();
                                        
                                    }
                                    _ => { 
                                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Indexed object is not a list", e.clone()));
                                        return ASTNode::new_noop() }//handle error here
                                }
                            }
                            let _ = self.current_scope.borrow_mut().set_var(list_name.clone(), &ASTNode::new(AST::VAR_DEF{name : list_name.clone(), value : Box::new(orig_list_ref.borrow().clone()) }, e.clone()));
                        }
                       // list_ref.print();
                        return ASTNode::new_noop();
                       
                    },
                    _ => return ASTNode::new_noop()
                }
            }
            _ => ASTNode::new_noop()
        }
    }
    // this gets a mutable reference (in the form of Rc<RefCell<ASTNode>>) to an element of an n-dimensional list
    pub fn list_get_mut(&mut self, node : &ASTNode) -> Rc<RefCell<ASTNode>> {
        match &node.kind {
            AST::INDEX{target, indices} => {
                let list_name = match &target.kind {
                    AST::VAR{name} => name.clone(),
                    _ => String::new()
                }; 
                let list_def_ref = self.current_scope.borrow().resolve_var(list_name.clone()).unwrap();
                let list_def_ref_borrowed = list_def_ref.borrow();
                match &list_def_ref_borrowed.kind {
                    AST::VAR_DEF { name:_, value } => {
                        
                        match &value.kind {
                            AST::LIST{contents: s_contents} => {
                                let mut curr_ref = Rc::new(RefCell::new(ASTNode::new_noop()));
                                for(i, i_node) in indices.iter().enumerate() {
                                    let i_val = self.visit(i_node);
                                    let actual_i = match i_val.kind {
                                        AST::INT{int_value} => int_value,
                                        _ => {
                                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Expected integer to index list", i_val.einfo.clone()));
                                            self.errorstack.borrow().terminate_gs();
                                            0
                                        }
                                    };
                                    if i == 0 {
                                        curr_ref = s_contents[actual_i as usize].clone();
                                        continue;
                                    }
                                    if i == indices.len() - 1 {
                                        match &curr_ref.borrow().kind {
                                            AST::LIST{contents} => {
                                                return contents[actual_i as usize].clone();
                                            }
                                                _ => { println!("err1 {:#?}", curr_ref.borrow()); return Rc::new(RefCell::new(ASTNode::new_noop()))}
                                        }
                                    } else {
                                        let thingy = curr_ref.clone();
                                        let thingy2 = thingy.borrow();
                                        match &thingy2.kind {
                                            AST::LIST{contents} => {
                                                curr_ref = contents[actual_i as usize].clone()
                                            }
                                            _ => return Rc::new(RefCell::new(ASTNode::new_noop()))
                                        }
                                    }
                                }

                                curr_ref
                                
                            }
                            _ => Rc::new(RefCell::new(ASTNode::new_noop()))
                        }
                    }
                    _ => Rc::new(RefCell::new(ASTNode::new_noop()))
                }
            }
            _ => Rc::new(RefCell::new(ASTNode::new_noop()))
        }
    }
   
    pub fn visit_blueprint(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::CLASS{name, ..} => {
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, "Illegal use of keyword for blueprint definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
                if let Err(s) = self.current_scope.borrow_mut().add_blueprint(node) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, s.as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                }
                node.clone()
            },
            _ => ASTNode::new_noop()
        }
    }
    //--ISSUE--
    //scopes and their parents are owned, so when I clone them, original values are not modified in the end
    //--SOLUTION--
    //but to fix this I will need to make parent scope Rc RefCell
    //fixed so far
    pub fn visit_new(&mut self, node : &ASTNode) -> ASTNode {
        let original_scope = self.current_scope.clone();
        match &node.kind {
            AST::NEW{name, args: new_args} => {
                let b_option = original_scope.borrow().resolve_blueprint(name.clone());
                if let Some(blueprint) =  b_option{
                    let class_e = blueprint.einfo.clone();
                    match &blueprint.kind {
                        AST::CLASS { name: _, properties, methods} => {
                            let obj_scope = Rc::new(RefCell::new(Scope::new(None)));
                            let root_scope = Scope::get_root_scope(self.current_scope.clone());
                            //adding properties
                            for (_name, prop) in properties {
                                let _ = obj_scope.borrow_mut().add_var(prop);
                            }
                            for (_name , bp) in &root_scope.borrow().classes {
                                let _ = obj_scope.borrow_mut().add_blueprint(bp);
                            }
                           // println!("methods len while visiting new: {}", methods.len());
                            if let Some(constructor) = methods.get("create") {
                                self.current_scope = obj_scope.clone();
                                self.visit(constructor);
                                self.current_scope = original_scope.clone();
                                let _constructor_call_node = ASTNode::new(AST::FUNC_CALL { name: String::from("create"), args: new_args.clone().iter().map(|x| self.visit(x)).collect() }, node.einfo.clone());
                                self.current_scope = obj_scope.clone();
                                //scoping issue fixed
                                //here we use the alternate functionality of visit_function_call by giving some instead of none
                                // -- ISSUE --
                                //args to constructor are not being evaluated in the correct scope,
                                //they should be evaluated in global scope but instead are being evaluated in obj scope
                                // -- SOLUTION --
                                // above you can see that I visited the args before we visit the function call itself
                                self.visit_function_call(&_constructor_call_node, Some(self.current_scope.clone()));
                                //println!("{:#?}", obj_scope);
                                //println!("{:#?}", self.current_scope);
                                //adding methods
                                let mut new_methods = methods.clone();
                                new_methods.remove("create");
                                for (_name, method) in new_methods {
                                    let _ = self.current_scope.borrow_mut().add_func(&method);
                                }
                                self.current_scope = original_scope;
                               // println!("{:#?}", obj_scope);
                                ASTNode::new(AST::OBJECT { class_name : name.clone(), scope: obj_scope}, node.einfo.clone())
                            } else {
                                //expected constructor method to exist error
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, "Expected constructor method 'create' for blueprint definition", class_e.clone()));
                                self.errorstack.borrow().terminate_gs();
                                ASTNode::new_noop()
                            }
                        },
                        _ => return ASTNode::new_noop()
                    }
                    
                } else {
                    //undefined blueprint error
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, format!("Blueprint '{}' does not exist in the current scope", name.clone()).as_str(), node.einfo.clone()));
                    self.errorstack.borrow().terminate_gs();
                    ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
    }

    //stdfuncs are now in their own file
    
}