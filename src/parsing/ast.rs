use crate::{errors::error::*, token::TokenType};

#[allow(non_camel_case_types)]
#[derive(Clone)]
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
    BINOP {
        left : Box<ASTNode>,
        op : TokenType, 
        right : Box<ASTNode>
    },
    VAR_DEF {
        name : String,
        value : Box<ASTNode>
    },
    FUNC_CALL {
        name : String,
        args : Vec<ASTNode>
    },
    COMPOUND {
        compound_value : Vec<ASTNode>
    },
    NOOP,
    EOF
    
}
#[derive(Clone)]
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
            AST::BINOP{left, op, right}=>{println!("binop: "); left.to_owned().print(); println!("operator: {:?}", op); right.to_owned().print();}
            AST::COMPOUND{compound_value}=>{println!("compound, values are:"); compound_value.iter().for_each(|x| x.print());}
            AST::FUNC_CALL { name, args }=>{println!("func call {}, args are:", name); args.iter().for_each(|x| x.print()); }
            AST::EOF => {println!("end of file"); }
            AST::NOOP => {println!("no operation");}
            _ => {println!("ast print not yet implemented for this ast node type");}
        }
    }
}