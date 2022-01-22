use crate::core::*;
use dyn_fmt::AsStrFormatExt;

impl Object for String {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(std::clone::Clone::clone(self)))
    }

    fn call(mut self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match method.as_str() {
                "len" => Ok(Box::new(self.len().to_string())),

                "_len" => {
                    let len = Box::new(self.len().to_string());
                    Ok(Box::new(vec![self as Box<dyn Object>, len]))
                }

                "chars" => Ok(Box::new(
                    self.chars()
                        .map(|x| Box::new(x.to_string()) as Box<dyn Object>)
                        .collect::<Vec<_>>(),
                )),

                "split" => {
                    let delimiter = match params.get("2") {
                        Some(del) => del.to_string(),
                        _ => " ".to_string(),
                    };

                    Ok(Box::new(
                        self.split(&delimiter)
                            .map(|x| Box::new(x.to_string()) as Box<dyn Object>)
                            .collect::<Vec<_>>(),
                    ))
                }

                "push" => {
                    let str_to_push = params
                        .get("2")
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| "".to_string());
                    self.push_str(&str_to_push);

                    Ok(self)
                }

                "eq" => {
                    let str_to_compare = params
                        .get("2")
                        .map(|x| x.to_string())
                        .ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    Ok(Box::new(*self == str_to_compare))
                }

                "_eq" => {
                    let str_to_compare = params
                        .get("2")
                        .map(|x| x.to_string())
                        .ok_or(TrashError::NotEnoughArgs(0, 1))?;

                    let r = Box::new(*self == str_to_compare);

                    Ok(Box::new(vec![
                        self as Box<dyn Object>,
                        Box::new(str_to_compare),
                        r,
                    ]))
                }

                "format" => Ok(Box::new(
                    self.to_string().format(
                        &(2..)
                            .map(|x| x.to_string())
                            .map(|x| params.get(&x).map(|x| x.to_string()))
                            .take_while(|x| x.is_some())
                            .map(|x| x.unwrap())
                            .collect::<Vec<_>>(),
                    ),
                )),

                _ => Err(TrashError::UnknownMethod(method).into()),
            },
            _ => Ok(self),
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
    fn clone(&self) -> error::TrashResult {
        let mut cloned = Vec::new();
        for el in self {
            cloned.push(Object::clone(el.as_ref())?);
        }
        Ok(Box::new(cloned))
    }

    fn call(mut self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(method) => match &method[..] {
                "len" => Ok(Box::new(self.len() as i64)),

                "_len" => {
                    let len = Box::new(self.len() as i64);
                    Ok(Box::new(vec![self as Box<dyn Object>, len]))
                }

                "push" => {
                    let val = params.get("2").ok_or(TrashError::NotEnoughArgs(0, 1))?;
                    self.push(val);
                    Ok(self as Box<dyn Object>)
                }

                "pop" => {
                    let el = self.pop().ok_or_else(|| {
                        TrashError::Custom("Can't pop value from empty tuple".to_string())
                    })?;
                    Ok(Box::new(vec![self as Box<dyn Object>, el]))
                }

                "is_empty" => Ok(Box::new(self.is_empty())),

                "_is_empty" => {
                    let r = Box::new(self.is_empty());
                    Ok(Box::new(vec![self as Box<dyn Object>, r]))
                }

                "with" => {
                    let ind_str = params
                        .get("2")
                        .ok_or(TrashError::NotEnoughArgs(0, 2))?
                        .to_string();

                    let ind = ind_str
                        .parse::<isize>()
                        .map_err(|_| TrashError::UnexpectedType("integer".to_owned(), ind_str))?;

                    let ind = if ind < 0 {
                        self.len() as isize + ind
                    } else {
                        ind
                    } as usize;

                    let clos = params.get("3").ok_or(TrashError::NotEnoughArgs(1, 2))?;
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(self.get_mut(ind).ok_or(TrashError::OutOfBounds)?, &mut t);
                    self[ind] = clos.call(Vars::from_vec(vec![t]), scope)?;
                    Ok(self)
                }

                "without" => {
                    let ind_str = params
                        .get("2")
                        .ok_or(TrashError::NotEnoughArgs(0, 2))?
                        .to_string();

                    let ind = ind_str
                        .parse::<isize>()
                        .map_err(|_| TrashError::UnexpectedType("integer".to_owned(), ind_str))?;

                    let ind = if ind < 0 {
                        self.len() as isize + ind
                    } else {
                        ind
                    } as usize;

                    let clos = params.get("3").ok_or(TrashError::NotEnoughArgs(1, 2))?;
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(self.get_mut(ind).ok_or(TrashError::OutOfBounds)?, &mut t);
                    let mut res = clos.call(Vars::from_vec(vec![t]), scope)?.to_tuple();
                    self[ind] = res.pop().ok_or_else(|| {
                        TrashError::Custom("Expected tuple with at least 2 elements".to_string())
                    })?;
                    Ok(Box::new(vec![
                        self as Box<dyn Object>,
                        res.pop().ok_or_else(|| {
                            TrashError::Custom(
                                "Expected tuple with at least 2 elements".to_string(),
                            )
                        })?,
                    ]))
                }

                other => {
                    let i = other
                        .parse::<usize>()
                        .map_err(|_| TrashError::UnknownMethod(other.to_string()))?;
                    let mut t = Box::new("".to_string()) as Box<dyn Object>;
                    std::mem::swap(self.get_mut(i).ok_or(TrashError::OutOfBounds)?, &mut t);
                    Ok(t)
                }
            },
            None => Ok(self),
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
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(Code(
            std::clone::Clone::clone(&self.0),
            self.1.clone(),
        )))
    }

    fn call(self: Box<Self>, params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        Ok(self.run(params, scope))
    }

    fn to_string(self: Box<Self>) -> String {
        self.0
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        vec![self]
    }
}

impl Object for MovClos {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(std::clone::Clone::clone(self)))
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        let mut scope_vars = scope
            .pop()
            .ok_or_else(|| TrashError::Custom("Can't move variables form scope".to_string()))?;
        for name in self.1 {
            let value = scope_vars
                .get(&name)
                .ok_or_else(|| TrashError::Custom(format!("No such variable {}", name)))?;
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
