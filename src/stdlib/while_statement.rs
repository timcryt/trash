use crate::core::*;

pub struct WhileStatement;

impl Object for WhileStatement {
    fn clone(&self) -> Box<dyn Object> {
        Box::new(WhileStatement)
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> Box<dyn Object> {
        let mut firstset = params.get("1").unwrap().to_tuple().into_iter();
        let condfunc = params.get("2").unwrap();
        let body = params.get("3").unwrap();
        loop {
            let first = firstset.next().unwrap();
            {
                let condset = Vars::from_vec(first.to_tuple());

                match Object::clone(condfunc.as_ref())
                    .call(condset, scope)
                    .to_string()
                    .as_str()
                {
                    "true" => (),
                    "false" => break Box::new(firstset.collect::<Vec<_>>()),
                    other => panic!("Expected true or fasle, found {}", other),
                }
            }
            let locset = Vars::from_vec(firstset.collect());

            firstset = Object::clone(body.as_ref())
                .call(locset, scope)
                .to_tuple()
                .into_iter();
        }
    }

    fn to_string(self: Box<Self>) -> String {
        "".to_string()
    }

    fn to_tuple(self: Box<Self>) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}
