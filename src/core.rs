pub mod error;
mod objects;

use std::{any::*, ops::Range, sync::Arc};

use fnv::{FnvHashMap, FnvHashSet};

use pest::Parser;

pub trait Object: Any {
    fn clone(&self) -> error::TrashResult;
    fn call(self: Box<Self>, params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult;
    fn to_string(self: Box<Self>) -> String;
    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>>;
    fn id(&self) -> TypeId {
        self.type_id()
    }
}

type Map<K, V> = FnvHashMap<K, V>;
type Set<K> = FnvHashSet<K>;

pub struct Vars(Map<String, Box<dyn Object>>);

impl Default for Vars {
    fn default() -> Self {
        Vars(FnvHashMap::default())
    }
}

impl Vars {
    pub fn new() -> Self {
        Vars::default()
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

    pub fn get_cloned(&self, scope: &&mut Vec<Self>, name: &str) -> Option<error::TrashResult> {
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
enum Def {
    String(String),
    Closure(Arc<Parsed>, String),
    MoveClosure(Arc<Parsed>, String, Vec<String>),
    ObjMove(String),
    ObjClone(String),
    Call(Box<(ObjDef, Vec<ObjDef>)>),
    Tuple(Vec<ObjDef>),
}

#[derive(Clone)]
struct ObjDef {
    span: Range<usize>,
    def: Def,
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
        let mut moved_vars = Set::default();
        Parsed(
            TrashParser::parse(Rule::code, code)
                .unwrap_or_else(|e| panic!("{}", Self::find_error(code, code, e)))
                .next()
                .unwrap()
                .into_inner()
                .filter_map(|pair| match pair.as_rule() {
                    Rule::call => {
                        let mut call_iter = pair.into_inner();
                        let first = call_iter.next().unwrap();
                        if first.as_str() == "$set" {
                            let names = call_iter.next().unwrap_or_else(|| {
                                panic!(
                                    "{}",
                                    pest::error::Error::<()>::new_from_span(
                                        pest::error::ErrorVariant::CustomError {
                                            message: "Expected variable name".to_owned()
                                        },
                                        first.as_span(),
                                    )
                                )
                            });
                            let values = call_iter.next().unwrap_or_else(|| {
                                panic!(
                                    "{}",
                                    pest::error::Error::<()>::new_from_span(
                                        pest::error::ErrorVariant::CustomError {
                                            message: "Expected variable value".to_owned()
                                        },
                                        first.as_span(),
                                    )
                                )
                            });
                            let obj = Self::parse_obj(values, &mut moved_vars);
                            Some(Call::SetOp(
                                Self::parse_set_names(names, &mut moved_vars),
                                obj,
                            ))
                        } else {
                            Some(Call::CallOp(Self::parse_call(
                                first,
                                call_iter,
                                &mut moved_vars,
                            )))
                        }
                    }
                    Rule::EOI => None,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    fn find_error(
        origin: &str,
        code: &str,
        error: pest::error::Error<Rule>,
    ) -> pest::error::Error<Rule> {
        if let pest::error::InputLocation::Pos(pos) = error.location {
            let err = if &code[pos..pos + 1] == "{" {
                Self::find_error(
                    origin,
                    &code[(pos + 1)..],
                    TrashParser::parse(Rule::code, &code[(pos + 1..)]).unwrap_err(),
                )
            } else if &code[pos..pos + 2] == "<{" {
                Self::find_error(
                    origin,
                    &code[(pos + 2)..],
                    TrashParser::parse(Rule::code, &code[(pos + 2..)]).unwrap_err(),
                )
            } else {
                return error;
            };
            match err.location {
                pest::error::InputLocation::Pos(rel_pos) => pest::error::Error::new_from_pos(
                    err.variant,
                    pest::Position::new(origin, rel_pos + pos).unwrap(),
                ),
                pest::error::InputLocation::Span((rel_start, rel_end)) => {
                    pest::error::Error::new_from_span(
                        err.variant,
                        pest::Span::new(origin, rel_start + pos, rel_end + pos).unwrap(),
                    )
                }
            }
        } else {
            error
        }
    }

    fn parse_set_names(
        names: pest::iterators::Pair<Rule>,
        moved_vars: &mut Set<String>,
    ) -> AssignTree {
        match names.as_rule() {
            Rule::string => {
                moved_vars.remove(&names.as_str().to_string());
                AssignTree::Leaf(names.as_str().to_string())
            }

            Rule::tuple => AssignTree::Node(
                names
                    .into_inner()
                    .map(|x| Self::parse_set_names(x, moved_vars))
                    .collect(),
            ),

            other => {
                panic!(
                    "{}",
                    pest::error::Error::<()>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: format!("Expected string or tuple, found {:?}", other)
                        },
                        names.as_span(),
                    )
                )
            }
        }
    }

    fn parse_call(
        obj: pest::iterators::Pair<Rule>,
        args: pest::iterators::Pairs<Rule>,
        moved_vars: &mut Set<String>,
    ) -> (ObjDef, Vec<ObjDef>) {
        (
            Self::parse_obj(obj, moved_vars),
            args.map(|x| Self::parse_obj(x, moved_vars)).collect(),
        )
    }

    fn span_into_range(span: pest::Span) -> Range<usize> {
        Range {
            start: span.start(),
            end: span.end(),
        }
    }

    fn parse_obj(obj: pest::iterators::Pair<Rule>, moved_vars: &mut Set<String>) -> ObjDef {
        match obj.as_rule() {
            Rule::string => ObjDef {
                span: Self::span_into_range(obj.as_span()),
                def: Def::String(obj.as_str().to_string()),
            },

            Rule::literal_inner => ObjDef {
                span: Self::span_into_range(obj.as_span()),
                def: Def::String(
                    obj.into_inner()
                        .map(|chr| match chr.as_str() {
                            "\\n" => '\n',
                            "\\\\" => '\\',
                            other => other.chars().next().unwrap(),
                        })
                        .collect(),
                ),
            },

            Rule::ident => match &obj.as_str()[0..=0] {
                "$" => {
                    if moved_vars.contains(&obj.as_str()[1..]) {
                        panic!(
                            "{}",
                            pest::error::Error::<()>::new_from_span(
                                pest::error::ErrorVariant::CustomError {
                                    message: format!(
                                        "Error, variable {} is guaranteed moved ",
                                        &obj.as_str()[1..]
                                    )
                                },
                                obj.as_span(),
                            )
                        );
                    } else {
                        moved_vars.insert(obj.as_str()[1..].to_string());
                    }

                    ObjDef {
                        span: Self::span_into_range(obj.as_span()),
                        def: Def::ObjMove(obj.as_str()[1..].to_string()),
                    }
                }
                "@" => ObjDef {
                    span: Self::span_into_range(obj.as_span()),
                    def: Def::ObjClone(obj.as_str()[1..].to_string()),
                },
                _ => unreachable!(),
            },

            Rule::call | Rule::call_inner => {
                let span = obj.as_span();
                let mut call_iter = obj.into_inner();
                let first = call_iter.next().unwrap();
                ObjDef {
                    span: Self::span_into_range(span),
                    def: Def::Call(Box::new(Self::parse_call(first, call_iter, moved_vars))),
                }
            }

            Rule::closure_inner => ObjDef {
                span: Self::span_into_range(obj.as_span()),
                def: Def::Closure(
                    Arc::new(Parsed::parse(obj.as_str())),
                    obj.as_str().to_string(),
                ),
            },

            Rule::tuple => ObjDef {
                span: Self::span_into_range(obj.as_span()),
                def: Def::Tuple(
                    obj.into_inner()
                        .map(|x| Self::parse_obj(x, moved_vars))
                        .collect(),
                ),
            },

            Rule::move_closure => {
                let span = obj.as_span();
                let mut clos_iter = obj.into_inner();
                let clos_str = clos_iter.next().unwrap().as_str();
                let clos_vars = clos_iter.map(|x| x.as_str().to_string()).collect();
                ObjDef {
                    span: Self::span_into_range(span),
                    def: Def::MoveClosure(
                        Arc::new(Parsed::parse(clos_str)),
                        clos_str.to_string(),
                        clos_vars,
                    ),
                }
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

    fn collect_args(
        &mut self,
        args_pairs: &[ObjDef],
        mut vars: Vars,
        scope: &mut Vec<Vars>,
    ) -> (Vars, Vars) {
        let mut args = Vars::new();
        for (arg_name, arg_value) in args_pairs.iter().enumerate() {
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
        match &value.def {
            Def::String(s) => (Box::new(s.to_string()), vars),

            Def::Closure(p, c) => (Box::new(Code(c.to_string(), Some(Arc::clone(p)))), vars),

            Def::ObjMove(name) => {
                let obj = vars.get(&name).unwrap_or_else(|| {
                    panic!(
                        "No such variable, {}\n at {}",
                        name,
                        &self.0[value.span.clone()]
                    )
                });
                (obj, vars)
            }

            Def::ObjClone(name) => {
                let obj = vars.get_cloned(&scope, &name).unwrap_or_else(|| {
                    panic!(
                        "No such variable, {}\n at {}",
                        name,
                        &self.0[value.span.clone()]
                    )
                });
                (
                    obj.unwrap_or_else(|e| {
                        panic!("Error: {} at {}", e, &self.0[value.span.clone()])
                    }),
                    vars,
                )
            }

            Def::Call(b) => {
                let (obj, args) = b.as_ref();
                let x = self.get_value(&obj, vars, scope);
                vars = x.1;
                let y = self.collect_args(&args, vars, scope);
                vars = y.1;
                scope.push(vars);
                let res = x.0.call(y.0, scope);
                vars = scope.pop().unwrap();
                (
                    res.unwrap_or_else(|e| {
                        panic!("Error: {} at {}", e, &self.0[value.span.clone()])
                    }),
                    vars,
                )
            }

            Def::Tuple(objs) => {
                let mut tup = Vec::new();
                for obj in objs {
                    let x = self.get_value(obj, vars, scope);
                    vars = x.1;
                    tup.push(x.0);
                }
                (Box::new(tup), vars)
            }

            Def::MoveClosure(p, c, args) => (
                Box::new(MovClos(
                    Code(c.to_string(), Some(Arc::clone(p))),
                    args.clone(),
                )),
                vars,
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
                    for (name, value) in names.iter().zip(values.into_iter()) {
                        vars = self.exec_set(name, value, vars, scope);
                    }
                }
            }
        }
        vars
    }

    fn parse_run(&mut self, mut vars: Vars, scope: &mut Vec<Vars>) -> (Box<dyn Object>, Vars) {
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
                    r = x.0.call(y.0, scope).unwrap();
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
