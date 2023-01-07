use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, PartialEq)]
pub enum LustError {
    UnexpectedChar(char),
    UnfinishedString,
    MissingCharacter,
    MalformedNumber,
}

impl Display for LustError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "Unexpected char: `{char}`"),
            Self::UnfinishedString => write!(f, "Unfinished string"),
            Self::MissingCharacter => write!(f, "Missing something"),
            Self::MalformedNumber => write!(f, "Malformed number"),
        }
    }
}

impl Error for LustError {}
