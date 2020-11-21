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

fn get_var(vars: &mut HashMap<String, Box<dyn Object>>, name: &str) -> Box<dyn Object> {
    vars.remove(name)
        .unwrap_or_else(|| panic!("No such variable, {}", name))
}

fn get_cloned_var(vars: &HashMap<String, Box<dyn Object>>, name: &str) -> Box<dyn Object> {
    Object::clone(
        vars.get(name)
            .unwrap_or_else(|| panic!("No sush variable, {}", name))
            .as_ref(),
    )
}

impl Code {
    fn from_string(s: String) -> Code {
        Code(s)
    }

    fn parse_run(
        pair: pest::iterators::Pair<Rule>,
        mut vars: HashMap<String, Box<dyn Object>>,
    ) -> (Box<dyn Object>, HashMap<String, Box<dyn Object>>) {
        let mut r: Box<dyn Object> = Box::new("".to_string());
        if let Rule::call = pair.as_rule() {
            let mut inner = pair.clone().into_inner();
            match inner.next().unwrap().as_str() {
                "$set" => {
                    let var_name = inner.next().unwrap().as_str().to_string();
                    let var_value = inner.next().unwrap();

                    match var_value.as_rule() {
                        Rule::string => {
                            vars.insert(var_name, Box::new(var_value.as_str().to_string()));
                        }

                        Rule::ident => match &var_value.as_str()[0..=0] {
                            "$" => {
                                let obj_name = &var_value.as_str()[1..];
                                let obj = get_var(&mut vars, obj_name);
                                vars.insert(var_name, obj);
                            }
                            "@" => {
                                let obj_name = &var_value.as_str()[1..];
                                let obj = get_cloned_var(&vars, obj_name);
                                vars.insert(var_name, obj);
                            }
                            _ => unreachable!(),
                        },
                        _ => todo!(),
                    }
                }
                "$puts" => {
                    for val in inner {
                        match val.as_rule() {
                            Rule::string => {
                                print!("{} ", val.as_str());
                            }
                            Rule::ident => match &val.as_str()[0..=0] {
                                "$" => {
                                    print!(
                                        "{} ",
                                        get_var(&mut vars, &val.as_str()[1..]).to_string()
                                    )
                                }
                                "@" => {
                                    print!(
                                        "{} ",
                                        get_cloned_var(&mut vars, &val.as_str()[1..]).to_string()
                                    )
                                }
                                _ => unreachable!(),
                            },
                            _ => todo!(),
                        }
                    }
                    println!();
                }
                _ => todo!(),
            }
        }
        for inner_pair in pair.into_inner() {
            let x = Code::parse_run(inner_pair, vars);
            r = x.0;
            vars = x.1;
        }
        (r, vars)
    }

    fn run(self, vars: HashMap<String, Box<dyn Object>>) -> Box<dyn Object> {
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
        self.run(params)
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
    println!("{}", Code::from_string(s).run(HashMap::new()).to_string());
}
