use crate::{
    expression_tree::{TokenNode, TokenTree},
    parser::Parser,
    token::TokenKind,
};

pub struct ExpressionParser<'a> {
    parser: Parser<'a>,
    weight: usize,
}

const OR: usize = 0;
const AND: usize = 1;
const RELATIONAL: usize = 2;
const BIT_OR: usize = 3;
const BIT_AND: usize = 4;
const BIT_SHIFT: usize = 5;
const CONCAT: usize = 6;
const ADDITIVE: usize = 7;
const MULTIPLICATIVE: usize = 8;
const UNARY: usize = 9;
const NULLARY: usize = 10;

impl<'a> ExpressionParser<'a> {
    pub fn new(parser: Parser<'a>) -> Self {
        Self { parser, weight: 0 }
    }

    pub fn parse(&mut self) -> TokenTree<'a> {
        let mut tree = TokenTree::new();

        tree
    }

    fn start(&mut self, tree: &mut TokenTree<'a>) {
        let Ok(token) = self.parser.peek_token() else {
            return;
        };
        let is_initial = matches!(
            token.lexeme,
            TokenKind::Nil
                | TokenKind::False
                | TokenKind::True
                | TokenKind::Number(_)
                | TokenKind::String(_)
                | TokenKind::Identifier(_)
                | TokenKind::Not
                | TokenKind::Tilde
                | TokenKind::Hash
                | TokenKind::Minus
                | TokenKind::LeftParenthesis
                | TokenKind::LeftBrace
                | TokenKind::TripleDot
        );
        if !is_initial {
            return;
        }
        let token = self.parser.next_token().unwrap();
        let node = match token.lexeme {
            TokenKind::Nil
            | TokenKind::False
            | TokenKind::True
            | TokenKind::Number(_)
            | TokenKind::String(_) => TokenNode::Nullary {
                value: token,
                weight: NULLARY,
            },
            _ => unreachable!("Literally impossible, just doesn't make sense: {:?}", token),
        };
        tree.insert(node);
    }
}
