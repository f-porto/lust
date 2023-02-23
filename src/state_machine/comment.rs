use super::StateMachine;

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
