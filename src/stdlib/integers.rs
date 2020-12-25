use crate::core::*;

use std::any::*;

pub struct Int;

impl Int {
    pub fn int_obj(var: Box<dyn Object>) -> error::TrashResult {
        if var.id() == 0i64.type_id() {
            Ok(var)
        } else if var.id() == 0.0f64.type_id() {
            Ok(Box::new(
                unsafe { *(var.as_ref() as *const dyn Object as *const f64) } as i64,
            ))
        } else {
            let t = var.to_string();
            Ok(Box::new(t.parse::<i64>().map_err(|_| {
                TrashError::UnexpectedType("int".to_owned(), t)
            })?))
        }
    }

    pub fn int(var: Box<dyn Object>) -> Result<i64, String> {
        if var.id() == 0i64.type_id() {
            Ok(unsafe { *(var.as_ref() as *const dyn Object as *const i64) })
        } else if var.id() == 0.0f64.type_id() {
            Ok(unsafe { *(var.as_ref() as *const dyn Object as *const f64) } as i64)
        } else {
            let t = var.to_string();
            t.parse::<i64>().ok().ok_or(t)
        }
    }
}

impl Object for Int {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(Int))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        Self::int_obj(params.get("1").ok_or(TrashError::NotEnoughArgs(0, 1))?)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}

impl Object for i64 {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(*self))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op if op == "add" || op == "sub" || op == "mul" || op == "div" || op == "rem" => {
                    let n = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    let num = Int::int(n)
                        .map_err(|e| TrashError::UnexpectedType("int".to_string(), e))?;

                    Ok(Box::new(match op {
                        "add" => *self + num,
                        "sub" => *self - num,
                        "mul" => *self * num,
                        "div" => *self / num,
                        "rem" => *self % num,
                        _ => unreachable!(),
                    }))
                }

                op if op == "eq" || op == "gt" || op == "lt" => {
                    let n = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    let num = Int::int(n)
                        .map_err(|e| TrashError::UnexpectedType("int".to_string(), e))?;
                    Ok(Box::new(
                        match op {
                            "eq" => *self == num,
                            "gt" => *self > num,
                            "lt" => *self < num,
                            _ => unreachable!(),
                        }
                        .to_string(),
                    ))
                }

                "chr" => {
                    let chr = std::char::from_u32(*self as u32)
                        .ok_or_else(|| TrashError::Custom("Invalid UTF-8 char".to_owned()))?;
                    Ok(Box::new(chr.to_string()))
                }

                other => panic!("Unknown method {}", other),
            },

            None => Ok(self),
        }
    }

    fn to_string(self: Box<Self>) -> String {
        self.as_ref().to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}
