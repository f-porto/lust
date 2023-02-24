use super::{State, StateMachine, Symbol};

#[derive(Debug)]
pub enum CommentState {
    Initial,
    FirstDash,
    SecondDash,
    FirstLeftBracket,
    SecondLeftBracket,
    FistRightBracket,
    AnythingLine,
    AnythingBlock,
    End,
}

// --[[]]
#[derive(Debug)]
pub struct CommentStateMachine {
    state: CommentState,
}

impl CommentStateMachine {
    pub fn new() -> Self {
        Self {
            state: CommentState::Initial,
        }
    }
}

impl State<char> for CommentState {
    fn is_final(&self) -> bool {
        matches!(
            self,
            Self::SecondDash | Self::AnythingLine | Self::SecondLeftBracket | Self::End
        )
    }

    fn expects(&self) -> Vec<Symbol<char>> {
        match self {
            CommentState::Initial => vec![Symbol::One('-')],
            CommentState::FirstDash => vec![Symbol::one('-')],
            CommentState::SecondDash => vec![Symbol::Any],
            CommentState::FirstLeftBracket => vec![Symbol::Any],
            CommentState::SecondLeftBracket => vec![Symbol::Any],
            CommentState::FistRightBracket => vec![Symbol::Any],
            CommentState::AnythingLine => vec![Symbol::Any],
            CommentState::AnythingBlock => vec![Symbol::Any],
            CommentState::End => vec![],
        }
    }
}

impl StateMachine<CommentState, char> for CommentStateMachine {
    fn next(&mut self, symbol: char) -> bool {
        self.state = match (&self.state, symbol) {
            (CommentState::Initial, '-') => CommentState::FirstDash,
            (CommentState::FirstDash, '-') => CommentState::SecondDash,
            (CommentState::SecondDash, '[') => CommentState::FirstLeftBracket,
            (CommentState::SecondDash, '\n') => CommentState::End,
            (CommentState::SecondDash, _) => CommentState::AnythingLine,
            (CommentState::FirstLeftBracket, '[') => CommentState::SecondLeftBracket,
            (CommentState::FirstLeftBracket, _) => CommentState::AnythingLine,
            (CommentState::SecondLeftBracket, ']') => CommentState::FistRightBracket,
            (CommentState::SecondLeftBracket, _) => CommentState::AnythingBlock,
            (CommentState::AnythingLine, '\n') => CommentState::End,
            (CommentState::AnythingLine, _) => CommentState::AnythingLine,
            (CommentState::AnythingBlock, ']') => CommentState::FistRightBracket,
            (CommentState::AnythingBlock, _) => CommentState::AnythingBlock,
            (CommentState::FistRightBracket, ']') => CommentState::End,
            (CommentState::FistRightBracket, _) => CommentState::AnythingBlock,
            _ => return false,
        };
        true
    }

    fn state(&self) -> &'_ CommentState {
        &self.state
    }

    fn reset(&mut self) {
        self.state = CommentState::Initial;
    }
}
