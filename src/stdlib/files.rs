use crate::core::*;

use std::{
    any::Any,
    io::prelude::*,
};

pub struct WriteStream<T: Write + Any>(T);

impl<T: Write + Any> WriteStream<T> {
    pub fn new(stream: T) -> WriteStream<T> {
        WriteStream(stream)
    }
}

impl<T: Write + Any> Object for WriteStream<T> {
    fn clone(&self) -> Box<dyn Object> {
        panic!("Cannot clone linear type")
    } 

    fn call(mut self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let mut i = 1;
        while let Some(x) = params.get(&i.to_string()) {
            write!(self.0 ,"{} ", x.to_string()).unwrap_or_else(|x| panic!("IO error {}", x));
            i += 1;
        }
        writeln!(self.0).unwrap_or_else(|x| panic!("IO error, {}", x));
        self
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}