use crate::core::*;

pub struct IfStatement;

impl Object for IfStatement {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(IfStatement)
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let cond = params
            .get("1")
            .unwrap_or_else(|| panic!("Expected condition"));
        let then_call = params
            .get("2")
            .unwrap_or_else(|| panic!("Expected then call"));
        let _ = params
            .get("3")
            .map(|x| match x.to_string().as_str() {
                "else" => (),
                other => panic!("Expected else, found {}", other),
            })
            .unwrap_or_else(|| panic!("Expected else"));
        let else_call = params
            .get("4")
            .unwrap_or_else(|| panic!("Expected else call"));

        match cond.call(Vars::new(), scope).to_string().as_str() {
            "true" => then_call,
            "false" => else_call,
            other => panic!("Expected true or false, found {}", other),
        }
        .call(Vars::new(), scope)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}
