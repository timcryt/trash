mod objects;

use std::{
    any::Any,
    collections::HashMap,
    io::prelude::*,
    sync::{Arc, Mutex},
};

use pest::Parser;

pub trait Object {
    fn clone(&self) -> Box<dyn Object>;
    fn call(self: Box<Self>, params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object>;
    fn to_string(self: Box<Self>) -> String;
    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>>;
}

pub struct Vars(HashMap<String, Box<dyn Object>>);

impl Vars {
    pub fn new() -> Self {
        Vars(HashMap::new())
    }

    pub fn from_vec(v: Vec<Box<dyn Object>>) -> Self {
        let mut r = Vars::new();
        for (arg_name, arg_value) in v
            .into_iter()
            .enumerate()
            .map(|x| ((x.0 + 1).to_string(), x.1))
        {
            r.add(arg_name, arg_value);
        }
        r
    }

    pub fn add(&mut self, name: String, value: Box<dyn Object>) {
        self.0.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<Box<dyn Object>> {
        self.0.remove(name)
    }

    pub fn get_cloned(&self, scope: &&mut Vec<Self>, name: &str) -> Option<Box<dyn Object>> {
        self.0
            .get(name)
            .map(|x| Object::clone(x.as_ref()))
            .or_else(|| {
                for sclvl in scope.iter().rev() {
                    if let Some(r) = sclvl.0.get(name) {
                        return Some(Object::clone(r.as_ref()));
                    }
                }
                None
            })
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TrashParser;

#[derive(Clone)]
pub struct Code<T: Write + Any>(String, Arc<Mutex<T>>);

impl<T: Write + Any> Code<T> {
    pub fn from_string(s: String, out: Arc<Mutex<T>>) -> Code<T> {
        Code(s, out)
    }

    fn collect_args<'a>(
        &mut self,
        args_pairs: impl Iterator<Item = pest::iterators::Pair<'a, Rule>>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars, Vars) {
        let mut args = Vars::new();
        for (arg_name, arg_value) in args_pairs.enumerate() {
            let x = self.get_value(arg_value, vars, scope);
            vars = x.1;
            args.add(arg_name.to_string(), x.0);
        }
        (args.get("0").unwrap_or_else(|| unreachable!()), args, vars)
    }

    fn get_value(
        &mut self,
        value: pest::iterators::Pair<Rule>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        match value.as_rule() {
            Rule::string => (Box::new(value.as_str().to_string()), vars),

            Rule::literal_inner => (Box::new(value.as_str().to_string()), vars),

            Rule::ident => {
                let obj_name = &value.as_str()[1..];
                (
                    match &value.as_str()[0..=0] {
                        "$" => vars.get(obj_name),
                        "@" => vars.get_cloned(&scope, obj_name),
                        _ => unreachable!(),
                    }
                    .unwrap_or_else(|| panic!("No such variable, {}", obj_name)),
                    vars,
                )
            }

            Rule::call | Rule::call_inner => {
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

            Rule::tuple => {
                let mut tup = Vec::new();
                for el in value.into_inner() {
                    let x = self.get_value(el, vars, scope);
                    vars = x.1;
                    tup.push(x.0);
                }
                (Box::new(tup), vars)
            }

            _ => todo!(),
        }
    }

    fn exec_set(
        &mut self,
        mut vars: Vars,
        names: pest::iterators::Pair<Rule>,
        values: Box<dyn Object>,
    ) -> Vars {
        match names.as_rule() {
            Rule::string => {
                vars.add(names.as_str().to_string(), values);
            }

            Rule::tuple => {
                let names = names.into_inner().collect::<Vec<_>>();
                let values = values.to_tuple();
                if names.len() == values.len() {
                    for (name, value) in names.into_iter().zip(values.into_iter()) {
                        vars = self.exec_set(vars, name, value);
                    }
                } else {
                    panic!(
                        "Error in set operator, expected tuple with length {}, found tuple with length {}",
                        names.len(),
                        values.len()
                    );
                }
            }

            other => {
                panic!("Expected string or tuple, found {:?}", other);
            }
        }
        vars
    }

    fn parse_run(
        &mut self,
        pair: pest::iterators::Pair<Rule>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        let mut r: Box<dyn Object> = Box::new("".to_string());
        for pair in pair.into_inner() {
            if let Rule::call | Rule::call_inner = pair.as_rule() {
                let mut inner = pair.into_inner();

                let first = inner.next().unwrap();
                match first.as_str() {
                    "$set" => {
                        let names = inner.next().unwrap();
                        let values = inner.next().unwrap();
                        let x = self.get_value(values, vars, scope);

                        vars = x.1;
                        vars = self.exec_set(vars, names, x.0);

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
                            write!(self.1.lock().unwrap(), "{} ", write_value)
                                .unwrap_or_else(|x| panic!("IO error: {}", x));
                        }
                        writeln!(self.1.lock().unwrap())
                            .unwrap_or_else(|x| panic!("IO error! {}", x));

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
