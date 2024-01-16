use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

use crate::{
    prefix_expression::{PrefixExpression, parse_prefix_expr},
    statement::{Block, Parameters},
    Rule,
};

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    True,
    False,
    Nil,
    VarArg,
    Negation(Box<Expression>),
    BooleanNegation(Box<Expression>),
    BitwiseNegation(Box<Expression>),
    Length(Box<Expression>),
    PrefixExpression(PrefixExpression),
    Lambda {
        parameters: Option<Parameters>,
        body: Block,
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
    Equals {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Different {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Greater {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Less {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    GreaterOrEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    LessOrEqual {
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

lazy_static::lazy_static! {
    static ref EXPR_PARSER: PrattParser<Rule> =
    PrattParser::new()
        .op(Op::infix(Rule::BooleanOr, Assoc::Left))
        .op(Op::infix(Rule::BooleanAnd, Assoc::Left))
        .op(Op::infix(Rule::Equals, Assoc::Left)
            | Op::infix(Rule::Different, Assoc::Left)
            | Op::infix(Rule::Greater, Assoc::Left)
            | Op::infix(Rule::GreaterOrEqual, Assoc::Left)
            | Op::infix(Rule::Less, Assoc::Left)
            | Op::infix(Rule::LessOrEqual, Assoc::Left))
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
        .op(Op::infix(Rule::Exponentiation, Assoc::Right));
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    EXPR_PARSER
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
            Rule::Lambda => todo!(),
            Rule::PrefixExpression => Expression::PrefixExpression(parse_prefix_expr(primary.into_inner())),
            Rule::Expression => parse_expr(primary.into_inner()),
            rule => unreachable!("Expected primary, found {:?} {}", rule, primary),
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
            Rule::Equals => Expression::Equals {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Rule::Different => Expression::Different {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Rule::Greater => Expression::Greater {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Rule::GreaterOrEqual => Expression::GreaterOrEqual {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Rule::Less => Expression::Less {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Rule::LessOrEqual => Expression::LessOrEqual {
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
            Rule::Negation => Expression::Negation(Box::new(rhs)),
            Rule::BooleanNegation => Expression::BooleanNegation(Box::new(rhs)),
            Rule::BitwiseNegation => Expression::BitwiseNegation(Box::new(rhs)),
            Rule::Length => Expression::Length(Box::new(rhs)),
            rule => unreachable!("Expected prefix operation, found {:?}", rule),
        })
        .parse(pairs)
}
