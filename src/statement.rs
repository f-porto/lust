use crate::{
    expressions::Expression,
    prefix_expression::{CallSuffix, PrefixExpression, Selector},
};

#[derive(Debug)]
pub struct Block {
    statements: Vec<Statement>,
    return_statement: Option<Return>,
}

#[derive(Debug)]
pub struct Return(Vec<Expression>);

#[derive(Debug)]
pub enum Statement {
    Empty,
    Assignment {
        variable_list: Vec<Variable>,
        expr_list: Vec<Expression>,
    },
    FunctionCall {
        prefix_exp: PrefixExpression,
        call: CallSuffix,
    },
    Label(String),
    Break,
    Goto(String),
    Do(Block),
    While {
        condition: Expression,
        block: Block,
    },
    Repeat {
        block: Block,
        condition: Expression,
    },
    If {
        condition: Expression,
        block: Block,
        else_if: Vec<Block>,
        r#else: Option<Block>,
    },
    NumericalFor {
        control: String,
        initial: Expression,
        limit: Expression,
        step: Option<Expression>,
    },
    GenericFor {
        variables: Vec<String>,
        exp_list: Vec<Expression>,
        block: Block,
    },
    FunctionDefinition {
        function_name: FunctionName,
        parameters: Option<Parameters>,
        body: Block,
    },
    LocalFunctionDefinition {
        name: String,
        parameters: Option<Parameters>,
        body: Block,
    },
    LocalVariables {
        variables: Vec<LocalVariable>,
        esp_list: Option<Vec<Expression>>,
    },
    Return(Return),
}

#[derive(Debug)]
pub struct LocalVariable {
    name: String,
    attribute: Option<String>,
}

#[derive(Debug)]
pub struct Parameters {
    name_list: Vec<String>,
    var_arg: bool,
}

#[derive(Debug)]
pub struct FunctionName {
    names: Vec<String>,
    method: Option<String>,
}

#[derive(Debug)]
pub enum Variable {
    Name(String),
    Selector {
        prefix_exp: PrefixExpression,
        selector: Selector,
    },
}
