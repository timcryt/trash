use crate::core::*;

pub struct WhileStatement;

impl Object for WhileStatement {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(WhileStatement)
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let mut firstset = params.get(&scope, "1").to_tuple().into_iter();
        let condfunc = params.get(&scope, "2");
        let body = params.get(&scope, "3"); 
        loop {
            let first = firstset.next().unwrap();
            {
                let mut condset = Vars::new(); 
                for (argname, argval) in first.to_tuple().into_iter().enumerate().map(|x| ((x.0 + 1).to_string(), x.1)) {
                    condset.add(argname, argval);
                }

                match Object::clone(condfunc.as_ref()).call(condset, scope).to_string().as_str() {
                    "true" => (),
                    "false" => break Box::new(firstset.collect::<Vec<_>>()),
                    other => panic!("Expected true or fasle, found {}", other),
                }
            }

            let locset = {
                let mut locset = Vars::new();
                for (argname, argval) in firstset.enumerate().map(|x| ((x.0 + 1).to_string(), x.1)) {
                    locset.add(argname, argval);
                }
                locset
            };

            firstset = Object::clone(body.as_ref()).call(locset, scope).to_tuple().into_iter();
        }

    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}