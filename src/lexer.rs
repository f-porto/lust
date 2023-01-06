use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::{error::LustError, tokens::Token};

pub struct Lexer<'a> {
    code: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            chars: code.chars().enumerate().peekable(),
        }
    }

    fn read_quoted_string(&mut self, quote: char) -> Result<usize, LustError> {
        let mut escaped = false;
        println!();
        while let Some(char) = self.chars.next() {
            print!("{:?}", char.1);
            match char {
                (_, '\\') => escaped = !escaped,
                (end, char) if char == quote => {
                    if !escaped {
                        return Ok(end);
                    } else {
                        escaped = false;
                    }
                }
                _ => escaped = false,
            }
        }

        Err(LustError::UnfinishedString)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LustError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self
            .chars
            .next_if(|(_, char)| char.is_whitespace())
            .is_some()
        {}

        let token = match self.chars.next()? {
            (_, '+') => Token::Plus,
            (_, '-') => Token::Minus,
            (_, '*') => Token::Asterisk,
            (_, '%') => Token::Percent,
            (_, '^') => Token::Circumflex,
            (_, '#') => Token::Hash,
            (_, '&') => Token::Ampersand,
            (_, '|') => Token::Bar,
            (_, ';') => Token::Semicolon,
            (_, ',') => Token::Comma,
            (_, '(') => Token::LeftParenthesis,
            (_, '[') => Token::LeftBracket,
            (_, '{') => Token::LeftBrace,
            (_, ')') => Token::RightParenthesis,
            (_, ']') => Token::RightBracket,
            (_, '}') => Token::RightBrace,
            (_, '/') => match self.chars.peek() {
                Some((_, '/')) => {
                    self.chars.next();
                    Token::DoubleBackSlash
                }
                _ => Token::BackSlash,
            },
            (_, '~') => match self.chars.peek() {
                Some((_, '=')) => {
                    self.chars.next();
                    Token::Different
                }
                _ => Token::Tilde,
            },
            (_, '>') => match self.chars.peek() {
                Some((_, '>')) => {
                    self.chars.next();
                    Token::RightShift
                }
                Some((_, '=')) => {
                    self.chars.next();
                    Token::GreaterThanOrEqual
                }
                _ => Token::GreaterThan,
            },
            (_, '<') => match self.chars.peek() {
                Some((_, '<')) => {
                    self.chars.next();
                    Token::LeftShift
                }
                Some((_, '=')) => {
                    self.chars.next();
                    Token::LessThanOrEqual
                }
                _ => Token::LessThan,
            },
            (_, '=') => match self.chars.peek() {
                Some((_, '=')) => {
                    self.chars.next();
                    Token::Equals
                }
                _ => Token::Assign,
            },
            (_, ':') => match self.chars.peek() {
                Some((_, ':')) => {
                    self.chars.next();
                    Token::DoubleColon
                }
                _ => Token::Colon,
            },
            (_, '.') => match self.chars.peek() {
                Some((_, '.')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some((_, '.')) => {
                            self.chars.next();
                            Token::TripleDot
                        }
                        _ => Token::DoubleDot,
                    }
                }
                _ => Token::Dot,
            },
            (start, quote @ ('\'' | '"')) => match self.read_quoted_string(quote) {
                Ok(end) => Token::String(&self.code[(start + 1)..end]),
                Err(why) => return Some(Err(why)),
            },
            (_, char) => return Some(Err(LustError::UnexpectedChar(char))),
        };

        Some(Ok(token))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn read_string() -> Result<(), LustError> {
        let path = Path::new("./lua/strings.lua");
        let content = read_to_string(path).expect("Should read ./lua/strings.lua should exists");

        compare(
            &content,
            &[
                Token::String("s alo\\n123\""),
                Token::String("d alo\\n123\\\""),
                Token::String("\\97lo\\10\\04923\""),
            ],
        )?;

        Ok(())
    }

    #[test]
    fn single_chars() -> Result<(), LustError> {
        let singles = "+-/*^%&|~({[]})=<>#.,;:";

        let expected_tokens = [
            Token::Plus,
            Token::Minus,
            Token::BackSlash,
            Token::Asterisk,
            Token::Circumflex,
            Token::Percent,
            Token::Ampersand,
            Token::Bar,
            Token::Tilde,
            Token::LeftParenthesis,
            Token::LeftBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::RightBrace,
            Token::RightParenthesis,
            Token::Assign,
            Token::LessThan,
            Token::GreaterThan,
            Token::Hash,
            Token::Dot,
            Token::Comma,
            Token::Semicolon,
            Token::Colon,
        ];

        compare(&singles, &expected_tokens)
    }

    #[test]
    fn double_chars() -> Result<(), LustError> {
        let code = "<<>>==~=<=>=..:://";

        let expected_tokens = [
            Token::LeftShift,
            Token::RightShift,
            Token::Equals,
            Token::Different,
            Token::LessThanOrEqual,
            Token::GreaterThanOrEqual,
            Token::DoubleDot,
            Token::DoubleColon,
            Token::DoubleBackSlash,
        ];

        compare(&code, &expected_tokens)
    }

    #[test]
    fn triple_chars() -> Result<(), LustError> {
        compare("...", &[Token::TripleDot])
    }

    fn compare(code: &str, expected_tokens: &[Token]) -> Result<(), LustError> {
        let mut lexer = Lexer::new(&code);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let Some(actual_token) = lexer.next() else {
                panic!("{i}: Expected {expected_token:?} but got nothing");
            };

            assert_eq!((i, expected_token), (i, &actual_token?));
        }
        assert_eq!(None, lexer.next());

        Ok(())
    }
}
