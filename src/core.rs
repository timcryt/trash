mod objects;

use std::{
    collections::HashMap,
    sync::Arc,
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
enum ObjDef {
    String(String),
    Closure(Arc<Parsed>, String),
    MoveClosure(Arc<Parsed>, String, Vec<String>),
    ObjMove(String),
    ObjClone(String),
    Call(Box<(ObjDef, Vec<ObjDef>)>),
    Tuple(Vec<ObjDef>),
}

#[derive(Clone)]
enum AssignTree {
    Leaf(String),
    Node(Vec<AssignTree>),
}

#[derive(Clone)]
enum Call {
    SetOp(AssignTree, ObjDef),
    CallOp((ObjDef, Vec<ObjDef>)),
}

#[derive(Clone)]
struct Parsed(Vec<Call>);

impl Parsed {
    fn parse(code: &str) -> Parsed {
        Parsed(TrashParser::parse(Rule::code, code)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap()
            .into_inner()
            .filter_map(|pair| {
                match pair.as_rule() {
                    Rule::call => {
                        let mut call_iter = pair.into_inner();
                        let first = call_iter.next().unwrap();
                        if first.as_str() == "$set" {
                            let names = call_iter.next().unwrap_or_else(|| panic!("Expected variable name"));
                            let values = call_iter.next().unwrap_or_else(|| panic!("Expected variable value"));
                            Some(Call::SetOp(Self::parse_set_names(names), Self::parse_obj(values)))
                        } else {
                            Some(Call::CallOp(Self::parse_call(first, call_iter)))
                        }
                    }
                    Rule::EOI => None,
                    other => panic!("{:?}", other),
                }
            })
            .collect()
        )
    }

    fn parse_set_names(names: pest::iterators::Pair<Rule>) -> AssignTree {
        match names.as_rule() {
            Rule::string => {
                AssignTree::Leaf(names.as_str().to_string())
            }

            Rule::tuple => {
                AssignTree::Node(names.into_inner().map(Self::parse_set_names).collect())
            }

            other => {
                panic!("Expected string or tuple, found {:?}", other);
            }
        }
    }

    fn parse_call(obj: pest::iterators::Pair<Rule>, args: pest::iterators::Pairs<Rule>) -> (ObjDef, Vec<ObjDef>) {
        (Self::parse_obj(obj), args.map(Self::parse_obj).collect())
    }

    fn parse_obj(obj: pest::iterators::Pair<Rule>) -> ObjDef {
        match obj.as_rule() {
            Rule::string | Rule::literal_inner => ObjDef::String(obj.as_str().to_string()),
            
            Rule::ident => match &obj.as_str()[0..=0] {
                "$" => ObjDef::ObjMove(obj.as_str()[1..].to_string()),
                "@" => ObjDef::ObjClone(obj.as_str()[1..].to_string()),
                _ => unreachable!(),
            }

            Rule::call | Rule::call_inner => {
                let mut call_iter = obj.into_inner();
                let first = call_iter.next().unwrap();
                ObjDef::Call(Box::new(Self::parse_call(first, call_iter)))
            }

            Rule::closure_inner => ObjDef::Closure(Arc::new(Parsed::parse(obj.as_str())), obj.as_str().to_string()),

            Rule::tuple => {
                ObjDef::Tuple(obj.into_inner().map(Self::parse_obj).collect())
            }

            Rule::move_closure => {
                let mut clos_iter = obj.into_inner();
                let clos_str = clos_iter.next().unwrap().as_str();
                let clos_vars = clos_iter.map(|x| x.as_str().to_string()).collect();
                ObjDef::MoveClosure(Arc::new(Parsed::parse(clos_str)), clos_str.to_string(), clos_vars)
            }

            _ => unreachable!(),
        }
    }
}


#[derive(Clone)]
pub struct Code(String, Option<Arc<Parsed>>);

#[derive(Clone)]
pub struct MovClos(Code, Vec<String>);

impl Code {
    pub fn from_string(s: String) -> Code {
        Code(s, None)
    }

    fn collect_args<'a>(
        &mut self,
        args_pairs: &Vec<ObjDef>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Vars, Vars) {
        let mut args = Vars::new();
        for (arg_name, arg_value) in args_pairs.into_iter().enumerate() {
            let x = self.get_value(arg_value, vars, scope);
            vars = x.1;
            args.add((arg_name + 1).to_string(), x.0);
        }
        (args, vars)
    }

    fn get_value(
        &mut self,
        value: &ObjDef,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        match value {
            ObjDef::String(s) => (Box::new(s.to_string()), vars),
            
            ObjDef::Closure(p, c) => (
                Box::new(Code(c.to_string(), Some(Arc::clone(p)))), 
                vars
            ),
            
            ObjDef::ObjMove(name) => {
                let obj = vars.get(&name).unwrap_or_else(|| panic!("No such variable, {}", name));
                (obj, vars)
            }

            ObjDef::ObjClone(name) => {
                let obj = vars.get_cloned(&scope, &name).unwrap_or_else(|| panic!("No such variable, {}", name));
                (obj, vars)
            }

            ObjDef::Call(b) => {
                let (obj, args) = b.as_ref();
                let x = self.get_value(&obj, vars, scope);
                vars = x.1;
                let y = self.collect_args(&args, vars, scope);
                vars = y.1;
                scope.push(vars);
                let res = x.0.call(y.0, scope);
                vars = scope.pop().unwrap();
                (res, vars)
            }

            ObjDef::Tuple(objs) => {
                let mut tup = Vec::new();
                for obj in objs {
                    let x = self.get_value(obj, vars, scope);
                    vars = x.1;
                    tup.push(x.0);
                }
                (Box::new(tup), vars)
            }

            ObjDef::MoveClosure(p, c, args) => (
                Box::new(MovClos(Code(c.to_string(), Some(Arc::clone(p))), args.clone())), 
                vars
            ),
        }
    }

    fn exec_set(
        &mut self,
        names: &AssignTree,
        values: Box<dyn Object>,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> Vars {
        match names {
            AssignTree::Leaf(name) => {
                vars.add(name.to_string(), values);
            }

            AssignTree::Node(names) => {
                let values = values.to_tuple();
                if values.len() != names.len() {
                    panic!(
                        "Error in set operator, expected tuple with length {}, found tuple with length {}",
                        names.len(),
                        values.len()
                    );
                } else {
                    for (name, value) in names.into_iter().zip(values.into_iter()) {
                        vars = self.exec_set(name, value, vars, scope);
                    }
                }
            }
        }
        vars
    }

    fn parse_run(
        &mut self,
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Box<dyn Object>, Vars) {
        let mut r: Box<dyn Object> = Box::new("".to_string());
        let code = Arc::clone(self.1.as_ref().unwrap());
        for call in &code.as_ref().0 {
            match call {
                Call::SetOp(names, values) => {
                    let x = self.get_value(&values, vars, scope);
                    vars = x.1;
                    vars = self.exec_set(&names, x.0, vars, scope);
                    r = Box::new("".to_string());
                }

                Call::CallOp((obj, args)) => {
                    let x = self.get_value(&obj, vars, scope);
                    vars = x.1;
                    let y = self.collect_args(&args, vars, scope);
                    vars = y.1;
                    scope.push(vars);
                    r = x.0.call(y.0, scope);
                    vars = scope.pop().unwrap();
                }
            }
        }
        (r, vars)
    }

    pub fn run(mut self, vars: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        if self.1.is_none() {
            self.1 = Some(Arc::new(Parsed::parse(&self.0)));
        }
        self.parse_run(vars, scope).0
    }
}
