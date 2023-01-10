use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

// TODO: Those are terrible errors, make it good
#[derive(Debug, Clone, PartialEq)]
pub enum LustError {
    UnexpectedChar(char),
    UnfinishedString,
    MissingCharacter,
    MalformedNumber,

    NothingToParse,
    UnexpectedToken(String),
    NotAStatement,

    ExpectedButGotNothing(String),
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
            Self::NotAStatement => write!(f, "Not a statement"),
            Self::ExpectedButGotNothing(msg) => write!(f, "Expected `{}` but got nothing", msg),
        }
    }
}

impl Error for LustError {}
