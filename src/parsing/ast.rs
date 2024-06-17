use crate::{errors::error::*, token::TokenType};

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
    COMPOUND {
        compound_value : Vec<ASTNode>
    },
    NOOP,
    EOF
    
}
pub struct ASTNode {
    pub kind : AST,
    pub einfo : ErrorInfo,
}
impl ASTNode {
    pub fn new(kind : AST, einfo : ErrorInfo) -> ASTNode {
        ASTNode { kind, einfo }
    }
    pub fn print(&self) {
        match &self.kind {
            AST::STRING{str_value} => { println!("string: {}", str_value)},
            AST::INT{int_value}=>{println!("int: {}",int_value)},
            AST::FLOAT{float_value}=>{println!("float: {}", float_value)},
            AST::BINOP{left, op, right}=>{println!("binop: "); left.to_owned().print(); println!("operator: {:?}", op); right.to_owned().print();}
            AST::COMPOUND{compound_value}=>{println!("compound, values are:"); compound_value.iter().for_each(|x| x.print());}
            AST::EOF => {}
            _=>println!("unknown")
        }
    }
}