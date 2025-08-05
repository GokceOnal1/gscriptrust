use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{errors::error::*, token::TokenType, scope::Scope};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum AST {
    STRING {
        str_value : String,
    },
    INT {
        int_value : i32,
    },
    FLOAT {
        float_value : f32,
    },
    BOOL {
        bool_value : bool
    },
    LIST {
        contents : Vec<Rc<RefCell<ASTNode>>>
    },
    INDEX {
        target : Box<ASTNode>,
        indices : Vec<ASTNode>
    },
    LIST_REASSIGN {
        target : Box<ASTNode>,
        value : Box<ASTNode>
    },
    OBJECT_REASSIGN {
        object_index : Box<ASTNode>,
        value : Box<ASTNode>
    },
    BINOP {
        left : Box<ASTNode>,
        op : TokenType, 
        right : Box<ASTNode>
    },
    UNOP {
        op : TokenType,
        body : Box<ASTNode>
    },
    VAR_DEF {
        name : String,
        value : Box<ASTNode>
    },
    VAR {
        name : String
    },
    VAR_REASSIGN {
        name : String,
        value : Box<ASTNode>
    },
    FUNC_DEF {
        body : Box<ASTNode>,
        name : String,
        args : Vec<ASTNode>
    },
    FUNC_CALL {
        name : String,
        args : Vec<ASTNode>
    },
    RETURN {
        value : Box<ASTNode>
    },
    CLASS {
        name : String,
        properties : HashMap<String, ASTNode>,
        methods : HashMap<String, ASTNode>
    },
    NEW {
        name : String,
        args : Vec<ASTNode>
    },
    OBJECT {
        class_name : String,
        scope : Rc<RefCell<Scope>>
    },
    OBJECT_INDEX {
        object : Box<ASTNode>,
        property : Box<ASTNode>
    },
    IF {
        conditions : Vec<ASTNode>,
        bodies : Vec<ASTNode>,
        else_body : Option<Box<ASTNode>>
    },
    WHILE {
        condition : Box<ASTNode>,
        body : Box<ASTNode>
    },
    TYPE {
        type_value : String
    },
    BREAK,
    COMPOUND {
        compound_value : Vec<ASTNode>
    },
    IMPORT {
        filename : String,
        object_name : String
    },
    NOOP,
    EOF
    
}
#[derive(Clone, Debug)]
pub struct ASTNode {
    pub kind : AST,
    pub einfo : ErrorInfo,
}
impl ASTNode {
    pub fn new(kind : AST, einfo : ErrorInfo) -> ASTNode {
        ASTNode { kind, einfo }
    }
    pub fn new_noop() -> ASTNode {
        ASTNode {
            kind : AST::NOOP,
            einfo : ErrorInfo::new(String::new(), String::new(), 0, 0, 0)
        }
    }
    pub fn print(&self) {
        match &self.kind {
            AST::STRING{str_value} => { println!("string: {}", str_value)},
            AST::INT{int_value}=>{println!("int: {}",int_value)},
            AST::FLOAT{float_value}=>{println!("float: {}", float_value)},
            AST::BOOL{bool_value}=>{println!("bool: {}", bool_value)},
            AST::BINOP{left, op, right}=>{println!("binop: "); left.to_owned().print(); println!("operator: {:?}", op); right.to_owned().print();}
            AST::COMPOUND{compound_value}=>{println!("compound, values are:"); compound_value.iter().for_each(|x| x.print());}
            AST::FUNC_CALL { name, args }=>{println!("func call {}, args are:", name); args.iter().for_each(|x| x.print()); }
            AST::EOF => {println!("end of file"); }
            AST::NOOP => {println!("no operation");}
            _ => {println!("ast print not yet implemented for this ast node type\n({:#?})", self.kind);}
        }
    }
}