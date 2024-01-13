use crate::expression::Expression;

#[derive(Debug)]
pub struct PrefixExpression {
    primary: Primary,
    actions: Vec<PExpAction>,
}

#[derive(Debug)]
pub enum Primary {
    Name(String),
    Expression(Expression),
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
    Method {
        name: String,
        argument: Argument,
    }
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