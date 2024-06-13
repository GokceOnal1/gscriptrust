pub enum AST {
    STRING {
        str_value : String,
    },
    
}
pub struct ASTNode {
    kind : AST,
    line : usize,
    col  : usize,
}
impl ASTNode {
    pub fn new(kind : AST, line : usize, col : usize) -> ASTNode {
        ASTNode { kind, line, col }
    }
}