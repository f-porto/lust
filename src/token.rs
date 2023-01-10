#[derive(Debug, PartialEq)]
pub enum Token<'a> {
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

impl<'a> Token<'a> {
    pub fn str_to_keyword(code: &'a str) -> Option<Token<'a>> {
        let keyword = match code {
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "end" => Token::End,
            "false" => Token::False,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "if" => Token::If,
            "in" => Token::In,
            "local" => Token::Local,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::True,
            "until" => Token::Until,
            "while" => Token::While,
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
