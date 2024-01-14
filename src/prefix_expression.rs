use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    expression::{Expression, ExpressionParser},
    Rule,
};

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

fn parse_selector(expr_parser: &PrattParser<Rule>, pair: Pair<Rule>) -> Selector {
    match pair.as_rule() {
        Rule::Name => Selector::Dot(pair.as_str().into()),
        Rule::Expression => {
            Selector::Key(ExpressionParser::parse_expr(expr_parser, pair.into_inner()))
        }
        _ => unreachable!("Expected selector, found {:?}", pair),
    }
}

fn parse_args(pair: Pair<Rule>) -> Argument {
    todo!()
}

fn parse_call_suffix(mut pairs: Pairs<Rule>) -> CallSuffix {
    let first = pairs.next().unwrap();
    match first.as_rule() {
        Rule::Arguments => CallSuffix::Simple(parse_args(first)),
        Rule::Name => {
            let arg = pairs.next().unwrap();
            CallSuffix::Method {
                name: first.as_str().into(),
                argument: parse_args(arg),
            }
        }
        _ => unreachable!("Expected call suffix, found {:?}", first),
    }
}

fn parse_p_exp(expr_parser: &PrattParser<Rule>, pair: Pair<Rule>) -> PExpAction {
    match pair.as_rule() {
        Rule::Selector => PExpAction::Selector(parse_selector(
            expr_parser,
            pair.into_inner().next().unwrap(),
        )),
        Rule::CallSuffix => PExpAction::Call(parse_call_suffix(pair.into_inner())),
        _ => unreachable!("Expected prefix expression, found {:?}", pair),
    }
}

pub fn parse_prefix_expression(
    expr_parser: &PrattParser<Rule>,
    mut pairs: Pairs<Rule>,
) -> PrefixExpression {
    let first = pairs.next().unwrap();
    let primary = first.into_inner().next().unwrap();
    let primary = match primary.as_rule() {
        Rule::Expression => Primary::Expression(Box::new(ExpressionParser::parse_expr(
            expr_parser,
            primary.into_inner(),
        ))),
        Rule::Name => Primary::Name(primary.as_str().into()),
        _ => unreachable!(),
    };
    let actions: Vec<_> = pairs.map(|x| parse_p_exp(expr_parser, x)).collect();
    PrefixExpression { primary, actions }
}
