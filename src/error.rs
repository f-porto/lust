use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, PartialEq)]
pub enum LustError {
    UnexpectedChar(char),
}

impl Display for LustError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LustError::UnexpectedChar(char) => write!(f, "Unexpected char: `{char}`"),
        }
    }
}

impl Error for LustError {}
