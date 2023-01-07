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

    fn read_hexadecimal(&mut self, start: usize) -> Result<&'a str, LustError> {
        while let Some((e, char)) = self.chars.peek() {
            match char {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    self.chars.next();
                }
                '.' => {
                    self.chars.next();
                    break;
                }
                'p' | 'P' => {
                    break;
                }
                _ => return Ok(&self.code[start..*e]),
            };
        }

        while let Some((e, char)) = self.chars.peek() {
            match char {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    self.chars.next();
                }
                'p' | 'P' => {
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
            Some((e, '0'..='9' | 'a'..='f' | 'A'..='F')) => {
                end = *e;
                self.chars.next();
            }
            _ => return Err(LustError::MalformedNumber),
        };

        while let Some((e, '0'..='9' | 'a'..='f' | 'A'..='F')) = self.chars.peek() {
            end = *e;
            self.chars.next();
        }

        Ok(&self.code[start..=end])
    }

    fn read_identifier(&mut self, start: usize) -> Result<&'a str, LustError> {
        let mut end = start;
        while let Some((e, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')) = self.chars.peek() {
            end = *e;
            self.chars.next();
        }

        Ok(&self.code[start..=end])
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
                    match self.read_hexadecimal(start) {
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
            (start, 'a'..='z' | 'A'..='Z' | '_') => match self.read_identifier(start) {
                Ok(str) => Token::str_to_keyword(str).unwrap_or(Token::Identifier(str)),
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

    fn read_lua_file(filename: &str) -> String {
        let path = format!("./lua/{}.lua", filename);
        let path = Path::new(&path);
        read_to_string(path).expect(&format!("Should read ./lua/{}.lua", filename))
    }

    #[test]
    fn read_keywords() -> Result<(), LustError> {
        compare(
            &read_lua_file("keywords"),
            &[
                Token::And,
                Token::Break,
                Token::Do,
                Token::Else,
                Token::Elseif,
                Token::End,
                Token::False,
                Token::For,
                Token::Function,
                Token::Goto,
                Token::If,
                Token::In,
                Token::Local,
                Token::Nil,
                Token::Not,
                Token::Or,
                Token::Repeat,
                Token::Return,
                Token::Then,
                Token::True,
                Token::Until,
                Token::While,
            ],
        )
    }

    #[test]
    fn read_identifiers() -> Result<(), LustError> {
        compare(
            &read_lua_file("identifiers"),
            &[
                Token::Identifier("a"),
                Token::Identifier("A"),
                Token::Identifier("_"),
                Token::Identifier("_02312"),
                Token::Identifier("_sdaf"),
                Token::Identifier("_ASFDS"),
                Token::Identifier("af_ads"),
                Token::Identifier("ASD_DFSD"),
                Token::Identifier("a03fDfsd_efwe839ruEEFwf43e_"),
            ],
        )
    }

    #[test]
    fn hexadecimal_numbers() -> Result<(), LustError> {
        compare(
            &read_lua_file("base_16_numbers"),
            &[
                Token::Number("0xff"),
                Token::Number("0xBEBADA"),
                Token::Number("0x0.1E"),
                Token::Number("0xA23p-4"),
                Token::Number("0X1.921FB54442D18P+1"),
                Token::Number("0x.23p-4"),
                Token::Number("0x3234.p+3"),
            ],
        )
    }

    #[test]
    fn decimal_numbers() -> Result<(), LustError> {
        compare(
            &read_lua_file("base_10_numbers"),
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
        compare(
            &read_lua_file("strings"),
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

    #[test]
    fn read_actual_script() -> Result<(), LustError> {
        compare(
            &read_lua_file("html_generator"),
            &[
                // fwrite function
                Token::Function,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::Identifier("fmt"),
                Token::Comma,
                Token::TripleDot,
                Token::RightParenthesis,
                Token::Return,
                Token::Identifier("io"),
                Token::Dot,
                Token::Identifier("write"),
                Token::LeftParenthesis,
                Token::Identifier("string"),
                Token::Dot,
                Token::Identifier("format"),
                Token::LeftParenthesis,
                Token::Identifier("fmt"),
                Token::Comma,
                Token::Identifier("unpack"),
                Token::LeftParenthesis,
                Token::Identifier("arg"),
                Token::RightParenthesis,
                Token::RightParenthesis,
                Token::RightParenthesis,
                Token::End,
                // BEGIN function
                Token::Function,
                Token::Identifier("BEGIN"),
                Token::LeftParenthesis,
                Token::RightParenthesis,
                Token::Identifier("io"),
                Token::Dot,
                Token::Identifier("write"),
                Token::LeftParenthesis,
                Token::String(
                    r##"
      <HTML>
      <HEAD><TITLE>Projects using Lua</TITLE></HEAD>
      <BODY BGCOLOR="#FFFFFF">
      Here are brief descriptions of some projects around the
      world that use <A HREF="home.html">Lua</A>.
      <BR>
    "##,
                ),
                Token::RightParenthesis,
                Token::End,
                // entry0 function
                Token::Function,
                Token::Identifier("entry0"),
                Token::LeftParenthesis,
                Token::Identifier("o"),
                Token::RightParenthesis,
                Token::Identifier("N"),
                Token::Assign,
                Token::Identifier("N"),
                Token::Plus,
                Token::Number("1"),
                Token::Local,
                Token::Identifier("title"),
                Token::Assign,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("title"),
                Token::Or,
                Token::String("(no title)"),
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<LI><A HREF=\"#%d\">%s</A>\\n"),
                Token::Comma,
                Token::Identifier("N"),
                Token::Comma,
                Token::Identifier("title"),
                Token::RightParenthesis,
                Token::End,
                // entry1 function
                Token::Function,
                Token::Identifier("entry1"),
                Token::LeftParenthesis,
                Token::Identifier("o"),
                Token::RightParenthesis,
                Token::Identifier("N"),
                Token::Assign,
                Token::Identifier("N"),
                Token::Plus,
                Token::Number("1"),
                Token::Local,
                Token::Identifier("title"),
                Token::Assign,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("title"),
                Token::Or,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("org"),
                Token::Or,
                Token::String("org"),
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<HR>\\n<H3>\\n"),
                Token::RightParenthesis,
                Token::Local,
                Token::Identifier("href"),
                Token::Assign,
                Token::String(""),
                // First if
                Token::If,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("url"),
                Token::Then,
                Token::Identifier("href"),
                Token::Assign,
                Token::Identifier("string"),
                Token::Dot,
                Token::Identifier("format"),
                Token::LeftParenthesis,
                Token::String(" HREF=\"%s\""),
                Token::Comma,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("url"),
                Token::RightParenthesis,
                Token::End,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<A NAME=\"%d\"%s>%s</A>\\n"),
                Token::Comma,
                Token::Identifier("N"),
                Token::Comma,
                Token::Identifier("href"),
                Token::Comma,
                Token::Identifier("title"),
                Token::RightParenthesis,
                // Second if
                Token::If,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("title"),
                Token::And,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("org"),
                Token::Then,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<BR>\\n<SMALL><EM>%s</EM></SMALL>"),
                Token::Comma,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("org"),
                Token::RightParenthesis,
                Token::End,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("\\n</H3>\\n"),
                Token::RightParenthesis,
                // Third if
                Token::If,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("description"),
                Token::Then,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("%s"),
                Token::Comma,
                Token::Identifier("string"),
                Token::Dot,
                Token::Identifier("gsub"),
                Token::LeftParenthesis,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("description"),
                Token::Comma,
                Token::String("\\n\\n\\n*"),
                Token::Comma,
                Token::String("<P>\\n"),
                Token::RightParenthesis,
                Token::RightParenthesis,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<P>\\n"),
                Token::RightParenthesis,
                Token::End,
                // Fourth if
                Token::If,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("email"),
                Token::Then,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("Contact: <A HREF=\"mailto:%s\">%s</A>\\n"),
                Token::Comma,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("email"),
                Token::Comma,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("contact"),
                Token::Or,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("email"),
                Token::RightParenthesis,
                // Elseif
                Token::Elseif,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("contact"),
                Token::Then,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("Contact: %s\\n"),
                Token::Comma,
                Token::Identifier("o"),
                Token::Dot,
                Token::Identifier("contact"),
                Token::RightParenthesis,
                Token::End,
                Token::End,
                // END function
                Token::Function,
                Token::Identifier("END"),
                Token::LeftParenthesis,
                Token::RightParenthesis,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("</BODY></HTML>\\n"),
                Token::RightParenthesis,
                Token::End,
                // main
                Token::Identifier("BEGIN"),
                Token::LeftParenthesis,
                Token::RightParenthesis,
                Token::Identifier("N"),
                Token::Assign,
                Token::Number("0"),
                Token::Identifier("entry"),
                Token::Assign,
                Token::Identifier("entry0"),
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("<UL>\\n"),
                Token::RightParenthesis,
                Token::Identifier("dofile"),
                Token::LeftParenthesis,
                Token::String("db.lua"),
                Token::RightParenthesis,
                Token::Identifier("fwrite"),
                Token::LeftParenthesis,
                Token::String("</UL>\\n"),
                Token::RightParenthesis,
                Token::Identifier("N"),
                Token::Assign,
                Token::Number("0"),
                Token::Identifier("entry"),
                Token::Assign,
                Token::Identifier("entry1"),
                Token::Identifier("dofile"),
                Token::LeftParenthesis,
                Token::String("db.lua"),
                Token::RightParenthesis,
                Token::Identifier("END"),
                Token::LeftParenthesis,
                Token::RightParenthesis,
            ],
        )
    }
}
