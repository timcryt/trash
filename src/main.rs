#[cfg(test)]
mod tests;

mod core;
mod stdlib;

#[macro_use]
extern crate pest_derive;

use std::{env::args_os, fs::File, io::prelude::*};

use crate::core::*;

fn main() {
    let mut file = File::open(
        args_os()
            .nth(1)
            .unwrap_or_else(|| panic!("Expected 1 argument, found 0")),
    )
    .unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let (vars, mut scope) = stdlib::stdlib(std::io::stdout(), std::io::stdin());
    println!("{}", Code::from_string(s).run(vars, &mut scope).to_string());
}
