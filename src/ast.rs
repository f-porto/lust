use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    expression::{Expression, ExpressionParser},
    prefix_expression::{parse_prefix_expression, PExpAction},
    statement::{Block, FunctionName, Parameters, Statement, Variable},
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

fn parse_parameter_list(pair: Pair<Rule>) -> Parameters {
    let mut pairs = pair.into_inner();
    let Some(first) = pairs.next() else {
        return Parameters {
            name_list: vec![],
            var_arg: false,
        };
    };
    let name_list;
    match first.as_rule() {
        Rule::VarArg => {
            return Parameters {
                name_list: vec![],
                var_arg: true,
            }
        }
        Rule::NameList => name_list = first.into_inner().map(|x| x.as_str().into()).collect(),
        _ => unreachable!("Expected parameter, found {:?}", first),
    };
    let var_arg = pairs.next().is_some();
    Parameters { name_list, var_arg }
}

pub fn parse_function_body(
    ast_builder: &ASTBuilder,
    pair: Pair<Rule>,
) -> (Option<Parameters>, Block) {
    let mut pairs = pair.into_inner();
    let Some(first) = pairs.next() else {
        return (
            None,
            Block {
                statements: vec![],
                return_statement: None,
            },
        );
    };
    let parameters;
    match first.as_rule() {
        Rule::ParameterList => parameters = parse_parameter_list(first),
        Rule::Block => {
            let block = ast_builder.build_ast(first.into_inner());
            return (None, block);
        }
        _ => unreachable!("Expected function body, found {:?}", first),
    };
    if pairs.peek().is_none() {
        return (
            Some(parameters),
            Block {
                statements: vec![],
                return_statement: None,
            },
        );
    };
    let block = ast_builder.build_ast(pairs);
    (Some(parameters), block)
}

impl ASTBuilder {
    fn parse_function_name(&self, pair: Pair<Rule>) -> FunctionName {
        let mut method: Option<String> = None;
        let mut names = vec![];
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Name => names.push(pair.as_str().into()),
                Rule::MethodName => method = Some(pair.as_str().into()),
                _ => unreachable!("Expected name, found {:?}", pair),
            }
        }
        FunctionName { names, method }
    }

    fn build_function_definition(&self, pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let function_name = self.parse_function_name(pairs.next().unwrap());
        let (parameters, body) = parse_function_body(&self, pairs.next().unwrap());
        Statement::FunctionDefinition {
            function_name,
            parameters,
            body,
        }
    }
}
