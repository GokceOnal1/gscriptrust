use crate::errors::error::*;
use crate::parsing::ast::*;
use crate::parsing::token::*;

pub struct Visitor {
    pub errorstack : ErrorStack
}
impl Visitor {
    pub fn new(errorstack : ErrorStack) -> Visitor {
        Visitor { errorstack }
    }
    pub fn visit(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::STRING{..} | AST::INT{..} | AST::FLOAT{..} => { return node.clone(); } ,
            AST::BINOP{..} => { return self.visit_binop(node); }
            AST::FUNC_CALL { .. } => { return self.visit_function_call(node); },
            AST::COMPOUND{ compound_value } => { compound_value.iter().for_each(|ast|  { self.visit(ast); }); return ASTNode::new_noop(); }
            _ => return ASTNode::new_noop()
        }
    }
    pub fn visit_binop(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::BINOP{ left, op, right} => {
                let nleft = self.visit(left);
                let nright = self.visit(right);
                match (nleft.kind, op, nright.kind) {
                    (AST::INT{ int_value: x } , TokenType::PLS, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::INT{ int_value : x + y}, node.einfo.clone())
                    },
                    (AST::INT{ int_value: x } , TokenType::PLS, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x as f32 + y}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::PLS, AST::INT{ int_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x + y as f32}, node.einfo.clone())
                    },
                    (AST::FLOAT{ float_value: x } , TokenType::PLS, AST::FLOAT{ float_value : y }) => {
                        return ASTNode::new(AST::FLOAT{ float_value : x + y}, node.einfo.clone())
                    }
                    _ => return ASTNode::new_noop()
                }
            },
            _ => return ASTNode::new_noop()
        }

    }
    pub fn visit_function_call(&mut self, node : &ASTNode) -> ASTNode {
        match &node.kind {
            AST::FUNC_CALL { name, args } => {
                match name.as_str() {
                    "write" => return self.std_func_write(args),
                    _ => return ASTNode::new_noop()
                }
            },
            _ => ASTNode::new_noop()
        }
         
    }
    //MOVE THIS TO DIFFERENT FILE LATER
    pub fn std_func_write(&mut self, args : &Vec<ASTNode> ) -> ASTNode {
        for arg in args {
            self.visit(arg).print()
        }
        ASTNode::new_noop()
    }
}