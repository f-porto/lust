use super::StateMachine;

#[derive(Debug)]
pub enum StringState {
    Initial,
    SingleQuote,
    DoubleQuote,
    FirstStartBlockQuote,
    SecondStartBlockQuote,
    FirstEndBlockQuote,
    SingleChar,
    DoubleChar,
    BlockChar,
    SingleEscape,
    DoubleEscape,
    End,
}

#[derive(Debug)]
pub struct StringStateMachine {
    state: StringState,
    skip: usize,
    equals_counter: usize,
    counter: usize,
}

impl StringStateMachine {
    pub fn new() -> Self {
        Self {
            state: StringState::Initial,
            skip: 1,
            equals_counter: 0,
            counter: 0,
        }
    }

    pub fn skip(&self) -> usize {
        self.skip
    }
}

impl StateMachine<StringState, char> for StringStateMachine {
    fn next(&mut self, symbol: char) -> bool {
        self.state = match (&self.state(), symbol) {
            (StringState::Initial, '\'') => StringState::SingleQuote,
            (StringState::Initial, '"') => StringState::DoubleQuote,
            (StringState::Initial, '[') => StringState::FirstStartBlockQuote,
            (StringState::SingleQuote, '\'') => StringState::End,
            (StringState::SingleQuote, '\\') => StringState::SingleEscape,
            (StringState::SingleQuote, _) => StringState::SingleChar,
            (StringState::SingleEscape, '\'') => StringState::SingleChar,
            (StringState::SingleEscape, _) => StringState::SingleChar,
            (StringState::SingleChar, '\\') => StringState::SingleEscape,
            (StringState::SingleChar, '\'') => StringState::End,
            (StringState::SingleChar, _) => StringState::SingleChar,
            (StringState::DoubleQuote, '"') => StringState::End,
            (StringState::DoubleQuote, '\\') => StringState::DoubleEscape,
            (StringState::DoubleQuote, _) => StringState::DoubleChar,
            (StringState::DoubleEscape, '"') => StringState::DoubleChar,
            (StringState::DoubleEscape, _) => StringState::DoubleChar,
            (StringState::DoubleChar, '\\') => StringState::DoubleEscape,
            (StringState::DoubleChar, '"') => StringState::End,
            (StringState::DoubleChar, _) => StringState::DoubleChar,
            (StringState::FirstStartBlockQuote, '[') => {
                self.skip += 1;
                StringState::SecondStartBlockQuote
            }
            (StringState::FirstStartBlockQuote, '=') => {
                self.skip += 1;
                self.equals_counter += 1;
                StringState::FirstStartBlockQuote
            }
            (StringState::SecondStartBlockQuote, ']') => {
                self.counter = 0;
                StringState::FirstEndBlockQuote
            }
            (StringState::SecondStartBlockQuote, _) => StringState::BlockChar,
            (StringState::BlockChar, ']') => {
                self.counter = 0;
                StringState::FirstEndBlockQuote
            }
            (StringState::BlockChar, _) => StringState::BlockChar,
            (StringState::FirstEndBlockQuote, ']') => {
                if self.counter == self.equals_counter {
                    StringState::End
                } else {
                    self.counter = 0;
                    StringState::FirstEndBlockQuote
                }
            }
            (StringState::FirstEndBlockQuote, '=') => {
                self.counter += 1;
                StringState::FirstEndBlockQuote
            }
            (StringState::FirstEndBlockQuote, _) => StringState::BlockChar,
            _ => return false,
        };
        true
    }

    fn state(&self) -> &'_ StringState {
        &self.state
    }

    fn reset(&mut self) {
        self.state = StringState::Initial;
    }
}
