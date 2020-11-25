use crate::core::*;

pub struct IfStatement;

impl Object for IfStatement {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(IfStatement)
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        if params.contains("1") {
            let cond = params.get(&scope, "1");

            if params.contains("2") {
                let then_call = params.get(&scope, "2");
                if params.contains("3") {
                    if &params.get(&scope, "3").to_string() == "else" {
                        if params.contains("4") {
                            let else_call = params.get(&scope, "4");
                            match cond.call(Vars::new(), scope).to_string().as_str() {
                                "true" => then_call.call(Vars::new(), scope),
                                "false" => else_call.call(Vars::new(), scope),
                                other => panic!("Expected true or false, found {}", other),
                            } 
                        } else {
                            panic!("Expected else call")
                        }
                    } else {
                        panic!("Expected else")
                    }
                } else {
                    panic!("Expected else")
                }
            } else {
                panic!("Expected then call")
            }
        } else {
            panic!("Expected condition")
        }
    } 

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}