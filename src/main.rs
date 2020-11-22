#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TrashParser;

trait Object {
    fn clone(&self) -> Box<dyn Object>;
    fn call(self: Box<Self>, params: HashMap<String, Box<dyn Object>>) -> Box<dyn Object>;
    fn to_string(self: Box<Self>) -> String;
    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>>;
}

impl Object for String {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, _params: HashMap<String, Box<dyn Object>>) -> Box<dyn Object> {
        self
    }

    fn to_string(self: Box<Self>) -> String {
        *self
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

#[derive(Clone)]
struct Code(String);

struct Vars(HashMap<String, Box<dyn Object>>);

impl Vars {
    fn new() -> Self {
        Vars(HashMap::new())
    }

    fn with_vars(vars: HashMap<String, Box<dyn Object>>) -> Self {
        Vars(vars)
    }

    fn add(&mut self, name: String, value: Box<dyn Object>) {
        self.0.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Box<dyn Object> {
        self.0
            .remove(name)
            .unwrap_or_else(|| panic!("No such variable, {}", name))
    }

    fn get_cloned(&self, name: &str) -> Box<dyn Object> {
        Object::clone(
            self.0
                .get(name)
                .unwrap_or_else(|| panic!("No sush variable, {}", name))
                .as_ref(),
        )
    }
}

impl Code {
    fn from_string(s: String) -> Code {
        Code(s)
    }

    fn parse_run(pair: pest::iterators::Pair<Rule>, mut vars: Vars) -> (Box<dyn Object>, Vars) {
        let mut r: Box<dyn Object> = Box::new("".to_string());
        for pair in pair.into_inner() {
            if let Rule::call = pair.as_rule() {
                let mut inner = pair.into_inner();

                let first = inner.next().unwrap();
                match first.as_str() {
                    "$set" => {
                        let var_name = inner.next().unwrap().as_str().to_string();
                        let var_value = inner.next().unwrap();

                        match var_value.as_rule() {
                            Rule::string => {
                                vars.add(var_name, Box::new(var_value.as_str().to_string()));
                            }

                            Rule::ident => {
                                let obj_name = &var_value.as_str()[1..];
                                let obj = match &var_value.as_str()[0..=0] {
                                    "$" => vars.get(obj_name),
                                    "@" => vars.get_cloned(obj_name),
                                    _ => unreachable!(),
                                };
                                vars.add(var_name, obj);
                            }

                            Rule::call => {
                                let mut args: HashMap<_, Box<dyn Object>> = HashMap::new();
                                for arg in var_value.into_inner().enumerate() {
                                    match arg.1.as_rule() {
                                        Rule::string => {
                                            args.insert(
                                                arg.0.to_string(),
                                                Box::new(arg.1.as_str().to_string()),
                                            );
                                        }
                                        _ => {
                                            todo!();
                                        }
                                    }
                                }
                                vars.add(var_name, args.remove("0").unwrap().call(args));
                            }

                            _ => todo!(),
                        }
                    }
                    "$puts" => {
                        for val in inner {
                            match val.as_rule() {
                                Rule::string => {
                                    print!("{} ", val.as_str());
                                }

                                Rule::ident => {
                                    print!(
                                        "{} ",
                                        match &val.as_str()[0..=0] {
                                            "$" => vars.get(&val.as_str()[1..]).to_string(),
                                            "@" => vars.get_cloned(&val.as_str()[1..]).to_string(),
                                            _ => unreachable!(),
                                        }
                                    )
                                }

                                Rule::call => {
                                    let mut args: HashMap<_, Box<dyn Object>> = HashMap::new();
                                    for arg in val.into_inner().enumerate() {
                                        match arg.1.as_rule() {
                                            Rule::string => {
                                                args.insert(
                                                    arg.0.to_string(),
                                                    Box::new(arg.1.as_str().to_string()),
                                                );
                                            }
                                            _ => {
                                                todo!();
                                            }
                                        }
                                    }
                                    println!(
                                        "{}",
                                        args.remove("0").unwrap().call(args).to_string()
                                    );
                                }

                                _ => todo!(),
                            }
                        }
                        println!();
                    }
                    _ => match first.as_rule() {
                        Rule::string => {
                            r = Box::new(first.as_str().to_string());
                        }

                        Rule::call => {
                            let mut args: HashMap<_, Box<dyn Object>> = HashMap::new();
                            for arg in first.into_inner().enumerate() {
                                match arg.1.as_rule() {
                                    Rule::string => {
                                        args.insert(
                                            arg.0.to_string(),
                                            Box::new(arg.1.as_str().to_string()),
                                        );
                                    }
                                    _ => {
                                        todo!();
                                    }
                                }
                            }
                            r = args.remove("0").unwrap().call(args)
                        }

                        _ => {
                            todo!();
                        }
                    },
                }
            }
        }
        (r, vars)
    }

    fn run(self, vars: Vars) -> Box<dyn Object> {
        let pair = TrashParser::parse(Rule::code, &self.0)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap();
        Code::parse_run(pair, vars).0
    }
}

impl Object for Code {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, params: HashMap<String, Box<dyn Object>>) -> Box<dyn Object> {
        self.run(Vars::with_vars(params))
    }

    fn to_string(self: Box<Self>) -> String {
        self.0
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut f = File::open(s.trim().to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", Code::from_string(s).run(Vars::new()).to_string());
}
