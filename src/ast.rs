use std::{env::var, io::Empty};

use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    expression::{Expression, ExpressionParser},
    prefix_expression::{Argument, CallSuffix, PExpAction, PrefixExpression, Primary, Selector, self, parse_prefix_expression},
    statement::{Block, Statement, Variable},
    Rule,
};

fn print_pair(pair: &Pair<Rule>) {
    println!(
        "{:?} ({}): {:?}",
        pair.as_rule(),
        pair.as_node_tag().unwrap_or("x"),
        pair.as_str()
    );
}

fn print_pairs(pairs: Pairs<Rule>, ident: usize) {
    for pair in pairs {
        print!("{}", " ".repeat(ident));
        print_pair(&pair);
        print_pairs(pair.into_inner(), ident + 2);
    }
}

pub struct ASTBuilder {
    expr_parser: PrattParser<Rule>,
}

impl ASTBuilder {
    pub fn new() -> Self {
        Self {
            expr_parser: ExpressionParser::new(),
        }
    }

    pub fn build_ast(&self, pairs: Pairs<Rule>) -> Block {
        for pair in pairs {
            self.build_statement(pair);
        }

        Block {
            statements: vec![Statement::Empty],
            return_statement: None,
        }
    }

    fn build_statement(&self, pair: Pair<Rule>) -> Statement {
        match pair.as_rule() {
            Rule::Assignment => self.build_assignment(pair),
            _ => Statement::Empty,
        }
    }

    fn parse_expr(&self, pairs: Pairs<Rule>) -> Expression {
        ExpressionParser::parse_expr(&self.expr_parser, pairs)
    }
}

impl ASTBuilder {
    fn build_variable(&self, mut pairs: Pairs<Rule>) -> Variable {
        let first = pairs.next().unwrap();
        if first.as_rule() == Rule::Name {
            return Variable::Name(first.as_str().into());
        }
        let mut prefix_exp = parse_prefix_expression(&self.expr_parser, pairs);
        let last = prefix_exp.actions.pop().unwrap();
        let PExpAction::Selector(selector) = last else {
            unreachable!("Expected selector, found {:?}", last);
        };
        return Variable::Selector {
            prefix_exp,
            selector,
        };
    }

    fn build_assignment(&self, pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let variables = pairs.next().unwrap();
        for variable in variables.into_inner() {
            self.build_variable(variable.into_inner());
        }
        pairs.next().unwrap();
        todo!()
    }
}
