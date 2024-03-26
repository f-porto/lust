use std::{collections::HashMap, error::Error, fmt::Debug, hash::Hash};

use pest::Parser;

use crate::{
    parser::{
        expression::{self, Expression, Field},
        statement::{Block, Parameters},
        LuaParser, Rule,
    },
    std::Builtin,
};

#[derive(Debug, Clone)]
pub struct Table {
    counter: usize,
    table: HashMap<Value, Value>,
}

impl Table {
    pub fn insert(&mut self, key: &Value, value: &Value) {
        self.table.insert(key.clone(), value.clone());
    }

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

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    False,
    True,
    Integer(i64),
    Float(f64),
    String(String),
    Table(Table),
    Lambda { parameters: Parameters, body: Block },
    Builtin(Builtin),
}

impl Value {
    pub fn is_truthy(self) -> bool {
        !matches!(self, Value::False | Value::Nil)
    }

    pub fn to_number(&self) -> Result<Self, Box<dyn Error>> {
        match self {
            Value::Float(_) | Value::Integer(_) => Ok(self.clone()),
            Value::String(s) => {
                let pairs = LuaParser::parse(Rule::Number, &s)?;
                let pair = pairs.into_iter().next().unwrap();
                match pair.as_rule() {
                    Rule::Integer => Ok(Value::Integer(expression::parse_integer(&s))),
                    Rule::HexInteger => Ok(Value::Integer(expression::parse_hex_integer(&s))),
                    Rule::Float => Ok(Value::Float(expression::parse_float(&s))),
                    Rule::HexFloat => Ok(Value::Float(expression::parse_hex_float(&s))),
                    p => unreachable!("Expected number, found {:?}", p),
                }
            }
            _ => Err(String::from(""))?,
        }
    }

    pub fn to_float(&self) -> Result<Self, Box<dyn Error>> {
        match self.to_number()? {
            Value::Float(f) => Ok(Value::Float(f)),
            Value::Integer(n) => Ok(Value::Float(n as f64)),
            v => unreachable!("Expected number, found {:?}", v),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::False => "false".to_string(),
            Value::True => "true".to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Table(_) => todo!(),
            Value::Lambda { parameters, body } => todo!(),
            Value::Builtin(_) => todo!(),
        }
    }
}

impl Value {
    pub fn add(&self, rhs: &Self) -> Result<Value, Box<dyn Error>> {
        let lhs = self.to_number()?;
        let rhs = rhs.to_number()?;
        let v = match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (lhs, rhs) => unreachable!("Expected numbers, found {:?} and {:?}", lhs, rhs),
        };
        Ok(v)
    }

    pub fn is_equal(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Value::Nil, Value::Nil) => true,
            (Value::False, Value::False) => true,
            (Value::True, Value::True) => true,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Integer(a), Value::Float(b)) => &(*a as f64) == b,
            (Value::Float(a), Value::Integer(b)) => a == &(*b as f64),
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Table(_), Value::Table(_)) => todo!(),
            (Value::Lambda { .. }, Value::Lambda { .. }) => todo!(),
            (_, _) => false,
        }
    }

    pub fn is_less_than(&self, rhs: &Self) -> Result<bool, Box<dyn Error>> {
        let r = match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => a < b,
            (Value::Integer(a), Value::Float(b)) => &(*a as f64) < b,
            (Value::Float(a), Value::Integer(b)) => a < &(*b as f64),
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            (_, _) => return Err("")?,
        };
        Ok(r)
    }

    pub fn is_less_or_equal(&self, rhs: &Self) -> Result<bool, Box<dyn Error>> {
        let r = match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => a <= b,
            (Value::Integer(a), Value::Float(b)) => &(*a as f64) <= b,
            (Value::Float(a), Value::Integer(b)) => a <= &(*b as f64),
            (Value::Float(a), Value::Float(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a <= b,
            (_, _) => return Err("")?,
        };
        Ok(r)
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
