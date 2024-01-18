use pest::iterators::Pairs;

use crate::{
    parser::expression::{parse_expr, Expression},
    parser::Rule,
};

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub primary: Primary,
    pub actions: Vec<PExprAction>,
}

#[derive(Debug, PartialEq)]
pub enum Primary {
    Name(String),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    List(Vec<Expression>),
    String(String),
    Table,
}

#[derive(Debug, PartialEq)]
pub enum CallSuffix {
    Simple(Argument),
    Method { name: String, argument: Argument },
}

#[derive(Debug, PartialEq)]
pub enum Selector {
    Dot(String),
    Key(Expression),
}

#[derive(Debug, PartialEq)]
pub enum PExprAction {
    Selector(Selector),
    Call(CallSuffix),
}

pub fn parse_prefix_expr(mut pairs: Pairs<Rule>) -> PrefixExpression {
    let primary = parse_primary_expr(pairs.next().unwrap().into_inner());
    let actions = pairs
        .map(|x| match x.as_rule() {
            Rule::Selector => PExprAction::Selector(parse_selector(x.into_inner())),
            Rule::CallSuffix => PExprAction::Call(parse_call_suffix(x.into_inner())),
            _ => unreachable!("Expected prefix expression action, found {:?}", x),
        })
        .collect();
    PrefixExpression { primary, actions }
}

pub fn parse_primary_expr(mut pairs: Pairs<Rule>) -> Primary {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::Expression => Primary::Expression(Box::new(parse_expr(pair.into_inner()))),
        Rule::Name => Primary::Name(pair.as_str().into()),
        _ => unreachable!("Expected primary, found {:?}", pair),
    }
}

fn parse_arguments(mut pairs: Pairs<Rule>) -> Argument {
    let Some(pair) = pairs.next() else {
        return Argument::List(vec![]);
    };
    match pair.as_rule() {
        Rule::SqString => Argument::String(pair.as_str().into()),
        Rule::DqString => Argument::String(pair.as_str().into()),
        Rule::RawString => Argument::String(pair.as_str().into()),
        Rule::Table => Argument::Table,
        Rule::ExpressionList => {
            let exprs = pair
                .into_inner()
                .map(|x| parse_expr(x.into_inner()))
                .collect();
            Argument::List(exprs)
        }
        _ => unreachable!("Expected argument, found {:?}", pair),
    }
}

pub fn parse_call_suffix(mut pairs: Pairs<Rule>) -> CallSuffix {
    let pair = pairs.next().unwrap();
    if pair.as_rule() == Rule::Arguments {
        let argument = parse_arguments(pair.into_inner());
        return CallSuffix::Simple(argument);
    }
    let name = pair.as_str().into();
    let argument = parse_arguments(pairs.next().unwrap().into_inner());
    CallSuffix::Method { name, argument }
}

pub fn parse_selector(mut pairs: Pairs<Rule>) -> Selector {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::Name => Selector::Dot(pair.as_str().into()),
        Rule::Expression => Selector::Key(parse_expr(pair.into_inner())),
        _ => unreachable!("Expected selector, found {:?}", pair),
    }
}
