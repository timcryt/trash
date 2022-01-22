use std::str::FromStr;

use crate::core::*;

#[derive(Debug)]
pub struct ScannerGen;

impl Object for ScannerGen {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(ScannerGen))
    }

    fn call(self: Box<Self>, mut params: Vars, _scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1") {
            Some(obj) => Ok(Box::new(Scanner(obj))),
            None => Ok(self),
        }
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}

pub struct Scanner(Box<dyn Object>);

impl Scanner {
    fn next(
        self,
        scope: &mut Vec<Vars>,
    ) -> Result<(Box<Self>, String), Box<dyn std::error::Error>> {
        let t = self
            .0
            .call(Vars::from_vec(vec![Box::new("next".to_string())]), scope)?
            .to_tuple();
        if t.len() != 2 {
            Err(TrashError::UnexpectedType(
                "tuple with 2 elements".to_string(),
                Box::new(t).to_string(),
            )
            .into())
        } else {
            let mut t = t.into_iter();
            Ok((
                Box::new(Scanner(t.next().unwrap())),
                t.next().unwrap().to_string(),
            ))
        }
    }

    fn scan<T: Object + FromStr>(
        mut self: Box<Self>,
        scope: &mut Vec<Vars>,
    ) -> Result<(Box<Self>, Option<T>), Box<dyn std::error::Error>>
    where
        <T as FromStr>::Err: std::error::Error,
    {
        let c = loop {
            let t = self.next(scope)?;
            self = t.0;
            let t = t.1;
            match t.chars().next() {
                None => return Ok((self, None)),
                Some(c) if c.is_whitespace() => {}
                Some(c) => break c,
            }
        };

        let mut s = String::from(c);
        loop {
            let t = self.next(scope)?;
            self = t.0;
            let t = t.1;
            match t.chars().next() {
                Some(c) if c.is_whitespace() => break Ok((self, s.parse::<T>().ok())),
                None => break Ok((self, s.parse::<T>().ok())),
                Some(c) => s.push(c),
            }
        }
    }
}

impl Object for Scanner {
    fn clone(&self) -> error::TrashResult {
        Err(TrashError::LinearTypeCloning.into())
    }

    fn call(mut self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        match params.get("1").map(|x| x.to_string()) {
            Some(s) => match s.as_str() {
                "scan" => (),
                "unwrap" => return Ok(self.0),
                _ => return Err(TrashError::UnknownMethod(s).into()),
            },

            None => return Ok(self),
        }

        match params
            .get("2")
            .ok_or(TrashError::NotEnoughArgs(1, 0))?
            .to_string()
            .as_str()
        {
            x if x == "int" || x == "float" || x == "string" => {
                let r = match x {
                    "int" => {
                        let t = self.scan::<i64>(scope)?;
                        (t.0, t.1.map(|x| Box::new(x) as Box<dyn Object>))
                    }

                    "float" => {
                        let t = self.scan::<f64>(scope)?;
                        (t.0, t.1.map(|x| Box::new(x) as Box<dyn Object>))
                    }

                    "string" => {
                        let t = self.scan::<String>(scope)?;
                        (t.0, t.1.map(|x| Box::new(x) as Box<dyn Object>))
                    }

                    _ => unreachable!(),
                };
                self = r.0;
                let r = r.1;
                Ok(Box::new(vec![
                    self as Box<dyn Object>,
                    r.unwrap_or_else(|| Box::new("".to_string())),
                ]))
            }

            other => Err(TrashError::UnknownMethod(other.to_string()).into()),
        }
    }

    fn to_string(self: Box<Self>) -> String {
        self.0.to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        self.0.to_tuple()
    }
}
