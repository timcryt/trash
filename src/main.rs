#[cfg(test)]
mod tests;

mod core;

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
    println!(
        "{}",
        Code::from_string(s, Arc::new(Mutex::new(std::io::stdout())))
            .run(Vars::new(), &mut Vec::new())
            .to_string()
    );
}
