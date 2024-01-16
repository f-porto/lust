use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{expression::Expression, Rule};

#[derive(Debug)]
pub struct PrefixExpression {
    pub primary: Primary,
    pub actions: Vec<PExpAction>,
}

#[derive(Debug)]
pub enum Primary {
    Name(String),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub enum Argument {
    List(Vec<Expression>),
    String(String),
    Table,
}

#[derive(Debug)]
pub enum CallSuffix {
    Simple(Argument),
    Method { name: String, argument: Argument },
}

#[derive(Debug)]
pub enum Selector {
    Dot(String),
    Key(Expression),
}

#[derive(Debug)]
pub enum PExpAction {
    Selector(Selector),
    Call(CallSuffix),
}
