#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    Number(&'a str),
    String(&'a str),

    And,
    Or,
    Not,

    If,
    Elseif,
    Else,

    For,
    Do,
    Until,
    While,
    Repeat,
    Goto,
    Break,

    Function,
    Return,

    Then,
    End,

    True,
    False,
    Nil,

    In,
    Local,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,

    Circumflex,
    Hash,

    Ampersand,
    Tilde,
    Bar,
    LeftShift,
    RightShift,

    DoubleSlash,

    Equals,
    Different,
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,

    Assign,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    DoubleColon,
    Semicolon,
    Colon,
    Comma,
    Dot,
    DoubleDot,
    TripleDot,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub start: usize,
    pub end: usize,
    pub lexeme: TokenKind<'a>,
}

impl<'a> TokenKind<'a> {
    pub fn str_to_keyword(code: &'a str) -> Option<TokenKind<'a>> {
        let keyword = match code {
            "and" => TokenKind::And,
            "break" => TokenKind::Break,
            "do" => TokenKind::Do,
            "else" => TokenKind::Else,
            "elseif" => TokenKind::Elseif,
            "end" => TokenKind::End,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "function" => TokenKind::Function,
            "goto" => TokenKind::Goto,
            "if" => TokenKind::If,
            "in" => TokenKind::In,
            "local" => TokenKind::Local,
            "nil" => TokenKind::Nil,
            "not" => TokenKind::Not,
            "or" => TokenKind::Or,
            "repeat" => TokenKind::Repeat,
            "return" => TokenKind::Return,
            "then" => TokenKind::Then,
            "true" => TokenKind::True,
            "until" => TokenKind::Until,
            "while" => TokenKind::While,
            _ => return None,
        };

        Some(keyword)
    }
}

// and       break     do        else      elseif    end
// false     for       function  goto      if        in
// local     nil       not       or        repeat    return
// then      true      until     while

// +     -     *     /     %     ^     #
// &     ~     |     <<    >>    //
// ==    ~=    <=    >=    <     >     =
// (     )     {     }     [     ]     ::
// ;     :     ,     .     ..    ...
