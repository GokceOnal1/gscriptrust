use crate::error::*;
use crate::visitor::*;
use crate::parsing::ast::*;
use rand::Rng;

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
///GScript: converts AST_STRING to AST_INT
pub fn std_func_to_int(v : &mut Visitor, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'to_int' requires 1 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
        return ASTNode::new_noop();
    } else {
        let arg = v.visit(&args[0]);
        let mut numval= 0;
        match &arg.kind {
            AST::STRING{str_value} => {
                numval = str_value.clone().parse::<i32>().unwrap_or_else(|_| {
                    v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TypeError, format!("Could not cast '{}' to type 'Integer'", str_value).as_str(), node.einfo.clone()));
                    0
                });
                
            }
            _ => {
                v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TypeError, format!("Invalid attempted type cast to type 'Integer'").as_str(), node.einfo.clone()));
            }
        }
        ASTNode::new(AST::INT{int_value : numval}, node.einfo.clone())
    }
}
///GScript: converts AST_STRING to AST_FLOAT
pub fn std_func_to_float(v : &mut Visitor, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
    if args.len() != 1 {
        v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'to_float' requires 1 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
        return ASTNode::new_noop();
    } else {
        let arg = v.visit(&args[0]);
        let mut numval= 0.0;
        match &arg.kind {
            AST::STRING{str_value} => {
                numval = str_value.clone().parse::<f32>().unwrap_or_else(|_| {
                    v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TypeError, format!("Could not cast '{}' to type 'Float'", str_value).as_str(), node.einfo.clone()));
                    0.0
                });
                
            }
            _ => {
                v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TypeError, format!("Invalid attempted type cast to type 'Float'").as_str(), node.einfo.clone()));
            }
        }
        ASTNode::new(AST::FLOAT{float_value : numval}, node.einfo.clone())
    }
}
///GScript: Generates random integer between provided range, inclusive
pub fn std_func_random_int(v : &mut Visitor, node : &ASTNode, args : &Vec<ASTNode>) -> ASTNode {
    if args.len() != 2 {
        v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::FunctionError, format!("Function 'random_int' requires 2 argument(s), not {}", args.len()).as_str(), node.einfo.clone()));
        return ASTNode::new_noop();
    } else {
        let arg1 = v.visit(&args[0]);
        let arg2 = v.visit(&args[1]);

        match (arg1.kind, arg2.kind) {
            (AST::INT{ int_value: n1}, AST::INT{ int_value : n2}) => {
                let mut rng = rand::thread_rng();
                let numval = rng.gen_range(n1..=n2);
                ASTNode::new(AST::INT{int_value : numval }, node.einfo.clone())
            },
            _ => {
                v.errorstack.borrow_mut().errors.push(GError::new_from_tok(ETypes::TypeError, format!("Invalid type(s) to function 'random_integer': Expected (Integer, Integer)").as_str(), node.einfo.clone()));
                ASTNode::new_noop()
            }
        }
    }
}