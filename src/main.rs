#[cfg(test)]
mod tests;

#[macro_use]
extern crate pest_derive;

use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TrashParser;

pub trait Object {
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
pub struct Code<T: Write + Any>(String, Arc<Mutex<T>>);

pub struct Vars(HashMap<String, Box<dyn Object>>);

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
        self.0.remove(name).unwrap_or_else(|| {
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

impl<T: Write + Any> Code<T> {
    pub fn from_string(s: String, out: Arc<Mutex<T>>) -> Code<T> {
        Code(s, out)
    }

    fn collect_args<'a>(
        &self,
        args_pairs: impl Iterator<Item = pest::iterators::Pair<'a, Rule>>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars, Vars) {
        let mut args = Vars::new();
        for arg in args_pairs.enumerate() {
            match arg.1.as_rule() {
                Rule::string => {
                    args.add(arg.0.to_string(), Box::new(arg.1.as_str().to_string()));
                }

                Rule::ident => args.add(
                    arg.0.to_string(),
                    match &arg.1.as_str()[0..=0] {
                        "$" => vars.get(&scope, &arg.1.as_str()[1..]),
                        "@" => vars.get_cloned(&scope, &arg.1.as_str()[1..]),
                        _ => unreachable!(),
                    },
                ),

                Rule::clojure_inner => args.add(
                    arg.0.to_string(),
                    Box::new(Code::from_string(
                        arg.1.as_str().to_string(),
                        <Arc<_> as std::clone::Clone>::clone(&self.1),
                    )),
                ),

                Rule::call => {
                    let (obj, call_args, x) = self.collect_args(arg.1.into_inner(), vars, scope);
                    vars = x;
                    let value;
                    scope.push(vars);
                    value = obj.call(call_args, scope);
                    vars = scope.pop().unwrap();
                    args.add(arg.0.to_string(), value);
                }

                _ => todo!(),
            }
        }
        (args.get(&&mut Vec::new(), "0"), args, vars)
    }

    fn get_value(
        &mut self,
        value: pest::iterators::Pair<Rule>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        match value.as_rule() {
            Rule::string => (Box::new(value.as_str().to_string()), vars),

            Rule::ident => {
                let obj_name = &value.as_str()[1..];
                (
                    match &value.as_str()[0..=0] {
                        "$" => vars.get(&scope, obj_name),
                        "@" => vars.get_cloned(&scope, obj_name),
                        _ => unreachable!(),
                    },
                    vars,
                )
            }

            Rule::call => {
                let (obj, args, x) = self.collect_args(value.into_inner(), vars, scope);
                vars = x;
                let var_value;
                scope.push(vars);
                var_value = obj.call(args, scope);
                vars = scope.pop().unwrap();
                (var_value, vars)
            }

            Rule::clojure_inner => (
                Box::new(Code::from_string(
                    value.as_str().to_string(),
                    self.1.clone(),
                )),
                vars,
            ),

            _ => todo!(),
        }
    }

    fn parse_run(
        &mut self,
        pair: pest::iterators::Pair<Rule>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        let mut r: Box<dyn Object> = Box::new("".to_string());
        for pair in pair.into_inner() {
            if let Rule::call = pair.as_rule() {
                let mut inner = pair.into_inner();

                let first = inner.next().unwrap();
                match first.as_str() {
                    "$set" => {
                        let var_name = inner.next().unwrap().as_str().to_string();
                        let var_value = inner.next().unwrap();

                        let x = self.get_value(var_value, vars, scope);
                        vars = x.1;
                        vars.add(var_name, x.0);

                        r = Box::new("".to_string());
                    }

                    "$puts" => {
                        for val in inner {
                            let write_value = {
                                let x = self.get_value(val, vars, scope);
                                vars = x.1;
                                x.0
                            }
                            .to_string();
                            write!(self.1.lock().unwrap(), "{} ", write_value).unwrap();
                        }
                        writeln!(self.1.lock().unwrap()).unwrap();

                        r = Box::new("".to_string());
                    }
                    _ => {
                        let (obj, args, x) =
                            self.collect_args(Some(first).into_iter().chain(inner), vars, scope);
                        scope.push(x);
                        r = obj.call(args, scope);
                        vars = scope.pop().unwrap();
                    }
                }
            }
        }
        (r, vars)
    }

    pub fn run(mut self, vars: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let s = std::clone::Clone::clone(&self.0);
        let pair = TrashParser::parse(Rule::code, &s)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap();
        self.parse_run(pair, vars, scope).0
    }
}

impl<T: Write + Any> Object for Code<T> {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Code(
            std::clone::Clone::clone(&self.0),
            <Arc<_> as std::clone::Clone>::clone(&self.1),
        ))
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
    println!(
        "{}",
        Code::from_string(s, Arc::new(Mutex::new(std::io::stdout())))
            .run(Vars::new(), &mut Vec::new())
            .to_string()
    );
}
