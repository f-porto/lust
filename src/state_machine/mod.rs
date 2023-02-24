pub mod comment;
pub mod number;
pub mod outer;
pub mod string;

pub trait StateMachine<SExpec, SMState: State<SExpec>, SMSymbol>: Sized {
    fn next(&mut self, symbol: SMSymbol) -> bool;

    fn state(&self) -> &'_ SMState;

    fn reset(&mut self);
}

pub enum Symbol {
    Letter,
    HexDigit,
    Digit,
    One(char),
    Any,
}

pub trait State<T> {
    fn is_final(&self) -> bool;

    fn expects(&self) -> Vec<T>;
}
