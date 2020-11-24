use crate::core::*;

use std::{any::Any, io::prelude::*};

impl Object for String {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, _params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        self
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

    fn call(self: Box<Self>, _params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        self
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
