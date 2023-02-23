pub mod outer;
pub mod number;

pub trait StateMachine<State, Symbol>: Sized {
    fn next(&mut self, symbol: Symbol) -> bool;

    fn state(&self) -> &'_ State;

    fn reset(&mut self);
}

