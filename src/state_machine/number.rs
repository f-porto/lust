use super::{State, StateMachine, Symbol};

#[derive(Debug)]
pub enum NumberState {
    Initial,
    Minus,
    Zero,
    Digit,
    Dot,
    Decimal,
    E,
    ESignal,
    EDigit,
    X,
    XDigit,
    XDecimal,
    XDot,
    XP,
    XPSignal,
    XPDigit,
}

#[derive(Debug)]
pub struct NumberStateMachine {
    state: NumberState,
}

impl NumberStateMachine {
    pub fn new() -> Self {
        Self {
            state: NumberState::Initial,
        }
    }
}

impl State<Symbol> for NumberState {
    fn is_final(&self) -> bool {
        matches!(
            self,
            Self::Zero
                | Self::Digit
                | Self::Decimal
                | Self::EDigit
                | Self::XDigit
                | Self::XDecimal
                | Self::XPDigit
        )
    }

    fn expects(&self) -> Vec<Symbol> {
        match self {
            Self::Initial => vec![Symbol::One('-'), Symbol::One('.'), Symbol::Digit],
            Self::Minus => vec![Symbol::One('.'), Symbol::Digit],
            Self::Zero => vec![
                Symbol::One('x'),
                Symbol::One('.'),
                Symbol::Digit,
                Symbol::One('e'),
            ],
            Self::Digit => vec![Symbol::Digit, Symbol::One('.'), Symbol::One('e')],
            Self::Dot => vec![Symbol::Digit],
            Self::Decimal => vec![Symbol::Digit, Symbol::One('e')],
            Self::E => vec![Symbol::One('+'), Symbol::One('-'), Symbol::Digit],
            Self::ESignal => vec![Symbol::Digit],
            Self::EDigit => vec![Symbol::Digit],
            Self::X => vec![Symbol::One('.'), Symbol::HexDigit],
            Self::XDigit => vec![Symbol::One('.'), Symbol::One('p'), Symbol::HexDigit],
            Self::XDecimal => vec![Symbol::One('p'), Symbol::HexDigit],
            Self::XDot => vec![Symbol::HexDigit],
            Self::XP => vec![Symbol::One('-'), Symbol::One('+'), Symbol::HexDigit],
            Self::XPSignal => vec![Symbol::HexDigit],
            Self::XPDigit => vec![Symbol::HexDigit],
        }
    }
}

impl StateMachine<Symbol, NumberState, char> for NumberStateMachine {
    fn next(&mut self, symbol: char) -> bool {
        self.state = match (&self.state, symbol) {
            (NumberState::Initial, '.') => NumberState::Dot,
            (NumberState::Initial, '-') => NumberState::Minus,
            (NumberState::Initial, '0') => NumberState::Zero,
            (NumberState::Initial, '1'..='9') => NumberState::Digit,
            (NumberState::Minus, '.') => NumberState::Dot,
            (NumberState::Minus, '0') => NumberState::Zero,
            (NumberState::Minus, '1'..='9') => NumberState::Digit,
            (NumberState::Zero, 'x' | 'X') => NumberState::X,
            (NumberState::Zero, '.') => NumberState::Decimal,
            (NumberState::Zero, 'e' | 'E') => NumberState::E,
            (NumberState::Zero, '0'..='9') => NumberState::Digit,
            (NumberState::Digit, '0'..='9') => NumberState::Digit,
            (NumberState::Digit, '.') => NumberState::Decimal,
            (NumberState::Digit, 'e' | 'E') => NumberState::E,
            (NumberState::Dot, '0'..='9') => NumberState::Decimal,
            (NumberState::Decimal, '0'..='9') => NumberState::Decimal,
            (NumberState::Decimal, 'e' | 'E') => NumberState::E,
            (NumberState::E, '-' | '+') => NumberState::ESignal,
            (NumberState::E, '0'..='9') => NumberState::EDigit,
            (NumberState::ESignal, '0'..='9') => NumberState::EDigit,
            (NumberState::EDigit, '0'..='9') => NumberState::EDigit,
            (NumberState::X, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XDigit,
            (NumberState::X, '.') => NumberState::XDot,
            (NumberState::XDigit, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XDigit,
            (NumberState::XDigit, '.') => NumberState::XDecimal,
            (NumberState::XDigit, 'p' | 'P') => NumberState::XP,
            (NumberState::XDot, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XDecimal,
            (NumberState::XDecimal, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XDecimal,
            (NumberState::XDecimal, 'p' | 'P') => NumberState::XP,
            (NumberState::XP, '-' | '+') => NumberState::XPSignal,
            (NumberState::XP, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XPDigit,
            (NumberState::XPSignal, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XPDigit,
            (NumberState::XPDigit, '0'..='9' | 'a'..='f' | 'A'..='F') => NumberState::XPDigit,
            _ => return false,
        };
        true
    }

    fn state(&self) -> &'_ NumberState {
        &self.state
    }

    fn reset(&mut self) {
        self.state = NumberState::Initial;
    }
}

#[cfg(test)]
mod tests {
    use crate::state_machine::{number::NumberState, StateMachine};

    use super::NumberStateMachine;

    #[test]
    fn integers() {
        let integers = [
            "0",
            "23",
            "93372",
            "7392749373",
            "-3",
            "-43",
            "-43552",
            "-5446262453",
        ];

        let mut machine = NumberStateMachine::new();
        for integer in integers {
            machine.reset();
            for s in integer.chars() {
                assert!(machine.next(s));
            }
            matches!(machine.state, NumberState::Digit);
        }
    }

    #[test]
    fn floats() {
        let floats = [
            ".0",
            ".53",
            ".34253",
            ".0373803523",
            "4.",
            "1.5",
            "5.54",
            "5.34546",
            "4.9457295203",
            "23.",
            "54.6",
            "54.53",
            "63.93719",
            "64.8401840748",
            "74562.",
            "83943.5",
            "45345.93",
            "76932.88302",
            "87493.8737281047",
            "8493974203.",
            "8739875024.4",
            "4893547658.54",
            "8743245242.543436",
            "9483720371.938729302739",
        ];

        let mut machine = NumberStateMachine::new();
        for float in floats {
            machine.reset();
            for s in float.chars() {
                assert!(machine.next(s));
            }
            matches!(machine.state, NumberState::Decimal);
        }
    }

    #[test]
    fn exponents() {
        let iexps = [
            "0e4",
            "5e43536",
            "0e-4",
            "5e-43536",
            "0e+4",
            "5e+43536",
            "93372e3",
            "43545e54352",
            "93372e-3",
            "43545e-54352",
            "93372e+3",
            "43545e+54352",
            "-0e4",
            "-5e43536",
            "-0e-4",
            "-5e-43536",
            "-0e+4",
            "-5e+43536",
            "-93372e3",
            "-43545e54352",
            "-93372e-3",
            "-43545e-54352",
            "-93372e+3",
            "-43545e+54352",
        ];

        let mut machine = NumberStateMachine::new();
        for exp in iexps {
            machine.reset();
            for s in exp.chars() {
                assert!(machine.next(s), "exp={exp}");
            }
            matches!(machine.state, NumberState::EDigit);
        }

        let fexps = [
            ".0e4",
            ".0e45435",
            ".34253e4",
            ".34253e45634",
            "4.e4",
            "4.e46346",
            "1.5e4",
            "1.5e4754",
            "5.34546e4",
            "5.34546e47427",
            "74562.e4",
            "74562.e462757",
            "83943.5e4",
            "83943.5e46275",
            "76932.88302e4",
            "76932.88302e49533",
            ".0e-4",
            ".0e-45435",
            ".34253e-4",
            ".34253e-45634",
            "4.e-4",
            "4.e-46346",
            "1.5e-4",
            "1.5e-4754",
            "5.34546e-4",
            "5.34546e-47427",
            "74562.e-4",
            "74562.e-462757",
            "83943.5e-4",
            "83943.5e-46275",
            "76932.88302e-4",
            "76932.88302e-49533",
            ".0e+4",
            ".0e+45435",
            ".34253e+4",
            ".34253e+45634",
            "4.e+4",
            "4.e+46346",
            "1.5e+4",
            "1.5e+4754",
            "5.34546e+4",
            "5.34546e+47427",
            "74562.e+4",
            "74562.e+462757",
            "83943.5e+4",
            "83943.5e+46275",
            "76932.88302e+4",
            "76932.88302e+49533",
        ];

        let mut machine = NumberStateMachine::new();
        for exp in fexps {
            machine.reset();
            for s in exp.chars() {
                assert!(machine.next(s));
            }
            matches!(machine.state, NumberState::EDigit);
        }
    }

    #[test]
    fn invalid() {
        let invalids = [
            "h4536",
            "4445k",
            "432d2325",
            "3545.g243",
            "4324.45345h",
            "43543.342g545",
            "g3453.5245",
            "43423d.3545",
            "435j5425.43425",
            "432.5454k-423",
            "4342.545e--5435",
            "3423.545e+-545",
            "434.5435e+3423h5",
            "34,54353e4534",
            "3432.543543e-43252g",
            "+3425",
        ];

        let mut machine = NumberStateMachine::new();
        for invalid in invalids {
            machine.reset();
            let succeed = invalid.chars().all(|s| machine.next(s));
            assert!(!succeed);
        }

        let invalids = ["-", ".", "-.", "1e-", "-1e+"];

        let mut machine = NumberStateMachine::new();
        for invalid in invalids {
            machine.reset();
            for s in invalid.chars() {
                assert!(machine.next(s));
            }
            let is_final = matches!(
                machine.state,
                NumberState::Digit | NumberState::Decimal | NumberState::EDigit
            );
            assert!(!is_final);
        }
    }
}
