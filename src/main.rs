#[cfg(test)]
mod tests;

mod core;
mod stdlib;

#[macro_use]
extern crate pest_derive;

use std::{
    fs::File,
    io::prelude::*,
    sync::{Arc, Mutex},
};

use crate::core::*;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut f = File::open(s.trim().to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut stdvars = vec![Vars::new()];
    stdvars[0].add(
        "if".to_string(),
        Box::new(crate::stdlib::if_statement::IfStatement),
    );
    stdvars[0].add(
        "while".to_string(),
        Box::new(crate::stdlib::while_statement::WhileStatement),
    );

    println!(
        "{}",
        Code::from_string(s, Arc::new(Mutex::new(std::io::stdout())))
            .run(Vars::new(), &mut stdvars)
            .to_string()
    );
}
