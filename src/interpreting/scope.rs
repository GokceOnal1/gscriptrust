use std::collections::HashMap;
use crate::ast::*;


#[derive(Clone, Default)]
pub struct Scope {
    pub parent : Option<Box<Scope>>,
    variables : HashMap<String, ASTNode>,
    functions : HashMap<String, ASTNode>,
}
impl Scope {
    pub fn new(parent : Option<Box<Scope>>) -> Scope {
        Scope {
            parent,
            variables : HashMap::new(),
            functions : HashMap::new(),
        }
    }
    pub fn add_var(&mut self, node : &ASTNode) -> Result<(), String> {
        match &node.kind {
            AST::VAR_DEF { name, .. } => {
                if self.resolve_var(name.to_string()).is_some() {
                    return Err(format!("Variable '{}' already exists in the current scope", name));
                } else {
                    self.variables.insert(name.clone(), node.clone());
                    return Ok(());
                }
            },
            _ => Err("Not a valid variable definition".to_string())
        }
    }
    pub fn set_var(&mut self, name : String, node : &ASTNode) -> Result<(), String> {
        if let Some(existing) = self.variables.get_mut(&name) {
            *existing = node.clone();
            Ok(())
        } else if let Some(ref mut par) = self.parent {
            par.set_var(name, node)
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
    pub fn resolve_var(&self, name : String) -> Option<&ASTNode> {
        self.variables.get(&name).or_else(|| {
            if let Some(ref par) = self.parent {
                par.resolve_var(name)
            } else {
                None
            }
        })
    }
    pub fn resolve_func(&self, name : String) -> Option<&ASTNode> {
        self.functions.get(&name).or_else(|| {
            if let Some(ref par) = self.parent {
                par.resolve_func(name)
            } else {
                None
            }
        })
    }
    pub fn get_root_scope(starting_scope : Scope) -> Scope {
        let mut cs = starting_scope;
        while let Some(par) = cs.parent {
            cs = (*par).clone();
        }
        cs
    }
}