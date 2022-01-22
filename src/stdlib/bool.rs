use crate::core::*;

use std::any::*;

pub struct Bool;

impl Bool {
    pub fn bool_obj(var: Box<dyn Object>) -> error::TrashResult {
        if var.id() == false.type_id() {
            Ok(var)
        } else {
            let t = var.to_string();
            Ok(Box::new(t.parse::<bool>().map_err(|_| {
                TrashError::UnexpectedType("float".to_owned(), t)
            })?))
        }
    }

    pub fn bool_res(var: Box<dyn Object>) -> Result<bool, String> {
        if var.id() == true.type_id() {
            Ok(unsafe { *(var.as_ref() as *const dyn Object as *const bool) })
        } else {
            let t = var.to_string();
            t.parse::<bool>().ok().ok_or(t)
        }
    }
}

impl Object for Bool {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(Bool))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        Self::bool_obj(params.get("1").ok_or(TrashError::NotEnoughArgs(0, 1))?)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

impl Object for bool {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(*self))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op @ "and" | op @ "or" | op @ "xor" | op @ "eq" => {
                    let n = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    let b = Bool::bool_res(n)
                        .map_err(|e| TrashError::UnexpectedType("bool".to_string(), e))?;

                    Ok(Box::new(match op {
                        "and" => *self & b,
                        "or" => *self | b,
                        "xor" => *self ^ b,
                        "eq" => !(*self ^ b),
                        _ => unreachable!(),
                    }))
                }

                "not" => Ok(Box::new(!*self)),

                other => panic!("Unknown method {}", other),
            },

            None => Ok(self),
        }
    }

    fn to_string(self: Box<Self>) -> String {
        if *self {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}
