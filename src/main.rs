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

struct Tokenize {
    code: String,
    index: usize,
}

impl Iterator for Tokenize {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.code.get(self.index..=self.index).map(str::to_string);
        if r.is_none() {
            self.index += 1;
        }
        r
    }
}

#[derive(Clone)]
struct Code(String);

impl Code {
    fn from_string(s: String) -> Code {
        Code(s)
    }

    fn run(self) -> Box<dyn Object> {
        todo!();
    }
}

impl Object for Code {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, _params: Vec<Box<dyn Object>>) -> Box<dyn Object> {
        todo!();
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
    println!("{}", Code::from_string(s).run().to_string());
}