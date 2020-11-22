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
    fn call(self: Box<Self>, params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object>;
    fn to_string(self: Box<Self>) -> String;
    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>>;
}

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

#[derive(Clone)]
struct Code(String);

struct Vars(HashMap<String, Box<dyn Object>>);

impl Vars {
    fn new() -> Self {
        Vars(HashMap::new())
    }

    fn add(&mut self, name: String, value: Box<dyn Object>) {
        self.0.insert(name, value);
    }

    fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    } 

    fn get(&mut self, scope: &&mut Vec<Self>, name: &str) -> Box<dyn Object> {
        self.0
            .remove(name)
            .unwrap_or_else(|| {
                for sclvl in scope.iter().rev() {
                    if sclvl.contains(name) {
                        panic!("Cannot move variable {} from scope", name);
                    }
                }
                panic!("No such variable, {}", name);
            }) 
    }

    fn get_cloned(&self, scope: &&mut Vec<Self>, name: &str) -> Box<dyn Object> {
        Object::clone(
            self.0
                .get(name)
                .unwrap_or_else(|| {
                    for sclvl in scope.iter().rev() {
                        if let Some(r) = sclvl.0.get(name) {
                            return r;
                        }
                    }
                    panic!("No sush variable, {}", name)
                })
                .as_ref(),
        )
    }
}

impl Code {
    fn from_string(s: String) -> Code {
        Code(s)
    }

    fn collect_args(
        args_pairs: pest::iterators::Pairs<Rule>,
    ) -> (Box<dyn Object>, Vars) {
        let mut args = Vars::new();
        for arg in args_pairs.enumerate() {
            match arg.1.as_rule() {
                Rule::string => {
                    args.add(arg.0.to_string(), Box::new(arg.1.as_str().to_string()));
                }
                _ => {
                    todo!();
                }
            }
        }
        (args.get(&&mut Vec::new(), "0"), args)
    }

    fn parse_run(pair: pest::iterators::Pair<Rule>, mut vars: Vars, scope: &mut Vec<Vars>) -> (Box<dyn Object>, Vars) {
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
                                    "$" => vars.get(&scope, obj_name),
                                    "@" => vars.get_cloned(&scope, obj_name),
                                    _ => unreachable!(),
                                };
                                vars.add(var_name, obj);
                            }

                            Rule::call => {
                                let (obj, args) = Code::collect_args(var_value.into_inner());
                                let var_value;
                                scope.push(vars);
                                var_value = obj.call(args, scope);
                                vars = scope.pop().unwrap();
                                vars.add(var_name, var_value);
                            }

                            _ => todo!(),
                        }

                        r = Box::new("".to_string());
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
                                            "$" => vars.get(&scope, &val.as_str()[1..]).to_string(),
                                            "@" => vars.get_cloned(&scope, &val.as_str()[1..]).to_string(),
                                            _ => unreachable!(),
                                        }
                                    )
                                }

                                Rule::call => {
                                    let (obj, args) = Code::collect_args(val.into_inner());
                                    scope.push(vars);
                                    println!("{}", obj.call(args, scope).to_string());
                                    vars = scope.pop().unwrap();
                                }

                                _ => todo!(),
                            }
                        }
                        println!();

                        r = Box::new("".to_string());
                    }
                    _ => match first.as_rule() {
                        Rule::string => {
                            r = Box::new(first.as_str().to_string());
                        }

                        Rule::call => {
                            let (obj, args) = Code::collect_args(first.into_inner());
                            vars = {
                                scope.push(vars);
                                r = obj.call(args, scope);
                                scope.pop().unwrap()
                            };                             
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

    fn run(self, vars: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let pair = TrashParser::parse(Rule::code, &self.0)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap();
        Code::parse_run(pair, vars, scope).0
    }
}

impl Object for Code {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
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

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut f = File::open(s.trim().to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", Code::from_string(s).run(Vars::new(), &mut Vec::new()).to_string());
}
