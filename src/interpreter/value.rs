use std::{collections::HashMap, hash::Hash};

use crate::parser::{
    expression::{Expression, Field},
    statement::{Block, Parameters},
};

#[derive(Debug)]
pub struct Table {
    counter: usize,
    table: HashMap<Value, Value>,
}

impl Table {
    pub fn insert(&mut self, key: &Value, value: &Value) {}

    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.table.get(key)
    }

    pub fn get_mut(&mut self, key: &Value) -> Option<&mut Value> {
        self.table.get_mut(key)
    }
}

impl Table {
    pub fn from_fields<F>(fields: &[Field], mut expr_evaluator: F) -> Table
    where
        F: FnMut(&Expression) -> Value,
    {
        let mut table = Table {
            counter: 1,
            table: HashMap::new(),
        };
        for field in fields {
            let (key, value) = match field {
                Field::ExprKey { key, value } => {
                    let key = expr_evaluator(key);
                    let value = expr_evaluator(value);
                    (key, value)
                }
                Field::NameKey { name, value } => {
                    let key = Value::String(name.clone());
                    let value = expr_evaluator(value);
                    (key, value)
                }
                Field::Expr(expr) => {
                    let key = Value::Integer(table.counter as i64);
                    let value = expr_evaluator(expr);
                    table.counter += 1;
                    (key, value)
                }
            };
            table.table.insert(key, value);
        }
        table
    }
}

#[derive(Debug)]
pub enum Value {
    Nil,
    False,
    True,
    Integer(i64),
    Float(f64),
    String(String),
    Table(Table),
    Lambda { parameters: Parameters, body: Block },
}

impl Value {
    pub fn is_truthy(self) -> bool {
        !matches!(self, Value::False | Value::Nil)
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.counter != other.counter {
            return false;
        }
        if self.table.len() != other.table.len() {
            return false;
        }
        for (key, value) in self.table.iter() {
            let Some(v) = other.table.get(key) else {
                return false;
            };
            if v != value {
                return false;
            };
        }
        true
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Table(l0), Self::Table(r0)) => l0 == r0,
            (
                Self::Lambda {
                    parameters: l_parameters,
                    body: l_body,
                },
                Self::Lambda {
                    parameters: r_parameters,
                    body: r_body,
                },
            ) => l_parameters == r_parameters && l_body == r_body,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Table {}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Hash for Table {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.counter.hash(state);
        for (key, value) in self.table.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}
