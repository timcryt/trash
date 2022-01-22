use crate::core::*;

pub struct WhileStatement;

impl Object for WhileStatement {
    fn clone(&self) -> error::TrashResult {
        Ok(Box::new(WhileStatement))
    }

    fn call(self: Box<Self>, mut params: Vars, scope: &mut Vec<Vars>) -> error::TrashResult {
        let mut firstset = params
            .get("1")
            .ok_or(TrashError::NotEnoughArgs(0, 3))?
            .to_tuple()
            .into_iter();
        let condfunc = params.get("2").ok_or(TrashError::NotEnoughArgs(1, 3))?;
        let body = params.get("3").ok_or(TrashError::NotEnoughArgs(2, 3))?;
        loop {
            let first = firstset.next().ok_or(TrashError::OutOfBounds)?;
            {
                let condset = Vars::from_vec(first.to_tuple());

                match crate::stdlib::bool::Bool::bool_res(
                    Object::clone(condfunc.as_ref())?.call(condset, scope)?,
                ) {
                    Ok(true) => (),
                    Ok(false) => break Ok(Box::new(firstset.collect::<Vec<_>>())),
                    Err(other) => {
                        return Err(TrashError::UnexpectedType(
                            "boolean".to_owned(),
                            other.to_owned(),
                        )
                        .into())
                    }
                }
            }
            let locset = Vars::from_vec(firstset.collect());

            firstset = Object::clone(body.as_ref())?
                .call(locset, scope)?
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
