#[cfg(test)]
mod tests;

mod core;
mod stdlib;

#[macro_use]
extern crate pest_derive;

use std::{
    fs::File,
    io::prelude::*,
};

use crate::core::*;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut f = File::open(s.trim().to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let (vars, mut scope) = stdlib::stdlib(std::io::stdout());
    println!(
        "{}",
        Code::from_string(s)
            .run(vars, &mut scope)
            .to_string()
    );
}
