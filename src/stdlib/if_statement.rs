use crate::core::*;

pub struct IfStatement;

impl Object for IfStatement {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(IfStatement))
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        let cond = params
            .get("1")
            .ok_or_else(|| TrashError::Custom("Expected condition".to_owned()))?;
        let then_call = params
            .get("2")
            .ok_or_else(|| TrashError::Custom("Expected then call".to_owned()))?;
        match params
            .get("3")
            .ok_or_else(|| TrashError::Custom("Expected else".to_owned()))?
            .to_string()
            .as_str()
        {
            "else" => (),
            other => {
                return Err(TrashError::Custom(format!("Expected 'else', found {}", other)).into())
            }
        };
        let else_call = params
            .get("4")
            .ok_or_else(|| TrashError::Custom("Expected else call".to_owned()))?;

        let (cond_res, cond) = {
            let mut res = cond.call(Vars::new(), scope)?.to_tuple();
            let cond = res.pop().ok_or_else(|| {
                TrashError::Custom("expected at least 1 element in tuple".to_string())
            })?;
            (res, cond)
        };
        match crate::stdlib::bool::Bool::bool_res(cond) {
            Ok(true) => then_call,
            Ok(false) => else_call,
            Err(other) => panic!("Expected true or false, found {}", other),
        }
        .call(Vars::from_vec(cond_res), scope)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}
