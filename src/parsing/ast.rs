use crate::errors::error::*;

pub enum AST {
    STRING {
        str_value : String,
    },
    INT {
        int_value : i32,
    },
    COMPOUND {
        compound_value : Vec<ASTNode>
    },
    
}
pub struct ASTNode {
    pub kind : AST,
    pub einfo : ErrorInfo,
}
impl ASTNode {
    pub fn new(kind : AST, einfo : ErrorInfo) -> ASTNode {
        ASTNode { kind, einfo }
    }
}