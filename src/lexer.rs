use std::{iter::Peekable, str::CharIndices};

use crate::{
    error::LustError,
    state_machine::{
        outer::{OuterState, OuterStateMachine},
        StateMachine,
    },
    token::Token,
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
            OuterState::Plus => Token::Plus,
            OuterState::Minus => Token::Minus,
            OuterState::Asterisk => Token::Asterisk,
            OuterState::Percent => Token::Percent,
            OuterState::Circumflex => Token::Circumflex,
            OuterState::Hash => Token::Hash,
            OuterState::Ampersand => Token::Ampersand,
            OuterState::Bar => Token::Bar,
            OuterState::Semicolon => Token::Semicolon,
            OuterState::Comma => Token::Comma,
            OuterState::LeftParenthesis => Token::LeftParenthesis,
            OuterState::RightParenthesis => Token::RightParenthesis,
            OuterState::LeftBrace => Token::LeftBrace,
            OuterState::RightBrace => Token::RightBrace,
            OuterState::RightBracket => Token::RightBracket,
            OuterState::LeftBracket => Token::LeftBracket,
            OuterState::Slash => Token::Slash,
            OuterState::Tilde => Token::Tilde,
            OuterState::LessThan => Token::LessThan,
            OuterState::GreaterThan => Token::GreaterThan,
            OuterState::Equals => Token::Assign,
            OuterState::Colon => Token::Colon,
            OuterState::Dot => Token::Dot,
            OuterState::DoubleDot => Token::DoubleDot,
            OuterState::TripleDot => Token::TripleDot,
            OuterState::DoubleEquals => Token::Equals,
            OuterState::Different => Token::Different,
            OuterState::GreaterThanOrEqual => Token::GreaterThanOrEqual,
            OuterState::LessThanOrEqual => Token::LessThanOrEqual,
            OuterState::LeftShift => Token::LeftShift,
            OuterState::RightShift => Token::RightShift,
            OuterState::DoubleColon => Token::DoubleColon,
            OuterState::DoubleSlash => Token::DoubleSlash,
            OuterState::Word => Token::str_to_keyword(&self.code[start..end])
                .unwrap_or(Token::Identifier(&self.code[start..end])),
            OuterState::Number(_) => Token::Number(&self.code[start..end]),
            OuterState::String(machine) => {
                let skip = machine.skip();
                Token::String(&self.code[(start + skip)..(end - skip)])
            }
            OuterState::Comment(_) => return self.next(),
            _ => return None,
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
                Token::Number("0x4342p+45"),
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
                Token::String("sadf"),
                Token::String("a]===]a"),
            ],
        )
    }

    #[test]
    fn single_chars() -> Result<(), LustError> {
        let singles = "+-/*^%&|~({[]})=<>#.,;:";

        let expected_tokens = [
            Token::Plus,
            Token::Minus,
            Token::Slash,
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
            Token::DoubleSlash,
        ];

        compare(&code, &expected_tokens)
    }

    #[test]
    fn triple_chars() -> Result<(), LustError> {
        compare("...", &[Token::TripleDot])
    }

    fn compare(code: &str, expected_tokens: &[Token]) -> Result<(), LustError> {
        let mut lexer = Lexer::new(code);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let Some(actual_token) = lexer.next() else {
                panic!("{i}: Expected {expected_token:?} but got nothing");
            };

            assert_eq!((i, &actual_token?), (i, expected_token));
        }
        assert_eq!(lexer.next(), None);

        Ok(())
    }

    #[test]
    fn comments() -> Result<(), LustError> {
        compare(
            &read_lua_file("comments"),
            &[
                Token::Identifier("print0"),
                Token::LeftParenthesis,
                Token::String("This is fine"),
                Token::RightParenthesis,
                Token::Identifier("print1"),
                Token::LeftParenthesis,
                Token::String("This execute"),
                Token::RightParenthesis,
                Token::Identifier("print2"),
                Token::LeftParenthesis,
                Token::String("between block comments"),
                Token::RightParenthesis,
                Token::Identifier("print3"),
                Token::LeftParenthesis,
                Token::String("between comments"),
                Token::RightParenthesis,
            ],
        )
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
