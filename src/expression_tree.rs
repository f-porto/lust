use std::mem::replace;

use crate::{
    ast::{Expression, Number, Variable},
    error::LustError,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct TokenTree<'a> {
    pub root: Option<Box<TokenNode<'a>>>,
}

impl<'a> TokenTree<'a> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, mut node: TokenNode<'a>) {
        if self.root.is_none() {
            self.root = Some(Box::new(node));
        } else if node.weight() <= self.root.as_ref().unwrap().weight() {
            let free_node = replace(&mut self.root, None);
            node.insert(*free_node.unwrap());
            self.root = Some(Box::new(node));
        } else {
            self.root.as_mut().unwrap().insert(node);
        }
    }
}

#[derive(Debug)]
pub enum TokenNode<'a> {
    Binary {
        value: Token<'a>,
        weight: usize,
        left: Option<Box<TokenNode<'a>>>,
        right: Option<Box<TokenNode<'a>>>,
    },
    Unary {
        value: Token<'a>,
        weight: usize,
        center: Option<Box<TokenNode<'a>>>,
    },
    Nullary {
        value: Token<'a>,
        weight: usize,
    },
    NullaryWithExpression {
        value: Token<'a>,
        weight: usize,
        expression: Expression<'a>,
    },
}

impl<'a> TokenNode<'a> {
    pub fn binary(token: Token<'a>, weight: usize) -> Self {
        Self::Binary {
            value: token,
            weight,
            left: None,
            right: None,
        }
    }

    pub fn unary(token: Token<'a>, weight: usize) -> Self {
        Self::Unary {
            value: token,
            weight,
            center: None,
        }
    }

    pub fn nullary(token: Token<'a>, weight: usize) -> Self {
        Self::Nullary {
            value: token,
            weight,
        }
    }

    fn nullary_with_expression(
        token: Token<'a>,
        expression: Expression<'a>,
        weight: usize,
    ) -> Self {
        Self::NullaryWithExpression {
            value: token,
            weight,
            expression,
        }
    }

    fn weight(&self) -> usize {
        match self {
            Self::Binary { weight, .. } => *weight,
            Self::Unary { weight, .. } => *weight,
            Self::Nullary { weight, .. } => *weight,
            Self::NullaryWithExpression { weight, .. } => *weight,
        }
    }

    // fn value(&self) -> Token {
    //     match self {
    //         Self::Binary { value, .. } => value.clone(),
    //         Self::Unary { value, .. } => value.clone(),
    //         Self::Nullary { value, .. } => value.clone(),
    //         Self::NullaryWithExpression { value, .. } => value.clone(),
    //     }
    // }

    fn insert(&mut self, mut node: TokenNode<'a>) {
        match self {
            Self::Binary { left, right, .. } => {
                if left.is_none() {
                    let _ = replace(left, Some(Box::new(node)));
                } else if right.is_none() {
                    let _ = replace(right, Some(Box::new(node)));
                } else if node.weight() <= right.as_ref().unwrap().weight() {
                    let free_node = right.take();
                    node.insert(*free_node.unwrap());
                    let _ = replace(right, Some(Box::new(node)));
                } else {
                    right.as_mut().unwrap().insert(node);
                }
            }
            Self::Unary { center, .. } => {
                if center.is_none() {
                    let _ = replace(center, Some(Box::new(node)));
                } else if node.weight() <= center.as_ref().unwrap().weight() {
                    let free_node = center.take();
                    node.insert(*free_node.unwrap());
                    let _ = replace(center, Some(Box::new(node)));
                } else {
                    center.as_mut().unwrap().insert(node);
                }
            }
            Self::Nullary { .. } => {
                unreachable!("I guess this is unreachable, feels very unreachable you know...");
            }
            Self::NullaryWithExpression { .. } => {
                unreachable!("That's unreachable, trust me bro");
            }
        }
    }

    fn into_expression(self) -> Result<Expression<'a>, LustError> {
        let expression = match self {
            Self::Binary {
                value, left, right, ..
            } => match value.lexeme {
                TokenKind::Number(number) => {
                    if let Some(token_node) = left {
                        return Err(LustError::UnexpectedToken(format!("{:?}", token_node)));
                    }
                    if let Some(token_node) = right {
                        return Err(LustError::UnexpectedToken(format!("{:?}", token_node)));
                    }
                    Expression::Number(Number {})
                }
                TokenKind::Plus => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Addition {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::Minus => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Subtraction {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::Asterisk => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Multiplication {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::Slash => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Division {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::LessThan => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::LessThan {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::GreaterThan => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::GreaterThan {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::LessThanOrEqual => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::LessThanOrEqual {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::GreaterThanOrEqual => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::GreaterThanOrEqual {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::Equals => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Equals {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                TokenKind::Different => {
                    let Some(lhs) = left else {
                        return Err(LustError::ExpectedButGotNothing("lhs".into()));
                    };
                    let Some(rhs) = right else {
                        return Err(LustError::ExpectedButGotNothing("rhs".into()));
                    };
                    Expression::Different {
                        lhs: Box::new(lhs.into_expression()?),
                        rhs: Box::new(rhs.into_expression()?),
                    }
                }
                token => unreachable!(
                    "This really really really should be unreachable, unless... `{:?}`",
                    token
                ),
            },
            Self::Unary { value, center, .. } => match value.lexeme {
                TokenKind::Not => {
                    let Some(center) = center else {
                        return Err(LustError::ExpectedButGotNothing("expression".into()));
                    };
                    Expression::Not {
                        expression: Box::new(center.into_expression()?),
                    }
                }
                TokenKind::Minus => {
                    let Some(center) = center else {
                        return Err(LustError::ExpectedButGotNothing("expression".into()));
                    };
                    Expression::UnaryMinus {
                        expression: Box::new(center.into_expression()?),
                    }
                }
                token => unreachable!("This is unreachable, `{:?}`", token),
            },
            Self::Nullary { value, .. } => match value.lexeme {
                TokenKind::Number(number) => Expression::Number(Number {}),
                TokenKind::Identifier(identifier) => {
                    Expression::Variable(Variable::new(identifier))
                }
                TokenKind::True => Expression::Bool(true),
                TokenKind::False => Expression::Bool(false),
                token => unreachable!("Unreachable right?, {:?}", token),
            },
            Self::NullaryWithExpression {
                value,
                weight,
                expression,
            } => match value {
                _ => unreachable!("This is unreachable for now"),
            },
        };

        Ok(expression)
    }
}
