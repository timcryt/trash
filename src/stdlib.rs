pub mod files;
pub mod if_statement;
pub mod integers;
pub mod while_statement;

use crate::core::Vars;

pub fn stdlib<T: std::io::Write + std::any::Any>(stdout: T) -> (Vars, Vec<Vars>) {
    let (mut s, mut v) = (Vars::new(), Vars::new());
    s.add("if".to_string(), Box::new(if_statement::IfStatement));
    s.add(
        "while".to_string(),
        Box::new(while_statement::WhileStatement),
    );
    s.add("int".to_string(), Box::new(integers::Int));
    v.add(
        "stdout".to_string(),
        Box::new(files::WriteStream::new(stdout)),
    );
    (v, vec![s])
}
