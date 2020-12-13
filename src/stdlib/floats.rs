use crate::core::*;

use std::any::*;

pub struct Float;

impl Float {
    fn float(var: Box<dyn Object>) -> Box<dyn Object> {
        if var.id() == 0.0f64.type_id() {
            var
        } else if var.id() == 0i64.type_id() {
            Box::new(unsafe {*(var.as_ref() as *const dyn Object as *const i64)} as f64)
        } else {
            Box::new(
                var.to_string()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Expected number, found string")),
            )
        }
    }
}

impl Object for Float {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Float)
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        Self::float(
            params
                .get("1")
                .unwrap_or_else(|| panic!("Expected 1 argument, found 0")),
        )
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}

impl Object for f64 {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(*self)
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op if op == "add" || op == "sub" || op == "mul" || op == "div" => {
                    let n = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    let num = unsafe {
                        *(Float::float(n).as_ref() as *const dyn Object as *const f64)
                    };

                    Box::new(match op {
                        "add" => *self + num,
                        "sub" => *self - num,
                        "mul" => *self * num,
                        "div" => *self / num,
                        _ => unreachable!(),
                    })
                }

                op if op == "eq" || op == "gt" || op == "lt" => {
                    let n = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    let num = unsafe {
                        *(Float::float(n).as_ref() as *const dyn Object as *const f64)
                    };
                    Box::new(
                        match op {
                            "eq" => (*self - num).abs() / (*self + num) < std::f64::EPSILON,
                            "gt" => *self > num,
                            "lt" => *self < num,
                            _ => unreachable!(),
                        }
                        .to_string(),
                    )
                }

                op if op == "sqrt"
                    || op == "sin"
                    || op == "cos"
                    || op == "tan"
                    || op == "tg"
                    || op == "log"
                    || op == "ln" =>
                {
                    Box::new(match op {
                        "sqrt" => self.sqrt(),
                        "sin" => self.sin(),
                        "cos" => self.cos(),
                        "tan" | "tg" => self.tan(),
                        "log" | "ln" => self.ln(),
                        _ => unreachable!(),
                    })
                }

                other => panic!("Unknown method {}", other),
            },

            None => self,
        }
    }

    fn to_string(self: Box<Self>) -> String {
        self.as_ref().to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}
