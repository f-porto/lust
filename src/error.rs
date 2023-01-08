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

    NothingToParse,
    UnexpectedToken(String),
}

impl Display for LustError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "Unexpected char: `{char}`"),
            Self::UnfinishedString => write!(f, "Unfinished string"),
            Self::MissingCharacter => write!(f, "Missing something"),
            Self::MalformedNumber => write!(f, "Malformed number"),
            Self::NothingToParse => write!(f, "Nothing to parse"),
            Self::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}

impl Error for LustError {}
