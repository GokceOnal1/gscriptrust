use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::ast::*;


#[derive(Clone, Default, Debug)]
pub struct Scope {
    pub parent : Option<Rc<RefCell<Scope>>>,
    pub variables : HashMap<String, Rc<RefCell<ASTNode>>>,
    pub functions : HashMap<String, ASTNode>,
    pub classes : HashMap<String, ASTNode>
}
impl Scope {
    pub fn new(parent : Option<Rc<RefCell<Scope>>>) -> Scope {
        Scope {
            parent,
            variables : HashMap::new(),
            functions : HashMap::new(),
            classes : HashMap::new(),
        }
    }
    pub fn add_blueprint(&mut self, node : &ASTNode) -> Result<(), String> {
        match &node.kind {
            AST::CLASS {name, ..} => {
                if self.resolve_blueprint(name.to_string()).is_some() {
                    return Err(format!("Blueprint '{}' already exists in the current scope", name));
                } else {
                    self.classes.insert(name.clone(), node.clone());
                    return Ok(());
                }
            },
            _ => Err("Not a valid blueprint definition".to_string())
        }
    }
    pub fn resolve_blueprint(& self, name : String) -> Option<ASTNode> {
        self.classes.get(&name).cloned().or_else(|| {
            if let Some(par) = self.parent.clone() {
                par.borrow().resolve_blueprint(name)
            } else {
                None
            }
        })
    } 
    pub fn add_var(&mut self, node : &ASTNode) -> Result<(), String> {
        match &node.kind {
            AST::VAR_DEF { name, .. } => {
                if self.resolve_var(name.to_string()).is_some() {
                    return Err(format!("Variable '{}' already exists in the current scope", name));
                } else {
                    self.variables.insert(name.clone(), Rc::new(RefCell::new(node.clone())));
                    return Ok(());
                }
            },
            _ => Err("Not a valid variable definition".to_string())
        }
    }
    pub fn set_var(&mut self, name : String, node : &ASTNode) -> Result<(), String> {
        if let Some(existing) = self.variables.get_mut(&name) {
            *existing = Rc::new(RefCell::new(node.clone()));
            Ok(())
        } else if let Some( par) = self.parent.clone() {
            par.borrow_mut().set_var(name, node)
        } else {
            Err(format!("Variable '{}' does not exist in the current scope", name))
        }
    }
    pub fn add_func(&mut self, node : &ASTNode) -> Result<(), String> {
        match &node.kind {
            AST::FUNC_DEF { name, .. } => {
                if self.resolve_func(name.to_string()).is_some() {
                    return Err(format!("Function '{}' already exists in the current scope", name));
                } else {
                    self.functions.insert(name.clone(), node.clone());
                    return Ok(());
                }
            },
            _ => Err("Not a valid function definition".to_string())
        }
    }
    pub fn resolve_var(& self, name : String) -> Option<Rc<RefCell<ASTNode>>> {
        self.variables.get(&name).cloned().or_else(|| {
            if let Some(par) = self.parent.clone() {
                par.borrow().resolve_var(name)
            } else {
                None
            }
        })
    }
    pub fn resolve_var_cloned(&self, name : String) -> Option<ASTNode> {
        Some(self.resolve_var(name)?.borrow().clone())
    }
    pub fn resolve_func(&self, name : String) -> Option<ASTNode> {
        self.functions.get(&name).cloned().or_else(|| {
            if let Some(par) = self.parent.clone() {
                par.borrow().resolve_func(name)
            } else {
                None
            }
        })
    }
    //-- REVISED BY CHATGPT ---
    // again, the reason for this is because I was dropping an owned value
    // while still having a borrow for it 
    pub fn get_root_scope(starting_scope : Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
        let mut cs = Rc::clone(&starting_scope);
        loop {
            let par = {
                let borrowed_cs = cs.borrow();
                if let Some(ref p) = borrowed_cs.parent {
                    Some(Rc::clone(p))
                } else {
                    None
                }
            };
            match par {
                Some(p) => cs = p,
                None => break
            }
        }
        cs
    }
    pub fn deep_clone(starting_scope : Option<Rc<RefCell<Scope>>>) -> Option<Rc<RefCell<Scope>>> {
        match starting_scope {
            Some(s) => {
                let new_s = Rc::new(RefCell::new(Scope::new(None)));
                for (name, vdef) in &s.borrow().variables {
                    new_s.borrow_mut().variables.insert(name.clone(), Rc::new(RefCell::new(vdef.borrow().clone())));
                }
                if s.borrow().parent.is_some() {
                    new_s.borrow_mut().parent = Scope::deep_clone(s.borrow().parent.clone());
                }
                Some(new_s)
            }
            None => None
        }
    }
}