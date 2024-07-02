use crate::errors::error::*;
use crate::parsing::ast::*;
use crate::parsing::token::*;
use crate::scope::*;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Visitor {
    current_scope : Scope,
    pub errorstack : Rc<RefCell<ErrorStack>>,
    keywords : Vec<String>
}
impl Visitor {
    pub fn new(errorstack : Rc<RefCell<ErrorStack>>) -> Visitor {
        Visitor { 
            errorstack, 
            current_scope : Scope::new(None), 
            keywords : vec!["assign", "funct", "if", "param", "return", "blueprint", "new", "while", "break", "true", "false"].iter().map(|x| x.to_string()).collect()
        }
    }
    pub fn visit(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::STRING{..} | AST::INT{..} | AST::FLOAT{..} | AST::BOOL{..} | AST::BREAK => { return node.clone(); } ,
            AST::BINOP{..} => { return self.visit_binop(node); }
            AST::UNOP{..} => { return self.visit_unop(node); }
            AST::LIST{..} => {return self.visit_list(node); }
            AST::INDEX{..} => { return self.visit_index(node); }
            AST::LIST_REASSIGN { .. } => { return self.visit_list_reassign(node); }
            AST::VAR_DEF{..} => { return self.visit_variable_definition(node); }
            AST::VAR_REASSIGN{..} => { return self.visit_variable_reassign(node); }
            AST::VAR{..} => { return self.visit_variable(node); }
            AST::FUNC_CALL { .. } => { return self.visit_function_call(node); },
            AST::FUNC_DEF {..} => { return self.visit_function_definition(node); },
            AST::RETURN{..} => { return self.visit_return(node); },
            AST::IF{..} => { return self.visit_if(node); }
            AST::WHILE{..} => { return self.visit_while(node); }
            AST::CLASS{..} => { return self.visit_blueprint(node); }
            AST::NEW{..} => { return self.visit_new(node); }
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
                                println!("aris");
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
    pub fn visit_function_call(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::FUNC_CALL { name, args } => {
                match name.as_str() {
                    "write" => return self.std_func_write(args),
                    "read" => return self.std_func_read(node, args),
                    "ast_debug" => return self.std_func_debug(args), 
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
            AST::FUNC_DEF { name, .. } => {
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionDefinitionError, "Illegal use of keyword for function definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
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
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::VariableDefinitionError, "Illegal use of keyword for variable definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
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
    pub fn visit_variable_reassign(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::VAR_REASSIGN { name, value } => {
                let val = self.visit(value);
                let var_def = ASTNode::new(AST::VAR_DEF { name: name.clone(), value: Box::new(val) }, node.einfo.clone());
                let res = self.current_scope.set_var(name.clone(), &var_def);
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
                // scope is doing weird shit
                // make parent Rc RefCell instead of owned
                // -- END ISSUE --
                // -- SOLUTION --
                // Take the memory of the parent of the temporary scope to reuse it on future iterations
                //kind of works for now but it might be a better idea to make parent an Rc Refcell reference instead of owned 
                    let mut origin = self.current_scope.clone();
                    loop {
                        let temp_par_scope = origin.clone();
                        self.current_scope = Scope::new(Some(Box::new(temp_par_scope)));
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

                        let origin_1 = std::mem::take(&mut self.current_scope.parent);
                        origin = *origin_1.unwrap();
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
                    s.push_str(&self.node_to_string(c));
                }
                s.push(']');
                s
            }
            AST::NOOP => "no operation".to_string(),
            _ => "undefined".to_string()
        }
    }
    pub fn visit_list(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::LIST{contents} => {
                ASTNode::new(AST::LIST{contents : contents.iter().map(|x| self.visit(x)).collect()}, node.einfo.clone())
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
                            combined_target = contents[ind_i as usize].clone()
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
    //(old, for testing purposes)
    //need to analyze why this didn't work...

    //--ANALYSIS OF SEVERE ISSUE -- 
    // it didn't work because I was mutably borrowing list_ref_i when trying to extract
    //the list from the var_def match but then also later on doing the actual algorithm which
    //also borrowed mutably.
    //--SOLUTION--
    //just self.visit target like normal and overwrite the old list value with the new one
    //--REVISED FUNCTION--
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
                        let mut orig_list = self.visit(list);
                        let mut list_ref_i = &mut orig_list;
                        
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
                            if i == indices.len() - 1 {
                                //now we know we have reached the point to actually set the variable
                                match &mut list_ref_i.kind {
                                    AST::LIST{contents} => {
                                        //println!("here");
                                        //make sure index is valid
                                        if actual_i < 0 || actual_i as usize >= contents.len() {
                                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, format!("Index {} is out of bounds for list of length {}", actual_i, contents.len()).as_str(), i_val.einfo.clone()));
                                            self.errorstack.borrow().terminate_gs();
                                        }
                                        contents[actual_i as usize] = value;
                                        let _ = self.current_scope.set_var(list_name.clone(), &ASTNode::new(AST::VAR_DEF{name : list_name.clone(), value : Box::new(orig_list.clone()) }, orig_list.einfo.clone()));
                                        return ASTNode::new_noop();
                                    },
                                    _ => { 
                                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Indexed object is not a list", list_ref_i.einfo.clone()));
                                        return ASTNode::new_noop() }  //handle error here
                                }
                            } else {
                                match &mut list_ref_i.kind {
                                    AST::LIST {contents} => {
                                        //make sure index is valid  
                                        if actual_i < 0 || actual_i as usize >= contents.len() {
                                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, format!("Index {} is out of bounds for list of length {}", actual_i, contents.len()).as_str(), i_val.einfo.clone()));
                                            self.errorstack.borrow().terminate_gs();
                                        }
                                        list_ref_i = &mut contents[actual_i as usize];
                                    }
                                    _ => { 
                                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::ListError, "Indexed object is not a list", list_ref_i.einfo.clone()));
                                        return ASTNode::new_noop() }//handle error here
                                }
                            }
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
    // SEVERE
    // bro... i hated this... up until 1 am
    // ended up copying from Chat GPT
    // f this function
    // this will need to studied
    /// use visit_list_reassign instead
    ///  --CHAT GPT PROVIDED THIS, I USED IT AS REFERENCE--
    pub fn chat_gpt_visit_list_reassign(&mut self, target: &ASTNode, value : &ASTNode) -> ASTNode {
        
        // ----------------
        // NONE OF THIS SHIT WORKS
        // ----------------
        // match &node.kind {
        //     AST::LIST_REASSIGN { target, value } => {
        //         let value = self.visit(value);
        //         let mut sc = self.current_scope.clone();
        //         match &target.kind {
        //             AST::INDEX { target: list, indices} => {
        //                 let mut list_ref_i;
        //                 match &list.kind {
        //                     AST::VAR{name} => {
        //                         // -- TODO --
        //                         //change this unwrap() into something safer later
                                
        //                         list_ref_i = sc.resolve_var_mut(name.to_owned()).unwrap();
        //                     },
        //                     _ => return ASTNode::new_noop()
        //                 }
        //                 let mut current_node = match list_ref_i.borrow_mut().kind {
        //                     AST::VAR_DEF {name:_, value} => {
        //                         Rc::new(RefCell::new(value))
        //                     },
        //                     _ => { std::process::exit(1); }
        //                 };
                       
        //                 //at this point list_ref is a mutable reference to the list
        //                 //referred to by the identifier value of "list"
        //                 // to test uncomment the following: *list_ref = ASTNode::new_noop();
        //                 //let mut list_ref_i = std::borrow::BorrowMut::borrow_mut(list_ref);
        //                 for(i, i_node) in indices.iter().enumerate() {
        //                     let i_val = self.visit(i_node);
        //                     let actual_i = match i_val.kind {
        //                         AST::INT{int_value} => int_value,
        //                         _ => {println!("herde"); 0} //handle error here
        //                     };
        //                     println!("{}", actual_i);
        //                     if i == indices.len() - 1 {
        //                         //now we know we have reached the point to actually set the variable
        //                         match  current_node.borrow_mut().kind {
        //                             AST::LIST{contents} => {
        //                                 println!("here");
        //                                 //make sure index is valid
        //                                 contents[actual_i as usize] = value;
        //                                 self.current_scope = sc;
        //                                 return ASTNode::new_noop();
        //                             },
        //                             _ => return ASTNode::new_noop() //handle error here
        //                         }
        //                     } else {
        //                         match current_node.borrow_mut().kind {
        //                             AST::LIST {contents} => {
        //                                 //make sure index is valid  
        //                                 *current_node. = contents[actual_i as usize];
        //                             }
        //                             _ => return ASTNode::new_noop() //handle error here
        //                         }
        //                     }
        //                 }
        //                // list_ref.print();
        //                 return ASTNode::new_noop();
                       
        //             },
        //             _ => return ASTNode::new_noop()
        //         }
        //     }
        //     _ => ASTNode::new_noop()
        // }
        // ----- HOPEFULLY THIS SHIT WILL WORK
        // match &node.kind {
        //     AST::LIST_REASSIGN { target, value } => {
        //         match &target.kind {
        //             AST::INDEX{target:list, indices} => {
        //                 let v_list = self.visit(list);

        //             }
        //             _ => ASTNode::new_noop()
        //         }
        //     }
        //     _ => ASTNode::new_noop()
        // }
        
        let evaluated_value = self.visit(value);

        if let AST::INDEX { target: list, indices } = &target.kind {
            let list_name = match &list.kind {
                AST::VAR{name} => name.clone(),
                _ => String::new()
            };
            let mut current_list = self.visit(list);
            let mut current_node = &mut current_list;

            for (i, index_node) in indices.iter().enumerate() {
                let index_value = self.visit(index_node);
                let index = match index_value.kind {
                    AST::INT { int_value } => int_value,
                    _ => {
                        self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                            ETypes::TypeError,
                            "Index must be an integer",
                            target.einfo.clone(),
                        ));
                        self.errorstack.borrow().terminate_gs();
                        return ASTNode::new_noop();
                    }
                };

                if i == indices.len() - 1 {
                    match &mut current_node.kind {
                        AST::LIST { contents: elements } => {
                            if index < 0 || (index as usize) >= elements.len() {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                                    ETypes::ListError,
                                    "Index out of bounds",
                                    target.einfo.clone(),
                                ));
                                self.errorstack.borrow().terminate_gs();
                                return ASTNode::new_noop();
                            }
                            elements[index as usize] = evaluated_value.clone();
                            let _ =self.current_scope.set_var(list_name.clone(), &ASTNode::new(AST::VAR_DEF{name : list_name.clone(), value: Box::new(current_list.clone())}, current_list.einfo.clone()));
                            return evaluated_value;
                        }
                        _ => {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                                ETypes::ListError,
                                "Target is not a list",
                                target.einfo.clone(),
                            ));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        }
                    }
                } else {
                    current_node = match &mut current_node.kind {
                        AST::LIST { contents: elements } => {
                            if index < 0 || (index as usize) >= elements.len() {
                                self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                                    ETypes::ListError,
                                    "Index out of bounds",
                                    target.einfo.clone(),
                                ));
                                self.errorstack.borrow().terminate_gs();
                                return ASTNode::new_noop();
                            }
                            &mut elements[index as usize]
                        }
                        _ => {
                            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                                ETypes::TypeError,
                                "Target is not a list",
                                target.einfo.clone(),
                            ));
                            self.errorstack.borrow().terminate_gs();
                            return ASTNode::new_noop();
                        }
                    };
                }
            }
        } else if let AST::VAR { name } = &target.kind {
            let _ = self.current_scope.set_var(name.clone(), &evaluated_value.clone());
            return evaluated_value;
        } else {
            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(
                ETypes::TypeError,
                "Invalid assignment target",
                target.einfo.clone(),
            ));
            self.errorstack.borrow().terminate_gs();
            return ASTNode::new_noop();
        }

        ASTNode::new_noop() // This should never be reached
    }
    pub fn visit_blueprint(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::CLASS{name, ..} => {
                if self.keywords.contains(name) {
                    self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::BlueprintError, "Illegal use of keyword for blueprint definition", node.einfo.clone()));
                    return ASTNode::new_noop();
                }
                if let Err(s) = self.current_scope.add_blueprint(node) {
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
    //not done yet, but to fix this I will need to make parent scope Rc RefCell
    pub fn visit_new(&mut self, node : &ASTNode) -> ASTNode {
        let original_scope = self.current_scope.clone();
        match &node.kind {
            AST::NEW{name, args: new_args} => {
                let blueprint_option = original_scope.resolve_blueprint(name.clone());
                if let Some(blueprint) = blueprint_option {
                    match &blueprint.kind {
                        AST::CLASS { name: _, properties, methods} => {
                            let mut obj_scope = Scope::new(None);
                            for (_name, prop) in properties {
                                let _ = obj_scope.add_var(prop);
                            }
                            if let Some(constructor) = methods.get("create") {
                                self.current_scope = obj_scope;
                                self.visit(constructor);
                                let _constructor_call_node = ASTNode::new(AST::FUNC_CALL { name: String::from("create"), args: new_args.clone() }, node.einfo.clone());
                                //fix the scoping issue to continue writing code here
                                ASTNode::new_noop() // this is temporary
                            } else {
                                //expected constructor method to exist error
                                ASTNode::new_noop()
                            }
                        },
                        _ => return ASTNode::new_noop()
                    }
                    
                } else {
                    //undefined blueprint error
                    ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
    }

    //MOVE THESE TO A DIFFERENT FILE LATER
    //??
    pub fn std_func_debug(&mut self, args : &Vec<ASTNode>) -> ASTNode {
        for arg in args {
            println!("{:#?}", arg);
        }
        ASTNode::new_noop()
    }
    pub fn std_func_write(&mut self, args : &Vec<ASTNode> ) -> ASTNode {
        for arg in args {
            let ast = self.visit(arg);
            print!("{}", self.node_to_string(&ast));
        }
        println!();
        ASTNode::new_noop()
    }
    pub fn std_func_read(&mut self, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
        if args.len() != 0 {
            self.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'read' requires 0 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
            return ASTNode::new_noop();
        } else {
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim_end().to_string();
            return ASTNode::new(AST::STRING{str_value : input}, node.einfo.clone());
        }
    }
}