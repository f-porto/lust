use pest::iterators::Pairs;

use crate::{
    parser::prefix_expression::{parse_prefix_expr, PExprAction},
    parser::statement::{
        Block, FunctionName, If, LocalVariable, Parameters, Return, Statement, Variable,
    },
    parser::Rule,
};

use super::expression::parse_expr;

pub fn build_ast(pairs: &mut Pairs<Rule>) -> Block {
    let mut block = Block {
        statements: vec![],
        return_statement: None,
    };
    let Some(pair) = pairs.next() else {
        return block;
    };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Statement => block.statements.push(parse_statement(pair.into_inner())),
            Rule::ReturnStatement => {
                block.return_statement = Some(parse_return_statement(pair.into_inner()))
            }
            _ => unreachable!("Expected statement, found {:?}", pair),
        };
    }
    return block;
}

fn parse_statement(mut pairs: Pairs<Rule>) -> Statement {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::Empty => Statement::Empty,
        Rule::Label => parse_label(pair.into_inner()),
        Rule::Break => Statement::Break,
        Rule::Goto => parse_goto(pair.into_inner()),
        Rule::Do => parse_do(pair.into_inner()),
        Rule::While => parse_while(pair.into_inner()),
        Rule::Repeat => parse_repeat(pair.into_inner()),
        Rule::If => parse_if(pair.into_inner()),
        Rule::NumericalFor => parse_numerical_for(pair.into_inner()),
        Rule::GenericFor => parse_generic_for(pair.into_inner()),
        Rule::FunctionDefinition => parse_function_definition(pair.into_inner()),
        Rule::LocalFunctionDefinition => parse_local_function_definition(pair.into_inner()),
        Rule::Assignment => parse_assignment(pair.into_inner()),
        Rule::LocalAssignment => parse_local_assignment(pair.into_inner()),
        Rule::FunctionCall => parse_function_call(pair.into_inner()),
        _ => unreachable!("Expected statement, found {:?}", pair),
    }
}

fn parse_return_statement(mut pairs: Pairs<Rule>) -> Return {
    Return(
        pairs
            .next()
            .map(|x| x.into_inner().map(|y| parse_expr(y.into_inner())).collect()),
    )
}

fn parse_function_call(pairs: Pairs<Rule>) -> Statement {
    let mut prefix_exp = parse_prefix_expr(pairs);
    let last = prefix_exp.actions.pop().unwrap();
    let PExprAction::Call(call) = last else {
        unreachable!("Expected call suffix, found {:?}", last);
    };
    Statement::FunctionCall { prefix_exp, call }
}

fn parse_attribute_list(mut pairs: Pairs<Rule>) -> Vec<LocalVariable> {
    let mut variables = vec![];
    while let Some(name) = pairs.next() {
        let name = name.as_str().into();
        let attribute = pairs
            .next()
            .unwrap()
            .into_inner()
            .next()
            .map(|x| x.as_str().into());
        variables.push(LocalVariable { name, attribute })
    }
    variables
}

fn parse_local_assignment(mut pairs: Pairs<Rule>) -> Statement {
    let variables = parse_attribute_list(pairs.next().unwrap().into_inner());
    let expr_list = pairs
        .next()
        .map(|x| x.into_inner().map(|y| parse_expr(y.into_inner())).collect());
    Statement::LocalVariables {
        variables,
        expr_list,
    }
}

fn parse_variable(mut pairs: Pairs<Rule>) -> Variable {
    let pair = pairs.peek().unwrap();
    if pair.as_rule() == Rule::Name {
        pairs.next();
        return Variable::Name(pair.as_str().into());
    }
    let mut prefix_expr = parse_prefix_expr(pairs);
    let last = prefix_expr.actions.pop().unwrap();
    let PExprAction::Selector(selector) = last else {
        unreachable!("Expected selector, found {:?}", last);
    };
    Variable::Selector {
        prefix_expr,
        selector,
    }
}

fn parse_assignment(mut pairs: Pairs<Rule>) -> Statement {
    let variable_list = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|x| parse_variable(x.into_inner()))
        .collect();
    let expr_list = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|x| parse_expr(x.into_inner()))
        .collect();
    Statement::Assignment {
        variable_list,
        expr_list,
    }
}

fn parse_function_name(pairs: Pairs<Rule>) -> FunctionName {
    let mut names = vec![];
    let mut method = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Name => names.push(pair.as_str().into()),
            Rule::MethodName => method = Some(pair.into_inner().next().unwrap().as_str().into()),
            _ => unreachable!("Expected name, found {:?}", pair),
        }
    }
    FunctionName { names, method }
}

fn parse_parameters(mut pairs: Pairs<Rule>) -> Parameters {
    let pair = pairs.next().unwrap();
    if pair.as_rule() == Rule::VarArg {
        return Parameters {
            name_list: vec![],
            var_arg: true,
        };
    }
    let name_list = pair.into_inner().map(|x| x.as_str().into()).collect();
    let var_arg = pairs.next().is_some();
    Parameters { name_list, var_arg }
}

pub fn parse_function_body(mut pairs: Pairs<Rule>) -> (Option<Parameters>, Block) {
    match pairs.peek().unwrap().as_rule() {
        Rule::Block => {
            let block = build_ast(&mut pairs);
            return (None, block);
        }
        _ => {}
    }
    let parameters = parse_parameters(pairs.next().unwrap().into_inner());
    let block = build_ast(&mut pairs);
    (Some(parameters), block)
}

fn parse_local_function_definition(mut pairs: Pairs<Rule>) -> Statement {
    let name = pairs.next().unwrap().as_str().into();
    let (parameters, body) = parse_function_body(pairs.next().unwrap().into_inner());
    Statement::LocalFunctionDefinition {
        name,
        parameters,
        body,
    }
}

fn parse_function_definition(mut pairs: Pairs<Rule>) -> Statement {
    let function_name = parse_function_name(pairs.next().unwrap().into_inner());
    let (parameters, body) = parse_function_body(pairs.next().unwrap().into_inner());
    Statement::FunctionDefinition {
        function_name,
        parameters,
        body,
    }
}

fn parse_if(mut pairs: Pairs<Rule>) -> Statement {
    let mut ifs = vec![];
    let mut r#else = None;
    loop {
        let Some(first) = pairs.peek() else {
            break;
        };
        if first.as_rule() == Rule::Block {
            r#else = Some(build_ast(&mut pairs));
            break;
        }
        let first = pairs.next().unwrap();
        let condition = parse_expr(first.into_inner());
        let block = build_ast(&mut pairs);
        ifs.push(If { condition, block });
    }
    Statement::If { ifs, r#else }
}

fn parse_generic_for(mut pairs: Pairs<Rule>) -> Statement {
    let variables = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|x| x.as_str().into())
        .collect();
    let expr_list = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|x| parse_expr(x.into_inner()))
        .collect();
    let block = build_ast(&mut pairs);
    Statement::GenericFor {
        variables,
        expr_list,
        block,
    }
}

fn parse_numerical_for(mut pairs: Pairs<Rule>) -> Statement {
    let control = pairs.next().unwrap().as_str().into();
    let initial = parse_expr(pairs.next().unwrap().into_inner());
    let limit = parse_expr(pairs.next().unwrap().into_inner());
    let mut step = None;
    match pairs.peek().unwrap().as_rule() {
        Rule::Expression => step = Some(parse_expr(pairs.next().unwrap().into_inner())),
        _ => {}
    }
    let block = build_ast(&mut pairs);
    Statement::NumericalFor {
        control,
        initial,
        limit,
        step,
        block,
    }
}

fn parse_repeat(mut pairs: Pairs<Rule>) -> Statement {
    let block = build_ast(&mut pairs);
    let condition = pairs.next().unwrap();
    let condition = parse_expr(condition.into_inner());
    Statement::Repeat { block, condition }
}

fn parse_while(mut pairs: Pairs<Rule>) -> Statement {
    let condition = pairs.next().unwrap();
    let condition = parse_expr(condition.into_inner());
    let block = build_ast(&mut pairs);
    Statement::While { condition, block }
}

fn parse_do(mut pairs: Pairs<Rule>) -> Statement {
    let block = build_ast(&mut pairs);
    Statement::Do(block)
}

fn parse_label(mut pairs: Pairs<Rule>) -> Statement {
    let name = pairs.next().unwrap().as_str().into();
    Statement::Label(name)
}

fn parse_goto(mut pairs: Pairs<Rule>) -> Statement {
    let name = pairs.next().unwrap().as_str().into();
    Statement::Goto(name)
}
