pub mod files;
pub mod floats;
pub mod if_statement;
pub mod integers;
pub mod while_statement;

use crate::core::Vars;

use std::{
    any::Any,
    io::{Read, Write},
};

pub fn stdlib<T: Write + Any, U: Read + Any>(stdout: T, stdin: U) -> (Vars, Vec<Vars>) {
    let (mut s, mut v) = (Vars::new(), Vars::new());
    s.add("if".to_string(), Box::new(if_statement::IfStatement));
    s.add(
        "while".to_string(),
        Box::new(while_statement::WhileStatement),
    );
    s.add("int".to_string(), Box::new(integers::Int));
    s.add("float".to_string(), Box::new(floats::Float));
    s.add("asc".to_string(), Box::new(integers::Asc));
    v.add(
        "stdout".to_string(),
        Box::new(files::WriteStream::new(stdout)),
    );
    v.add("stdin".to_string(), Box::new(files::ReadStream::new(stdin)));
    (v, vec![s])
}
