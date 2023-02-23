use super::{number::NumberStateMachine, StateMachine};

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
    Quote,
    Word,
    Number(NumberStateMachine),
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

impl StateMachine<OuterState, char> for OuterStateMachine {
    fn next(&mut self, symbol: char) -> bool {
        match &mut self.state {
            OuterState::Number(machine) => return machine.next(symbol),
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
            (OuterState::Initial, '\'' | '"') => OuterState::Quote,
            (OuterState::Initial, 'A'..='Z' | 'a'..='z' | '_') => OuterState::Word,
            (OuterState::Dot, '.') => OuterState::DoubleDot,
            (OuterState::Dot, '0'..='9') => {
                let mut machine = NumberStateMachine::new();
                machine.next('.');
                machine.next(symbol);
                OuterState::Number(machine)
            }
            (OuterState::DoubleDot, '.') => OuterState::TripleDot,
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
