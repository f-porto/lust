pub mod comment;
pub mod number;
pub mod outer;
pub mod string;

pub trait StateMachine<SMState: State<SMSymbol>, SMSymbol>: Sized {
    fn next(&mut self, symbol: SMSymbol) -> bool;

    fn state(&self) -> &'_ SMState;

    fn reset(&mut self);
}

pub enum Symbol<T> {
    One(T),
    Any,
}

pub trait State<T> {
    fn is_final(&self) -> bool;

    fn expects(&self) -> Vec<Symbol<T>>;
}
