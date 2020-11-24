use crate::core::*;

use std::{any::Any, io::prelude::*};

impl Object for String {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(mut self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        if params.contains("1") {
            let method = params.get(&scope, "1").to_string();
            match &method[..] {
                "len" => Box::new(self.len().to_string()),

                "_len" => {
                    let len = Box::new(self.len().to_string());
                    Box::new(vec![self as Box<dyn Object>, len])
                }

                "split" => {
                    let delimiter = if params.contains("2") {
                        params.get(&scope, "2").to_string()
                    } else {
                        " ".to_string()
                    };

                    Box::new(
                        self.split(&delimiter)
                            .map(|x| Box::new(x.to_string()) as Box<dyn Object>)
                            .collect::<Vec<_>>(),
                    )
                }

                "push" => {
                    let str_to_push = if params.contains("2") {
                        params.get(&scope, "2").to_string()
                    } else {
                        "".to_string()
                    };
                    self.push_str(&str_to_push);

                    self
                }

                "eq" => {
                    let str_to_compare = if params.contains("2") {
                        params.get(&scope, "2").to_string()
                    } else {
                        panic!("Expected 1 argument, found 0");
                    };

                    Box::new((*self == str_to_compare).to_string())
                }

                "_eq" => {
                    let str_to_compare = if params.contains("2") {
                        params.get(&scope, "2").to_string()
                    } else {
                        panic!("Expected 1 argument, found 0");
                    };

                    let r = Box::new((*self == str_to_compare).to_string());

                    Box::new(vec![self as Box<dyn Object>, Box::new(str_to_compare), r])
                }

                other => {
                    panic!("Unknown method: {}", other);
                }
            }
        } else {
            self
        }
    }

    fn to_string(self: Box<Self>) -> String {
        *self
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

impl Object for Vec<Box<dyn Object>> {
    fn clone(&self) -> Box<dyn Object> {
        let mut cloned = Vec::new();
        for el in self {
            cloned.push(Object::clone(el.as_ref()));
        }
        Box::new(cloned)
    }

    fn call(mut self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        if params.contains("1") {
            let method = params.get(&scope, "1").to_string();
            match &method[..] {
                "push" => {
                    let val = if params.contains("2") {
                        params.get(&scope, "2")
                    } else {
                        panic!("Expected 1 argument, found 0")
                    };
                    self.push(val);
                    self
                }

                "pop" => {
                    let el = self
                        .pop()
                        .unwrap_or_else(|| panic!("Can't pop value from empty tuple"));
                    Box::new(vec![self as Box<dyn Object>, el])
                }

                "is_empty" => Box::new(self.is_empty().to_string()),

                "_is_empty" => {
                    let r = Box::new(self.is_empty().to_string());
                    Box::new(vec![self as Box<dyn Object>, r])
                }

                other => {
                    let i = other
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Unknown method: {}", other));
                    Object::clone(
                        self.get(i)
                            .unwrap_or_else(|| {
                                panic!("Index out of bounds: index is {}, len is {}", i, self.len())
                            })
                            .as_ref(),
                    )
                }
            }
        } else {
            self
        }
    }

    fn to_string(self: Box<Self>) -> String {
        let mut s = "( ".to_string();
        for el in self.into_iter() {
            s += &el.to_string();
            s += " ";
        }
        s + ")"
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        *self
    }
}

impl<T: Write + Any> Object for Code<T> {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Code(
            std::clone::Clone::clone(&self.0),
            <Arc<_> as std::clone::Clone>::clone(&self.1),
        ))
    }

    fn call(self: Box<Self>, params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        self.run(params, scope)
    }

    fn to_string(self: Box<Self>) -> String {
        self.0
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}
