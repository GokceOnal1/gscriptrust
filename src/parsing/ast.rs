pub enum AST {
    STRING(String),
    
}
pub struct ASTNode {
    kind : AST,
    line : usize,
    col  : usize,
}