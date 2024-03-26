use crate::{
    parser::expression::Expression,
    parser::prefix_expression::{CallSuffix, PrefixExpression, Selector},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_statement: Option<Return>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            statements: vec![],
            return_statement: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return(pub Option<Vec<Expression>>);

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug, PartialEq, Clone)]
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
        ifs: Vec<If>,
        r#else: Option<Block>,
    },
    NumericalFor {
        control: String,
        initial: Expression,
        limit: Expression,
        step: Option<Expression>,
        block: Block,
    },
    GenericFor {
        variables: Vec<String>,
        expr_list: Vec<Expression>,
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
        expr_list: Option<Vec<Expression>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalVariable {
    pub name: String,
    pub attribute: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameters {
    pub name_list: Vec<String>,
    pub var_arg: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionName {
    pub names: Vec<String>,
    pub method: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
    Name(String),
    Selector {
        prefix_expr: PrefixExpression,
        selector: Selector,
    },
}
