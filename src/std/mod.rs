use std::fmt::Debug;

use crate::interpreter::{value::Value, Scope};

#[derive(Debug, Clone)]
pub enum Builtin {
    Print,
}

pub fn load_std(scope: &mut Scope) {
    scope.insert("print".to_string(), Value::Builtin(Builtin::Print));
}

impl Builtin {
    pub fn call(&self, parameters: Vec<Value>) -> Value {
        match self {
            Builtin::Print => global::print(parameters),
        }
    }
}

pub mod global {
    use crate::interpreter::value::Value;

    pub fn print(parameters: Vec<Value>) -> Value {
        for p in parameters {
            print!("{}\t", p.to_string());
        }
        println!();
        Value::Nil
    }
}
