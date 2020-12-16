use std::{error::Error, fmt, fmt::Display};

pub type TrashResult = Result<Box<dyn super::Object>, Box<dyn Error>>;

#[derive(Debug)]
pub enum TrashError {
    UnknownMethod(String),
    NotEnoughArgs(usize, usize),
    UnexpectedType(String, String),
    OutOfBounds,
    LinearTypeCloning,
    Custom(String),
}

impl Display for TrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrashError::UnknownMethod(name) => {
                writeln!(f, "Unknown method: {}", name)
            }

            TrashError::NotEnoughArgs(pres, exp) => {
                writeln!(f, "Expected {} arguments, found {}", pres, exp)
            }

            TrashError::UnexpectedType(exp, val) => {
                writeln!(f, "Expected value of type {}, found value {}", exp, val)
            }

            TrashError::OutOfBounds => {
                writeln!(f, "Index out of bounds")
            }

            TrashError::LinearTypeCloning => {
                writeln!(f, "Can't clone value of linear type")
            }

            TrashError::Custom(s) => {
                writeln!(f, "{}", s)
            }
        }
    }
}

impl Error for TrashError {}
