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
    BackSlash,
    Percent,

    Circumflex,
    Hash,

    Ampersand,
    Tilde,
    Bar,
    LeftShift,
    RightShift,

    DoubleBackSlash,

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

// and       break     do        else      elseif    end
// false     for       function  goto      if        in
// local     nil       not       or        repeat    return
// then      true      until     while

// +     -     *     /     %     ^     #
// &     ~     |     <<    >>    //
// ==    ~=    <=    >=    <     >     =
// (     )     {     }     [     ]     ::
// ;     :     ,     .     ..    ...
