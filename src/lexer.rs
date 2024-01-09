use std::{iter::Peekable, str::CharIndices};

use crate::{
    error::LustError,
    state_machine::{
        outer::{OuterState, OuterStateMachine},
        StateMachine,
    },
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    code: &'a str,
    chars: Peekable<CharIndices<'a>>,
    state_machine: OuterStateMachine,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            chars: code.char_indices().peekable(),
            state_machine: OuterStateMachine::new(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .chars
            .next_if(|(_, char)| char.is_whitespace())
            .is_some()
        {}
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LustError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.state_machine.reset();
        let start = self.chars.peek()?.0;
        let mut end = start;
        while let Some((_, char)) = self.chars.peek() {
            if !self.state_machine.next(*char) {
                break;
            }
            self.chars.next();
            end += 1;
        }

        let token = match self.state_machine.state() {
            OuterState::Plus => TokenKind::Plus,
            OuterState::Minus => TokenKind::Minus,
            OuterState::Asterisk => TokenKind::Asterisk,
            OuterState::Percent => TokenKind::Percent,
            OuterState::Circumflex => TokenKind::Circumflex,
            OuterState::Hash => TokenKind::Hash,
            OuterState::Ampersand => TokenKind::Ampersand,
            OuterState::Bar => TokenKind::Bar,
            OuterState::Semicolon => TokenKind::Semicolon,
            OuterState::Comma => TokenKind::Comma,
            OuterState::LeftParenthesis => TokenKind::LeftParenthesis,
            OuterState::RightParenthesis => TokenKind::RightParenthesis,
            OuterState::LeftBrace => TokenKind::LeftBrace,
            OuterState::RightBrace => TokenKind::RightBrace,
            OuterState::RightBracket => TokenKind::RightBracket,
            OuterState::LeftBracket => TokenKind::LeftBracket,
            OuterState::Slash => TokenKind::Slash,
            OuterState::Tilde => TokenKind::Tilde,
            OuterState::LessThan => TokenKind::LessThan,
            OuterState::GreaterThan => TokenKind::GreaterThan,
            OuterState::Equals => TokenKind::Assign,
            OuterState::Colon => TokenKind::Colon,
            OuterState::Dot => TokenKind::Dot,
            OuterState::DoubleDot => TokenKind::DoubleDot,
            OuterState::TripleDot => TokenKind::TripleDot,
            OuterState::DoubleEquals => TokenKind::Equals,
            OuterState::Different => TokenKind::Different,
            OuterState::GreaterThanOrEqual => TokenKind::GreaterThanOrEqual,
            OuterState::LessThanOrEqual => TokenKind::LessThanOrEqual,
            OuterState::LeftShift => TokenKind::LeftShift,
            OuterState::RightShift => TokenKind::RightShift,
            OuterState::DoubleColon => TokenKind::DoubleColon,
            OuterState::DoubleSlash => TokenKind::DoubleSlash,
            OuterState::Word => TokenKind::str_to_keyword(&self.code[start..end])
                .unwrap_or(TokenKind::Identifier(&self.code[start..end])),
            OuterState::Number(_) => TokenKind::Number(&self.code[start..end]),
            OuterState::String(machine) => {
                let skip = machine.skip();
                TokenKind::String(&self.code[(start + skip)..(end - skip)])
            }
            OuterState::Comment(_) => return self.next(),
            _ => return None,
        };
        Some(Ok(Token {
            start,
            end,
            lexeme: token,
        }))
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
                TokenKind::And,
                TokenKind::Break,
                TokenKind::Do,
                TokenKind::Else,
                TokenKind::Elseif,
                TokenKind::End,
                TokenKind::False,
                TokenKind::For,
                TokenKind::Function,
                TokenKind::Goto,
                TokenKind::If,
                TokenKind::In,
                TokenKind::Local,
                TokenKind::Nil,
                TokenKind::Not,
                TokenKind::Or,
                TokenKind::Repeat,
                TokenKind::Return,
                TokenKind::Then,
                TokenKind::True,
                TokenKind::Until,
                TokenKind::While,
            ],
        )
    }

    #[test]
    fn read_identifiers() -> Result<(), LustError> {
        compare(
            &read_lua_file("identifiers"),
            &[
                TokenKind::Identifier("a"),
                TokenKind::Identifier("A"),
                TokenKind::Identifier("_"),
                TokenKind::Identifier("_02312"),
                TokenKind::Identifier("_sdaf"),
                TokenKind::Identifier("_ASFDS"),
                TokenKind::Identifier("af_ads"),
                TokenKind::Identifier("ASD_DFSD"),
                TokenKind::Identifier("a03fDfsd_efwe839ruEEFwf43e_"),
            ],
        )
    }

    #[test]
    fn hexadecimal_numbers() -> Result<(), LustError> {
        compare(
            &read_lua_file("base_16_numbers"),
            &[
                TokenKind::Number("0xff"),
                TokenKind::Number("0xBEBADA"),
                TokenKind::Number("0x0.1E"),
                TokenKind::Number("0xA23p-4"),
                TokenKind::Number("0X1.921FB54442D18P+1"),
                TokenKind::Number("0x.23p-4"),
                TokenKind::Number("0x3234.p+3"),
                TokenKind::Number("0x4342p+45"),
            ],
        )
    }

    #[test]
    fn decimal_numbers() -> Result<(), LustError> {
        compare(
            &read_lua_file("base_10_numbers"),
            &[
                TokenKind::Number("3"),
                TokenKind::Number("345"),
                TokenKind::Number("3.0"),
                TokenKind::Number("3.1416"),
                TokenKind::Number("314.16e-2"),
                TokenKind::Number("0.31416E1"),
                TokenKind::Number("34e1"),
                TokenKind::Number("123e+43"),
                TokenKind::Number(".12e34"),
                TokenKind::Number("1.e34"),
            ],
        )
    }

    #[test]
    fn read_string() -> Result<(), LustError> {
        compare(
            &read_lua_file("strings"),
            &[
                TokenKind::String("s alo\\n123\""),
                TokenKind::String("d alo\\n123\\\""),
                TokenKind::String("\\97lo\\10\\04923\""),
                TokenKind::String("0 alo\n123\""),
                TokenKind::String("2\nalo\n123\""),
                TokenKind::String("sadf"),
                TokenKind::String("a]===]a"),
            ],
        )
    }

    #[test]
    fn single_chars() -> Result<(), LustError> {
        let singles = "+-/*^%&|~({[]})=<>#.,;:";

        let expected_tokens = [
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::Circumflex,
            TokenKind::Percent,
            TokenKind::Ampersand,
            TokenKind::Bar,
            TokenKind::Tilde,
            TokenKind::LeftParenthesis,
            TokenKind::LeftBrace,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::RightBrace,
            TokenKind::RightParenthesis,
            TokenKind::Assign,
            TokenKind::LessThan,
            TokenKind::GreaterThan,
            TokenKind::Hash,
            TokenKind::Dot,
            TokenKind::Comma,
            TokenKind::Semicolon,
            TokenKind::Colon,
        ];

        compare(&singles, &expected_tokens)
    }

    #[test]
    fn double_chars() -> Result<(), LustError> {
        let code = "<<>>==~=<=>=..:://";

        let expected_tokens = [
            TokenKind::LeftShift,
            TokenKind::RightShift,
            TokenKind::Equals,
            TokenKind::Different,
            TokenKind::LessThanOrEqual,
            TokenKind::GreaterThanOrEqual,
            TokenKind::DoubleDot,
            TokenKind::DoubleColon,
            TokenKind::DoubleSlash,
        ];

        compare(&code, &expected_tokens)
    }

    #[test]
    fn triple_chars() -> Result<(), LustError> {
        compare("...", &[TokenKind::TripleDot])
    }

    fn compare(code: &str, expected_tokens: &[TokenKind]) -> Result<(), LustError> {
        let mut lexer = Lexer::new(code);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let Some(actual_token) = lexer.next() else {
                panic!("{i}: Expected {expected_token:?} but got nothing");
            };

            assert_eq!((i, &actual_token?.lexeme), (i, expected_token));
        }
        assert_eq!(lexer.next(), None);

        Ok(())
    }

    #[test]
    fn comments() -> Result<(), LustError> {
        compare(
            &read_lua_file("comments"),
            &[
                TokenKind::Identifier("print0"),
                TokenKind::LeftParenthesis,
                TokenKind::String("This is fine"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("print1"),
                TokenKind::LeftParenthesis,
                TokenKind::String("This execute"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("print2"),
                TokenKind::LeftParenthesis,
                TokenKind::String("between block comments"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("print3"),
                TokenKind::LeftParenthesis,
                TokenKind::String("between comments"),
                TokenKind::RightParenthesis,
            ],
        )
    }

    #[test]
    fn read_actual_script() -> Result<(), LustError> {
        compare(
            &read_lua_file("html_generator"),
            &[
                // fwrite function
                TokenKind::Function,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("fmt"),
                TokenKind::Comma,
                TokenKind::TripleDot,
                TokenKind::RightParenthesis,
                TokenKind::Return,
                TokenKind::Identifier("io"),
                TokenKind::Dot,
                TokenKind::Identifier("write"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("string"),
                TokenKind::Dot,
                TokenKind::Identifier("format"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("fmt"),
                TokenKind::Comma,
                TokenKind::Identifier("unpack"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("arg"),
                TokenKind::RightParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::End,
                // BEGIN function
                TokenKind::Function,
                TokenKind::Identifier("BEGIN"),
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Identifier("io"),
                TokenKind::Dot,
                TokenKind::Identifier("write"),
                TokenKind::LeftParenthesis,
                TokenKind::String(
                    r##"
      <HTML>
      <HEAD><TITLE>Projects using Lua</TITLE></HEAD>
      <BODY BGCOLOR="#FFFFFF">
      Here are brief descriptions of some projects around the
      world that use <A HREF="home.html">Lua</A>.
      <BR>
    "##,
                ),
                TokenKind::RightParenthesis,
                TokenKind::End,
                // entry0 function
                TokenKind::Function,
                TokenKind::Identifier("entry0"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("o"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("N"),
                TokenKind::Assign,
                TokenKind::Identifier("N"),
                TokenKind::Plus,
                TokenKind::Number("1"),
                TokenKind::Local,
                TokenKind::Identifier("title"),
                TokenKind::Assign,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("title"),
                TokenKind::Or,
                TokenKind::String("(no title)"),
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<LI><A HREF=\"#%d\">%s</A>\\n"),
                TokenKind::Comma,
                TokenKind::Identifier("N"),
                TokenKind::Comma,
                TokenKind::Identifier("title"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                // entry1 function
                TokenKind::Function,
                TokenKind::Identifier("entry1"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("o"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("N"),
                TokenKind::Assign,
                TokenKind::Identifier("N"),
                TokenKind::Plus,
                TokenKind::Number("1"),
                TokenKind::Local,
                TokenKind::Identifier("title"),
                TokenKind::Assign,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("title"),
                TokenKind::Or,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("org"),
                TokenKind::Or,
                TokenKind::String("org"),
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<HR>\\n<H3>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::Local,
                TokenKind::Identifier("href"),
                TokenKind::Assign,
                TokenKind::String(""),
                // First if
                TokenKind::If,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("url"),
                TokenKind::Then,
                TokenKind::Identifier("href"),
                TokenKind::Assign,
                TokenKind::Identifier("string"),
                TokenKind::Dot,
                TokenKind::Identifier("format"),
                TokenKind::LeftParenthesis,
                TokenKind::String(" HREF=\"%s\""),
                TokenKind::Comma,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("url"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<A NAME=\"%d\"%s>%s</A>\\n"),
                TokenKind::Comma,
                TokenKind::Identifier("N"),
                TokenKind::Comma,
                TokenKind::Identifier("href"),
                TokenKind::Comma,
                TokenKind::Identifier("title"),
                TokenKind::RightParenthesis,
                // Second if
                TokenKind::If,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("title"),
                TokenKind::And,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("org"),
                TokenKind::Then,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<BR>\\n<SMALL><EM>%s</EM></SMALL>"),
                TokenKind::Comma,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("org"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("\\n</H3>\\n"),
                TokenKind::RightParenthesis,
                // Third if
                TokenKind::If,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("description"),
                TokenKind::Then,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("%s"),
                TokenKind::Comma,
                TokenKind::Identifier("string"),
                TokenKind::Dot,
                TokenKind::Identifier("gsub"),
                TokenKind::LeftParenthesis,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("description"),
                TokenKind::Comma,
                TokenKind::String("\\n\\n\\n*"),
                TokenKind::Comma,
                TokenKind::String("<P>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<P>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                // Fourth if
                TokenKind::If,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("email"),
                TokenKind::Then,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("Contact: <A HREF=\"mailto:%s\">%s</A>\\n"),
                TokenKind::Comma,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("email"),
                TokenKind::Comma,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("contact"),
                TokenKind::Or,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("email"),
                TokenKind::RightParenthesis,
                // Elseif
                TokenKind::Elseif,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("contact"),
                TokenKind::Then,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("Contact: %s\\n"),
                TokenKind::Comma,
                TokenKind::Identifier("o"),
                TokenKind::Dot,
                TokenKind::Identifier("contact"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                TokenKind::End,
                // END function
                TokenKind::Function,
                TokenKind::Identifier("END"),
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("</BODY></HTML>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::End,
                // main
                TokenKind::Identifier("BEGIN"),
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Identifier("N"),
                TokenKind::Assign,
                TokenKind::Number("0"),
                TokenKind::Identifier("entry"),
                TokenKind::Assign,
                TokenKind::Identifier("entry0"),
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("<UL>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("dofile"),
                TokenKind::LeftParenthesis,
                TokenKind::String("db.lua"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("fwrite"),
                TokenKind::LeftParenthesis,
                TokenKind::String("</UL>\\n"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("N"),
                TokenKind::Assign,
                TokenKind::Number("0"),
                TokenKind::Identifier("entry"),
                TokenKind::Assign,
                TokenKind::Identifier("entry1"),
                TokenKind::Identifier("dofile"),
                TokenKind::LeftParenthesis,
                TokenKind::String("db.lua"),
                TokenKind::RightParenthesis,
                TokenKind::Identifier("END"),
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
            ],
        )
    }
}
