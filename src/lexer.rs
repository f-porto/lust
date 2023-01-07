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

    fn read_quoted_string(&mut self, start: usize, quote: char) -> Result<&'a str, LustError> {
        let mut escaped = false;
        while let Some(char) = self.chars.next() {
            match char {
                (_, '\\') => escaped = !escaped,
                (end, char) if char == quote => {
                    if !escaped {
                        return Ok(&self.code[start..end]);
                    } else {
                        escaped = false;
                    }
                }
                _ => escaped = false,
            }
        }

        Err(LustError::UnfinishedString)
    }

    fn read_raw_string(&mut self) -> Result<&'a str, LustError> {
        let mut equals = 0usize;
        while self.chars.next_if(|(_, char)| *char == '=').is_some() {
            equals += 1;
        }

        let start;
        match self.chars.next() {
            Some((pos, '[')) => start = pos + 1,
            Some((_, char)) => return Err(LustError::UnexpectedChar(char)),
            None => return Err(LustError::MissingCharacter),
        };

        while let Some((end, char)) = self.chars.next() {
            if char == ']' {
                let mut end_equals = 0usize;
                while self.chars.next_if(|(_, char)| *char == '=').is_some() {
                    end_equals += 1;
                }
                match self.chars.next() {
                    Some((_, ']')) if equals == end_equals => return Ok(&self.code[start..end]),
                    _ => {}
                };
            }
        }

        Err(LustError::UnfinishedString)
    }

    fn read_number(
        &mut self,
        start: usize,
        started_with_digit: bool,
    ) -> Result<&'a str, LustError> {
        if started_with_digit {
            while let Some((e, char)) = self.chars.peek() {
                match char {
                    '0'..='9' => {
                        self.chars.next();
                    }
                    '.' => {
                        self.chars.next();
                        break;
                    }
                    'e' | 'E' => {
                        break;
                    }
                    _ => return Ok(&self.code[start..*e]),
                };
            }
        } else {
            let Some((_, '0'..='9')) = self.chars.next() else {
                return Err(LustError::MalformedNumber);
            };
        }

        while let Some((e, char)) = self.chars.peek() {
            match char {
                '0'..='9' => {
                    self.chars.next();
                }
                'e' | 'E' => {
                    self.chars.next();
                    break;
                }
                _ => return Ok(&self.code[start..*e]),
            };
        }

        match self.chars.peek() {
            Some((_, '-' | '+')) => {
                self.chars.next();
            }
            _ => {}
        };

        let mut end;
        match self.chars.peek() {
            Some((e, '0'..='9')) => {
                end = *e;
                self.chars.next();
            }
            _ => return Err(LustError::MalformedNumber),
        };

        while let Some((e, _)) = self.chars.next_if(|(_, char)| ('0'..='9').contains(char)) {
            end = e;
        }

        Ok(&self.code[start..=end])
    }

    fn read_hexadecimal(&mut self) -> Result<&'a str, LustError> {
        Ok("")
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
            (_, ')') => Token::RightParenthesis,
            (_, '{') => Token::LeftBrace,
            (_, '}') => Token::RightBrace,
            (_, ']') => Token::RightBracket,
            (_, '[') => match self.chars.peek() {
                Some((_, '[' | '=')) => match self.read_raw_string() {
                    Ok(str) => Token::String(str),
                    Err(why) => return Some(Err(why)),
                },
                _ => Token::LeftBracket,
            },
            (start, '0') => match self.chars.peek() {
                Some((_, 'x' | 'X')) => {
                    self.chars.next();
                    match self.read_hexadecimal() {
                        Ok(str) => Token::Number(str),
                        Err(why) => return Some(Err(why)),
                    }
                }
                _ => match self.read_number(start, true) {
                    Ok(str) => Token::Number(str),
                    Err(why) => return Some(Err(why)),
                },
            },
            (start, '1'..='9') => match self.read_number(start, true) {
                Ok(str) => Token::Number(str),
                Err(why) => return Some(Err(why)),
            },
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
            (start, '.') => match self.chars.peek() {
                Some((_, '0'..='9')) => match self.read_number(start, false) {
                    Ok(str) => Token::Number(str),
                    Err(why) => return Some(Err(why)),
                },
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
            (start, quote @ ('\'' | '"')) => match self.read_quoted_string(start + 1, quote) {
                Ok(str) => Token::String(str),
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
    fn decimal_numbers() -> Result<(), LustError> {
        let path = Path::new("./lua/base_10_numbers.lua");
        let content = read_to_string(path).expect("Should read ./lua/base_10_numbers.lua");

        compare(
            &content,
            &[
                Token::Number("3"),
                Token::Number("345"),
                Token::Number("3.0"),
                Token::Number("3.1416"),
                Token::Number("314.16e-2"),
                Token::Number("0.31416E1"),
                Token::Number("34e1"),
                Token::Number("123e+43"),
                Token::Number(".12e34"),
                Token::Number("1.e34"),
            ],
        )
    }

    #[test]
    fn read_string() -> Result<(), LustError> {
        let path = Path::new("./lua/strings.lua");
        let content = read_to_string(path).expect("Should read ./lua/strings.lua");

        compare(
            &content,
            &[
                Token::String("s alo\\n123\""),
                Token::String("d alo\\n123\\\""),
                Token::String("\\97lo\\10\\04923\""),
                Token::String("0 alo\n123\""),
                Token::String("2\nalo\n123\""),
            ],
        )
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
