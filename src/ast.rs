use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    expression::{Expression, ExpressionParser},
    prefix_expression::{
        self, parse_prefix_expression, Argument, CallSuffix, PExpAction, PrefixExpression, Primary,
        Selector,
    },
    print_pair, print_pairs,
    statement::{self, Block, Parameters, Statement, Variable, FunctionName},
    Rule,
};

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
        let mut statements = vec![];
        for pair in pairs {
            let statement = self.build_statement(pair);
            statements.push(statement);
        }

        Block {
            statements,
            return_statement: None,
        }
    }

    fn build_statement(&self, pair: Pair<Rule>) -> Statement {
        match pair.as_rule() {
            Rule::Assignment => self.build_assignment(pair),
            Rule::FunctionCall => self.build_function_call(pair),
            Rule::FunctionDefinition => self.build_function_definition(pair),
            Rule::EOI => Statement::Empty,
            _ => unreachable!("Expected statement, found {:?}", pair),
        }
    }

    fn parse_expr(&self, pairs: Pairs<Rule>) -> Expression {
        ExpressionParser::parse_expr(&self.expr_parser, pairs)
    }
}

impl ASTBuilder {
    fn build_function_call(&self, pair: Pair<Rule>) -> Statement {
        let mut prefix_exp = parse_prefix_expression(&self.expr_parser, pair.into_inner());
        let last = prefix_exp.actions.pop().unwrap();
        let PExpAction::Call(call) = last else {
            unreachable!("Expected call suffix, found {:?}", last);
        };
        Statement::FunctionCall { prefix_exp, call }
    }
}

impl ASTBuilder {
    fn build_variable(&self, pairs: Pairs<Rule>) -> Variable {
        let first = pairs.peek().unwrap();
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

        let variable_list = pairs
            .next()
            .unwrap()
            .into_inner()
            .map(|x| self.build_variable(x.into_inner()))
            .collect();

        let expr_list = pairs
            .next()
            .unwrap()
            .into_inner()
            .map(|x| self.parse_expr(x.into_inner()))
            .collect();

        Statement::Assignment {
            variable_list,
            expr_list,
        }
    }
}

pub fn parse_function_body(
    ast_builder: &ASTBuilder,
    pair: Pair<Rule>,
) -> (Option<Parameters>, Block) {
    todo!()
}

impl ASTBuilder {
    fn parse_function_name(&self, pair: Pair<Rule>) -> FunctionName {
        print_pair(&pair);
        print_pairs(pair.into_inner(), 2);
        todo!()
    }

    fn build_function_definition(&self, pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        self.parse_function_name(pairs.next().unwrap());
        parse_function_body(&self, pairs.next().unwrap());
        todo!()
    }
}
