use crate::core::*;

use std::any::*;

pub struct Float;

impl Float {
    pub fn float_obj(var: Box<dyn Object>) -> error::TrashResult {
        if var.id() == 0.0f64.type_id() {
            Ok(var)
        } else if var.id() == 0i64.type_id() {
            Ok(Box::new(
                unsafe { *(var.as_ref() as *const dyn Object as *const i64) } as f64,
            ))
        } else {
            let t = var.to_string();
            Ok(Box::new(t.parse::<f64>().map_err(|_| {
                TrashError::UnexpectedType("float".to_owned(), t)
            })?))
        }
    }

    pub fn float(var: Box<dyn Object>) -> Result<f64, String> {
        if var.id() == 0.0f64.type_id() {
            Ok(unsafe { *(var.as_ref() as *const dyn Object as *const f64) })
        } else if var.id() == 0i64.type_id() {
            Ok(unsafe { *(var.as_ref() as *const dyn Object as *const i64) } as f64)
        } else {
            let t = var.to_string();
            t.parse::<f64>().ok().ok_or(t)
        }
    }
}

impl Object for Float {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(Float))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        Self::float_obj(params.get("1").ok_or(TrashError::NotEnoughArgs(0, 1))?)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}

impl Object for f64 {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(*self))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op @ "add" | op @ "sub" | op @ "mul" | op @ "div" => {
                    let n = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    let num = Float::float(n)
                        .map_err(|e| TrashError::UnexpectedType("float".to_string(), e))?;

                    Ok(Box::new(match op {
                        "add" => *self + num,
                        "sub" => *self - num,
                        "mul" => *self * num,
                        "div" => *self / num,
                        _ => unreachable!(),
                    }))
                }

                op @ "eq" | op @ "gt" | op @ "lt" => {
                    let n = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    let num = Float::float(n)
                        .map_err(|e| TrashError::UnexpectedType("float".to_string(), e))?;
                    Ok(Box::new(
                        match op {
                            "eq" => (*self - num).abs() / (*self + num) < std::f64::EPSILON,
                            "gt" => *self > num,
                            "lt" => *self < num,
                            _ => unreachable!(),
                        }
                        .to_string(),
                    ))
                }

                op @ "sqrt"
                | op @ "sin"
                | op @ "cos"
                | op @ "tan"
                | op @ "tg"
                | op @ "log"
                | op @ "ln" => Ok(Box::new(match op {
                    "sqrt" => self.sqrt(),
                    "sin" => self.sin(),
                    "cos" => self.cos(),
                    "tan" | "tg" => self.tan(),
                    "log" | "ln" => self.ln(),
                    _ => unreachable!(),
                })),

                other => Err(TrashError::UnknownMethod(other.to_string()).into()),
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
