use crate::core::*;

pub struct Int;

impl Object for Int {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Int)
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        Box::new(
            dbg!(params
                .get("1")
                .unwrap_or_else(|| panic!("Expected 1 argument, found 0"))
                .to_string())
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Expected number, found string")),
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

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                op if op == "div" || op == "add" || op == "sub" || op == "mul" || op == "div" || op == "rem" => {
                    let n = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    let num = Box::new(Int)
                        .call(Vars::from_vec(vec![n]), scope)
                        .to_string()
                        .parse::<i64>()
                        .unwrap();
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
                    let num = Box::new(Int)
                        .call(Vars::from_vec(vec![n]), scope)
                        .to_string()
                        .parse::<i64>()
                        .unwrap();
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
