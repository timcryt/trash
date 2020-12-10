use crate::core::*;

use std::any::*;

pub struct Int;

impl Int {
    fn to_int(var: Box<dyn Object>) -> Box<dyn Object> {
        if var.id() == 0i64.type_id() {
            var
        } else {
            Box::new(
                var.to_string()
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Expected number, found string")),
            )
        }
    }
}

impl Object for Int {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Int)
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        Self::to_int(
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

impl Object for i64 {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(*self)
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op if op == "div"
                    || op == "add"
                    || op == "sub"
                    || op == "mul"
                    || op == "div"
                    || op == "rem" =>
                {
                    let n = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    let num =
                        unsafe { *(Int::to_int(n).as_ref() as *const dyn Object as *const i64) };

                    Box::new(match op {
                        "add" => *self + num,
                        "sub" => *self - num,
                        "mul" => *self * num,
                        "div" => *self / num,
                        "rem" => *self % num,
                        _ => unreachable!(),
                    })
                }

                op if op == "eq" || op == "gt" || op == "lt" => {
                    let n = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    let num =
                        unsafe { *(Int::to_int(n).as_ref() as *const dyn Object as *const i64) };
                    Box::new(
                        match op {
                            "eq" => *self == num,
                            "gt" => *self > num,
                            "lt" => *self < num,
                            _ => unreachable!(),
                        }
                        .to_string(),
                    )
                }

                "chr" => {
                    let chr = std::char::from_u32(*self as u32)
                        .unwrap_or_else(|| panic!("Invalid UTF-8 char"));
                    Box::new(chr.to_string())
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
