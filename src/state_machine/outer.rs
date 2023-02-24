use super::{
    comment::CommentStateMachine, number::NumberStateMachine, string::StringStateMachine, State,
    StateMachine, Symbol,
};

#[derive(Debug)]
pub enum OuterState {
    Initial,
    Plus,
    Minus,
    Asterisk,
    Percent,
    Circumflex,
    Hash,
    Ampersand,
    Bar,
    Semicolon,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    RightBracket,
    LeftBracket,
    Slash,
    Tilde,
    LessThan,
    GreaterThan,
    Equals,
    Colon,
    Dot,
    DoubleDot,
    TripleDot,
    GreaterThanOrEqual,
    LessThanOrEqual,
    DoubleEquals,
    Different,
    LeftShift,
    RightShift,
    DoubleColon,
    DoubleSlash,
    Word,
    Number(NumberStateMachine),
    Comment(CommentStateMachine),
    String(StringStateMachine),
}

impl State<Symbol> for OuterState {
    fn is_final(&self) -> bool {
        !matches!(self, Self::Initial)
    }

    fn expects(&self) -> Vec<Symbol> {
        match self {
            Self::Initial => vec![Symbol::Any],
            Self::LeftBracket => vec![Symbol::One('['), Symbol::One('=')],
            Self::Dot => vec![Symbol::One('.'), Symbol::Digit],
            Self::DoubleDot => vec![Symbol::One('.')],
            Self::GreaterThan => vec![Symbol::One('='), Symbol::One('>')],
            Self::LessThan => vec![Symbol::One('='), Symbol::One('<')],
            Self::Equals => vec![Symbol::One('=')],
            Self::Tilde => vec![Symbol::One('=')],
            Self::Colon => vec![Symbol::One(':')],
            Self::Slash => vec![Symbol::One('/')],
            Self::Minus => vec![Symbol::One('-')],
            Self::Word => vec![Symbol::Letter, Symbol::Digit, Symbol::One('_')],
            Self::Number(machine) => machine.state().expects(),
            Self::String(machine) => machine.state().expects(),
            Self::Comment(machine) => machine.state().expects(),
            _ => vec![],
        }
    }
}

#[derive(Debug)]
pub struct OuterStateMachine {
    state: OuterState,
}

impl OuterStateMachine {
    pub fn new() -> Self {
        Self {
            state: OuterState::Initial,
        }
    }
}

impl StateMachine<Symbol, OuterState, char> for OuterStateMachine {
    fn next(&mut self, symbol: char) -> bool {
        match &mut self.state {
            OuterState::Number(machine) => return machine.next(symbol),
            OuterState::Comment(machine) => return machine.next(symbol),
            OuterState::String(machine) => return machine.next(symbol),
            _ => {}
        };

        self.state = match (&self.state, symbol) {
            (OuterState::Initial, '+') => OuterState::Plus,
            (OuterState::Initial, '-') => OuterState::Minus,
            (OuterState::Initial, '*') => OuterState::Asterisk,
            (OuterState::Initial, '%') => OuterState::Percent,
            (OuterState::Initial, '^') => OuterState::Circumflex,
            (OuterState::Initial, '#') => OuterState::Hash,
            (OuterState::Initial, '&') => OuterState::Ampersand,
            (OuterState::Initial, '|') => OuterState::Bar,
            (OuterState::Initial, ';') => OuterState::Semicolon,
            (OuterState::Initial, ',') => OuterState::Comma,
            (OuterState::Initial, '(') => OuterState::LeftParenthesis,
            (OuterState::Initial, ')') => OuterState::RightParenthesis,
            (OuterState::Initial, '{') => OuterState::LeftBrace,
            (OuterState::Initial, '}') => OuterState::RightBrace,
            (OuterState::Initial, ']') => OuterState::RightBracket,
            (OuterState::Initial, '[') => OuterState::LeftBracket,
            (OuterState::Initial, '0'..='9') => {
                let mut machine = NumberStateMachine::new();
                machine.next(symbol);
                OuterState::Number(machine)
            }
            (OuterState::Initial, '/') => OuterState::Slash,
            (OuterState::Initial, '~') => OuterState::Tilde,
            (OuterState::Initial, '>') => OuterState::GreaterThan,
            (OuterState::Initial, '<') => OuterState::LessThan,
            (OuterState::Initial, '=') => OuterState::Equals,
            (OuterState::Initial, ':') => OuterState::Colon,
            (OuterState::Initial, '.') => OuterState::Dot,
            (OuterState::Initial, 'A'..='Z' | 'a'..='z' | '_') => OuterState::Word,
            (OuterState::Initial, '\'' | '"') => {
                let mut machine = StringStateMachine::new();
                machine.next(symbol);
                OuterState::String(machine)
            }
            (OuterState::LeftBracket, '[' | '=') => {
                let mut machine = StringStateMachine::new();
                machine.next('[');
                machine.next(symbol);
                OuterState::String(machine)
            }
            (OuterState::Dot, '.') => OuterState::DoubleDot,
            (OuterState::Dot, '0'..='9') => {
                let mut machine = NumberStateMachine::new();
                machine.next('.');
                machine.next(symbol);
                OuterState::Number(machine)
            }
            (OuterState::DoubleDot, '.') => OuterState::TripleDot,
            (OuterState::GreaterThan, '=') => OuterState::GreaterThanOrEqual,
            (OuterState::GreaterThan, '>') => OuterState::RightShift,
            (OuterState::LessThan, '=') => OuterState::LessThanOrEqual,
            (OuterState::LessThan, '<') => OuterState::LeftShift,
            (OuterState::Equals, '=') => OuterState::DoubleEquals,
            (OuterState::Tilde, '=') => OuterState::Different,
            (OuterState::Colon, ':') => OuterState::DoubleColon,
            (OuterState::Slash, '/') => OuterState::DoubleSlash,
            (OuterState::Minus, '-') => {
                let mut machine = CommentStateMachine::new();
                machine.next('-');
                machine.next('-');
                OuterState::Comment(machine)
            }
            (OuterState::Word, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9') => OuterState::Word,
            _ => return false,
        };
        true
    }

    fn state(&self) -> &'_ OuterState {
        &self.state
    }

    fn reset(&mut self) {
        self.state = OuterState::Initial;
    }
}
