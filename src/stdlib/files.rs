use crate::core::*;

use std::{any::Any, io::prelude::*};

pub struct WriteStream<T: Write + Any>(T);

impl<T: Write + Any> WriteStream<T> {
    pub fn new(stream: T) -> WriteStream<T> {
        WriteStream(stream)
    }
}

pub struct ReadStream<T: Read + Any>(T);

impl<T: Read + Any> ReadStream<T> {
    pub fn new(stream: T) -> ReadStream<T> {
        ReadStream(stream)
    }
}

impl<T: Read + Any> Object for ReadStream<T> {
    fn clone(&self) -> error::TrashResult {
        Err(error::TrashError::LinearTypeCloning.into())
    }

    fn call(mut self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        if params.get("1").is_some() {
            let mut buf = [0u8; 1];
            let val = match self.0.read_exact(&mut buf) {
                Ok(_) => buf[0] as i64,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => -1i64,
                Err(e) => return Err(e.into()),
            };
            Ok(Box::new(vec![self as Box<dyn Object>, Box::new(val)]))
        } else {
            Ok(self)
        }
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

impl<T: Write + Any> Object for WriteStream<T> {
    fn clone(&self) -> error::TrashResult {
        Err(error::TrashError::LinearTypeCloning.into())
    }

    fn call(mut self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        let mut i = 1;
        while let Some(x) = params.get(&i.to_string()) {
            if i != 1 {
                write!(self.0, " ")?;
            }
            write!(self.0, "{}", x.to_string())?;
            i += 1;
        }
        Ok(self)
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}
