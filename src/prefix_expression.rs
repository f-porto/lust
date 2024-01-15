use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    expression::{Expression, ExpressionParser},
    print_pair, print_pairs, Rule,
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

fn parse_args(expr_parser: &PrattParser<Rule>, pair: Pair<Rule>) -> Argument {
    let Some(pair) = pair.into_inner().next() else {
        return Argument::List(vec![]);
    };
    match pair.as_rule() {
        Rule::DqString => Argument::String(pair.into_inner().as_str().into()),
        Rule::SqString => Argument::String(pair.into_inner().as_str().into()),
        Rule::RawString => Argument::String(pair.into_inner().as_str().into()),
        Rule::Table => Argument::Table,
        Rule::ExpressionList => {
            let exp_list = pair
                .into_inner()
                .map(|x| ExpressionParser::parse_expr(expr_parser, x.into_inner()))
                .collect();
            Argument::List(exp_list)
        }
        _ => unreachable!("Expected argument found, {:?}", pair),
    }
}

fn parse_call_suffix(expr_parser: &PrattParser<Rule>, mut pairs: Pairs<Rule>) -> CallSuffix {
    let first = pairs.next().unwrap();
    match first.as_rule() {
        Rule::Arguments => CallSuffix::Simple(parse_args(expr_parser, first)),
        Rule::Name => {
            let arg = pairs.next().unwrap();
            CallSuffix::Method {
                name: first.as_str().into(),
                argument: parse_args(expr_parser, arg),
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
        Rule::CallSuffix => PExpAction::Call(parse_call_suffix(expr_parser, pair.into_inner())),
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
