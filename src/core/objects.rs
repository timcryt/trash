use crate::core::*;

impl Object for String {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(mut self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> Box<dyn Object> {
        match params.get("1") {
            Some(method) => match &method.to_string()[..] {
                "len" => Box::new(self.len().to_string()),

                "_len" => {
                    let len = Box::new(self.len().to_string());
                    Box::new(vec![self as Box<dyn Object>, len])
                }

                "chars" => Box::new(
                    self.chars()
                        .map(|x| Box::new(x.to_string()) as Box<dyn Object>)
                        .collect::<Vec<_>>(),
                ),

                "split" => {
                    let delimiter = match params.get("2") {
                        Some(del) => del.to_string(),
                        _ => " ".to_string(),
                    };

                    Box::new(
                        self.split(&delimiter)
                            .map(|x| Box::new(x.to_string()) as Box<dyn Object>)
                            .collect::<Vec<_>>(),
                    )
                }

                "push" => {
                    let str_to_push = params
                        .get("2")
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| "".to_string());
                    self.push_str(&str_to_push);

                    self
                }

                "eq" => {
                    let str_to_compare = params
                        .get("2")
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    Box::new((*self == str_to_compare).to_string())
                }

                "_eq" => {
                    let str_to_compare = params
                        .get("2")
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));

                    let r = Box::new((*self == str_to_compare).to_string());

                    Box::new(vec![self as Box<dyn Object>, Box::new(str_to_compare), r])
                }

                other => {
                    panic!("Unknown method: {}", other);
                }
            },
            _ => self,
        }
    }

    fn to_string(self: Box<Self>) -> String {
        *self
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

impl Object for Vec<Box<dyn Object>> {
    fn clone(&self) -> Box<dyn Object> {
        let mut cloned = Vec::new();
        for el in self {
            cloned.push(Object::clone(el.as_ref()));
        }
        Box::new(cloned)
    }

    fn call(mut self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match &method[..] {
                "len" => {
                    Box::new(self.len() as i64)
                }

                "_len" => {
                    let len = Box::new(self.len() as i64);
                    Box::new(vec![self as Box<dyn Object>, len])
                }

                "push" => {
                    let val = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 1 argument, found 0"));
                    self.push(val);
                    self as Box<dyn Object>
                }

                "pop" => {
                    let el = self
                        .pop()
                        .unwrap_or_else(|| panic!("Can't pop value from empty tuple"));
                    Box::new(vec![self as Box<dyn Object>, el])
                }

                "is_empty" => Box::new(self.is_empty().to_string()),

                "_is_empty" => {
                    let r = Box::new(self.is_empty().to_string());
                    Box::new(vec![self as Box<dyn Object>, r])
                }

                "with" => {
                    let ind = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 2 arguments, found 0"))
                        .to_string()
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Expected number, found string"));
                    let clos = params
                        .get("3")
                        .unwrap_or_else(|| panic!("Expected 2 arguments, found 1"));
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(
                        self.get_mut(ind)
                            .unwrap_or_else(|| panic!("Index out of bounds")),
                        &mut t,
                    );
                    self[ind] = clos.call(Vars::from_vec(vec![t]), scope);
                    self
                }

                "without" => {
                    let ind = params
                        .get("2")
                        .unwrap_or_else(|| panic!("Expected 2 argumets, found 0"))
                        .to_string()
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Expected number, found 1"));
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(
                        self.get_mut(ind)
                            .unwrap_or_else(|| panic!("Index out of bounds")),
                        &mut t,
                    );
                    Box::new(vec![self as Box<dyn Object>, t])
                }

                other => {
                    let i = other
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Unknown method: {}", other));
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(
                        self.get_mut(i)
                            .unwrap_or_else(|| panic!("Index out of bounds")),
                        &mut t,
                    );
                    t
                }
            },
            None => self,
        }
    }

    fn to_string(self: Box<Self>) -> String {
        let mut s = "( ".to_string();
        for el in self.into_iter() {
            s += &el.to_string();
            s += " ";
        }
        s + ")"
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        *self
    }
}

impl Object for Code {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(Code(std::clone::Clone::clone(&self.0), self.1.clone()))
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

impl Object for MovClos {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let mut scope_vars = scope
            .pop()
            .unwrap_or_else(|| panic!("Can't move scope into closure"));
        for name in self.1 {
            let value = scope_vars
                .get(&name)
                .unwrap_or_else(|| panic!("No such variable {}", name));
            params.add(name, value);
        }
        scope.push(scope_vars);
        Box::new(self.0).call(params, scope)
    }

    fn to_string(self: Box<Self>) -> String {
        Box::new(self.0).to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Box::new(self.0).to_tuple()
    }
}
