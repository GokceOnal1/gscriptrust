use crate::error::*;
use crate::visitor::*;
use crate::parsing::ast::*;

///GScript: Writes formatted AST to stdout
pub fn std_func_debug(_v : &mut Visitor, args : &Vec<ASTNode>) -> ASTNode {
    for arg in args {
        println!("{:#?}", arg);
    }
    ASTNode::new_noop()
}
///GScript: Writes to stdout
pub fn std_func_write(v : &mut Visitor, args : &Vec<ASTNode> ) -> ASTNode {
    for arg in args {
        let ast = v.visit(arg);
        print!("{}", v.node_to_string(&ast));
    }
    println!();
    ASTNode::new_noop()
}
///GScript: Reads line from stdin and returns string of it, no args
pub fn std_func_read(v : &mut Visitor, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
    if args.len() != 0 {
        v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'read' requires 0 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
        return ASTNode::new_noop();
    } else {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end().to_string();
        return ASTNode::new(AST::STRING{str_value : input}, node.einfo.clone());
    }
}
///GScript: returns AST_TYPE of arg[0]
pub fn std_func_type(v : &mut Visitor, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'type' requires 1 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
        return ASTNode::new_noop();
    } else {
        let arg = v.visit(&args[0]);
        let typeval : String;
        match &arg.kind {
            AST::STRING{..} => typeval = "String".to_string(),
            AST::INT{..} => typeval = "Int".to_string(),
            AST::FLOAT{..} => typeval = "Float".to_string(),
            AST::BOOL{..} => typeval = "Boolean".to_string(),
            AST::LIST{..} => typeval = "List_Obj".to_string(),
            AST::OBJECT{class_name, ..} => typeval = class_name.clone(),
            _ => typeval = "Null".to_string()
        }
        ASTNode::new(AST::TYPE{type_value : typeval}, args[0].einfo.clone())
    }
}