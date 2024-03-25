use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

use crate::{
    parser::ast::parse_function_body,
    parser::prefix_expression::{parse_prefix_expr, PrefixExpression},
    parser::statement::{Block, Parameters},
    parser::Rule,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    ExprKey { key: Expression, value: Expression },
    NameKey { name: String, value: Expression },
    Expr(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    True,
    False,
    Nil,
    VarArg,
    Table(Vec<Field>),
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

fn parse_field(mut pairs: Pairs<Rule>) -> Field {
    let first = pairs.next().unwrap();
    match first.as_rule() {
        Rule::Name => {
            let name = first.as_str().into();
            let second = pairs.next().unwrap();
            let value = parse_expr(second.into_inner());
            Field::NameKey { name, value }
        }
        Rule::Expression => {
            let first = parse_expr(first.into_inner());
            if let Some(second) = pairs.next() {
                let value = parse_expr(second.into_inner());
                Field::ExprKey { key: first, value }
            } else {
                Field::Expr(first)
            }
        }
        _ => unreachable!("Expected field, found {:?}", first),
    }
}

pub fn parse_table(mut pairs: Pairs<Rule>) -> Expression {
    let Some(pair) = pairs.next() else {
        return Expression::Table(vec![]);
    };
    let fields = pair
        .into_inner()
        .map(|x| parse_field(x.into_inner()))
        .collect();
    Expression::Table(fields)
}

pub fn parse_integer(n: &str) -> i64 {
    n.parse().unwrap()
}

pub fn parse_hex_integer(n: &str) -> i64 {
    i64::from_str_radix(&n[2..], 16).unwrap()
}

pub fn parse_float(n: &str) -> f64 {
    n.parse().unwrap()
}

pub fn parse_hex_float(n: &str) -> f64 {
    let n = &n[2..];
    let dot = n.find('.');
    let p = n.find('p');

    let integer;
    let decimal;
    let decimal_size;
    let exponent;
    let mut number = 0f64;
    if let Some(dot) = dot {
        if let Some(p) = p {
            integer = i64::from_str_radix(&n[..dot], 16).unwrap_or(0);
            decimal = i64::from_str_radix(&n[(dot + 1)..p], 16).unwrap_or(0);
            decimal_size = p - dot - 1;
            exponent = i64::from_str_radix(&n[(p + 1)..], 16).unwrap_or(0);
        } else {
            integer = i64::from_str_radix(&n[..dot], 16).unwrap_or(0);
            decimal = i64::from_str_radix(&n[(dot + 1)..], 16).unwrap_or(0);
            decimal_size = n.len() - dot - 1;
            exponent = 0;
        }
    } else {
        if let Some(p) = p {
            integer = i64::from_str_radix(&n[..p], 16).unwrap_or(0);
            decimal = 0;
            decimal_size = 0;
            exponent = i64::from_str_radix(&n[(p + 1)..], 16).unwrap_or(0);
        } else {
            integer = i64::from_str_radix(&n[..], 16).unwrap_or(0);
            decimal = 0;
            decimal_size = 0;
            exponent = 0;
        }
    }
    number += integer as f64;
    if decimal_size > 0 {
        number += decimal as f64 / 16f64.powf(decimal_size as f64);
    }
    number = number * 2f64.powf(exponent as f64);
    number
}

pub fn parse_string(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut is_escaped = false;
    for c in text.chars() {
        if is_escaped {
            let cc = match c {
                'n' => '\n',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                _ => todo!("Unknown escapes character '\\{c}'"),
            };
            result.push(cc);
            is_escaped = false;
        } else if c == '\\' {
            is_escaped = true;
        } else {
            result.push(c);
        }
    }
    // println!("{}:{}:{}: {result}", file!(), line!(), column!());
    result
}

pub fn parse_raw_string(text: &str) -> String {
    text.into()
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    EXPR_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::True => Expression::True,
            Rule::False => Expression::False,
            Rule::VarArg => Expression::VarArg,
            Rule::Nil => Expression::Nil,
            Rule::Integer => Expression::Integer(parse_integer(primary.as_str())),
            Rule::HexInteger => Expression::Integer(parse_hex_integer(primary.as_str())),
            Rule::Float => Expression::Float(parse_float(primary.as_str())),
            Rule::HexFloat => Expression::Float(parse_hex_float(primary.as_str())),
            Rule::SqString => Expression::String(parse_string(primary.into_inner().as_str())),
            Rule::DqString => Expression::String(parse_string(primary.into_inner().as_str())),
            Rule::RawString => Expression::String(parse_raw_string(primary.into_inner().as_str())),
            Rule::Lambda => {
                let (parameters, body) =
                    parse_function_body(primary.into_inner().next().unwrap().into_inner());
                Expression::Lambda { parameters, body }
            }
            Rule::PrefixExpression => {
                Expression::PrefixExpression(parse_prefix_expr(primary.into_inner()))
            }
            Rule::Expression => parse_expr(primary.into_inner()),
            Rule::Table => parse_table(primary.into_inner()),
            _ => unreachable!("Expected primary, found {:?}", primary),
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
            _ => unreachable!("Expected infix operation, found {:?}", op),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::Negation => Expression::Negation(Box::new(rhs)),
            Rule::BooleanNegation => Expression::BooleanNegation(Box::new(rhs)),
            Rule::BitwiseNegation => Expression::BitwiseNegation(Box::new(rhs)),
            Rule::Length => Expression::Length(Box::new(rhs)),
            _ => unreachable!("Expected prefix operation, found {:?}", op),
        })
        .parse(pairs)
}
