use std::collections::HashMap;

trait Object {
    fn clone(&self) -> Box<dyn Object>;
    fn call(self: Box<Self>, params: Vec<Box<dyn Object>>) -> Box<dyn Object>;
    fn to_string(self: Box<Self>) -> String;
    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>>;
}

impl Object for String {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, _params: Vec<Box<dyn Object>>) -> Box<dyn Object> {
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

impl Code {
    fn from_string(s: String) -> Code {
        Code(s)
    }

    fn run(self, mut vars: HashMap<String, Box<dyn Object>>) -> Box<dyn Object> {
        let r = Box::new("".to_string());
        for line in self.0.trim().split(";") {
            let mut tokens = line.split(' ').filter(|x| x != &"");
            match tokens.next() {
                Some("$set") => {
                    let name = tokens.next().unwrap();
                    let content = tokens.next().unwrap();
                    match tokens.next() {
                        Some(_) => panic!(),
                        None => {
                            vars.insert(name.to_string(), Box::new(content.to_string()));
                        }
                    }
                }
                Some("$puts") => {
                    let name = tokens.next().unwrap();

                    if &name[0..=0] == "$" {
                        match vars.remove(&name[1..]) {
                            Some(var) => {
                                println!("{}", var.to_string());
                            }
                            None => {
                                panic!("No such variable {}", &name[1..]);
                            }
                        }
                    } else if &name[0..=0] == "@" {
                        match vars.get(&name[1..]) {
                            Some(var) => {
                                println!("{}", Object::clone(var.as_ref()).to_string())
                            }
                            None => {
                                panic!("No such variable {}", &name[1..]);
                            }
                        }
                    } else {
                        println!("{}", name);
                    }
                }
                Some(s) => {
                    panic!("No such command {}", s);
                }
                _ => {}
            }
        }
        r
    }
}

impl Object for Code {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, params: Vec<Box<dyn Object>>) -> Box<dyn Object> {
        self.run(
            params
                .into_iter()
                .enumerate()
                .map(|(n, x)| ((n + 1).to_string(), x))
                .collect(),
        )
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
    println!("{}", Code::from_string(s).run(HashMap::new()).to_string());
}
