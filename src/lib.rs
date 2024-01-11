use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "lua.pest"]
pub struct LuaParser;

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    True,
    False,
    Nil,
    VarArg,
    Negation {
        expr: Box<Expression>,
    },
    BooleanNegation {
        expr: Box<Expression>,
    },
    BitwiseNegation {
        expr: Box<Expression>,
    },
    Length {
        expr: Box<Expression>,
    },
    Addition {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Subtraction {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Multiplication {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Division {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IntegerDivision {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Modulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BooleanOr {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BooleanAnd {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsDifferent {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsGreater {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsLess {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsGreaterOrEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsLessOrEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BitwiseAnd {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BitwiseOr {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BitwiseXor {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BitwiseLeftShift {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    BitwiseRightShift {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Concatenation {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Exponentiation {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn new() -> PrattParser<Rule> {
        PrattParser::new()
            .op(Op::infix(Rule::BooleanOr, Assoc::Left))
            .op(Op::infix(Rule::BooleanAnd, Assoc::Left))
            .op(Op::infix(Rule::IsEqual, Assoc::Left)
                | Op::infix(Rule::IsDifferent, Assoc::Left)
                | Op::infix(Rule::IsGreater, Assoc::Left)
                | Op::infix(Rule::IsGreaterOrEqual, Assoc::Left)
                | Op::infix(Rule::IsLess, Assoc::Left)
                | Op::infix(Rule::IsLessOrEqual, Assoc::Left))
            .op(Op::infix(Rule::BitwiseOr, Assoc::Left))
            .op(Op::infix(Rule::BitwiseXor, Assoc::Left))
            .op(Op::infix(Rule::BitwiseAnd, Assoc::Left))
            .op(Op::infix(Rule::BitwiseLeftShift, Assoc::Left)
                | Op::infix(Rule::BitwiseRightShift, Assoc::Left))
            .op(Op::infix(Rule::Concatenation, Assoc::Right))
            .op(Op::infix(Rule::Addition, Assoc::Left) | Op::infix(Rule::Subtraction, Assoc::Left))
            .op(Op::infix(Rule::Multiplication, Assoc::Left)
                | Op::infix(Rule::Division, Assoc::Left)
                | Op::infix(Rule::IntegerDivision, Assoc::Left)
                | Op::infix(Rule::Modulo, Assoc::Left))
            .op(Op::prefix(Rule::Negation)
                | Op::prefix(Rule::BitwiseNegation)
                | Op::prefix(Rule::BooleanNegation)
                | Op::prefix(Rule::Length))
            .op(Op::infix(Rule::Exponentiation, Assoc::Right))
    }

    pub fn parse_expr(expr_parser: &PrattParser<Rule>, pairs: Pairs<Rule>) -> Expression {
        expr_parser
            .map_primary(|primary| match primary.as_rule() {
                Rule::True => Expression::True,
                Rule::False => Expression::False,
                Rule::VarArg => Expression::VarArg,
                Rule::Nil => Expression::Nil,
                Rule::Integer => Expression::Integer(primary.as_str().parse().unwrap()),
                Rule::Float => Expression::Float(primary.as_str().parse().unwrap()),
                Rule::SqString => Expression::String(primary.into_inner().as_str().into()),
                Rule::DqString => Expression::String(primary.into_inner().as_str().into()),
                Rule::RawString => Expression::String(primary.into_inner().as_str().into()),
                Rule::Expression => ExpressionParser::parse_expr(expr_parser, primary.into_inner()),
                rule => unreachable!("Expected Integer, found {:?} {}", rule, primary),
            })
            .map_infix(|lhs, op, rhs| match op.as_rule() {
                Rule::Addition => Expression::Addition {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Subtraction => Expression::Subtraction {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Multiplication => Expression::Multiplication {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Division => Expression::Division {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IntegerDivision => Expression::IntegerDivision {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Modulo => Expression::Modulo {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BooleanAnd => Expression::BooleanAnd {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BooleanOr => Expression::BooleanOr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsEqual => Expression::IsEqual {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsDifferent => Expression::IsDifferent {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsGreater => Expression::IsGreater {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsGreaterOrEqual => Expression::IsGreaterOrEqual {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsLess => Expression::IsLess {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::IsLessOrEqual => Expression::IsLessOrEqual {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BitwiseOr => Expression::BitwiseOr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BitwiseXor => Expression::BitwiseXor {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BitwiseAnd => Expression::BitwiseAnd {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BitwiseLeftShift => Expression::BitwiseLeftShift {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::BitwiseRightShift => Expression::BitwiseRightShift {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Concatenation => Expression::Concatenation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::Exponentiation => Expression::Exponentiation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                rule => unreachable!("Expected infix operation, found {:?}", rule),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::Negation => Expression::Negation {
                    expr: Box::new(rhs),
                },
                Rule::BooleanNegation => Expression::BooleanNegation {
                    expr: Box::new(rhs),
                },
                Rule::BitwiseNegation => Expression::BitwiseNegation {
                    expr: Box::new(rhs),
                },
                Rule::Length => Expression::Length {
                    expr: Box::new(rhs),
                },
                rule => unreachable!("Expected prefix operation, found {:?}", rule),
            })
            .parse(pairs)
    }
}
